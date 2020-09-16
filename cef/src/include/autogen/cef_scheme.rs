pub type CefSchemeRegistrar = crate::include::base::CefProxy<cef_sys::cef_scheme_registrar_t>;
#[allow(non_snake_case)]
impl CefSchemeRegistrar {
  pub fn add_custom_scheme(&mut self, scheme_name: &crate::include::internal::CefString, options: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_custom_scheme {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(scheme_name),options,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
/// Class that creates CefResourceHandler instances for handling scheme requests.
/// The methods of this class will always be called on the IO thread.
#[allow(non_snake_case)]
pub trait SchemeHandlerFactory {
  fn create(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, scheme_name: &crate::include::internal::CefString, request: crate::include::CefRequest) -> Option<crate::include::CefResourceHandler> { Default::default() }
}
define_refcounted!(SchemeHandlerFactory, scheme_handler_factory, create,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_scheme_handler_factory_t_create(_self: *mut cef_sys::cef_scheme_handler_factory_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, scheme_name: *const cef_sys::cef_string_t, request: *mut cef_sys::cef_request_t) -> *mut cef_sys::cef_resource_handler_t {
  let ret = CefSchemeHandlerFactory::from_cef(_self, true).get().create(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),&crate::include::internal::CefString::from_cef(scheme_name).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),);
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefResourceHandler::to_cef_own(o))
}
