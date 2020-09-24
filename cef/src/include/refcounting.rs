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
        // in theory could also use mem::transmute(raw) if CefProxy is repr(C)
        // as Option<CefProxy<X>> should have the same layout as *mut X but
        // this is safer and clearer
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

    pub unsafe fn to_cef_ref(object: &mut Option<CefProxy<X>>) -> *mut X {
        match object {
            Some(p) => p.raw.as_ptr(),
            None => std::ptr::null_mut()
        }
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
