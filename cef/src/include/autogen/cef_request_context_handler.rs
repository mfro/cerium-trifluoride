/// Implement this interface to provide handler implementations. The handler
/// instance will not be released until all objects related to the context have
/// been destroyed.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait RequestContextHandler {
  /// Called on the browser process UI thread immediately after the request
  /// context has been initialized.
  fn on_request_context_initialized(&mut self, request_context: crate::include::CefRequestContext) -> () { Default::default() }
  /// Called on multiple browser process threads before a plugin instance is
  /// loaded. |mime_type| is the mime type of the plugin that will be loaded.
  /// |plugin_url| is the content URL that the plugin will load and may be empty.
  /// |is_main_frame| will be true if the plugin is being loaded in the main
  /// (top-level) frame, |top_origin_url| is the URL for the top-level frame that
  /// contains the plugin when loading a specific plugin instance or empty when
  /// building the initial list of enabled plugins for 'navigator.plugins'
  /// JavaScript state. |plugin_info| includes additional information about the
  /// plugin that will be loaded. |plugin_policy| is the recommended policy.
  /// Modify |plugin_policy| and return true to change the policy. Return false
  /// to use the recommended policy. The default plugin policy can be set at
  /// runtime using the `--plugin-policy=[allow|detect|block]` command-line flag.
  /// Decisions to mark a plugin as disabled by setting |plugin_policy| to
  /// PLUGIN_POLICY_DISABLED may be cached when |top_origin_url| is empty. To
  /// purge the plugin list cache and potentially trigger new calls to this
  /// method call CefRequestContext::PurgePluginListCache.
  fn on_before_plugin_load(&mut self, mime_type: &crate::include::internal::CefString, plugin_url: Option<&crate::include::internal::CefString>, is_main_frame: bool, top_origin_url: Option<&crate::include::internal::CefString>, plugin_info: crate::include::CefWebPluginInfo, plugin_policy: &mut crate::include::internal::CefPluginPolicy) -> bool { Default::default() }
  /// Called on the browser process IO thread before a resource request is
  /// initiated. The |browser| and |frame| values represent the source of the
  /// request, and may be NULL for requests originating from service workers or
  /// CefURLRequest. |request| represents the request contents and cannot be
  /// modified in this callback. |is_navigation| will be true if the resource
  /// request is a navigation. |is_download| will be true if the resource request
  /// is a download. |request_initiator| is the origin (scheme + domain) of the
  /// page that initiated the request. Set |disable_default_handling| to true to
  /// disable default handling of the request, in which case it will need to be
  /// handled via CefResourceRequestHandler::GetResourceHandler or it will be
  /// canceled. To allow the resource load to proceed with default handling
  /// return NULL. To specify a handler for the resource return a
  /// CefResourceRequestHandler object. This method will not be called if the
  /// client associated with |browser| returns a non-NULL value from
  /// CefRequestHandler::GetResourceRequestHandler for the same request
  /// (identified by CefRequest::GetIdentifier).
  fn get_resource_request_handler(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, request: crate::include::CefRequest, is_navigation: bool, is_download: bool, request_initiator: Option<&crate::include::internal::CefString>, disable_default_handling: &mut bool) -> Option<crate::include::CefResourceRequestHandler> { Default::default() }
}
define_refcounted!(RequestContextHandler, CefRequestContextHandler, cef_request_context_handler_t, on_request_context_initialized: cef_request_context_handler_t_on_request_context_initialized,on_before_plugin_load: cef_request_context_handler_t_on_before_plugin_load,get_resource_request_handler: cef_request_context_handler_t_get_resource_request_handler,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_context_handler_t_on_request_context_initialized(_self: *mut cef_sys::cef_request_context_handler_t, request_context: *mut cef_sys::cef_request_context_t) -> () {
  let ret = CefRequestContextHandler::from_cef(_self, true).get().on_request_context_initialized(crate::include::CefRequestContext::from_cef_own(request_context).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_context_handler_t_on_before_plugin_load(_self: *mut cef_sys::cef_request_context_handler_t, mime_type: *const cef_sys::cef_string_t, plugin_url: *const cef_sys::cef_string_t, is_main_frame: i32, top_origin_url: *const cef_sys::cef_string_t, plugin_info: *mut cef_sys::cef_web_plugin_info_t, plugin_policy: *mut cef_sys::cef_plugin_policy_t) -> i32 {
  let ret = CefRequestContextHandler::from_cef(_self, true).get().on_before_plugin_load(&*(mime_type as *const _),if plugin_url.is_null() { None } else { Some(&*(plugin_url as *const _)) },if is_main_frame == 0 { false } else { true },if top_origin_url.is_null() { None } else { Some(&*(top_origin_url as *const _)) },crate::include::CefWebPluginInfo::from_cef_own(plugin_info).unwrap(),&mut *(plugin_policy as *mut _),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_context_handler_t_get_resource_request_handler(_self: *mut cef_sys::cef_request_context_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, is_navigation: i32, is_download: i32, request_initiator: *const cef_sys::cef_string_t, disable_default_handling: *mut i32) -> *mut cef_sys::cef_resource_request_handler_t {
  let mut disable_default_handling__tmp = if *disable_default_handling == 0 { false } else { true };
  let ret = CefRequestContextHandler::from_cef(_self, true).get().get_resource_request_handler(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),crate::include::CefRequest::from_cef_own(request).unwrap(),if is_navigation == 0 { false } else { true },if is_download == 0 { false } else { true },if request_initiator.is_null() { None } else { Some(&*(request_initiator as *const _)) },&mut disable_default_handling__tmp,);
  *disable_default_handling = if disable_default_handling__tmp { 1 } else { 0 };
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefResourceRequestHandler::to_cef_own(o))
}
