use crate::{CefString, CefV8Value};
use std::convert::{TryFrom, TryInto};

#[derive(Clone)]
pub struct V8Function(CefV8Value);

impl V8Function {
    pub fn apply(&self, this: Option<CefV8Value>, args: &[CefV8Value]) -> Option<CefV8Value> {
        self.0.execute_function(this, args)
    }
}

impl From<V8Function> for CefV8Value {
    fn from(v: V8Function) -> Self {
        v.0
    }
}

impl TryFrom<CefV8Value> for V8Function {
    type Error = CefV8Value;

    fn try_from(v: CefV8Value) -> Result<Self, Self::Error> {
        if v.is_function() {
            Ok(V8Function(v))
        } else {
            Err(v)
        }
    }
}

impl From<()> for CefV8Value {
    fn from(_: ()) -> Self {
        CefV8Value::create_undefined().unwrap()
    }
}

impl TryFrom<CefV8Value> for () {
    type Error = CefV8Value;

    fn try_from(v: CefV8Value) -> Result<Self, Self::Error> {
        if v.is_undefined() {
            Ok(())
        } else {
            Err(v)
        }
    }
}

impl From<&str> for CefV8Value {
    fn from(v: &str) -> Self {
        CefV8Value::create_string(Some(&v.into())).unwrap()
    }
}

impl TryFrom<CefV8Value> for String {
    type Error = CefV8Value;

    fn try_from(v: CefV8Value) -> Result<Self, Self::Error> {
        if v.is_string() {
            Ok(v.get_string_value().to_string())
        } else {
            Err(v)
        }
    }
}

impl From<bool> for CefV8Value {
    fn from(v: bool) -> Self {
        CefV8Value::create_bool(v).unwrap()
    }
}

impl TryFrom<CefV8Value> for bool {
    type Error = CefV8Value;

    fn try_from(v: CefV8Value) -> Result<Self, Self::Error> {
        if v.is_bool() {
            Ok(v.get_bool_value())
        } else {
            Err(v)
        }
    }
}

impl From<f32> for CefV8Value {
    fn from(v: f32) -> Self {
        CefV8Value::create_double(v as _).unwrap()
    }
}

impl TryFrom<CefV8Value> for f32 {
    type Error = CefV8Value;

    fn try_from(v: CefV8Value) -> Result<Self, Self::Error> {
        if v.is_double() {
            Ok(v.get_double_value() as _)
        } else {
            Err(v)
        }
    }
}

impl From<f64> for CefV8Value {
    fn from(v: f64) -> Self {
        CefV8Value::create_double(v).unwrap()
    }
}

impl TryFrom<CefV8Value> for f64 {
    type Error = CefV8Value;

    fn try_from(v: CefV8Value) -> Result<Self, Self::Error> {
        if v.is_double() {
            Ok(v.get_double_value())
        } else {
            Err(v)
        }
    }
}

macro_rules! int {
    ( $num:ty ) => {
        impl From<$num> for CefV8Value {
            fn from(v: $num) -> Self {
                CefV8Value::create_int(v as i32).unwrap()
            }
        }

        impl TryFrom<CefV8Value> for $num {
            type Error = CefV8Value;

            fn try_from(v: CefV8Value) -> Result<Self, Self::Error> {
                if v.is_int() {
                    let value = v.get_int_value();
                    match value.try_into() {
                        Ok(v) => Ok(v),
                        Err(_) => Err(v),
                    }
                } else {
                    Err(v)
                }
            }
        }
    };
}

macro_rules! uint {
    ( $num:ty ) => {
        impl From<$num> for CefV8Value {
            fn from(v: $num) -> Self {
                CefV8Value::create_uint(v as u32).unwrap()
            }
        }

        impl TryFrom<CefV8Value> for $num {
            type Error = CefV8Value;

            fn try_from(v: CefV8Value) -> Result<Self, Self::Error> {
                if v.is_int() {
                    let value = v.get_uint_value();
                    match value.try_into() {
                        Ok(v) => Ok(v),
                        Err(_) => Err(v),
                    }
                } else {
                    Err(v)
                }
            }
        }
    };
}

int!(i8);
int!(i16);
int!(i32);
int!(i64);
int!(i128);
int!(isize);

uint!(u8);
uint!(u16);
uint!(u32);
uint!(u64);
uint!(u128);
uint!(usize);

macro_rules! v8_function {
    ( $funcname:ident, $structname:ident $(, $argname:ident )* ) => {
        #[allow(unused_parens)]
        struct $structname<$( $argname, )* R, F>(F, std::marker::PhantomData<($( $argname, )* R)>);

        #[allow(non_snake_case, unused_mut, unused_variables)]
        impl< $( $argname: std::convert::TryFrom<CefV8Value>, )* R: std::convert::Into<CefV8Value>, F: FnMut($( $argname ),*) -> R>
            crate::V8Handler for $structname<$( $argname, )* R, F>
        {
            fn execute(
                &mut self,
                _name: &CefString,
                _object: CefV8Value,
                arguments: &[CefV8Value],
                retval: &mut Option<CefV8Value>,
                exception: &mut CefString,
            ) -> bool {
                let mut args = arguments.iter();

                $(
                    let $argname = match $argname::try_from(args.next().unwrap().clone()) {
                        Ok(v) => v,
                        Err(_) => {
                            *exception = stringify!($argname).into();
                            return true;
                        }
                    };
                )*

                let R = (self.0)( $( $argname ),* );
                *retval = Some(R.into());

                true
            }
        }
        pub fn $funcname<
            S: Into<CefString>,
            $( $argname: 'static + std::convert::TryFrom<CefV8Value>, )*
            R: 'static + std::convert::Into<CefV8Value>,
            F: 'static + FnMut($( $argname ),*) -> R,
        >(
            name: S,
            f: F,
        ) -> CefV8Value {
            CefV8Value::create_function(&name.into(), $structname(f, std::marker::PhantomData)).unwrap()
        }
    };
}

v8_function!(v8_function0, V8F0);
v8_function!(v8_function1, V8F1, A0);
v8_function!(v8_function2, V8F2, A0, A1);
v8_function!(v8_function3, V8F3, A0, A1, A2);
v8_function!(v8_function4, V8F4, A0, A1, A2, A3);
v8_function!(v8_function5, V8F5, A0, A1, A2, A3, A4);
v8_function!(v8_function6, V8F6, A0, A1, A2, A3, A4, A5);

pub fn v8_array<I: ExactSizeIterator<Item = CefV8Value>>(src: I) -> CefV8Value {
    let l = CefV8Value::create_array(src.len() as i32).unwrap();
    for (i, value) in src.enumerate() {
        l.set_value_byindex(i as i32, value.clone());
    }
    l
}
