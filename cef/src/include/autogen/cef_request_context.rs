/// Callback interface for CefRequestContext::ResolveHost.
#[allow(non_snake_case)]
pub trait ResolveCallback {
  fn on_resolve_completed(&mut self, result: crate::include::internal::CefErrorcode, resolved_ips: &crate::include::internal::CefStringList) -> () { Default::default() }
}
define_refcounted!(ResolveCallback, resolve_callback, on_resolve_completed,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resolve_callback_t_on_resolve_completed(_self: *mut cef_sys::cef_resolve_callback_t, result: cef_sys::cef_errorcode_t, resolved_ips: cef_sys::cef_string_list_t) -> () {
  let ret = CefResolveCallback::from_cef(_self, true).get().on_resolve_completed(result.into(),&resolved_ips.into(),);
  ret
}
pub type CefRequestContext = crate::include::base::CefProxy<cef_sys::cef_request_context_t>;
#[allow(non_snake_case)]
impl CefRequestContext {
  pub fn is_same(&mut self, other: crate::include::CefRequestContext) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefRequestContext::to_cef_own(other),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_sharing_with(&mut self, other: crate::include::CefRequestContext) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_sharing_with {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefRequestContext::to_cef_own(other),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_global(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_global {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_handler(&mut self) -> Option<crate::include::CefRequestContextHandler> {
    unsafe {
      let ret = match self.raw.as_ref().get_handler {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefRequestContextHandler::from_cef_own(ret)
    }
  }
  pub fn get_cache_path(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_cache_path {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_cookie_manager(&mut self, callback: Option<crate::include::CefCompletionCallback>) -> Option<crate::include::CefCookieManager> {
    unsafe {
      let ret = match self.raw.as_ref().get_cookie_manager {
        Some(f) => f(self.raw.as_ptr(),callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),),
        None => panic!(),
      };
      crate::include::CefCookieManager::from_cef_own(ret)
    }
  }
  pub fn register_scheme_handler_factory(&mut self, scheme_name: &crate::include::internal::CefString, domain_name: &crate::include::internal::CefString, factory: crate::include::CefSchemeHandlerFactory) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().register_scheme_handler_factory {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(scheme_name),crate::include::internal::IntoCef::into_cef(domain_name),crate::include::CefSchemeHandlerFactory::to_cef_own(factory),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn clear_scheme_handler_factories(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().clear_scheme_handler_factories {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn purge_plugin_list_cache(&mut self, reload_pages: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().purge_plugin_list_cache {
        Some(f) => f(self.raw.as_ptr(),if reload_pages { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn has_preference(&mut self, name: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_preference {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_preference(&mut self, name: &crate::include::internal::CefString) -> Option<crate::include::CefValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_preference {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      crate::include::CefValue::from_cef_own(ret)
    }
  }
  pub fn get_all_preferences(&mut self, include_defaults: bool) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_all_preferences {
        Some(f) => f(self.raw.as_ptr(),if include_defaults { 1 } else { 0 },),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  pub fn can_set_preference(&mut self, name: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().can_set_preference {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_preference(&mut self, name: &crate::include::internal::CefString, value: Option<crate::include::CefValue>, error: &mut crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_preference {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),value.map_or(std::ptr::null_mut(), |o| crate::include::CefValue::to_cef_own(o)),crate::include::internal::IntoCef::into_cef(error),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn clear_certificate_exceptions(&mut self, callback: Option<crate::include::CefCompletionCallback>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().clear_certificate_exceptions {
        Some(f) => f(self.raw.as_ptr(),callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn clear_http_auth_credentials(&mut self, callback: Option<crate::include::CefCompletionCallback>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().clear_http_auth_credentials {
        Some(f) => f(self.raw.as_ptr(),callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn close_all_connections(&mut self, callback: Option<crate::include::CefCompletionCallback>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().close_all_connections {
        Some(f) => f(self.raw.as_ptr(),callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn resolve_host(&mut self, origin: &crate::include::internal::CefString, callback: crate::include::CefResolveCallback) -> () {
    unsafe {
      let ret = match self.raw.as_ref().resolve_host {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(origin),crate::include::CefResolveCallback::to_cef_own(callback),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn load_extension(&mut self, root_directory: &crate::include::internal::CefString, manifest: crate::include::CefDictionaryValue, handler: crate::include::CefExtensionHandler) -> () {
    unsafe {
      let ret = match self.raw.as_ref().load_extension {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(root_directory),crate::include::CefDictionaryValue::to_cef_own(manifest),crate::include::CefExtensionHandler::to_cef_own(handler),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn did_load_extension(&mut self, extension_id: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().did_load_extension {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(extension_id),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_extension(&mut self, extension_id: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_extension {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(extension_id),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_extension(&mut self, extension_id: &crate::include::internal::CefString) -> Option<crate::include::CefExtension> {
    unsafe {
      let ret = match self.raw.as_ref().get_extension {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(extension_id),),
        None => panic!(),
      };
      crate::include::CefExtension::from_cef_own(ret)
    }
  }
  pub fn get_media_router(&mut self) -> Option<crate::include::CefMediaRouter> {
    unsafe {
      let ret = match self.raw.as_ref().get_media_router {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefMediaRouter::from_cef_own(ret)
    }
  }
}
