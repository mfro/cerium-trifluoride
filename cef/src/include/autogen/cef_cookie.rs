pub type CefCookieManager = crate::include::base::CefProxy<cef_sys::cef_cookie_manager_t>;
#[allow(non_snake_case)]
impl CefCookieManager {
  pub fn set_supported_schemes(&mut self, schemes: &crate::include::internal::CefStringList, include_defaults: bool, callback: Option<crate::include::CefCompletionCallback>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_supported_schemes {
        Some(f) => f(self.raw.as_ptr(),schemes.into(),if include_defaults { 1 } else { 0 },callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn visit_all_cookies(&mut self, visitor: crate::include::CefCookieVisitor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().visit_all_cookies {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefCookieVisitor::to_cef_own(visitor),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn visit_url_cookies(&mut self, url: &crate::include::internal::CefString, includeHttpOnly: bool, visitor: crate::include::CefCookieVisitor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().visit_url_cookies {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),if includeHttpOnly { 1 } else { 0 },crate::include::CefCookieVisitor::to_cef_own(visitor),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_cookie(&mut self, url: &crate::include::internal::CefString, cookie: &crate::include::internal::CefCookie, callback: Option<crate::include::CefSetCookieCallback>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_cookie {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),&cookie.raw,callback.map_or(std::ptr::null_mut(), |o| crate::include::CefSetCookieCallback::to_cef_own(o)),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn delete_cookies(&mut self, url: &crate::include::internal::CefString, cookie_name: &crate::include::internal::CefString, callback: crate::include::CefDeleteCookiesCallback) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().delete_cookies {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),crate::include::internal::IntoCef::into_cef(cookie_name),crate::include::CefDeleteCookiesCallback::to_cef_own(callback),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn flush_store(&mut self, callback: Option<crate::include::CefCompletionCallback>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().flush_store {
        Some(f) => f(self.raw.as_ptr(),callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
/// Interface to implement for visiting cookie values. The methods of this class
/// will always be called on the UI thread.
#[allow(non_snake_case)]
pub trait CookieVisitor {
  fn visit(&mut self, cookie: &crate::include::internal::CefCookie, count: i32, total: i32, deleteCookie: &mut bool) -> bool { Default::default() }
}
define_refcounted!(CookieVisitor, cookie_visitor, visit,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_cookie_visitor_t_visit(_self: *mut cef_sys::cef_cookie_visitor_t, cookie: *const cef_sys::cef_cookie_t, count: i32, total: i32, deleteCookie: *mut i32) -> i32 {
  let mut deleteCookie__tmp = if *deleteCookie == 0 { false } else { true };
  let ret = CefCookieVisitor::from_cef(_self, true).get().visit(&*(cookie as *const _),count,total,&mut deleteCookie__tmp,);
  *deleteCookie = if deleteCookie__tmp { 1 } else { 0 };
  if ret { 1 } else { 0 }
}
/// Interface to implement to be notified of asynchronous completion via
/// CefCookieManager::SetCookie().
#[allow(non_snake_case)]
pub trait SetCookieCallback {
  fn on_complete(&mut self, success: bool) -> () { Default::default() }
}
define_refcounted!(SetCookieCallback, set_cookie_callback, on_complete,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_set_cookie_callback_t_on_complete(_self: *mut cef_sys::cef_set_cookie_callback_t, success: i32) -> () {
  let ret = CefSetCookieCallback::from_cef(_self, true).get().on_complete(if success == 0 { false } else { true },);
  ret
}
/// Interface to implement to be notified of asynchronous completion via
/// CefCookieManager::DeleteCookies().
#[allow(non_snake_case)]
pub trait DeleteCookiesCallback {
  fn on_complete(&mut self, num_deleted: i32) -> () { Default::default() }
}
define_refcounted!(DeleteCookiesCallback, delete_cookies_callback, on_complete,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_delete_cookies_callback_t_on_complete(_self: *mut cef_sys::cef_delete_cookies_callback_t, num_deleted: i32) -> () {
  let ret = CefDeleteCookiesCallback::from_cef(_self, true).get().on_complete(num_deleted,);
  ret
}
