pub type CefGetExtensionResourceCallback = crate::include::refcounting::CefProxy<cef_sys::cef_get_extension_resource_callback_t>;
#[allow(non_snake_case)]
impl CefGetExtensionResourceCallback {
  /// Continue the request. Read the resource contents from |stream|.
  pub fn cont(&mut self, stream: Option<crate::include::CefStreamReader>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),stream.map_or(std::ptr::null_mut(), |o| crate::include::CefStreamReader::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  /// Cancel the request.
  pub fn cancel(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cancel {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Implement this interface to handle events related to browser extensions.
/// The methods of this class will be called on the UI thread. See
/// CefRequestContext::LoadExtension for information about extension loading.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait ExtensionHandler {
  /// Called if the CefRequestContext::LoadExtension request fails. |result| will
  /// be the error code.
  fn on_extension_load_failed(&mut self, result: crate::include::internal::CefErrorcode) -> () { Default::default() }
  /// Called if the CefRequestContext::LoadExtension request succeeds.
  /// |extension| is the loaded extension.
  fn on_extension_loaded(&mut self, extension: crate::include::CefExtension) -> () { Default::default() }
  /// Called after the CefExtension::Unload request has completed.
  fn on_extension_unloaded(&mut self, extension: crate::include::CefExtension) -> () { Default::default() }
  /// Called when an extension needs a browser to host a background script
  /// specified via the "background" manifest key. The browser will have no
  /// visible window and cannot be displayed. |extension| is the extension that
  /// is loading the background script. |url| is an internally generated
  /// reference to an HTML page that will be used to load the background script
  /// via a <script> src attribute. To allow creation of the browser optionally
  /// modify |client| and |settings| and return false. To cancel creation of the
  /// browser (and consequently cancel load of the background script) return
  /// true. Successful creation will be indicated by a call to
  /// CefLifeSpanHandler::OnAfterCreated, and CefBrowserHost::IsBackgroundHost
  /// will return true for the resulting browser. See
  /// https://developer.chrome.com/extensions/event_pages for more information
  /// about extension background script usage.
  fn on_before_background_browser(&mut self, extension: crate::include::CefExtension, url: &crate::include::internal::CefString, client: &mut crate::include::CefClient, settings: &mut crate::include::internal::CefBrowserSettings) -> bool { Default::default() }
  /// Called when an extension API (e.g. chrome.tabs.create) requests creation of
  /// a new browser. |extension| and |browser| are the source of the API call.
  /// |active_browser| may optionally be specified via the windowId property or
  /// returned via the GetActiveBrowser() callback and provides the default
  /// |client| and |settings| values for the new browser. |index| is the position
  /// value optionally specified via the index property. |url| is the URL that
  /// will be loaded in the browser. |active| is true if the new browser should
  /// be active when opened.  To allow creation of the browser optionally modify
  /// |windowInfo|, |client| and |settings| and return false. To cancel creation
  /// of the browser return true. Successful creation will be indicated by a call
  /// to CefLifeSpanHandler::OnAfterCreated. Any modifications to |windowInfo|
  /// will be ignored if |active_browser| is wrapped in a CefBrowserView.
  fn on_before_browser(&mut self, extension: crate::include::CefExtension, browser: crate::include::CefBrowser, active_browser: crate::include::CefBrowser, index: i32, url: &crate::include::internal::CefString, active: bool, windowInfo: &mut crate::include::internal::CefWindowInfo, client: &mut crate::include::CefClient, settings: &mut crate::include::internal::CefBrowserSettings) -> bool { Default::default() }
  /// Called when no tabId is specified to an extension API call that accepts a
  /// tabId parameter (e.g. chrome.tabs.*). |extension| and |browser| are the
  /// source of the API call. Return the browser that will be acted on by the API
  /// call or return NULL to act on |browser|. The returned browser must share
  /// the same CefRequestContext as |browser|. Incognito browsers should not be
  /// considered unless the source extension has incognito access enabled, in
  /// which case |include_incognito| will be true.
  fn get_active_browser(&mut self, extension: crate::include::CefExtension, browser: crate::include::CefBrowser, include_incognito: bool) -> Option<crate::include::CefBrowser> { Default::default() }
  /// Called when the tabId associated with |target_browser| is specified to an
  /// extension API call that accepts a tabId parameter (e.g. chrome.tabs.*).
  /// |extension| and |browser| are the source of the API call. Return true
  /// to allow access of false to deny access. Access to incognito browsers
  /// should not be allowed unless the source extension has incognito access
  /// enabled, in which case |include_incognito| will be true.
  fn can_access_browser(&mut self, extension: crate::include::CefExtension, browser: crate::include::CefBrowser, include_incognito: bool, target_browser: crate::include::CefBrowser) -> bool { Default::default() }
  /// Called to retrieve an extension resource that would normally be loaded from
  /// disk (e.g. if a file parameter is specified to chrome.tabs.executeScript).
  /// |extension| and |browser| are the source of the resource request. |file| is
  /// the requested relative file path. To handle the resource request return
  /// true and execute |callback| either synchronously or asynchronously. For the
  /// default behavior which reads the resource from the extension directory on
  /// disk return false. Localization substitutions will not be applied to
  /// resources handled via this method.
  fn get_extension_resource(&mut self, extension: crate::include::CefExtension, browser: crate::include::CefBrowser, file: &crate::include::internal::CefString, callback: crate::include::CefGetExtensionResourceCallback) -> bool { Default::default() }
}
define_refcounted!(ExtensionHandler, CefExtensionHandler, cef_extension_handler_t, on_extension_load_failed: cef_extension_handler_t_on_extension_load_failed,on_extension_loaded: cef_extension_handler_t_on_extension_loaded,on_extension_unloaded: cef_extension_handler_t_on_extension_unloaded,on_before_background_browser: cef_extension_handler_t_on_before_background_browser,on_before_browser: cef_extension_handler_t_on_before_browser,get_active_browser: cef_extension_handler_t_get_active_browser,can_access_browser: cef_extension_handler_t_can_access_browser,get_extension_resource: cef_extension_handler_t_get_extension_resource,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_on_extension_load_failed(_self: *mut cef_sys::cef_extension_handler_t, result: cef_sys::cef_errorcode_t) -> () {
  let ret = CefExtensionHandler::from_cef(_self, true).get().on_extension_load_failed(result.into(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_on_extension_loaded(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t) -> () {
  let ret = CefExtensionHandler::from_cef(_self, true).get().on_extension_loaded(crate::include::CefExtension::from_cef_own(extension).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_on_extension_unloaded(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t) -> () {
  let ret = CefExtensionHandler::from_cef(_self, true).get().on_extension_unloaded(crate::include::CefExtension::from_cef_own(extension).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_on_before_background_browser(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t, url: *const cef_sys::cef_string_t, client: *mut *mut cef_sys::cef_client_t, settings: *mut cef_sys::cef_browser_settings_t) -> i32 {
  let mut client__tmp = crate::include::CefClient::from_cef_own(*client).unwrap();
  let ret = CefExtensionHandler::from_cef(_self, true).get().on_before_background_browser(crate::include::CefExtension::from_cef_own(extension).unwrap(),&*(url as *const _),&mut client__tmp,&mut *(settings as *mut _),);
  *client = crate::include::CefClient::to_cef_own(client__tmp);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_on_before_browser(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t, browser: *mut cef_sys::cef_browser_t, active_browser: *mut cef_sys::cef_browser_t, index: i32, url: *const cef_sys::cef_string_t, active: i32, windowInfo: *mut cef_sys::cef_window_info_t, client: *mut *mut cef_sys::cef_client_t, settings: *mut cef_sys::cef_browser_settings_t) -> i32 {
  let mut client__tmp = crate::include::CefClient::from_cef_own(*client).unwrap();
  let ret = CefExtensionHandler::from_cef(_self, true).get().on_before_browser(crate::include::CefExtension::from_cef_own(extension).unwrap(),crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefBrowser::from_cef_own(active_browser).unwrap(),index,&*(url as *const _),if active == 0 { false } else { true },&mut *(windowInfo as *mut _),&mut client__tmp,&mut *(settings as *mut _),);
  *client = crate::include::CefClient::to_cef_own(client__tmp);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_get_active_browser(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t, browser: *mut cef_sys::cef_browser_t, include_incognito: i32) -> *mut cef_sys::cef_browser_t {
  let ret = CefExtensionHandler::from_cef(_self, true).get().get_active_browser(crate::include::CefExtension::from_cef_own(extension).unwrap(),crate::include::CefBrowser::from_cef_own(browser).unwrap(),if include_incognito == 0 { false } else { true },);
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefBrowser::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_can_access_browser(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t, browser: *mut cef_sys::cef_browser_t, include_incognito: i32, target_browser: *mut cef_sys::cef_browser_t) -> i32 {
  let ret = CefExtensionHandler::from_cef(_self, true).get().can_access_browser(crate::include::CefExtension::from_cef_own(extension).unwrap(),crate::include::CefBrowser::from_cef_own(browser).unwrap(),if include_incognito == 0 { false } else { true },crate::include::CefBrowser::from_cef_own(target_browser).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_get_extension_resource(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t, browser: *mut cef_sys::cef_browser_t, file: *const cef_sys::cef_string_t, callback: *mut cef_sys::cef_get_extension_resource_callback_t) -> i32 {
  let ret = CefExtensionHandler::from_cef(_self, true).get().get_extension_resource(crate::include::CefExtension::from_cef_own(extension).unwrap(),crate::include::CefBrowser::from_cef_own(browser).unwrap(),&*(file as *const _),crate::include::CefGetExtensionResourceCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
