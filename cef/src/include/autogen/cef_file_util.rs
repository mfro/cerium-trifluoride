/// Creates a directory and all parent directories if they don't already exist.
/// Returns true on successful creation or if the directory already exists. The
/// directory is only readable by the current user. Calling this function on the
/// browser process UI or IO threads is not allowed.
#[allow(non_snake_case)]
pub fn cef_create_directory(full_path: &crate::include::internal::CefString, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_create_directory(full_path as *const _ as *const _,);
    if ret == 0 { false } else { true }
  }
}
/// Get the temporary directory provided by the system.
/// 
/// WARNING: In general, you should use the temp directory variants below instead
/// of this function. Those variants will ensure that the proper permissions are
/// set so that other users on the system can't edit them while they're open
/// (which could lead to security issues).
#[allow(non_snake_case)]
pub fn cef_get_temp_directory(temp_dir: &mut crate::include::internal::CefString, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_get_temp_directory(temp_dir as *mut _ as *mut _,);
    if ret == 0 { false } else { true }
  }
}
/// Creates a new directory. On Windows if |prefix| is provided the new directory
/// name is in the format of "prefixyyyy". Returns true on success and sets
/// |new_temp_path| to the full path of the directory that was created. The
/// directory is only readable by the current user. Calling this function on the
/// browser process UI or IO threads is not allowed.
#[allow(non_snake_case)]
pub fn cef_create_new_temp_directory(prefix: Option<&crate::include::internal::CefString>, new_temp_path: &mut crate::include::internal::CefString, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_create_new_temp_directory(match prefix { Some(prefix) => prefix as *const _ as *const _, None => std::ptr::null_mut() },new_temp_path as *mut _ as *mut _,);
    if ret == 0 { false } else { true }
  }
}
/// Creates a directory within another directory. Extra characters will be
/// appended to |prefix| to ensure that the new directory does not have the same
/// name as an existing directory. Returns true on success and sets |new_dir| to
/// the full path of the directory that was created. The directory is only
/// readable by the current user. Calling this function on the browser process
/// UI or IO threads is not allowed.
#[allow(non_snake_case)]
pub fn cef_create_temp_directory_in_directory(base_dir: &crate::include::internal::CefString, prefix: Option<&crate::include::internal::CefString>, new_dir: &mut crate::include::internal::CefString, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_create_temp_directory_in_directory(base_dir as *const _ as *const _,match prefix { Some(prefix) => prefix as *const _ as *const _, None => std::ptr::null_mut() },new_dir as *mut _ as *mut _,);
    if ret == 0 { false } else { true }
  }
}
/// Returns true if the given path exists and is a directory. Calling this
/// function on the browser process UI or IO threads is not allowed.
#[allow(non_snake_case)]
pub fn cef_directory_exists(path: &crate::include::internal::CefString, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_directory_exists(path as *const _ as *const _,);
    if ret == 0 { false } else { true }
  }
}
/// Deletes the given path whether it's a file or a directory. If |path| is a
/// directory all contents will be deleted.  If |recursive| is true any sub-
/// directories and their contents will also be deleted (equivalent to executing
/// "rm -rf", so use with caution). On POSIX environments if |path| is a symbolic
/// link then only the symlink will be deleted. Returns true on successful
/// deletion or if |path| does not exist. Calling this function on the browser
/// process UI or IO threads is not allowed.
#[allow(non_snake_case)]
pub fn cef_delete_file(path: &crate::include::internal::CefString, recursive: bool, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_delete_file(path as *const _ as *const _,if recursive { 1 } else { 0 },);
    if ret == 0 { false } else { true }
  }
}
/// Writes the contents of |src_dir| into a zip archive at |dest_file|. If
/// |include_hidden_files| is true files starting with "." will be included.
/// Returns true on success.  Calling this function on the browser process UI or
/// IO threads is not allowed.
#[allow(non_snake_case)]
pub fn cef_zip_directory(src_dir: &crate::include::internal::CefString, dest_file: &crate::include::internal::CefString, include_hidden_files: bool, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_zip_directory(src_dir as *const _ as *const _,dest_file as *const _ as *const _,if include_hidden_files { 1 } else { 0 },);
    if ret == 0 { false } else { true }
  }
}
/// Loads the existing "Certificate Revocation Lists" file that is managed by
/// Google Chrome. This file can generally be found in Chrome's User Data
/// directory (e.g. "C:\Users\[User]\AppData\Local\Google\Chrome\User Data\" on
/// Windows) and is updated periodically by Chrome's component updater service.
/// Must be called in the browser process after the context has been initialized.
/// See https://dev.chromium.org/Home/chromium-security/crlsets for background.
#[allow(non_snake_case)]
pub fn cef_load_crlsets_file(path: &crate::include::internal::CefString, ) -> () {
  unsafe {
    let ret = cef_sys::cef_load_crlsets_file(path as *const _ as *const _,);
    ret
  }
}
