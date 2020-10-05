
#[cfg(cef_string_type_utf8)]
pub fn cef_string_to_string(src: &cef_sys::cef_string_t) -> String {
    unsafe {
        if src.length == 0 {
            String::new()
        } else {
            let slice = std::slice::from_raw_parts(src.str_ as *const u8, src.length as usize);
            String::from_utf8_lossy(slice).to_string()
        }
    }
}

#[cfg(cef_string_type_utf16)]
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


#[cfg(cef_string_type_utf8)]
pub fn str_to_cef_string(field: &mut cef_sys::cef_string_t, value: &str) {
    if let Some(dtor) = field.dtor {
        unsafe { dtor(field.str_) };
    }

    if value.len() == 0 {
        field.length = 0;
        field.str_ = std::ptr::null_mut();
        field.dtor = None;
    } else {
        let v: Vec<_> = value.as_bytes().to_vec();
        field.length = v.len() as u64;
        field.str_ = Box::into_raw(v.into_boxed_slice()) as *mut _;
        field.dtor = Some(string_dtor);
    }
}

#[cfg(cef_string_type_utf16)]
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

#[cfg(cef_string_type_utf8)]
unsafe extern "C" fn string_dtor(str_: *mut i8) {
    drop(Box::from_raw(str_))
}


#[cfg(cef_string_type_utf16)]
unsafe extern "C" fn string_dtor(str_: *mut u16) {
    drop(Box::from_raw(str_))
}
