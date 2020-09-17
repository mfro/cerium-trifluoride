use std::os::raw::c_int;
use std::ptr::NonNull;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{Relaxed, Release};
use std::sync::{Mutex, MutexGuard};

use cef_sys::cef_base_ref_counted_t;

macro_rules! translate_cef_ptr {
    ( $ptr:expr ) => {{
        let ptr = ($ptr as *mut *const u8).offset(-1);
        let ptr = (ptr as *mut std::sync::atomic::AtomicUsize).offset(-1);
        &mut *(ptr as *mut crate::include::refcounting::CefShim)
    }};
}

unsafe extern "C" fn add_ref(ptr: *mut cef_base_ref_counted_t) {
    let shim = translate_cef_ptr!(ptr);
    let _count = shim.count.fetch_add(1, Relaxed);

    // println!(
    //     "increment {:?} {} -> {}",
    //     shim as *mut CefShim,
    //     _count,
    //     _count + 1,
    // );
}

unsafe extern "C" fn release(ptr: *mut cef_base_ref_counted_t) -> c_int {
    let shim = translate_cef_ptr!(ptr);
    let count = shim.count.fetch_sub(1, Release);

    // println!(
    //     "decrement {:?} {} -> {}",
    //     shim as *mut CefShim,
    //     count,
    //     count - 1,
    // );

    match count {
        1 => {
            drop(Box::from_raw(shim));
            1
        }
        _ => 0,
    }
}

unsafe extern "C" fn has_one_ref(_self: *mut cef_base_ref_counted_t) -> c_int {
    1
}

unsafe extern "C" fn has_at_least_one_ref(_self: *mut cef_base_ref_counted_t) -> c_int {
    1
}

pub fn init_ref_count<T>() -> cef_base_ref_counted_t {
    cef_base_ref_counted_t {
        size: std::mem::size_of::<T>() as u64,
        add_ref: Some(add_ref),
        release: Some(release),
        has_one_ref: Some(has_one_ref),
        has_at_least_one_ref: Some(has_at_least_one_ref),
    }
}

pub struct CefProxy<X> {
    pub raw: NonNull<X>,
}

impl<X> CefProxy<X> {
    unsafe fn increment(&self) {
        let ptr = self.raw.as_ptr() as *mut cef_base_ref_counted_t;
        let vtable = &mut *ptr;
        if let Some(func) = vtable.add_ref {
            func(vtable);
        }
    }

    /// Create a new reference by transferring ownership from CEF.
    ///
    /// This should be used when the reference is already counted, such as
    /// when it is passed as a parameter
    pub unsafe fn from_cef_own(raw: *mut X) -> Option<CefProxy<X>> {
        match NonNull::new(raw) {
            Some(raw) => Some(CefProxy { raw }),
            None => None,
        }
    }

    /// Create a new pseudo-reference that references a CEF reference
    ///
    /// This should be used when the reference is not counted, such as
    /// in CefApp::OnRegisterCustomSchemes
    pub unsafe fn from_cef_ref(raw: *mut X) -> Option<CefProxy<X>> {
        let proxy = Self::from_cef_own(raw);
        if let Some(proxy) = &proxy {
            proxy.increment()
        }
        proxy
    }

    /// Transferring ownership to CEF.
    ///
    /// This should be used when the reference is already counted, such as
    /// when it is passed as a parameter
    pub unsafe fn to_cef_own(object: CefProxy<X>) -> *mut X {
        let ptr = object.raw.as_ptr();
        std::mem::forget(object);
        ptr
    }

    pub unsafe fn to_cef_ref(object: &mut CefProxy<X>) -> *mut X {
        object.raw.as_ptr() as *const _ as *mut _
    }
}

impl<X> Clone for CefProxy<X> {
    fn clone(&self) -> CefProxy<X> {
        unsafe { self.increment() };
        CefProxy { raw: self.raw }
    }
}

pub struct CefObject<T: ?Sized, X> {
    pub ptr: NonNull<CefObjectImplInner<T, X>>,
}

impl<T: ?Sized, X> CefObject<T, X> {
    pub fn get(&self) -> MutexGuard<T> {
        unsafe { self.ptr.as_ref() }.value.lock().unwrap()
    }

    pub unsafe fn from_cef_own(_raw: *mut X) -> Option<CefObject<T, X>> {
        panic!()
    }

    pub unsafe fn to_cef_own(object: CefObject<T, X>) -> *mut X {
        let ptr = &object.ptr.as_ref().raw as *const _ as *mut _;
        std::mem::forget(object);
        ptr
    }
}

impl<T: ?Sized, X: Clone> Clone for CefObject<T, X> {
    fn clone(&self) -> CefObject<T, X> {
        unsafe {
            let ptr = self.ptr.as_ptr() as *mut CefShim;
            let vtable = &mut (*ptr).raw;
            if let Some(func) = vtable.add_ref {
                func(vtable);
            }
        }

        let ptr = self.ptr.clone();
        CefObject { ptr }
    }
}

impl<T: ?Sized, X> Drop for CefObject<T, X> {
    fn drop(&mut self) {
        unsafe {
            let ptr = self.ptr.as_ptr() as *mut CefShim;
            let vtable = &mut (*ptr).raw;
            if let Some(func) = vtable.release {
                func(vtable);
            }
        }
    }
}

#[repr(C)]
pub struct CefShim<T = cef_base_ref_counted_t> {
    pub count: AtomicUsize,
    pub vtable: *const u8,
    pub raw: T,
}

#[repr(C)]
pub struct CefObjectImplInner<T: ?Sized, X> {
    pub count: AtomicUsize,
    pub vtable: *const u8,
    pub raw: X,
    pub value: Mutex<T>,
}

/// note: relies on the layout of `&dyn Trait` as described here:
/// https://rust-lang.github.io/unsafe-code-guidelines/layout/pointers.html#notes
#[repr(C)]
pub struct DynObject {
    pub data: *const u8,
    pub vtable: *const u8,
}

macro_rules! define_refcounted {
    ( $trait_name:ident, $object_name:ident, $c_name:ident, $( $method: ident: $c_method: ident, )* ) => {
        pub type $object_name = crate::include::refcounting::CefObject<dyn $trait_name, cef_sys::$c_name>;

        impl $object_name {
            pub unsafe fn from_cef(ptr: *mut cef_sys::$c_name, self_ref: bool) -> $object_name {
                let shim = translate_cef_ptr!(ptr);

                // CEF does not increment reference counting when passing a structure to its own member function
                // (ie for `this` pointer), so from_cef can be called with the reference already counted or not.
                // this parameter is necessary but this solution involves double counting the `this` reference
                // for the duration of the member function.
                if self_ref {
                    shim.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                // relies on the layout of `&dyn Trait` as described here:
                // https://rust-lang.github.io/unsafe-code-guidelines/layout/pointers.html#notes
                let fat_pointer = crate::include::refcounting::DynObject {
                    data: shim as *const _ as *const u8,
                    vtable: shim.vtable,
                };

                let ptr = std::mem::transmute(fat_pointer);
                let ptr = std::ptr::NonNull::new(ptr).unwrap();
                $object_name { ptr }
            }

            pub fn new<T: $trait_name + 'static>(value: T) -> $object_name {
                // call the init_ref_count fn to initialize the 'base' field of our struct.
                // The 'init_ref_count' fn is preferable to initializing that field directly
                // in this macro as that would require increasing the visibility on the extern
                // fns at the top of this file.
                let base = crate::include::refcounting::init_ref_count::<cef_sys::$c_name>();

                // Initialize the cef vtable struct, using Default::default() to null-initialize
                // any methods that are not implemented in rust. This reduces the amount of
                // boilerplate needed for partially implementing a large CEF class.
                let raw = cef_sys::$c_name {
                    base,
                    $( $method: Some($c_method), )*
                    ..Default::default()
                };

                let value = std::sync::Mutex::new(value);

                // Create the CefObject which is where we put the actual
                // implemention, vtable, and refcount. refcount starts at
                // 1 and is incremented whenever the CefObjectImpl is cloned or
                // add_ref is called from CEF code.
                //
                // We can be sure that, excepting bugs in this implementation, the rust calls will
                // all be appropriate, but we are trusting that CEF refcounts properly and doesn't
                // cause us any memory concerns
                let wrapper: Box<crate::include::refcounting::CefObjectImplInner<dyn $trait_name, cef_sys::$c_name>> = Box::new(crate::include::refcounting::CefObjectImplInner {
                    count: std::sync::atomic::AtomicUsize::new(1),
                    raw,
                    vtable: std::ptr::null_mut(),
                    value,
                });

                let ptr = Box::into_raw(wrapper);

                unsafe {
                    // relies on the layout of `&dyn Trait` as described here:
                    // https://rust-lang.github.io/unsafe-code-guidelines/layout/pointers.html#notes
                    let x = &ptr as *const _ as *const crate::include::refcounting::DynObject;
                    (*ptr).vtable = (*x).vtable;
                }

                // println!("create {}: {:?}", stringify!($trait_name), ptr);

                // leak the object from the box, we are now taking responsibility for keeping track of it.
                let ptr = std::ptr::NonNull::new(ptr).unwrap();
                crate::include::refcounting::CefObject { ptr }
            }
        }

        impl<T: $trait_name + 'static> From<T> for $object_name {
            fn from(src: T) -> $object_name {
                $object_name::new(src)
            }
        }
    };
}
