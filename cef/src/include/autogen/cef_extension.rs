pub type CefExtension = crate::include::base::CefProxy<cef_sys::cef_extension_t>;
#[allow(non_snake_case)]
impl CefExtension {
  /// Returns the unique extension identifier. This is calculated based on the
  /// extension public key, if available, or on the extension path. See
  /// https://developer.chrome.com/extensions/manifest/key for details.
  pub fn get_identifier(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_identifier {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the absolute path to the extension directory on disk. This value
  /// will be prefixed with PK_DIR_RESOURCES if a relative path was passed to
  /// CefRequestContext::LoadExtension.
  pub fn get_path(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_path {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the extension manifest contents as a CefDictionaryValue object. See
  /// https://developer.chrome.com/extensions/manifest for details.
  pub fn get_manifest(&mut self) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_manifest {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  /// Returns true if this object is the same extension as |that| object.
  /// Extensions are considered the same if identifier, path and loader context
  /// match.
  pub fn is_same(&mut self, that: crate::include::CefExtension) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefExtension::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the handler for this extension. Will return NULL for internal
  /// extensions or if no handler was passed to CefRequestContext::LoadExtension.
  pub fn get_handler(&mut self) -> Option<crate::include::CefExtensionHandler> {
    unsafe {
      let ret = match self.raw.as_ref().get_handler {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefExtensionHandler::from_cef_own(ret)
    }
  }
  /// Returns the request context that loaded this extension. Will return NULL
  /// for internal extensions or if the extension has been unloaded. See the
  /// CefRequestContext::LoadExtension documentation for more information about
  /// loader contexts. Must be called on the browser process UI thread.
  pub fn get_loader_context(&mut self) -> Option<crate::include::CefRequestContext> {
    unsafe {
      let ret = match self.raw.as_ref().get_loader_context {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefRequestContext::from_cef_own(ret)
    }
  }
  /// Returns true if this extension is currently loaded. Must be called on the
  /// browser process UI thread.
  pub fn is_loaded(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_loaded {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Unload this extension if it is not an internal extension and is currently
  /// loaded. Will result in a call to CefExtensionHandler::OnExtensionUnloaded
  /// on success.
  pub fn unload(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().unload {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
