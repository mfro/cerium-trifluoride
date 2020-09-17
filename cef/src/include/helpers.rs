
pub fn cef_string_to_string(src: &cef_sys::cef_string_t) -> String {
    unsafe {
        if src.length == 0 {
            String::new()
        } else {
            let slice = std::slice::from_raw_parts(src.str_, src.length as usize);
            String::from_utf16_lossy(slice)
        }
    }
}

pub fn str_to_cef_string(field: &mut cef_sys::cef_string_t, value: &str) {
    if let Some(dtor) = field.dtor {
        unsafe { dtor(field.str_) };
    }

    if value.len() == 0 {
        field.length = 0;
        field.str_ = std::ptr::null_mut();
        field.dtor = None;
    } else {
        let v: Vec<_> = value.encode_utf16().collect();
        field.length = v.len() as u64;
        field.str_ = Box::into_raw(v.into_boxed_slice()) as *mut _;
        field.dtor = Some(string_dtor);
    }
}

unsafe extern "C" fn string_dtor(str_: *mut u16) {
    drop(Box::from_raw(str_))
}
