/// Callback interface for CefRequestContext::ResolveHost.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait ResolveCallback {
  /// Called on the UI thread after the ResolveHost request has completed.
  /// |result| will be the result code. |resolved_ips| will be the list of
  /// resolved IP addresses or empty if the resolution failed.
  fn on_resolve_completed(&mut self, result: crate::include::internal::CefErrorcode, resolved_ips: &crate::include::internal::CefStringList) -> () { Default::default() }
}
define_refcounted!(ResolveCallback, CefResolveCallback, cef_resolve_callback_t, on_resolve_completed: cef_resolve_callback_t_on_resolve_completed,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resolve_callback_t_on_resolve_completed(_self: *mut cef_sys::cef_resolve_callback_t, result: cef_sys::cef_errorcode_t, resolved_ips: cef_sys::cef_string_list_t) -> () {
  let ret = CefResolveCallback::from_cef(_self, true).get().on_resolve_completed(result.into(),&resolved_ips.into(),);
  ret
}
pub type CefRequestContext = crate::include::base::CefProxy<cef_sys::cef_request_context_t>;
#[allow(non_snake_case)]
impl CefRequestContext {
  /// Returns the global context object.
  #[allow(non_snake_case)]
  pub fn get_global_context() -> Option<crate::include::CefRequestContext> {
    unsafe {
      let ret = cef_sys::cef_request_context_get_global_context();
      crate::include::CefRequestContext::from_cef_own(ret)
    }
  }
  /// Creates a new context object with the specified |settings| and optional
  /// |handler|.
  #[allow(non_snake_case)]
  pub fn create_context(settings: &crate::include::internal::CefRequestContextSettings, handler: Option<crate::include::CefRequestContextHandler>, ) -> Option<crate::include::CefRequestContext> {
    unsafe {
      let ret = cef_sys::cef_request_context_create_context(settings as *const _ as *const _,handler.map_or(std::ptr::null_mut(), |o| crate::include::CefRequestContextHandler::to_cef_own(o)),);
      crate::include::CefRequestContext::from_cef_own(ret)
    }
  }
  /// Creates a new context object that shares storage with |other| and uses an
  /// optional |handler|.
  #[allow(non_snake_case)]
  pub fn cef_create_context_shared(other: crate::include::CefRequestContext, handler: Option<crate::include::CefRequestContextHandler>, ) -> Option<crate::include::CefRequestContext> {
    unsafe {
      let ret = cef_sys::cef_create_context_shared(crate::include::CefRequestContext::to_cef_own(other),handler.map_or(std::ptr::null_mut(), |o| crate::include::CefRequestContextHandler::to_cef_own(o)),);
      crate::include::CefRequestContext::from_cef_own(ret)
    }
  }
  /// Returns true if this object is pointing to the same context as |that|
  /// object.
  pub fn is_same(&mut self, other: crate::include::CefRequestContext) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefRequestContext::to_cef_own(other),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object is sharing the same storage as |that| object.
  pub fn is_sharing_with(&mut self, other: crate::include::CefRequestContext) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_sharing_with {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefRequestContext::to_cef_own(other),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object is the global context. The global context is
  /// used by default when creating a browser or URL request with a NULL context
  /// argument.
  pub fn is_global(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_global {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the handler for this context if any.
  pub fn get_handler(&mut self) -> Option<crate::include::CefRequestContextHandler> {
    unsafe {
      let ret = match self.raw.as_ref().get_handler {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefRequestContextHandler::from_cef_own(ret)
    }
  }
  /// Returns the cache path for this object. If empty an "incognito mode"
  /// in-memory cache is being used.
  pub fn get_cache_path(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_cache_path {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the cookie manager for this object. If |callback| is non-NULL it
  /// will be executed asnychronously on the IO thread after the manager's
  /// storage has been initialized.
  pub fn get_cookie_manager(&mut self, callback: Option<crate::include::CefCompletionCallback>) -> Option<crate::include::CefCookieManager> {
    unsafe {
      let ret = match self.raw.as_ref().get_cookie_manager {
        Some(f) => f(self.raw.as_ptr(),callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),),
        None => panic!(),
      };
      crate::include::CefCookieManager::from_cef_own(ret)
    }
  }
  /// Register a scheme handler factory for the specified |scheme_name| and
  /// optional |domain_name|. An empty |domain_name| value for a standard scheme
  /// will cause the factory to match all domain names. The |domain_name| value
  /// will be ignored for non-standard schemes. If |scheme_name| is a built-in
  /// scheme and no handler is returned by |factory| then the built-in scheme
  /// handler factory will be called. If |scheme_name| is a custom scheme then
  /// you must also implement the CefApp::OnRegisterCustomSchemes() method in all
  /// processes. This function may be called multiple times to change or remove
  /// the factory that matches the specified |scheme_name| and optional
  /// |domain_name|. Returns false if an error occurs. This function may be
  /// called on any thread in the browser process.
  pub fn register_scheme_handler_factory(&mut self, scheme_name: &crate::include::internal::CefString, domain_name: Option<&crate::include::internal::CefString>, factory: Option<crate::include::CefSchemeHandlerFactory>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().register_scheme_handler_factory {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(scheme_name),crate::include::internal::IntoCef::into_cef(domain_name),factory.map_or(std::ptr::null_mut(), |o| crate::include::CefSchemeHandlerFactory::to_cef_own(o)),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Clear all registered scheme handler factories. Returns false on error. This
  /// function may be called on any thread in the browser process.
  pub fn clear_scheme_handler_factories(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().clear_scheme_handler_factories {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Tells all renderer processes associated with this context to throw away
  /// their plugin list cache. If |reload_pages| is true they will also reload
  /// all pages with plugins. CefRequestContextHandler::OnBeforePluginLoad may
  /// be called to rebuild the plugin list cache.
  pub fn purge_plugin_list_cache(&mut self, reload_pages: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().purge_plugin_list_cache {
        Some(f) => f(self.raw.as_ptr(),if reload_pages { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if a preference with the specified |name| exists. This method
  /// must be called on the browser process UI thread.
  pub fn has_preference(&mut self, name: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_preference {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the value for the preference with the specified |name|. Returns
  /// NULL if the preference does not exist. The returned object contains a copy
  /// of the underlying preference value and modifications to the returned object
  /// will not modify the underlying preference value. This method must be called
  /// on the browser process UI thread.
  pub fn get_preference(&mut self, name: &crate::include::internal::CefString) -> Option<crate::include::CefValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_preference {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      crate::include::CefValue::from_cef_own(ret)
    }
  }
  /// Returns all preferences as a dictionary. If |include_defaults| is true then
  /// preferences currently at their default value will be included. The returned
  /// object contains a copy of the underlying preference values and
  /// modifications to the returned object will not modify the underlying
  /// preference values. This method must be called on the browser process UI
  /// thread.
  pub fn get_all_preferences(&mut self, include_defaults: bool) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_all_preferences {
        Some(f) => f(self.raw.as_ptr(),if include_defaults { 1 } else { 0 },),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  /// Returns true if the preference with the specified |name| can be modified
  /// using SetPreference. As one example preferences set via the command-line
  /// usually cannot be modified. This method must be called on the browser
  /// process UI thread.
  pub fn can_set_preference(&mut self, name: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().can_set_preference {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Set the |value| associated with preference |name|. Returns true if the
  /// value is set successfully and false otherwise. If |value| is NULL the
  /// preference will be restored to its default value. If setting the preference
  /// fails then |error| will be populated with a detailed description of the
  /// problem. This method must be called on the browser process UI thread.
  pub fn set_preference(&mut self, name: &crate::include::internal::CefString, value: Option<crate::include::CefValue>, error: &mut crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_preference {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),value.map_or(std::ptr::null_mut(), |o| crate::include::CefValue::to_cef_own(o)),crate::include::internal::IntoCef::into_cef(error),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Clears all certificate exceptions that were added as part of handling
  /// CefRequestHandler::OnCertificateError(). If you call this it is
  /// recommended that you also call CloseAllConnections() or you risk not
  /// being prompted again for server certificates if you reconnect quickly.
  /// If |callback| is non-NULL it will be executed on the UI thread after
  /// completion.
  pub fn clear_certificate_exceptions(&mut self, callback: Option<crate::include::CefCompletionCallback>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().clear_certificate_exceptions {
        Some(f) => f(self.raw.as_ptr(),callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  /// Clears all HTTP authentication credentials that were added as part of
  /// handling GetAuthCredentials. If |callback| is non-NULL it will be executed
  /// on the UI thread after completion.
  pub fn clear_http_auth_credentials(&mut self, callback: Option<crate::include::CefCompletionCallback>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().clear_http_auth_credentials {
        Some(f) => f(self.raw.as_ptr(),callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  /// Clears all active and idle connections that Chromium currently has.
  /// This is only recommended if you have released all other CEF objects but
  /// don't yet want to call CefShutdown(). If |callback| is non-NULL it will be
  /// executed on the UI thread after completion.
  pub fn close_all_connections(&mut self, callback: Option<crate::include::CefCompletionCallback>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().close_all_connections {
        Some(f) => f(self.raw.as_ptr(),callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  /// Attempts to resolve |origin| to a list of associated IP addresses.
  /// |callback| will be executed on the UI thread after completion.
  pub fn resolve_host(&mut self, origin: &crate::include::internal::CefString, callback: crate::include::CefResolveCallback) -> () {
    unsafe {
      let ret = match self.raw.as_ref().resolve_host {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(origin),crate::include::CefResolveCallback::to_cef_own(callback),),
        None => panic!(),
      };
      ret
    }
  }
  /// Load an extension.
  /// 
  /// If extension resources will be read from disk using the default load
  /// implementation then |root_directory| should be the absolute path to the
  /// extension resources directory and |manifest| should be NULL. If extension
  /// resources will be provided by the client (e.g. via CefRequestHandler and/or
  /// CefExtensionHandler) then |root_directory| should be a path component
  /// unique to the extension (if not absolute this will be internally prefixed
  /// with the PK_DIR_RESOURCES path) and |manifest| should contain the contents
  /// that would otherwise be read from the "manifest.json" file on disk.
  /// 
  /// The loaded extension will be accessible in all contexts sharing the same
  /// storage (HasExtension returns true). However, only the context on which
  /// this method was called is considered the loader (DidLoadExtension returns
  /// true) and only the loader will receive CefRequestContextHandler callbacks
  /// for the extension.
  /// 
  /// CefExtensionHandler::OnExtensionLoaded will be called on load success or
  /// CefExtensionHandler::OnExtensionLoadFailed will be called on load failure.
  /// 
  /// If the extension specifies a background script via the "background"
  /// manifest key then CefExtensionHandler::OnBeforeBackgroundBrowser will be
  /// called to create the background browser. See that method for additional
  /// information about background scripts.
  /// 
  /// For visible extension views the client application should evaluate the
  /// manifest to determine the correct extension URL to load and then pass that
  /// URL to the CefBrowserHost::CreateBrowser* function after the extension has
  /// loaded. For example, the client can look for the "browser_action" manifest
  /// key as documented at https://developer.chrome.com/extensions/browserAction.
  /// Extension URLs take the form "chrome-extension://<extension_id>/<path>".
  /// 
  /// Browsers that host extensions differ from normal browsers as follows:
  /// - Can access chrome.* JavaScript APIs if allowed by the manifest. Visit
  /// chrome://extensions-support for the list of extension APIs currently
  /// supported by CEF.
  /// - Main frame navigation to non-extension content is blocked.
  /// - Pinch-zooming is disabled.
  /// - CefBrowserHost::GetExtension returns the hosted extension.
  /// - CefBrowserHost::IsBackgroundHost returns true for background hosts.
  /// 
  /// See https://developer.chrome.com/extensions for extension implementation
  /// and usage documentation.
  pub fn load_extension(&mut self, root_directory: &crate::include::internal::CefString, manifest: Option<crate::include::CefDictionaryValue>, handler: Option<crate::include::CefExtensionHandler>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().load_extension {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(root_directory),manifest.map_or(std::ptr::null_mut(), |o| crate::include::CefDictionaryValue::to_cef_own(o)),handler.map_or(std::ptr::null_mut(), |o| crate::include::CefExtensionHandler::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if this context was used to load the extension identified by
  /// |extension_id|. Other contexts sharing the same storage will also have
  /// access to the extension (see HasExtension). This method must be called on
  /// the browser process UI thread.
  pub fn did_load_extension(&mut self, extension_id: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().did_load_extension {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(extension_id),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this context has access to the extension identified by
  /// |extension_id|. This may not be the context that was used to load the
  /// extension (see DidLoadExtension). This method must be called on the browser
  /// process UI thread.
  pub fn has_extension(&mut self, extension_id: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_extension {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(extension_id),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the extension matching |extension_id| or NULL if no matching
  /// extension is accessible in this context (see HasExtension). This method
  /// must be called on the browser process UI thread.
  pub fn get_extension(&mut self, extension_id: &crate::include::internal::CefString) -> Option<crate::include::CefExtension> {
    unsafe {
      let ret = match self.raw.as_ref().get_extension {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(extension_id),),
        None => panic!(),
      };
      crate::include::CefExtension::from_cef_own(ret)
    }
  }
  /// Returns the MediaRouter object associated with this context.
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
