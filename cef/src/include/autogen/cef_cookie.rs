pub type CefCookieManager = crate::include::base::CefProxy<cef_sys::cef_cookie_manager_t>;
#[allow(non_snake_case)]
impl CefCookieManager {
  /// Returns the global cookie manager. By default data will be stored at
  /// CefSettings.cache_path if specified or in memory otherwise. If |callback|
  /// is non-NULL it will be executed asnychronously on the UI thread after the
  /// manager's storage has been initialized. Using this method is equivalent to
  /// calling CefRequestContext::GetGlobalContext()->GetDefaultCookieManager().
  #[allow(non_snake_case)]
  pub fn get_global_manager(callback: Option<crate::include::CefCompletionCallback>, ) -> Option<crate::include::CefCookieManager> {
    unsafe {
      let ret = cef_sys::cef_cookie_manager_get_global_manager(callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),);
      crate::include::CefCookieManager::from_cef_own(ret)
    }
  }
  /// Set the schemes supported by this manager. If |include_defaults| is true
  /// the default schemes ("http", "https", "ws" and "wss") will also be
  /// supported. Calling this method with an empty |schemes| value and
  /// |include_defaults| set to false will disable all loading and saving of
  /// cookies for this manager. If |callback| is non-NULL it will be executed
  /// asnychronously on the UI thread after the change has been applied. Must be
  /// called before any cookies are accessed.
  pub fn set_supported_schemes(&mut self, schemes: &crate::include::internal::CefStringList, include_defaults: bool, callback: Option<crate::include::CefCompletionCallback>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_supported_schemes {
        Some(f) => f(self.raw.as_ptr(),schemes.into(),if include_defaults { 1 } else { 0 },callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  /// Visit all cookies on the UI thread. The returned cookies are ordered by
  /// longest path, then by earliest creation date. Returns false if cookies
  /// cannot be accessed.
  pub fn visit_all_cookies(&mut self, visitor: crate::include::CefCookieVisitor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().visit_all_cookies {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefCookieVisitor::to_cef_own(visitor),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Visit a subset of cookies on the UI thread. The results are filtered by the
  /// given url scheme, host, domain and path. If |includeHttpOnly| is true
  /// HTTP-only cookies will also be included in the results. The returned
  /// cookies are ordered by longest path, then by earliest creation date.
  /// Returns false if cookies cannot be accessed.
  pub fn visit_url_cookies(&mut self, url: &crate::include::internal::CefString, includeHttpOnly: bool, visitor: crate::include::CefCookieVisitor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().visit_url_cookies {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),if includeHttpOnly { 1 } else { 0 },crate::include::CefCookieVisitor::to_cef_own(visitor),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets a cookie given a valid URL and explicit user-provided cookie
  /// attributes. This function expects each attribute to be well-formed. It will
  /// check for disallowed characters (e.g. the ';' character is disallowed
  /// within the cookie value attribute) and fail without setting the cookie if
  /// such characters are found. If |callback| is non-NULL it will be executed
  /// asnychronously on the UI thread after the cookie has been set. Returns
  /// false if an invalid URL is specified or if cookies cannot be accessed.
  pub fn set_cookie(&mut self, url: &crate::include::internal::CefString, cookie: &crate::include::internal::CefCookie, callback: Option<crate::include::CefSetCookieCallback>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_cookie {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),cookie as *const _ as *const _,callback.map_or(std::ptr::null_mut(), |o| crate::include::CefSetCookieCallback::to_cef_own(o)),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Delete all cookies that match the specified parameters. If both |url| and
  /// |cookie_name| values are specified all host and domain cookies matching
  /// both will be deleted. If only |url| is specified all host cookies (but not
  /// domain cookies) irrespective of path will be deleted. If |url| is empty all
  /// cookies for all hosts and domains will be deleted. If |callback| is
  /// non-NULL it will be executed asnychronously on the UI thread after the
  /// cookies have been deleted. Returns false if a non-empty invalid URL is
  /// specified or if cookies cannot be accessed. Cookies can alternately be
  /// deleted using the Visit*Cookies() methods.
  pub fn delete_cookies(&mut self, url: Option<&crate::include::internal::CefString>, cookie_name: Option<&crate::include::internal::CefString>, callback: Option<crate::include::CefDeleteCookiesCallback>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().delete_cookies {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),crate::include::internal::IntoCef::into_cef(cookie_name),callback.map_or(std::ptr::null_mut(), |o| crate::include::CefDeleteCookiesCallback::to_cef_own(o)),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Flush the backing store (if any) to disk. If |callback| is non-NULL it will
  /// be executed asnychronously on the UI thread after the flush is complete.
  /// Returns false if cookies cannot be accessed.
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
#[allow(unused_variables)]
pub trait CookieVisitor {
  /// Method that will be called once for each cookie. |count| is the 0-based
  /// index for the current cookie. |total| is the total number of cookies.
  /// Set |deleteCookie| to true to delete the cookie currently being visited.
  /// Return false to stop visiting cookies. This method may never be called if
  /// no cookies are found.
  fn visit(&mut self, cookie: &crate::include::internal::CefCookie, count: i32, total: i32, deleteCookie: &mut bool) -> bool { Default::default() }
}
define_refcounted!(CookieVisitor, CefCookieVisitor, cef_cookie_visitor_t, visit: cef_cookie_visitor_t_visit,);
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
#[allow(unused_variables)]
pub trait SetCookieCallback {
  /// Method that will be called upon completion. |success| will be true if the
  /// cookie was set successfully.
  fn on_complete(&mut self, success: bool) -> () { Default::default() }
}
define_refcounted!(SetCookieCallback, CefSetCookieCallback, cef_set_cookie_callback_t, on_complete: cef_set_cookie_callback_t_on_complete,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_set_cookie_callback_t_on_complete(_self: *mut cef_sys::cef_set_cookie_callback_t, success: i32) -> () {
  let ret = CefSetCookieCallback::from_cef(_self, true).get().on_complete(if success == 0 { false } else { true },);
  ret
}
/// Interface to implement to be notified of asynchronous completion via
/// CefCookieManager::DeleteCookies().
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait DeleteCookiesCallback {
  /// Method that will be called upon completion. |num_deleted| will be the
  /// number of cookies that were deleted.
  fn on_complete(&mut self, num_deleted: i32) -> () { Default::default() }
}
define_refcounted!(DeleteCookiesCallback, CefDeleteCookiesCallback, cef_delete_cookies_callback_t, on_complete: cef_delete_cookies_callback_t_on_complete,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_delete_cookies_callback_t_on_complete(_self: *mut cef_sys::cef_delete_cookies_callback_t, num_deleted: i32) -> () {
  let ret = CefDeleteCookiesCallback::from_cef(_self, true).get().on_complete(num_deleted,);
  ret
}
