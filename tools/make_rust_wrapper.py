import os
import sys
import regex

from cef_parser import *
from optparse import OptionParser

# cannot be loaded as a module
if __name__ != "__main__":
  sys.stderr.write('This file cannot be loaded as a module!')
  sys.exit()

# parse command-line options
disc = """
This utility generates files for the rust to C API translation layer.
"""

parser = OptionParser(description=disc)
parser.add_option(
    '--cef-dir',
    dest='rootdir',
    metavar='DIR',
    help='CEF root directory. Can also be specified with CEF_PATH environment variable [required]')
parser.add_option(
    '--output-dir',
    dest='outputdir',
    metavar='DIR',
    help='output directory [required]')
(options, args) = parser.parse_args()

structures = set()
classes = set()
interfaces = set()

cef_binding_ns = 'cef_sys'
cef_struct_ns = 'crate::include::internal'
cef_object_ns = 'crate::include'
cef_wrapper_ns = 'crate::include::base'

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

  def write_line(self, content = ''):
    self.write(content)
    self.write('\n')
    self.do_indent = True

class type_translation_context:
  def __init__(self, extern_definition, rust_definition, convert, restore):
    self.extern_definition = extern_definition
    self.rust_definition = rust_definition
    self.convert = convert
    self.restore = restore

class type_translation:
  def __init__(self):
    self.iface_parameter = None
    self.iface_return = None
    self.class_parameter = None
    self.class_return = None

  def as_iface_parameter(self, extern_definition, rust_definition, convert, restore):
    self.iface_parameter = type_translation_context(extern_definition, rust_definition, convert, restore)
    return self
  def as_iface_return(self, extern_definition, rust_definition, convert, default = 'Default::default()'):
    self.iface_return = type_translation_context(extern_definition, rust_definition, convert, restore = None)
    self.iface_return_default = default
    return self
  def as_class_parameter(self, extern_definition, rust_definition, convert, restore):
    self.class_parameter = type_translation_context(extern_definition, rust_definition, convert, restore)
    return self
  def as_class_return(self, extern_definition, rust_definition, convert):
    self.class_return = type_translation_context(extern_definition, rust_definition, convert, restore = None)
    return self

class type_base:
  def __init__(self):
    self.rust_default = 'Default::default()'

  def extern_to_rust_prepare(self, name):
    return None

  def extern_to_rust_restore(self, name):
    return None

  def extern_parameter_to_rust_parameter(self, name):
    return self.extern_to_rust(name)

  def rust_return_to_extern_return(self, name):
    return self.rust_to_extern(name)


  def rust_to_extern_prepare(self, name):
    return None

  def rust_to_extern_restore(self, name):
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

  def rust_to_extern(self, name):
    return f'{self.rust_type}::to_cef_own({name})'

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

class type_mut_refptr(type_base):
  def __init__(self, raw):
    super().__init__()

    self.inner = type_refptr(raw)

    self.rust_type = f'&mut {self.inner.rust_type}'
    self.extern_type = f'*mut {self.inner.extern_type}'

  def extern_to_rust_prepare(self, name):
    return f'let mut {name}__tmp = {self.inner.extern_parameter_to_rust_parameter("*" + name)};'

  def extern_to_rust_restore(self, name):
    return f'*{name} = {self.inner.rust_return_to_extern_return(name + "__tmp")};'

  def extern_parameter_to_rust_parameter(self, name):
    return f'&mut {name}__tmp'

  def rust_to_extern_prepare(self, name):
    return f'let mut {name}__tmp = {self.inner.rust_type}::to_cef_ref({name});'

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

class type_string(type_base):
  def __init__(self):
    super().__init__()

    self.rust_type = f'{cef_struct_ns}::CefString'
    self.extern_type = f'*mut {cef_binding_ns}::cef_string_t'

  def extern_return_to_rust_return(self, name):
    return f'{cef_struct_ns}::CefString::userfree({name})'

class type_ref_string(type_base):
  def __init__(self):
    super().__init__()

    self.rust_type = f'&{cef_struct_ns}::CefString'
    self.extern_type = f'*const {cef_binding_ns}::cef_string_t'

  def extern_parameter_to_rust_parameter(self, name):
    return f'&{cef_struct_ns}::CefString::from_cef({name}).unwrap()'

  def rust_parameter_to_extern_parameter(self, name):
    return f'crate::include::internal::IntoCef::into_cef({name})'

class type_ref_string_option(type_base):
  def __init__(self):
    super().__init__()

    self.inner = type_ref_string()

    self.rust_type = f'Option<{self.inner.rust_type}>'
    self.extern_type = self.inner.extern_type

  def extern_parameter_to_rust_parameter(self, name):
    return f'match {self.inner.rust_type}::from_cef({name}) {{ Some(ref x) => Some(x), None => None }}'

  def rust_parameter_to_extern_parameter(self, name):
    return f'crate::include::internal::IntoCef::into_cef({name})'

class type_mut_string(type_base):
  def __init__(self):
    super().__init__()

    self.rust_type = f'&mut {cef_struct_ns}::CefString'
    self.extern_type = f'*mut {cef_binding_ns}::cef_string_t'

  def extern_parameter_to_rust_parameter(self, name):
    return f'&mut {cef_struct_ns}::CefString::from_cef({name}).unwrap()'

  def rust_parameter_to_extern_parameter(self, name):
    return f'crate::include::internal::IntoCef::into_cef({name})'

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

  def extern_to_rust(self, name):
    return f'&*({name} as *const _)'

  def rust_to_extern(self, name):
    return f'{name} as *const _ as *const _'

class type_ref_struct_option(type_base):
  def __init__(self, raw_name):
    super().__init__()

    self.inner = type_ref_struct(raw_name)

    self.rust_type = f'Option<{self.inner.rust_type}>'
    self.extern_type = f'{self.inner.extern_type}'

  def extern_to_rust(self, name):
    return f'if {name}.is_null() {{ None }} else {{ Some({self.inner.extern_to_rust(name)}) }}'

  def rust_to_extern(self, name):
    return f'match {name} {{ Some({name}) => {self.inner.rust_to_extern(name)}, None => std::ptr::null_mut() }}'

class type_mut_struct(type_base):
  def __init__(self, raw_name):
    super().__init__()

    self.inner = type_struct(raw_name)

    self.rust_type = f'&mut {self.inner.rust_type}'
    self.extern_type = f'*mut {self.inner.extern_type}'

  def extern_to_rust(self, name):
    return f'&mut *({name} as *mut _)'
    # sorry

  def rust_to_extern(self, name):
    return f'{name} as *mut _ as *mut _'

class type_ref_list_string(type_base):
  def __init__(self):
    super().__init__()

    self.inner = type_string()

    self.rust_type = f'&{cef_struct_ns}::CefStringList'
    self.extern_type = f'{cef_binding_ns}::cef_string_list_t'

  def extern_to_rust(self, name):
    return f'&{name}.into()'

  def rust_to_extern(self, name):
    return f'{name}.into()'

class type_ref_list(type_base):
  def __init__(self, inner):
    super().__init__()

    raise RuntimeError('lists are not supported')

    self.inner = inner

    self.rust_type = f'&[{self.inner.rust_type}]'
    self.extern_type = [f'u64', f'*const {self.inner.extern_type}']

  def extern_to_rust(self, name):
    return f'({name[0]}, {name[1]}).into()'

  def rust_to_extern(self, name):
    return [f'{name}.len() as u64', f'{name}.as_ptr()']

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
    self.extern_type = f'*mut i32'

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
    self.extern_type = f'i32'

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
  structures_copy = structures.copy()

  fields = set()

  for match in regex.finditer(r'(\/\/(.*)\n)*(typedef enum {([^}]*)} (\w+);|typedef struct \w+ {([^}]*)} (\w+);)', contents):
    if match.group(5):
      body = match.group(4)
      extern_name = match.group(5)
      rust_name = map_identifier(get_rust_type_name(extern_name))
      label = regex.match(r'cef_(.*)_t', extern_name).group(1)

      if extern_name in structures_copy: structures_copy.remove(extern_name)
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

      writer.write_line(f'impl {rust_name} {{')
      writer.indent(+1)

      for (rust_variant, c_variant, comment) in variants:
        generate_comment(writer, comment)
        writer.write_line(f'pub const {rust_variant}: {rust_name} = {rust_name}({cef_binding_ns}::{extern_name}_{c_variant});')

      writer.indent(-1)
      writer.write_line(f'}}')

      writer.write_line(f'impl From<{cef_binding_ns}::{extern_name}> for {rust_name} {{')
      writer.indent(+1)
      writer.write_line(f'fn from(raw: {cef_binding_ns}::{extern_name}) -> {rust_name} {{')
      writer.indent(+1)
      writer.write_line(f'{rust_name}(raw)')
      writer.indent(-1)
      writer.write_line(f'}}')
      writer.indent(-1)
      writer.write_line(f'}}')

      writer.write_line(f'impl From<{rust_name}> for {cef_binding_ns}::{extern_name} {{')
      writer.indent(+1)
      writer.write_line(f'fn from(src: {rust_name}) -> {cef_binding_ns}::{extern_name} {{')
      writer.indent(+1)
      writer.write_line(f'src.0')
      writer.indent(-1)
      writer.write_line(f'}}')
      writer.indent(-1)
      writer.write_line(f'}}')

    elif match.group(7):
      body = match.group(6)
      extern_name = match.group(7)
      rust_name = map_identifier(get_rust_type_name(extern_name))
      label = regex.match(r'cef_(.*)_t', extern_name).group(1)

      if extern_name in structures_copy: structures_copy.remove(extern_name)
      structures.add(extern_name)

      # writer.write(f'simple_struct!({rust_name}, {label}, ')
      generate_comment(writer, match.captures(2))

      writer.write_line(f'#[repr(C)]')
      writer.write_line(f'#[derive(Copy, Clone)]')
      writer.write_line(f'pub struct {rust_name} {{ pub raw: {cef_binding_ns}::{extern_name} }}')

      writer.write_line(f'impl Default for {rust_name} {{')
      writer.indent(+1)
      writer.write_line(f'fn default() -> {rust_name} {{')
      writer.indent(+1)
      writer.write_line(f'let /* mut */ raw = {cef_binding_ns}::{extern_name}::default();')
      writer.write_line(f'// raw.size = std::mem::size_of::<{cef_binding_ns}::{extern_name}>() as u64;')
      writer.write_line(f'{rust_name} {{ raw }}')
      writer.indent(-1)
      writer.write_line(f'}}')
      writer.indent(-1)
      writer.write_line(f'}}')

      writer.write_line(f'impl From<{rust_name}> for {cef_binding_ns}::{extern_name} {{')
      writer.indent(+1)
      writer.write_line(f'fn from(src: {rust_name}) -> {cef_binding_ns}::{extern_name} {{')
      writer.indent(+1)
      writer.write_line(f'src.raw')
      writer.indent(-1)
      writer.write_line(f'}}')
      writer.indent(-1)
      writer.write_line(f'}}')

      writer.write_line(f'impl From<{cef_binding_ns}::{extern_name}> for {rust_name} {{')
      writer.indent(+1)
      writer.write_line(f'fn from(raw: {cef_binding_ns}::{extern_name}) -> {rust_name} {{')
      writer.indent(+1)
      writer.write_line(f'{rust_name} {{ raw }}')
      writer.indent(-1)
      writer.write_line(f'}}')
      writer.indent(-1)
      writer.write_line(f'}}')

      writer.write_line(f'#[allow(non_snake_case)]')
      writer.write_line(f'impl {rust_name} {{')
      writer.indent(+1)

      for field in regex.finditer(r'  ( *\/\/(.*)\n)* *(\w+) (.*?);', body):
        field_name = map_identifier(field.group(4))
        field_extern_type = field.group(3)
        parsed = parse_type([], field_extern_type)

        if parsed[0] == 'string':
          read = ('String', f'super::super::cef_string_to_string(&self.raw.{field_name})')
          write = ('&str', f'super::super::str_to_cef_string(&mut self.raw.{field_name}, v)')
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

      writer.indent(-1)
      writer.write_line(f'}}')

  print(structures_copy)

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
  arg_types = [parse_type(typedefs, x.get_type()) for x in arguments]

  translations = []

  while len(arg_types) > 0:
    arg = arguments[-len(arg_types)]
    translated = translate_arguments(arg_types, is_optional(arg, func))
    translations.append((map_identifier(arg.get_name()), translated))

  rettype = parse_type(typedefs, func.get_retval().get_type())
  if rettype[0] == 'primitive' and rettype[1] == 'void':
    retarg = type_unit()
  else:
    retarg = translate_type(rettype, True)

  generate_comment(writer, func.get_comment())
  writer.write_line(f'#[allow(non_snake_case)]')
  writer.write(f'pub fn {func_name}(')

  for (name, arg) in translations:
    writer.write(f'{name}: {arg.rust_type}, ')

  writer.write_line(f') -> {retarg.rust_type} {{')
  writer.indent(+1)
  writer.write_line('unsafe {')
  writer.indent(+1)

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

  writer.indent(-1)
  writer.write_line(f'}}')
  writer.indent(-1)
  writer.write_line(f'}}')

def generate_class(writer, cls):
  c_name = get_c_type_name(cls.get_name())
  trait_name = cls.get_name().replace('Cef', '')

  writer.write_line(f'pub type {cls.get_name()} = {cef_wrapper_ns}::CefProxy<{cef_binding_ns}::{c_name}>;')
  writer.write_line(f'#[allow(non_snake_case)]')
  writer.write_line(f'impl {cls.get_name()} {{')
  writer.indent(+1)

  for func in cls.get_static_funcs():
    try:
      generate_static_func(writer, func)
    except RuntimeError as e:
      print(f'skipping {cls.get_name()}::{func.get_name()}: {e}')

  for func in cls.get_virtual_funcs():
    try:
      generate_class_func(writer, func)
    except RuntimeError as e:
      print(f'skipping {cls.get_name()}::{func.get_name()}: {e}')

  writer.indent(-1)
  writer.write_line(f'}}')

  return ''

def generate_class_func(writer, func):
  if func.has_attrib('capi_name'):
    func_name = func.get_attrib('capi_name')
  else:
    func_name = get_c_func_name(func.get_name())

  typedefs = [*func.parent.get_typedefs(), *func.parent.parent.get_typedefs()]
  arguments = func.get_arguments()
  arg_types = [parse_type(typedefs, x.get_type()) for x in arguments]

  translations = []

  while len(arg_types) > 0:
    arg = arguments[-len(arg_types)]
    translated = translate_arguments(arg_types, is_optional(arg, func))
    translations.append((map_identifier(arg.get_name()), translated))

  rettype = parse_type(typedefs, func.get_retval().get_type())
  if rettype[0] == 'primitive' and rettype[1] == 'void':
    retarg = type_unit()
  else:
    retarg = translate_type(rettype, True)

  generate_comment(writer, func.get_comment())
  writer.write(f'pub fn {func_name}(&mut self')

  for (name, arg) in translations:
    writer.write(f', {name}: {arg.rust_type}')

  writer.write_line(f') -> {retarg.rust_type} {{')
  writer.indent(+1)
  writer.write_line('unsafe {')
  writer.indent(+1)

  for (name, arg) in translations:
    if not arg.rust_to_extern_prepare(name): continue
    if isinstance(arg.rust_to_extern_prepare(name), str):
      writer.write_line(f'{arg.rust_to_extern_prepare(name)}')
    else:
      label = 0
      for mapping in arg.rust_to_extern_prepare(name):
        writer.write_line(f'{mapping}')
        label += 1

  writer.write_line(f'let ret = match self.raw.as_ref().{func_name} {{')
  writer.indent(+1)
  writer.write(f'Some(f) => f(self.raw.as_ptr(),')

  for (name, arg) in translations:
    if isinstance(arg.extern_type, str):
      writer.write(f'{arg.rust_parameter_to_extern_parameter(name)},')
    else:
      for mapping in arg.rust_parameter_to_extern_parameter(name):
        writer.write(f'{mapping},')

  writer.write_line(f'),')
  writer.write_line(f'None => panic!(),')

  writer.indent(-1)
  writer.write_line('};')

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

  writer.indent(-1)
  writer.write_line(f'}}')
  writer.indent(-1)
  writer.write_line(f'}}')

  pass

def generate_interface(main_writer, cls):
  for line in cls.get_comment():
    line = line.strip()
    if line == '/': continue
    main_writer.write_line('/// ' + line)

  object_name = cls.get_name()
  trait_name = object_name.replace('Cef', '')
  c_name = get_c_type_name(object_name)

  main_writer.write_line(f'#[allow(non_snake_case)]')
  main_writer.write_line(f'#[allow(unused_variables)]')
  main_writer.write_line(f'pub trait {trait_name} {{')
  main_writer.indent(+1)

  for func in cls.get_static_funcs():
    try:
      generate_static_func(main_writer, func)
    except RuntimeError as e:
      print(f'skipping {cls.get_name()}::{func.get_name()}: {e}')

  macro_impl = writer()
  macro_impl.write(f'define_refcounted!({trait_name}, {object_name}, {c_name}, ')

  unsafe_impl = writer()

  for func in cls.get_virtual_funcs():
    try:
      generate_interface_func(main_writer, unsafe_impl, macro_impl, func)
    except RuntimeError as e:
      print(f'skipping {cls.get_name()}::{func.get_name()}: {e}')

  macro_impl.write_line(');')

  main_writer.indent(-1)
  main_writer.write_line(f'}}')

  main_writer.merge(macro_impl)
  main_writer.merge(unsafe_impl)

def generate_interface_func(trait_impl, unsafe_impl, macro_impl, func):
  iface_name = get_c_type_name(func.parent.get_name())
  func_name = get_c_func_name(func.get_name())

  typedefs = [*func.parent.get_typedefs(), *func.parent.parent.get_typedefs()]
  arguments = func.get_arguments()
  arg_types = [parse_type(typedefs, x.get_type()) for x in arguments]

  translations = []

  while len(arg_types) > 0:
    arg = arguments[-len(arg_types)]
    translated = translate_arguments(arg_types, is_optional(arg, func))
    if translated:
      translations.append((map_identifier(arg.get_name()), translated))

  thisarg = translate_type(('refptr', ('interface', func.parent.get_name())))
  rettype = parse_type(func.parent.get_typedefs(), func.get_retval().get_type())
  if rettype[0] == 'primitive' and rettype[1] == 'void':
    retarg = type_unit()
  else:
    retarg = translate_type(rettype, True)

  macro_impl.write(f'{func_name}: {iface_name}_{func_name},')

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

  unsafe_impl.write_line(f') -> {retarg.extern_type} {{')
  unsafe_impl.indent(+1)

  for (name, arg) in translations:
    if not arg.extern_to_rust_prepare(name): continue
    if isinstance(arg.extern_to_rust_prepare(name), str):
      unsafe_impl.write_line(f'{arg.extern_to_rust_prepare(name)}')
    else:
      label = 0
      for mapping in arg.extern_to_rust_prepare(name):
        unsafe_impl.write_line(f'{mapping}')
        label += 1

  unsafe_impl.write(f'let ret = {func.parent.get_name()}::from_cef(_self, true).get().{func_name}(')

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

  unsafe_impl.indent(-1)
  unsafe_impl.write_line(f'}}')

def translate_arguments(queue, optional):
  ty = queue.pop(0)

  # &[_] - primitive slice
  if ty[0] == 'ptr' and ty[1][0] == 'const' and ty[1][1][0] == 'primitive' and len(queue) > 0 and queue[0][1][0] == 'primitive' and queue[0][1][2] == 'u64':
    (size_name, size_ty) = queue.pop()

    return type_ref_list(translate_type(ty[1][1], False))

  else:
    return translate_type(ty, optional)

def translate_type(ty, optional = False):
  def is_base(ty):
    return ty[0] in ['string', 'primitive', 'struct', 'object']

  def is_ref_or_ptr(ty):
    return ty[0] in ['ref', 'ptr']

  def is_primitive(ty):
    if ty[0] == 'primitive':
      return True
    if ty[0] in ['ref', 'ptr', 'const']:
      return is_primitive(ty[1])
    return False

  # CefObject
  if ty[0] == 'refptr':
    if ty[1][1] == 'CefBaseRefCounted':
      raise RuntimeError(f'unsupported type: {ty[1]}')

    if optional: return type_refptr_option(ty[1])
    return type_refptr(ty[1])

  # ?? &mut CefObject
  elif is_ref_or_ptr(ty) and ty[1][0] == 'refptr':
    if optional: print(f'optional {ty}')
    return type_mut_refptr(ty[1][1])

  # ?? &mut CefObject
  elif ty[0] == 'rawptr':
    if optional: print(f'optional {ty}')
    return type_rawptr(ty[1])

  # CefString
  elif ty[0] == 'string':
    if optional: print(f'optional {ty}')
    return type_string()

  # &mut CefString
  elif is_ref_or_ptr(ty) and ty[1][0] == 'string':
    if optional: print(f'optional {ty}')
    return type_mut_string()

  # &CefString
  elif is_ref_or_ptr(ty) and ty[1][0] == 'const' and ty[1][1][0] == 'string':
    if optional: return type_ref_string_option()
    return type_ref_string()

  # CefStruct
  elif ty[0] == 'struct':
    if optional: print(f'optional {ty}')
    return type_struct(ty[1])

  # &mut CefStruct
  elif is_ref_or_ptr(ty) and ty[1][0] == 'struct':
    if optional: print(f'optional {ty}')
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
      if optional: print(f'optional {ty}')
      return type_ref_list_string()

    element = translate_type(ty[1][1][1])
    print(f'list: {ty[1][1][1]}')
    print(f'      {element.extern_to_rust_prepare("v")}')

    if optional: print(f'optional {ty}')
    return type_ref_list(element)

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
    if optional: print(f'optional {ty}')
    return type_mut_bool()

  # bool
  elif ty[0] == 'primitive' and ty[1] == 'bool':
    if optional: print(f'optional {ty}')
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
    if optional: print(f'optional {ty}')
    return type_primitive(ty[2])

  else: raise RuntimeError(f'unsupported type {ty}')

def parse_type(typedefs, ty):
  def parse_type(raw):
    raw = raw.strip()
    if raw in classes: return ('class', raw)
    if raw in interfaces: return ('interface', raw)
    if raw == 'CefString': return ('string', raw)
    if raw == 'cef_string_t': return ('string', raw)
    if raw == 'int': return ('primitive', raw, 'i32')
    if raw == 'uint32': return ('primitive', raw, 'u32')
    if raw == 'int64': return ('primitive', raw, 'i64')
    if raw == 'uint64': return ('primitive', raw, 'u64')
    if raw == 'size_t': return ('primitive', raw, 'u64')
    if raw == 'char16': return ('primitive', raw, 'u16')
    if raw == 'bool': return ('primitive', raw, 'bool')
    if raw == 'char': return ('primitive', raw, 'u8')
    if raw == 'float': return ('primitive', raw, 'f32')
    if raw == 'double': return ('primitive', raw, 'f64')
    if raw == 'void': return ('primitive', raw, 'std::os::raw::c_void')

    if raw == 'CefEventHandle':
      print(raw)

    typedef = next((x for x in typedefs if x.alias == raw), None)
    if typedef: return parse_type(typedef.get_value().get_value())

    match = re.match(r'^CefRefPtr<(.*)>$', raw)
    if match: return ('refptr', parse_type(match.group(1)))

    match = re.match(r'^CefRawPtr<(.*)>$', raw)
    if match: return ('rawptr', parse_type(match.group(1)))

    match = re.match(r'^std::vector<(.*)>$', raw)
    if match: return ('vector', parse_type(match.group(1)))

    match = re.match(r'^std::map<(.*),(.*)>$', raw)
    if match: return ('map', parse_type(match.group(1)), parse_type(match.group(2)))

    match = re.match(r'^std::multimap<(.*),(.*)>$', raw)
    if match: return ('multimap', parse_type(match.group(1)), parse_type(match.group(2)))

    match = re.match(r'^(.*)&$', raw)
    if match: return ('ref', parse_type(match.group(1)))

    match = re.match(r'^(.*)\*$', raw)
    if match: return ('ptr', parse_type(match.group(1)))

    match = re.match(r'^(.*) const$', raw) or re.match(r'^const (.*)$', raw)
    if match: return ('const', parse_type(match.group(1)))

    match = re.match(r'^cef.*$', raw, re.IGNORECASE)
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

  def is_base(parsed):
    return parsed[0] in ['string', 'primitive', 'struct', 'object']

  def is_ref_or_ptr(parsed):
    return parsed[0] in ['ref', 'ptr']

  def is_primitive(parsed):
    if parsed[0] == 'primitive':
      return True
    if parsed[0] in ['ref', 'ptr', 'const']:
      return is_primitive(parsed[1])
    return False

  return ''

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
  # if ident == 'to': return 'to_'
  if len(ident) > 0 and ident[0].isdigit(): return 'X' + ident
  return ident

def is_optional(arg, func):
  optional = func.get_attrib_list('optional_param')
  return optional and arg.get_name() in optional

# the rootdir option is required
if options.rootdir is None:
  options.rootdir = os.environ.get('CEF_PATH')

if options.rootdir is None:
  parser.print_help(sys.stdout)
  sys.exit()

if options.outputdir is None:
  options.outputdir = os.path.join(os.path.dirname(os.path.dirname(__file__)), 'autogen')

# determine the paths
root_dir = os.path.abspath(options.rootdir)
cpp_header_dir = os.path.join(root_dir, 'include')

output_dir = os.path.abspath(options.outputdir)
output_include_dir = os.path.join(output_dir, 'include', 'autogen')
output_internal_dir = os.path.join(output_dir, 'include', 'internal', 'autogen')

# make sure the header directory exists
if not path_exists(cpp_header_dir):
  sys.stderr.write('Directory ' + cpp_header_dir + ' does not exist.')
  sys.exit()

# create the header object
sys.stdout.write('Parsing C++ headers from ' + cpp_header_dir + '...\n')
header = obj_header()

if not path_exists(output_include_dir): os.makedirs(output_include_dir)
if not path_exists(output_internal_dir): os.makedirs(output_internal_dir)

# add include files to be processed
header.set_root_directory(cpp_header_dir)
excluded_files = ['cef_api_hash.h', 'cef_application_mac.h', 'cef_version.h']
header.add_directory(cpp_header_dir, excluded_files)

filenames = sorted(header.get_file_names())
for cls in header.get_classes():
  if cls.get_attrib('source') == 'library':
    classes.add(cls.get_name())
  else:
    interfaces.add(cls.get_name())

skip = set([])
modfile = writer()
wrapperh = writer()
for filename in filenames:
  if filename in skip: continue

  w = writer()
  for cls in header.get_classes(filename):
    if cls.get_attrib('source') == 'library':
      generate_class(w, cls)
    else:
      generate_interface(w, cls)

  for func in header.get_funcs(filename):
    try:
      generate_static_func(w, func)
    except RuntimeError as e:
      print(f'skipping {func.get_name()}: {e}')

  modname = filename.replace('.h', '')
  with open(os.path.join(output_include_dir, modname + '.rs'), 'w') as x:
    x.write(w.result)
  modfile.write_line(f'pub mod {modname};')
  modfile.write_line(f'pub use {modname}::*;')
  wrapperh.write_line(f'#include "include/capi/{modname}_capi.h"')

with open(os.path.join(output_include_dir, 'mod.rs'), 'w') as x:
  x.write(modfile.result)

with open(os.path.join(output_dir, 'wrapper.h'), 'w') as x:
  x.write(wrapperh.result)

with open(os.path.join(cpp_header_dir, 'internal', 'cef_types.h')) as src:
  contents = src.read()
  w = writer()
  generate_types(w, contents)
  with open(os.path.join(output_internal_dir, 'cef_types.rs'), 'w') as x:
    x.write(w.result)

# buildfile = writer()
# for structure in structures:
#   buildfile.write_line(f'.whitelist_type("{structure}")')
# for cls in classes:
#   buildfile.write_line(f'.whitelist_type("{get_c_type_name(cls)}")')
# for iface in interfaces:
#   buildfile.write_line(f'.whitelist_type("{get_c_type_name(iface)}")')
# for func in header.get_funcs():
#   buildfile.write_line(f'.whitelist_function("{func.get_capi_name()}")')
# for cls in header.get_classes():
#   for static_func in cls.get_static_funcs():
#     buildfile.write_line(f'.whitelist_function("{static_func.get_capi_name()}")')
# with open(os.path.join(output_dir, 'build.rs'), 'w') as x:
#   x.write(buildfile.result)
