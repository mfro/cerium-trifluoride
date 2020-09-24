import os
import sys
import regex

from cef_parser import *
from argparse import ArgumentParser

# cannot be loaded as a module
if __name__ != "__main__":
  sys.stderr.write('This file cannot be loaded as a module!')
  sys.exit()

# parse command-line options
disc = """
This utility generates files for the rust to C API translation layer.
"""

parser = ArgumentParser(description=disc)
# parser.add_argument(
#     '--cef-dir',
#     dest='rootdir',
#     metavar='DIR',
#     help='CEF root directory. Can also be specified with CEF_ROOT environment variable [required]')
parser.add_argument(
    '--output-dir',
    dest='outputdir',
    metavar='DIR',
    help='output directory [required]')
# parser.add_argument(
#     '--mode',
#     dest='mode',
#     metavar='MODE',
#     help='autorun mode (cef or cef-sys) [optional]')
options = parser.parse_args()

structures = set()
classes = dict()
interfaces = dict()

cef_binding_ns = 'cef_sys'
cef_struct_ns = 'crate::include::internal'
cef_object_ns = 'crate::include'
cef_helpers_ns = 'crate::include::helpers'
cef_refcounting_ns = 'crate::include::refcounting'

class writer:
  def __init__(self):
    self.result = ''
    self.do_indent = False
    self.current_indent = 0

  def merge(self, other):
    self.write(other.result.replace('\n', '\n' + '  ' * self.current_indent))

  def indent(self, delta):
    self.current_indent += delta

  def write(self, content):
    if self.do_indent:
      self.result += '  ' * self.current_indent
      self.do_indent = False
    self.result += content

  def write_line(self, a = '', b = None):
    if b is None:
      self.write(a)
      self.write('\n')
      self.do_indent = True
    elif a < 0:
      self.indent(a)
      self.write(b)
      self.write('\n')
      self.do_indent = True
    else:
      self.write(b)
      self.write('\n')
      self.indent(a)
      self.do_indent = True

class type_base:
  def __init__(self):
    self.rust_default = 'Default::default()'

  def extern_to_rust_prepare(self, name):
    return None

  def extern_to_rust_restore(self, name):
    return None

  def extern_to_rust_generic(self):
    return None

  def extern_parameter_to_rust_parameter(self, name):
    return self.extern_to_rust(name)

  def rust_return_to_extern_return(self, name):
    return self.rust_to_extern(name)


  def rust_to_extern_prepare(self, name):
    return None

  def rust_to_extern_restore(self, name):
    return None

  def rust_to_extern_generic(self):
    return None

  def rust_parameter_to_extern_parameter(self, name):
    return self.rust_to_extern(name)

  def extern_return_to_rust_return(self, name):
    return self.extern_to_rust(name)

class type_refptr(type_base):
  def __init__(self, raw):
    super().__init__()

    c_name = get_c_type_name(raw[1])
    rust_name = get_rust_type_name(raw[1])

    self.rust_type = f'{cef_object_ns}::{rust_name}'
    self.extern_type = f'*mut {cef_binding_ns}::{c_name}'

  def extern_to_rust(self, name):
    return f'{self.rust_type}::from_cef_own({name}).unwrap()'

  def rust_to_extern_generic(self):
    return f'Into<{self.rust_type}>'

  def rust_to_extern(self, name):
    return f'{self.rust_type}::to_cef_own({name}.into())'

class type_refptr_option(type_base):
  def __init__(self, raw):
    super().__init__()

    self.inner = type_refptr(raw)

    self.rust_type = f'Option<{self.inner.rust_type}>'
    self.extern_type = self.inner.extern_type

  def extern_to_rust(self, name):
    return f'{self.inner.rust_type}::from_cef_own({name})'

  def rust_to_extern(self, name):
    return f'{name}.map_or(std::ptr::null_mut(), |o| {self.inner.rust_type}::to_cef_own(o))'

class type_ref_refptr(type_base):
  def __init__(self, raw):
    super().__init__()

    self.inner = type_refptr(raw)

    assert raw[0] == 'class'

    self.rust_type = f'&{self.inner.rust_type}'
    self.extern_type = f'{self.inner.extern_type}'

  def extern_parameter_to_rust_parameter(self, name):
    return f'&*({name} as *const _)'

  def rust_parameter_to_extern_parameter(self, name):
    return f'{name} as *const _ as *const _'

class type_mut_refptr(type_base):
  def __init__(self, raw):
    super().__init__()

    self.inner = type_refptr_option(raw)

    self.rust_type = f'&mut {self.inner.rust_type}'
    self.extern_type = f'*mut {self.inner.extern_type}'

  def extern_to_rust_prepare(self, name):
    return f'let mut {name}__tmp = {self.inner.extern_parameter_to_rust_parameter("*" + name)};'

  def extern_to_rust_restore(self, name):
    return f'*{name} = {self.inner.rust_return_to_extern_return(name + "__tmp")};'

  def extern_parameter_to_rust_parameter(self, name):
    return f'&mut {name}__tmp'

  def rust_to_extern_prepare(self, name):
    return f'let mut {name}__tmp = {self.inner.inner.rust_type}::to_cef_ref({name});'

  def rust_to_extern_restore(self, name):
    return f'*{name} = {self.inner.extern_parameter_to_rust_parameter(name + "__tmp")};'

  def rust_parameter_to_extern_parameter(self, name):
    return f'&mut {name}__tmp'

class type_rawptr(type_base):
  def __init__(self, raw):
    super().__init__()

    self.inner = type_refptr(raw)

    self.rust_type = self.inner.rust_type
    self.extern_type = self.inner.extern_type

  def extern_to_rust(self, name):
    return f'{self.rust_type}::from_cef_ref({name}).unwrap()'

  def rust_to_extern(self, name):
    return f'{self.rust_type}::to_cef_own({name})'

class type_string_userfree(type_base):
  def __init__(self):
    super().__init__()

    self.rust_type = f'{cef_struct_ns}::CefStringUserFree'
    self.extern_type = f'*mut {cef_binding_ns}::cef_string_t'

  def extern_return_to_rust_return(self, name):
    return f'{cef_struct_ns}::CefStringUserFree::from_cef({name}).unwrap()'

class type_struct(type_base):
  def __init__(self, raw_name):
    super().__init__()

    c_name = get_c_type_name(raw_name)
    rust_name = get_rust_type_name(raw_name)

    self.rust_type = f'{cef_struct_ns}::{rust_name}'
    self.extern_type = f'{cef_binding_ns}::{c_name}'

  def extern_to_rust(self, name):
    return f'{name}.into()'

  def rust_to_extern(self, name):
    return f'{name}.into()'

class type_ref_struct(type_base):
  def __init__(self, raw_name):
    super().__init__()

    self.inner = type_struct(raw_name)

    self.rust_type = f'&{self.inner.rust_type}'
    self.extern_type = f'*const {self.inner.extern_type}'

  def extern_parameter_to_rust_parameter(self, name):
    return f'&*({name} as *const _)'

  def rust_parameter_to_extern_parameter(self, name):
    return f'{name} as *const _ as *const _'

class type_ref_struct_option(type_base):
  def __init__(self, raw_name):
    super().__init__()

    self.inner = type_ref_struct(raw_name)

    self.rust_type = f'Option<{self.inner.rust_type}>'
    self.extern_type = f'{self.inner.extern_type}'

  def extern_parameter_to_rust_parameter(self, name):
    return f'if {name}.is_null() {{ None }} else {{ Some({self.inner.extern_parameter_to_rust_parameter(name)}) }}'

  def rust_parameter_to_extern_parameter(self, name):
    return f'match {name} {{ Some({name}) => {self.inner.rust_parameter_to_extern_parameter(name)}, None => std::ptr::null_mut() }}'

class type_mut_struct(type_base):
  def __init__(self, raw_name):
    super().__init__()

    self.inner = type_struct(raw_name)

    self.rust_type = f'&mut {self.inner.rust_type}'
    self.extern_type = f'*mut {self.inner.extern_type}'

  def extern_to_rust(self, name):
    return f'&mut *({name} as *mut _)'

  def rust_to_extern(self, name):
    return f'{name} as *mut _ as *mut _'

class type_ref_list_string(type_base):
  def __init__(self):
    super().__init__()

    self.rust_type = f'&{cef_struct_ns}::CefStringList'
    self.extern_type = f'{cef_binding_ns}::cef_string_list_t'

  def extern_to_rust(self, name):
    return f'&{name}.into()'

  def rust_to_extern(self, name):
    return f'{name}.into()'

class type_ref_list(type_base):
  def __init__(self, inner, size, size_first):
    super().__init__()

    if not type(inner) in [type_primitive, type_struct, type_refptr]:
      raise RuntimeError(f'lists are not supported: {inner.rust_type} {inner.extern_type}')

    self.inner = inner
    self.size = size
    self.size_first = size_first

    if self.inner.rust_type == 'std::os::raw::c_void':
      self.rust_type = '&[u8]'
    else:
      self.rust_type = f'&[{self.inner.rust_type}]'

    self.extern_type = [f'*const {self.inner.extern_type}', f'{self.size.rust_type}']
    if size_first: self.extern_type.reverse()

  def extern_to_rust(self, name):
    if self.size_first:
      ptr = name[1]
      size = name[0]
    else:
      ptr = name[0]
      size = name[1]
    return f'std::slice::from_raw_parts({ptr} as *const {self.rust_type[2:-1]}, {size} as usize)'

  def rust_to_extern(self, name):
    v = [f'{name}.as_ptr() as *const {self.inner.extern_type}', f'{name}.len() as {self.size.rust_type}']
    if self.size_first: v.reverse()
    return v

class type_mut_list(type_base):
  def __init__(self, inner, size, size_first):
    super().__init__()

    if not type(inner) in [type_primitive, type_struct, type_refptr]:
      raise RuntimeError(f'lists are not supported: {inner.rust_type} {inner.extern_type}')

    self.inner = inner
    self.size = size
    self.size_first = size_first

    if self.inner.rust_type == 'std::os::raw::c_void':
      self.rust_type = '&mut [u8]'
    else:
      self.rust_type = f'&mut [{self.inner.rust_type}]'

    self.extern_type = [f'*mut {self.inner.extern_type}', f'{self.size.rust_type}']

    if size_first: self.extern_type.reverse()

  def extern_to_rust(self, name):
    if self.size_first:
      ptr = name[1]
      size = name[0]
    else:
      ptr = name[0]
      size = name[1]
    return f'std::slice::from_raw_parts_mut({ptr} as *mut {self.rust_type[6:-1]}, {size} as usize)'

  def rust_to_extern(self, name):
    v = [f'{name}.as_mut_ptr() as *mut {self.inner.extern_type}', f'{name}.len() as {self.size.rust_type}']
    if self.size_first: v.reverse()
    return v

class type_unit(type_base):
  def __init__(self):
    super().__init__()

    self.rust_type = f'()'
    self.extern_type = f'()'

  def extern_to_rust(self, name):
    return f'{name}'

  def rust_to_extern(self, name):
    return f'{name}'

class type_mut_bool(type_base):
  def __init__(self):
    super().__init__()

    self.inner = type_bool()

    self.rust_type = f'&mut bool'
    self.extern_type = f'*mut std::os::raw::c_int'

  def extern_to_rust_prepare(self, name):
    return f'let mut {name}__tmp = {self.inner.extern_parameter_to_rust_parameter("*" + name)};'

  def extern_to_rust_restore(self, name):
    return f'*{name} = {self.inner.rust_return_to_extern_return(name + "__tmp")};'

  def extern_parameter_to_rust_parameter(self, name):
    return f'&mut {name}__tmp'

  def rust_to_extern_prepare(self, name):
    return f'let mut {name}__tmp = {self.inner.rust_parameter_to_extern_parameter("*" + name)};'

  def rust_to_extern_restore(self, name):
    return f'*{name} = {self.inner.extern_return_to_rust_return(name + "__tmp")};'

  def rust_parameter_to_extern_parameter(self, name):
    return f'&mut {name}__tmp'

class type_bool(type_base):
  def __init__(self):
    super().__init__()

    self.rust_type = f'bool'
    self.extern_type = f'std::os::raw::c_int'

  def extern_to_rust(self, name):
    return f'if {name} == 0 {{ false }} else {{ true }}'

  def rust_to_extern(self, name):
    return f'if {name} {{ 1 }} else {{ 0 }}'

class type_mut_primitive(type_base):
  def __init__(self, ty):
    super().__init__()

    self.inner = type_primitive(ty)

    self.rust_type = f'&mut {self.inner.rust_type}'
    self.extern_type = f'*mut {self.inner.extern_type}'

  def extern_to_rust(self, name):
    return f'&mut *{name}'

  def rust_to_extern(self, name):
    return f'{name}'

class type_mut_primitive_option(type_base):
  def __init__(self, ty):
    super().__init__()

    self.inner = type_primitive(ty)

    self.rust_type = f'Option<&mut {self.inner.rust_type}>'
    self.extern_type = f'*mut {self.inner.extern_type}'

  def extern_to_rust(self, name):
    return f'if {name}.is_null() {{ None }} else {{ Some(&mut *{name}) }}'

  def rust_to_extern(self, name):
    return f'match {name} {{ Some({name}) => {name}, None => std::ptr::null_mut() }}'

class type_primitive(type_base):
  def __init__(self, ty):
    super().__init__()

    self.rust_type = f'{ty}'
    self.extern_type = f'{ty}'

  def extern_to_rust(self, name):
    return f'{name}'

  def rust_to_extern(self, name):
    return f'{name}'

def generate_comment(writer, lines):
  for line in lines:
    if line is None: continue
    line = line.strip()
    if line == '/': continue
    writer.write_line(f'/// {line}')

def generate_types(writer, contents):
  fields = set()

  for match in regex.finditer(r'(\/\/(.*)\n)*(typedef enum {([^}]*)} (\w+);|typedef struct \w+ {([^}]*)} (\w+);)', contents):
    if match.group(5):
      body = match.group(4)
      extern_name = match.group(5)
      rust_name = map_identifier(get_rust_type_name(extern_name))
      label = regex.match(r'cef_(.*)_t', extern_name).group(1)

      structures.add(extern_name)

      variants = []
      for variant in regex.finditer(r'  ( *\/\/(.*)\n)* *(\w+)( =[^,]*)?,?', body):
        variant_name = map_identifier(variant.group(3))
        variants.append((variant_name, variant.captures(2)))

      names = [x[0].split('_') for x in variants]
      prefix = names[0][:-1]

      for name in names:
        for (i, (l, r)) in enumerate(zip(prefix, name)):
          if l != r:
            prefix = prefix[:i]
            break

      prefix = '_'.join(prefix) + '_'
      variants = [(map_identifier(x[0][len(prefix):]), *x) for x in variants]

      generate_comment(writer, match.captures(2))

      writer.write_line(f'#[repr(C)]')
      writer.write_line(f'#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]')
      writer.write_line(f'pub struct {rust_name}({cef_binding_ns}::{extern_name});')

      writer.write_line(+1, f'impl {rust_name} {{')

      for (rust_variant, c_variant, comment) in variants:
        generate_comment(writer, comment)
        writer.write_line(f'pub const {rust_variant}: {rust_name} = {rust_name}({cef_binding_ns}::{extern_name}_{c_variant});')

      writer.write_line(-1, f'}}')

      writer.write_line(+1, f'impl From<{cef_binding_ns}::{extern_name}> for {rust_name} {{')
      writer.write_line(+1, f'fn from(raw: {cef_binding_ns}::{extern_name}) -> {rust_name} {{')
      writer.write_line(f'{rust_name}(raw)')
      writer.write_line(-1, f'}}')
      writer.write_line(-1, f'}}')

      writer.write_line(+1, f'impl From<{rust_name}> for {cef_binding_ns}::{extern_name} {{')
      writer.write_line(+1, f'fn from(src: {rust_name}) -> {cef_binding_ns}::{extern_name} {{')
      writer.write_line(f'src.0')
      writer.write_line(-1, f'}}')
      writer.write_line(-1, f'}}')

      writer.write_line(+1, f'impl std::ops::BitOr for {rust_name} {{')
      writer.write_line(f'type Output = Self;')
      writer.write_line(+1, f'fn bitor(self, rhs: Self) -> Self::Output {{')
      writer.write_line(f'Self(self.0 | rhs.0)')
      writer.write_line(-1, f'}}')
      writer.write_line(-1, f'}}')

    elif match.group(7):
      body = match.group(6)
      extern_name = match.group(7)
      rust_name = map_identifier(get_rust_type_name(extern_name))
      label = regex.match(r'cef_(.*)_t', extern_name).group(1)

      structures.add(extern_name)

      generate_comment(writer, match.captures(2))

      writer.write_line(f'#[repr(C)]')
      writer.write_line(f'#[derive(Copy, Clone)]')
      writer.write_line(f'pub struct {rust_name} {{ pub raw: {cef_binding_ns}::{extern_name} }}')

      writer.write_line(+1, f'impl Default for {rust_name} {{')
      writer.write_line(+1, f'fn default() -> {rust_name} {{')
      writer.write_line(f'let /* mut */ raw = {cef_binding_ns}::{extern_name}::default();')
      writer.write_line(f'// raw.size = std::mem::size_of::<{cef_binding_ns}::{extern_name}>() as u64;')
      writer.write_line(f'{rust_name} {{ raw }}')
      writer.write_line(-1, f'}}')
      writer.write_line(-1, f'}}')

      writer.write_line(+1, f'impl From<{rust_name}> for {cef_binding_ns}::{extern_name} {{')
      writer.write_line(+1, f'fn from(src: {rust_name}) -> {cef_binding_ns}::{extern_name} {{')
      writer.write_line(f'src.raw')
      writer.write_line(-1, f'}}')
      writer.write_line(-1, f'}}')

      writer.write_line(+1, f'impl From<{cef_binding_ns}::{extern_name}> for {rust_name} {{')
      writer.write_line(+1, f'fn from(raw: {cef_binding_ns}::{extern_name}) -> {rust_name} {{')
      writer.write_line(f'{rust_name} {{ raw }}')
      writer.write_line(-1, f'}}')
      writer.write_line(-1, f'}}')

      writer.write_line(f'#[allow(non_snake_case)]')
      writer.write_line(+1, f'impl {rust_name} {{')

      for field in regex.finditer(r'  ( *\/\/(.*)\n)* *(\w+) (.*?);', body):
        field_name = map_identifier(field.group(4))
        field_extern_type = field.group(3)
        parsed = parse_type(field_extern_type)

        if parsed[0] == 'string':
          read = ('String', f'{cef_helpers_ns}::cef_string_to_string(&self.raw.{field_name})')
          write = ('&str', f'{cef_helpers_ns}::str_to_cef_string(&mut self.raw.{field_name}, v)')
        elif parsed[0] == 'primitive':
          read = (parsed[2], f'self.raw.{field_name}')
          write = (parsed[2], f'self.raw.{field_name} = v')
        else:
          read = (f'&{cef_struct_ns}::{parsed[1]}', f'unsafe {{ &*(self as *const _ as *const _) }}')
          write = (f'{cef_struct_ns}::{parsed[1]}', f'self.raw.{field_name} = v.into()')

        generate_comment(writer, field.captures(2))
        writer.write_line(f'pub fn {field_name}(&self) -> {read[0]} {{ {read[1]} }}')

        generate_comment(writer, field.captures(2))
        writer.write_line(f'pub fn set_{field_name}(&mut self, v: {write[0]}) -> &mut Self {{ {write[1]}; self }}')

      writer.write_line(f'pub fn build(&self) -> Self {{ Self {{ raw: self.raw }} }}')

      writer.write_line(-1, f'}}')

def generate_static_func(writer, func):
  if func.has_attrib('capi_name'):
    func_name = func.get_attrib('capi_name')
  else:
    func_name = get_c_func_name(func.get_name())

  extern_name = func.get_capi_name()

  typedefs = []
  if hasattr(func, 'parent'): typedefs.extend(func.parent.get_typedefs())
  if hasattr(func.parent, 'parent'): typedefs.extend(func.parent.parent.get_typedefs())

  arguments = func.get_arguments()
  arg_types = [parse_type(x.get_type(), typedefs) for x in arguments]

  translations = []

  while len(arg_types) > 0:
    arg = arguments[-len(arg_types)]
    translated = translate_arguments(arg_types, is_optional(arg, func), func)
    translations.append((map_identifier(arg.get_name()), translated))

  rettype = parse_type(func.get_retval().get_type(), typedefs)
  if rettype[0] == 'primitive' and rettype[1] == 'void':
    retarg = type_unit()
  else:
    retarg = translate_type(rettype, True)

  generate_comment(writer, func.get_comment())
  writer.write_line(f'#[allow(non_snake_case)]')
  writer.write(f'pub fn {func_name}<')

  label = 0
  for (name, arg) in translations:
    if not arg.rust_to_extern_generic(): continue
    writer.write(f'T{label}: {arg.rust_to_extern_generic()}, ')
    label += 1

  writer.write(f'>(')

  label = 0
  for (name, arg) in translations:
    writer.write(f'{name}: ')
    if arg.rust_to_extern_generic():
      writer.write(f'T{label}, ')
      label += 1
    else:
      writer.write(f'{arg.rust_type}, ')

  writer.write_line(+1, f') -> {retarg.rust_type} {{')
  writer.write_line(+1, 'unsafe {')

  for (name, arg) in translations:
    if not arg.rust_to_extern_prepare(name): continue
    if isinstance(arg.rust_to_extern_prepare(name), str):
      writer.write_line(f'{arg.rust_to_extern_prepare(name)}')
    else:
      label = 0
      for mapping in arg.rust_to_extern_prepare(name):
        writer.write_line(f'{mapping}')
        label += 1

  writer.write(f'let ret = {cef_binding_ns}::{extern_name}(')

  for (name, arg) in translations:
    if isinstance(arg.extern_type, str):
      writer.write(f'{arg.rust_parameter_to_extern_parameter(name)},')
    else:
      for mapping in arg.rust_parameter_to_extern_parameter(name):
        writer.write(f'{mapping},')

  writer.write_line(');')

  for (name, arg) in translations:
    if not arg.rust_to_extern_restore(name): continue
    if isinstance(arg.rust_to_extern_restore(name), str):
      writer.write_line(f'{arg.rust_to_extern_restore(name)}')
    else:
      label = 0
      for mapping in arg.rust_to_extern_restore(name):
        writer.write_line(f'{mapping}')
        label += 1

  if retarg.extern_return_to_rust_return('ret'):
    writer.write_line(f'{retarg.extern_return_to_rust_return("ret")}')
  else:
    writer.write_line('ret')

  writer.write_line(-1, f'}}')
  writer.write_line(-1, f'}}')

def generate_class(writer, cls):
  c_name = get_c_type_name(cls.get_name())

  writer.write_line(f'pub type {cls.get_name()} = {cef_refcounting_ns}::CefProxy<{cef_binding_ns}::{c_name}>;')
  writer.write_line(f'#[allow(non_snake_case)]')
  writer.write_line(+1, f'impl {cls.get_name()} {{')

  for func in cls.get_static_funcs():
    try:
      generate_static_func(writer, func)
    except RuntimeError as e:
      print(f'skipping {cls.get_name()}::{func.get_name()}: {e}')

  node = cls
  chain = [node]
  while node.get_parent_name() in classes:
    node = classes[node.get_parent_name()]
    chain.insert(0, node)

  for (depth, node) in enumerate(chain):
    for func in node.get_virtual_funcs():
      try:
        generate_class_func(writer, func, len(chain) - depth - 1)
      except RuntimeError as e:
        print(f'skipping {cls.get_name()}::{func.get_name()}: {e}')

  writer.write_line(-1, f'}}')

  for node in chain[:-1]:
    writer.write_line(+1, f'impl From<{cls.get_name()}> for {cef_object_ns}::{node.get_name()} {{')
    writer.write_line(+1, f'fn from(src: {cls.get_name()}) -> {cef_object_ns}::{node.get_name()} {{')
    writer.write_line(f'{cef_object_ns}::{node.get_name()} {{ raw: src.raw.cast() }}')
    writer.write_line(-1, f'}}')
    writer.write_line(-1, f'}}')

  return ''

def generate_class_func(writer, func, inheritence_depth):
  if func.has_attrib('capi_name'):
    func_name = func.get_attrib('capi_name')
  else:
    func_name = get_c_func_name(func.get_name())

  typedefs = [*func.parent.get_typedefs(), *func.parent.parent.get_typedefs()]
  arguments = func.get_arguments()
  arg_types = [parse_type(x.get_type(), typedefs) for x in arguments]

  translations = []

  while len(arg_types) > 0:
    arg = arguments[-len(arg_types)]
    translated = translate_arguments(arg_types, is_optional(arg, func), func)
    translations.append((map_identifier(arg.get_name()), translated))

  rettype = parse_type(func.get_retval().get_type(), typedefs)
  if rettype[0] == 'primitive' and rettype[1] == 'void':
    retarg = type_unit()
  else:
    retarg = translate_type(rettype, True)

  generate_comment(writer, func.get_comment())
  writer.write_line(f'#[allow(non_snake_case)]')
  writer.write(f'pub fn {func_name}<')

  label = 0
  for (name, arg) in translations:
    if not arg.rust_to_extern_generic(): continue
    writer.write(f'T{label}: {arg.rust_to_extern_generic()}, ')
    label += 1

  writer.write(f'>(&self')

  label = 0
  for (name, arg) in translations:
    writer.write(f', {name}: ')
    if arg.rust_to_extern_generic():
      writer.write(f'T{label}')
      label += 1
    else:
      writer.write(f'{arg.rust_type}')

  writer.write_line(+1, f') -> {retarg.rust_type} {{')
  writer.write_line(+1, 'unsafe {')

  for (name, arg) in translations:
    if not arg.rust_to_extern_prepare(name): continue
    if isinstance(arg.rust_to_extern_prepare(name), str):
      writer.write_line(f'{arg.rust_to_extern_prepare(name)}')
    else:
      label = 0
      for mapping in arg.rust_to_extern_prepare(name):
        writer.write_line(f'{mapping}')
        label += 1

  writer.write(f'let ret = match self.raw.as_ref()')
  for _ in range(0, inheritence_depth):
    writer.write(f'.base')
  writer.write_line(+1, f'.{func_name} {{')

  if inheritence_depth == 0:
    writer.write(f'Some(f) => f(self.raw.as_ptr(), ')
  else:
    writer.write(f'Some(f) => f(&self.raw.as_ref()')
    for _ in range(0, inheritence_depth):
      writer.write(f'.base')
    writer.write(' as *const _ as *mut _, ')

  for (name, arg) in translations:
    if isinstance(arg.extern_type, str):
      writer.write(f'{arg.rust_parameter_to_extern_parameter(name)},')
    else:
      for mapping in arg.rust_parameter_to_extern_parameter(name):
        writer.write(f'{mapping},')

  writer.write_line(f'),')
  writer.write_line(f'None => panic!(),')

  writer.write_line(-1, '};')

  for (name, arg) in translations:
    if not arg.rust_to_extern_restore(name): continue
    if isinstance(arg.rust_to_extern_restore(name), str):
      writer.write_line(f'{arg.rust_to_extern_restore(name)}')
    else:
      label = 0
      for mapping in arg.rust_to_extern_restore(name):
        writer.write_line(f'{mapping}')
        label += 1

  if retarg.extern_return_to_rust_return('ret'):
    writer.write_line(f'{retarg.extern_return_to_rust_return("ret")}')
  else:
    writer.write_line('ret')

  writer.write_line(-1, f'}}')
  writer.write_line(-1, f'}}')

  pass

def generate_interface(main_writer, cls):
  for line in cls.get_comment():
    line = line.strip()
    if line == '/': continue
    main_writer.write_line('/// ' + line)

  object_name = cls.get_name()
  trait_name = object_name.replace('Cef', '')
  c_name = get_c_type_name(object_name)

  main_writer.write_line(f'pub type {object_name} = {cef_refcounting_ns}::CefObject<dyn {trait_name}, {cef_binding_ns}::{c_name}>;')

  main_writer.write_line(+1, f'impl {object_name} {{')
  main_writer.write_line(+1, f'pub unsafe fn from_cef(ptr: *mut {cef_binding_ns}::{c_name}, self_ref: bool) -> {object_name} {{')
  main_writer.write_line(f'let shim = translate_cef_ptr!(ptr);')
  # CEF does not increment reference counting when passing a structure to its own member function
  # (ie for `this` pointer), so from_cef can be called with the reference already counted or not.
  # this parameter is necessary but this solution involves double counting the `this` reference
  # for the duration of the member function.
  main_writer.write_line(+1, f'if self_ref {{')
  main_writer.write_line(f'shim.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);')
  main_writer.write_line(-1, f'}}')
  # relies on the layout of `&dyn Trait` as described here:
  # https://rust-lang.github.io/unsafe-code-guidelines/layout/pointers.html#notes
  main_writer.write_line(+1, f'let fat_pointer = {cef_refcounting_ns}::DynObject {{')
  main_writer.write_line(f'data: shim as *const _ as *const u8,')
  main_writer.write_line(f'vtable: shim.vtable,')
  main_writer.write_line(-1, f'}};')
  main_writer.write_line(f'let ptr = std::mem::transmute(fat_pointer);')
  main_writer.write_line(f'let ptr = std::ptr::NonNull::new(ptr).unwrap();')
  main_writer.write_line(f'{object_name} {{ ptr }}')
  main_writer.write_line(-1, f'}}')

  main_writer.write_line(+1, f'pub fn new<T: {trait_name} + \'static>(value: T) -> {object_name} {{')
  # call the init_ref_count fn to initialize the 'base' field of our struct.
  # The 'init_ref_count' fn is preferable to initializing that field directly
  # in this macro as that would require increasing the visibility on the extern
  # fns at the top of this file.
  main_writer.write_line(f'let base = {cef_refcounting_ns}::init_ref_count::<{cef_binding_ns}::{c_name}>();')

  node = cls
  chain = [node]
  while node.get_parent_name() in interfaces:
    node = interfaces[node.get_parent_name()]
    chain.insert(0, node)

  # Initialize the cef vtable struct inheritence chain
  for node in chain:
    main_writer.write_line(+1, f'let base = {cef_binding_ns}::{get_c_type_name(node.get_name())} {{')
    main_writer.write_line(f'base,')

    for func in node.get_virtual_funcs():
      if func.has_attrib('capi_name'):
        func_name = func.get_attrib('capi_name')
      else:
        func_name = get_c_func_name(func.get_name())

      try:
        generate_interface_func(writer(), writer(), func, cls, 0)
      except RuntimeError as e:
        main_writer.write_line(f'{func_name}: None,')
        continue

      main_writer.write_line(f'{func_name}: Some({c_name}_{func_name}),')

    main_writer.write_line(-1, f'}};')

  # Create the CefObject which is where we put the actual
  # implemention, vtable, and refcount. refcount starts at
  # 1 and is incremented whenever the CefObjectImpl is cloned or
  # add_ref is called from CEF code.
  #
  # Safety here relies on CEF refcounting appropriately as well as
  # the rust refcounting implementation.
  main_writer.write_line(+1, f'let wrapper: Box<{cef_refcounting_ns}::CefObjectImplInner<dyn {trait_name}, {cef_binding_ns}::{c_name}>> = Box::new({cef_refcounting_ns}::CefObjectImplInner {{')
  main_writer.write_line(f'count: std::sync::atomic::AtomicUsize::new(1),')
  main_writer.write_line(f'raw: base,')
  main_writer.write_line(f'vtable: std::ptr::null_mut(),')
  main_writer.write_line(f'value: std::sync::Mutex::new(value),')
  main_writer.write_line(-1, f'}});')

  main_writer.write_line(f'let ptr = Box::into_raw(wrapper);')
  main_writer.write_line(+1, f'unsafe {{')
  # relies on the layout of `&dyn Trait` as described here:
  # https://rust-lang.github.io/unsafe-code-guidelines/layout/pointers.html#notes
  main_writer.write_line(f'let x = &ptr as *const _ as *const {cef_refcounting_ns}::DynObject;')
  main_writer.write_line(f'(*ptr).vtable = (*x).vtable;')
  main_writer.write_line(-1, f'}}')

  # println!("create {{}}: {{:?}}", stringify!({trait_name}), ptr);
  # leak the object from the box, we are now taking responsibility for keeping track of it.
  main_writer.write_line(f'let ptr = std::ptr::NonNull::new(ptr).unwrap();')
  main_writer.write_line(f'{cef_refcounting_ns}::CefObject {{ ptr }}')
  main_writer.write_line(-1, f'}}')
  main_writer.write_line(-1, f'}}')

  main_writer.write_line(+1, f'impl<T: {trait_name} + \'static> From<T> for {object_name} {{')
  main_writer.write_line(+1, f'fn from(src: T) -> {object_name} {{')
  main_writer.write_line(f'{object_name}::new(src)')
  main_writer.write_line(-1, f'}}')
  main_writer.write_line(-1, f'}}')

  unsafe_impl = writer()

  main_writer.write_line(f'#[allow(non_snake_case)]')
  main_writer.write_line(f'#[allow(unused_variables)]')
  main_writer.write_line(+1, f'pub trait {trait_name} {{')

  assert len(cls.get_static_funcs()) == 0

  # for func in cls.get_virtual_funcs():
  #   try:
  #     generate_interface_func(main_writer, unsafe_impl, func, 0)
  #   except RuntimeError as e:
  #     print(f'skipping {cls.get_name()}::{func.get_name()}: {e}')

  for node in chain:
    for func in node.get_virtual_funcs():
      try:
        generate_interface_func(main_writer, unsafe_impl, func, cls, 0)
      except RuntimeError as e:
        print(f'skipping {cls.get_name()}::{func.get_name()}: ({node.get_name()}) {e}')

  # depth = 0
  # node = cls
  # while node.get_parent_name() in classes:
  #   depth += 1
  #   node = classes[node.get_parent_name()]

  #   for func in node.get_virtual_funcs():
  #     try:
  #       generate_interface_func(main_writer, unsafe_impl, macro_impl, func, depth)
  #     except RuntimeError as e:
  #       print(f'skipping {cls.get_name()}::{func.get_name()}: {e}')

  main_writer.write_line(-1, f'}}')

  main_writer.merge(unsafe_impl)

def generate_interface_func(trait_impl, unsafe_impl, func, cls, inheritence_depth):
  iface_name = get_c_type_name(cls.get_name())
  if func.has_attrib('capi_name'):
    func_name = func.get_attrib('capi_name')
  else:
    func_name = get_c_func_name(func.get_name())

  typedefs = [*func.parent.get_typedefs(), *func.parent.parent.get_typedefs()]
  arguments = func.get_arguments()
  arg_types = [parse_type(x.get_type(), typedefs) for x in arguments]

  translations = []

  while len(arg_types) > 0:
    arg = arguments[-len(arg_types)]
    translated = translate_arguments(arg_types, is_optional(arg, func), func)
    if translated:
      translations.append((map_identifier(arg.get_name()), translated))

  thisarg = translate_type(('refptr', ('interface', func.parent.get_name())))
  rettype = parse_type(func.get_retval().get_type(), typedefs)
  if rettype[0] == 'primitive' and rettype[1] == 'void':
    retarg = type_unit()
  else:
    retarg = translate_type(rettype, True)

  generate_comment(trait_impl, func.get_comment())
  trait_impl.write(f'fn {func_name}(&mut self')

  for (name, arg) in translations:
    if isinstance(arg.rust_type, str):
      trait_impl.write(f', {name}: {arg.rust_type}')
    else:
      label = 0
      for extern_parameter in arg.rust_type:
        trait_impl.write(f', {name}{label}: {extern_parameter}')
        label += 1

  trait_impl.write_line(f') -> {retarg.rust_type} {{ {retarg.rust_default} }}')

  unsafe_impl.write_line(f'#[allow(non_snake_case)]')
  unsafe_impl.write(f'unsafe extern "C" fn {iface_name}_{func_name}(')
  unsafe_impl.write(f'_self: {thisarg.extern_type}')

  for (name, arg) in translations:
    if isinstance(arg.extern_type, str):
      unsafe_impl.write(f', {name}: {arg.extern_type}')
    else:
      label = 0
      for extern_parameter in arg.extern_type:
        unsafe_impl.write(f', {name}{label}: {extern_parameter}')
        label += 1

  unsafe_impl.write_line(+1, f') -> {retarg.extern_type} {{')

  for (name, arg) in translations:
    if not arg.extern_to_rust_prepare(name): continue
    if isinstance(arg.extern_to_rust_prepare(name), str):
      unsafe_impl.write_line(f'{arg.extern_to_rust_prepare(name)}')
    else:
      label = 0
      for mapping in arg.extern_to_rust_prepare(name):
        unsafe_impl.write_line(f'{mapping}')
        label += 1

  unsafe_impl.write(f'let ret = {cls.get_name()}::from_cef(_self as _, true).get().{func_name}(')

  for (name, arg) in translations:
    if isinstance(arg.extern_type, str):
      unsafe_impl.write(f'{arg.extern_parameter_to_rust_parameter(name)},')
    else:
      names = [f'{name}{i}' for i in range(0, len(arg.extern_type))]
      unsafe_impl.write(f'{arg.extern_parameter_to_rust_parameter(names)},')

  unsafe_impl.write_line(');')

  for (name, arg) in translations:
    if not arg.extern_to_rust_restore(name): continue
    if isinstance(arg.extern_to_rust_restore(name), str):
      unsafe_impl.write_line(f'{arg.extern_to_rust_restore(name)}')
    else:
      label = 0
      for mapping in arg.extern_to_rust_restore(name):
        unsafe_impl.write_line(f'{mapping}')
        label += 1

  unsafe_impl.write_line(f'{retarg.rust_return_to_extern_return("ret")}')

  unsafe_impl.write_line(-1, f'}}')

def translate_arguments(queue, optional, func):
  ty = queue.pop(0)

  def can_slice(element):
    if element[0] == 'primitive' and element[2] != 'bool':
      return True

    if element[0] == 'struct':
      return True

    return False

  def can_slice_index(element):
    return element[0] == 'primitive' and element[1] in ['size_t', 'int']

  # &mut [_] - primitive slice
  if ty[0] == 'ptr' and can_slice(ty[1]) and len(queue) > 0 and can_slice_index(queue[0]):
    size_ty = queue.pop(0)

    # print(f'primitive slice mut: {func.get_name()} {ty[1]}[{size_ty}]')

    return type_mut_list(translate_type(ty[1], False), translate_type(size_ty, False), False)

  # &[_] - primitive slice
  if ty[0] == 'ptr' and ty[1][0] == 'const' and can_slice(ty[1][1]) and len(queue) > 0 and can_slice_index(queue[0]):
    size_ty = queue.pop(0)

    # print(f'primitive slice: {func.get_name()} {ty[1][1]}[{size_ty}]')

    return type_ref_list(translate_type(ty[1][1], False), translate_type(size_ty, False), False)

  # &mut [_] - primitive slice
  if can_slice_index(ty) and len(queue) > 0 and queue[0][0] == 'ptr' and can_slice(queue[0][1]):
    element_ty = queue.pop(0)[1]

    # print(f'primitive slice mut B: {func.get_name()} {element_ty}[{ty}]')

    return type_mut_list(translate_type(element_ty, False), translate_type(ty, False), True)

  # &[_] - primitive slice
  if can_slice_index(ty) and len(queue) > 0 and queue[0][0] == 'ptr' and queue[0][1][0] == 'const' and can_slice(queue[0][1][1]):
    element_ty = queue.pop(0)[1][1]

    # print(f'primitive slice B: {func.get_name()} {element_ty}[{ty}]')

    return type_ref_list(translate_type(element_ty, False), translate_type(ty, False), True)

  else:
    return translate_type(ty, optional)

def translate_type(ty, optional = False):
  def is_ref_or_ptr(ty):
    return ty[0] in ['ref', 'ptr']

  # CefObject
  if ty[0] == 'refptr':
    if ty[1][1] == 'CefBaseRefCounted':
      raise RuntimeError(f'unsupported type: {ty[1]}')

    if optional: return type_refptr_option(ty[1])
    return type_refptr(ty[1])

  # ?? &mut CefObject
  elif is_ref_or_ptr(ty) and ty[1][0] == 'refptr':
    # if optional: print(f'optional {ty}')
    return type_mut_refptr(ty[1][1])

  # ?? &CefObject
  elif ty[0] == 'const' and ty[1][0] == 'refptr':
    # if optional: print(f'optional {ty}')
    return type_refptr(ty[1][1])

  # ?? &mut CefObject
  elif ty[0] == 'rawptr':
    # if optional: print(f'optional {ty}')
    return type_rawptr(ty[1])

  # CefString
  elif ty[0] == 'string':
    # if optional: print(f'optional {ty}')
    return type_string_userfree()

  # &mut CefString
  elif is_ref_or_ptr(ty) and ty[1][0] == 'string':
    # if optional: print(f'optional {ty}')
    return type_mut_struct('CefString')

  # &CefString
  elif is_ref_or_ptr(ty) and ty[1][0] == 'const' and ty[1][1][0] == 'string':
    if optional: return type_ref_struct_option('CefString')
    return type_ref_struct('CefString')

  # CefStruct
  elif ty[0] == 'struct':
    # if optional: print(f'optional {ty}')
    return type_struct(ty[1])

  # &mut CefStruct
  elif is_ref_or_ptr(ty) and ty[1][0] == 'struct':
    # if optional: print(f'optional {ty}')
    return type_mut_struct(ty[1][1])

  # &CefStruct
  elif is_ref_or_ptr(ty) and ty[1][0] == 'const' and ty[1][1][0] == 'struct':
    if optional: return type_ref_struct_option(ty[1][1][1])
    return type_ref_struct(ty[1][1][1])

  # &mut CefStringList or &mut [_]
    # elif is_ref_or_ptr(ty) and ty[1][0] == 'vector':
    #   if ty[1][1][0] == 'string':
    #     return type_translation(
    #       f'cef_string_list',
    #       f'&mut CefStringList',
    #       lambda name: f'{name}.into();',
    #     )

    #   else:
    #     element = translate_interface_argument([ty[1][1]])

    #     return type_translation(
    #       [f'*mut u64', f'*mut *mut {element.extern_definition}'],
    #       f'&[{element.rust_definition}]',
    #       lambda name1, name2: f'({name1}, {name2}).into()',
    #     )

  # &CefStringList or &[_]
  elif is_ref_or_ptr(ty) and ty[1][0] == 'const' and ty[1][1][0] == 'vector':
    if ty[1][1][1][0] == 'string':
      # if optional: print(f'optional {ty}')
      return type_ref_list_string()

    element = translate_type(ty[1][1][1])

    # if optional: print(f'optional {ty}')
    return type_ref_list(element, translate_type(('primitive', 'size_t', 'u64'), False), True)

  # &mut CefStringMap
    # elif is_ref_or_ptr(ty) and ty[1][0] == 'map':
    #   if ty[1][1][0] == 'string' and ty[1][2][0] == 'string':
    #     return type_translation(
    #       f'cef_string_map',
    #       lambda name: f'{name}.into();',
    #       f'&mut CefStringMap',
    #     )

    #   else:
    #     raise RuntimeError(f'unsupported map type {func.parent.get_name()}::{func.get_name()} {name}: {ty[1][1]}')

  # &CefStringMap
    # elif is_ref_or_ptr(ty) and ty[1][0] == 'const' and ty[1][1][0] == 'map':
    #   if ty[1][1][1][0] == 'string' and ty[1][1][2][0] == 'string':
    #     return type_translation(
    #       f'cef_string_map',
    #       lambda name: f'{name}.into();',
    #       f'&CefStringMap',
    #     )

    #   else:
    #     raise RuntimeError(f'unsupported map type {func.parent.get_name()}::{func.get_name()} {name}: {ty[1][1][1]}, {ty[1][1][2]}')

  # &mut CefStringMultimap
    # elif is_ref_or_ptr(ty) and ty[1][0] == 'multimap':
    #   if ty[1][1][0] == 'string' and ty[1][2][0] == 'string':
    #     return type_translation(
    #       f'cef_string_multimap',
    #       lambda name: f'{name}.into();',
    #       f'&mut CefStringMultimap',
    #     )

    #   else:
    #     raise RuntimeError(f'unsupported multimap type {func.parent.get_name()}::{func.get_name()} {name}: {ty[1][1]}')

  # &CefStringMultimap
    # elif is_ref_or_ptr(ty) and ty[1][0] == 'const' and ty[1][1][0] == 'multimap':
    #   if ty[1][1][1][0] == 'string' and ty[1][1][2][0] == 'string':
    #     return type_translation(
    #       f'cef_string_map',
    #       lambda name: f'{name}.into();',
    #       f'&CefStringMap',
    #     )

    #   else:
    #     raise RuntimeError(f'unsupported multimap type {func.parent.get_name()}::{func.get_name()} {name}: {ty[1][1][1]}, {ty[1][1][2]}')

  # &mut _ - primitive out bool
  elif is_ref_or_ptr(ty) and ty[1][0] == 'primitive' and ty[1][1] == 'bool':
    # if optional: print(f'optional {ty}')
    return type_mut_bool()

  # bool
  elif ty[0] == 'primitive' and ty[1] == 'bool':
    # if optional: print(f'optional {ty}')
    return type_bool()

  # # &mut _ - primitive out bool
  # elif is_ref_or_ptr(ty) and ty[1][0] == 'primitive' and ty[1][1] == 'void':
  #   return type_mut_primitive(ty[1][2])

  # # bool
  # elif ty[0] == 'primitive' and ty[1] == 'void':
  #   return type_primitive(ty[2])

  # &mut _ - primitive out parameter
  elif is_ref_or_ptr(ty) and ty[1][0] == 'primitive':
    if optional: return type_mut_primitive_option(ty[1][2])
    return type_mut_primitive(ty[1][2])

  # _ - primitive
  elif ty[0] == 'primitive':
    # if optional: print(f'optional {ty}')
    return type_primitive(ty[2])

  else:
    raise RuntimeError(f'unsupported type {ty}')

def parse_type(ty, typedefs = []):
  def parse_type(raw):
    raw = raw.strip()
    if raw in classes: return ('class', raw)
    if raw in interfaces: return ('interface', raw)
    if raw == 'CefString': return ('string', raw)
    if raw == 'cef_string_t': return ('string', raw)
    if raw == 'int': return ('primitive', raw, 'std::os::raw::c_int')
    if raw == 'int16': return ('primitive', raw, 'i16')
    if raw == 'int32': return ('primitive', raw, 'std::os::raw::c_int')
    if raw == 'int64': return ('primitive', raw, 'i64')
    if raw == 'uint16': return ('primitive', raw, 'u16')
    if raw == 'uint32': return ('primitive', raw, 'u32')
    if raw == 'uint64': return ('primitive', raw, 'u64')
    if raw == 'size_t': return ('primitive', raw, f'{cef_binding_ns}::size_t')
    if raw == 'char16': return ('primitive', raw, 'u16')
    if raw == 'bool': return ('primitive', raw, 'bool')
    if raw == 'char': return ('primitive', raw, 'u8')
    if raw == 'float': return ('primitive', raw, 'f32')
    if raw == 'double': return ('primitive', raw, 'f64')
    if raw == 'void': return ('primitive', raw, 'std::os::raw::c_void')

    typedef = next((x for x in typedefs if x.alias == raw), None)
    if typedef: return parse_type(typedef.get_value().get_value())

    match = regex.match(r'^CefRefPtr<(.*)>$', raw)
    if match: return ('refptr', parse_type(match.group(1)))

    match = regex.match(r'^CefRawPtr<(.*)>$', raw)
    if match: return ('rawptr', parse_type(match.group(1)))

    match = regex.match(r'^std::vector<(.*)>$', raw)
    if match: return ('vector', parse_type(match.group(1)))

    match = regex.match(r'^std::map<(.*),(.*)>$', raw)
    if match: return ('map', parse_type(match.group(1)), parse_type(match.group(2)))

    match = regex.match(r'^std::multimap<(.*),(.*)>$', raw)
    if match: return ('multimap', parse_type(match.group(1)), parse_type(match.group(2)))

    match = regex.match(r'^(.*)&$', raw)
    if match: return ('ref', parse_type(match.group(1)))

    match = regex.match(r'^(.*)\*$', raw)
    if match: return ('ptr', parse_type(match.group(1)))

    match = regex.match(r'^(.*) const$', raw) or regex.match(r'^const (.*)$', raw)
    if match: return ('const', parse_type(match.group(1)))

    match = regex.match(r'^cef.*$', raw, regex.IGNORECASE)
    if match:
      c_name = get_c_type_name(raw)
      rust_name = get_rust_type_name(raw)
      structures.add(c_name)
      return ('struct', rust_name)

    raise RuntimeError(f'unknown type: {raw}')

  if isinstance(ty, str):
    raw = ty
  elif ty.get_name():
    raw = ty.get_value()[:-len(ty.get_name())]
  else:
    raw = ty.get_type()

  return parse_type(raw)

def get_rust_type_name(name):
  if name[-2:] == '_t':
    name = name[:-2]
  out = ''
  upper = True
  for ch in name:
    if ch == '_':
      upper = True
    elif upper:
      out += ch.upper()
      upper = False
    else:
      out += ch
  return map_identifier(out)

def get_c_type_name(name):
  out = get_c_func_name(name)
  if not out.endswith('_t'):
    out += '_t'
  return map_identifier(out)

def get_c_func_name(name):
  was_upper = True
  out = ''
  for ch in name:
    if ch.isupper():
      if was_upper:
        out += ch.lower()
      else:
        out += '_' + ch.lower()
      was_upper = True
    elif ch.isdigit():
      out += ch
      was_upper = True
    else:
      out += ch
      was_upper = False
  return map_identifier(out)

def map_identifier(ident):
  if ident == 'continue': return 'cont'
  if ident == 'type': return 'type_'
  if ident == 'CefUrlparts': return 'CefURLParts'
  if len(ident) > 0 and ident[0].isdigit(): return 'X' + ident
  return ident

def is_optional(arg, func):
  optional = func.get_attrib_list('optional_param')
  return optional and arg.get_name() in optional

if options.outputdir is None:
  parser.print_help(sys.stdout)
  sys.exit()

# determine the paths
root_dir = os.path.abspath(os.environ.get("CEF_ROOT"))
cpp_header_dir = os.path.join(root_dir, 'include')
cpp_header_views_dir = os.path.join(cpp_header_dir, 'views')

output_dir = os.path.abspath(options.outputdir)

# make sure the header directory exists
if not os.path.exists(cpp_header_dir):
  sys.stderr.write('Directory ' + cpp_header_dir + ' does not exist.')
  sys.exit()

# create the header object
sys.stdout.write('Parsing C++ headers from ' + cpp_header_dir + '...\n')
header = obj_header()

# add include files to be processed
header.set_root_directory(cpp_header_dir)
excluded_files = ['cef_api_hash.h', 'cef_application_mac.h', 'cef_version.h']
header.add_directory(cpp_header_dir, excluded_files)
header.add_directory(cpp_header_views_dir)

for cls in header.get_classes():
  if cls.get_attrib('source') == 'library':
    classes[cls.get_name()] = cls
  else:
    interfaces[cls.get_name()] = cls

api_autogen = writer()

for filename in sorted(header.get_file_names()):
  modname = filename.replace('.h', '')

  for cls in header.get_classes(filename):
    if cls.get_attrib('source') == 'library':
      generate_class(api_autogen, cls)
    else:
      generate_interface(api_autogen, cls)

  for func in header.get_funcs(filename):
    try:
      generate_static_func(api_autogen, func)
    except RuntimeError as e:
      print(f'skipping {func.get_name()}: {e}')

with open(os.path.join(output_dir, 'api_autogen.rs'), 'w') as x:
  x.write(api_autogen.result)

internal_autogen = writer()

with open(os.path.join(cpp_header_dir, 'internal', 'cef_types.h')) as src:
  contents = src.read()
  generate_types(internal_autogen, contents)

with open(os.path.join(output_dir, 'internal_autogen.rs'), 'w') as x:
  x.write(internal_autogen.result)
