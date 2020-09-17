pub type CefSchemeRegistrar = crate::include::refcounting::CefProxy<cef_sys::cef_scheme_registrar_t>;
#[allow(non_snake_case)]
impl CefSchemeRegistrar {
  /// Register a custom scheme. This method should not be called for the built-in
  /// HTTP, HTTPS, FILE, FTP, ABOUT and DATA schemes.
  /// 
  /// See cef_scheme_options_t for possible values for |options|.
  /// 
  /// This function may be called on any thread. It should only be called once
  /// per unique |scheme_name| value. If |scheme_name| is already registered or
  /// if an error occurs this method will return false.
  pub fn add_custom_scheme(&mut self, scheme_name: &crate::include::internal::CefString, options: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_custom_scheme {
        Some(f) => f(self.raw.as_ptr(),scheme_name as *const _ as *const _,options,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
/// Class that creates CefResourceHandler instances for handling scheme requests.
/// The methods of this class will always be called on the IO thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait SchemeHandlerFactory {
  /// Return a new resource handler instance to handle the request or an empty
  /// reference to allow default handling of the request. |browser| and |frame|
  /// will be the browser window and frame respectively that originated the
  /// request or NULL if the request did not originate from a browser window
  /// (for example, if the request came from CefURLRequest). The |request| object
  /// passed to this method cannot be modified.
  fn create(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, scheme_name: &crate::include::internal::CefString, request: crate::include::CefRequest) -> Option<crate::include::CefResourceHandler> { Default::default() }
}
define_refcounted!(SchemeHandlerFactory, CefSchemeHandlerFactory, cef_scheme_handler_factory_t, create: cef_scheme_handler_factory_t_create,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_scheme_handler_factory_t_create(_self: *mut cef_sys::cef_scheme_handler_factory_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, scheme_name: *const cef_sys::cef_string_t, request: *mut cef_sys::cef_request_t) -> *mut cef_sys::cef_resource_handler_t {
  let ret = CefSchemeHandlerFactory::from_cef(_self, true).get().create(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),&*(scheme_name as *const _),crate::include::CefRequest::from_cef_own(request).unwrap(),);
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefResourceHandler::to_cef_own(o))
}
/// Register a scheme handler factory with the global request context. An empty
/// |domain_name| value for a standard scheme will cause the factory to match all
/// domain names. The |domain_name| value will be ignored for non-standard
/// schemes. If |scheme_name| is a built-in scheme and no handler is returned by
/// |factory| then the built-in scheme handler factory will be called. If
/// |scheme_name| is a custom scheme then you must also implement the
/// CefApp::OnRegisterCustomSchemes() method in all processes. This function may
/// be called multiple times to change or remove the factory that matches the
/// specified |scheme_name| and optional |domain_name|. Returns false if an error
/// occurs. This function may be called on any thread in the browser process.
/// Using this function is equivalent to calling
/// CefRequestContext::GetGlobalContext()->RegisterSchemeHandlerFactory().
#[allow(non_snake_case)]
pub fn cef_register_scheme_handler_factory(scheme_name: &crate::include::internal::CefString, domain_name: Option<&crate::include::internal::CefString>, factory: Option<crate::include::CefSchemeHandlerFactory>, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_register_scheme_handler_factory(scheme_name as *const _ as *const _,match domain_name { Some(domain_name) => domain_name as *const _ as *const _, None => std::ptr::null_mut() },factory.map_or(std::ptr::null_mut(), |o| crate::include::CefSchemeHandlerFactory::to_cef_own(o)),);
    if ret == 0 { false } else { true }
  }
}
/// Clear all scheme handler factories registered with the global request
/// context. Returns false on error. This function may be called on any thread in
/// the browser process. Using this function is equivalent to calling
/// CefRequestContext::GetGlobalContext()->ClearSchemeHandlerFactories().
#[allow(non_snake_case)]
pub fn cef_clear_scheme_handler_factories() -> bool {
  unsafe {
    let ret = cef_sys::cef_clear_scheme_handler_factories();
    if ret == 0 { false } else { true }
  }
}
