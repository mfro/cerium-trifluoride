/// Implement this interface to provide handler implementations. Methods will be
/// called by the process and/or thread indicated.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait App {
  /// Provides an opportunity to view and/or modify command-line arguments before
  /// processing by CEF and Chromium. The |process_type| value will be empty for
  /// the browser process. Do not keep a reference to the CefCommandLine object
  /// passed to this method. The CefSettings.command_line_args_disabled value
  /// can be used to start with an empty command-line object. Any values
  /// specified in CefSettings that equate to command-line arguments will be set
  /// before this method is called. Be cautious when using this method to modify
  /// command-line arguments for non-browser processes as this may result in
  /// undefined behavior including crashes.
  fn on_before_command_line_processing(&mut self, process_type: Option<&crate::include::internal::CefString>, command_line: crate::include::CefCommandLine) -> () { Default::default() }
  /// Provides an opportunity to register custom schemes. Do not keep a reference
  /// to the |registrar| object. This method is called on the main thread for
  /// each process and the registered schemes should be the same across all
  /// processes.
  fn on_register_custom_schemes(&mut self, registrar: crate::include::CefSchemeRegistrar) -> () { Default::default() }
  /// Return the handler for resource bundle events. If
  /// CefSettings.pack_loading_disabled is true a handler must be returned. If no
  /// handler is returned resources will be loaded from pack files. This method
  /// is called by the browser and render processes on multiple threads.
  fn get_resource_bundle_handler(&mut self) -> Option<crate::include::CefResourceBundleHandler> { Default::default() }
  /// Return the handler for functionality specific to the browser process. This
  /// method is called on multiple threads in the browser process.
  fn get_browser_process_handler(&mut self) -> Option<crate::include::CefBrowserProcessHandler> { Default::default() }
  /// Return the handler for functionality specific to the render process. This
  /// method is called on the render process main thread.
  fn get_render_process_handler(&mut self) -> Option<crate::include::CefRenderProcessHandler> { Default::default() }
}
define_refcounted!(App, CefApp, cef_app_t, on_before_command_line_processing: cef_app_t_on_before_command_line_processing,on_register_custom_schemes: cef_app_t_on_register_custom_schemes,get_resource_bundle_handler: cef_app_t_get_resource_bundle_handler,get_browser_process_handler: cef_app_t_get_browser_process_handler,get_render_process_handler: cef_app_t_get_render_process_handler,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_app_t_on_before_command_line_processing(_self: *mut cef_sys::cef_app_t, process_type: *const cef_sys::cef_string_t, command_line: *mut cef_sys::cef_command_line_t) -> () {
  let ret = CefApp::from_cef(_self, true).get().on_before_command_line_processing(match &crate::include::internal::CefString::from_cef(process_type) { Some(ref x) => Some(x), None => None },crate::include::CefCommandLine::from_cef_own(command_line).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_app_t_on_register_custom_schemes(_self: *mut cef_sys::cef_app_t, registrar: *mut cef_sys::cef_scheme_registrar_t) -> () {
  let ret = CefApp::from_cef(_self, true).get().on_register_custom_schemes(crate::include::CefSchemeRegistrar::from_cef_ref(registrar).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_app_t_get_resource_bundle_handler(_self: *mut cef_sys::cef_app_t) -> *mut cef_sys::cef_resource_bundle_handler_t {
  let ret = CefApp::from_cef(_self, true).get().get_resource_bundle_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefResourceBundleHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_app_t_get_browser_process_handler(_self: *mut cef_sys::cef_app_t) -> *mut cef_sys::cef_browser_process_handler_t {
  let ret = CefApp::from_cef(_self, true).get().get_browser_process_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefBrowserProcessHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_app_t_get_render_process_handler(_self: *mut cef_sys::cef_app_t) -> *mut cef_sys::cef_render_process_handler_t {
  let ret = CefApp::from_cef(_self, true).get().get_render_process_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefRenderProcessHandler::to_cef_own(o))
}
/// This function should be called from the application entry point function to
/// execute a secondary process. It can be used to run secondary processes from
/// the browser client executable (default behavior) or from a separate
/// executable specified by the CefSettings.browser_subprocess_path value. If
/// called for the browser process (identified by no "type" command-line value)
/// it will return immediately with a value of -1. If called for a recognized
/// secondary process it will block until the process should exit and then return
/// the process exit code. The |application| parameter may be empty. The
/// |windows_sandbox_info| parameter is only used on Windows and may be NULL (see
/// cef_sandbox_win.h for details).
#[allow(non_snake_case)]
pub fn cef_execute_process(args: &crate::include::internal::CefMainArgs, application: Option<crate::include::CefApp>, windows_sandbox_info: Option<&mut std::os::raw::c_void>, ) -> i32 {
  unsafe {
    let ret = cef_sys::cef_execute_process(args as *const _ as *const _,application.map_or(std::ptr::null_mut(), |o| crate::include::CefApp::to_cef_own(o)),match windows_sandbox_info { Some(windows_sandbox_info) => windows_sandbox_info, None => std::ptr::null_mut() },);
    ret
  }
}
/// This function should be called on the main application thread to initialize
/// the CEF browser process. The |application| parameter may be empty. A return
/// value of true indicates that it succeeded and false indicates that it failed.
/// The |windows_sandbox_info| parameter is only used on Windows and may be NULL
/// (see cef_sandbox_win.h for details).
#[allow(non_snake_case)]
pub fn cef_initialize(args: &crate::include::internal::CefMainArgs, settings: &crate::include::internal::CefSettings, application: Option<crate::include::CefApp>, windows_sandbox_info: Option<&mut std::os::raw::c_void>, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_initialize(args as *const _ as *const _,settings as *const _ as *const _,application.map_or(std::ptr::null_mut(), |o| crate::include::CefApp::to_cef_own(o)),match windows_sandbox_info { Some(windows_sandbox_info) => windows_sandbox_info, None => std::ptr::null_mut() },);
    if ret == 0 { false } else { true }
  }
}
/// This function should be called on the main application thread to shut down
/// the CEF browser process before the application exits.
#[allow(non_snake_case)]
pub fn cef_shutdown() -> () {
  unsafe {
    let ret = cef_sys::cef_shutdown();
    ret
  }
}
/// Perform a single iteration of CEF message loop processing. This function is
/// provided for cases where the CEF message loop must be integrated into an
/// existing application message loop. Use of this function is not recommended
/// for most users; use either the CefRunMessageLoop() function or
/// CefSettings.multi_threaded_message_loop if possible. When using this function
/// care must be taken to balance performance against excessive CPU usage. It is
/// recommended to enable the CefSettings.external_message_pump option when using
/// this function so that CefBrowserProcessHandler::OnScheduleMessagePumpWork()
/// callbacks can facilitate the scheduling process. This function should only be
/// called on the main application thread and only if CefInitialize() is called
/// with a CefSettings.multi_threaded_message_loop value of false. This function
/// will not block.
#[allow(non_snake_case)]
pub fn cef_do_message_loop_work() -> () {
  unsafe {
    let ret = cef_sys::cef_do_message_loop_work();
    ret
  }
}
/// Run the CEF message loop. Use this function instead of an application-
/// provided message loop to get the best balance between performance and CPU
/// usage. This function should only be called on the main application thread and
/// only if CefInitialize() is called with a
/// CefSettings.multi_threaded_message_loop value of false. This function will
/// block until a quit message is received by the system.
#[allow(non_snake_case)]
pub fn cef_run_message_loop() -> () {
  unsafe {
    let ret = cef_sys::cef_run_message_loop();
    ret
  }
}
/// Quit the CEF message loop that was started by calling CefRunMessageLoop().
/// This function should only be called on the main application thread and only
/// if CefRunMessageLoop() was used.
#[allow(non_snake_case)]
pub fn cef_quit_message_loop() -> () {
  unsafe {
    let ret = cef_sys::cef_quit_message_loop();
    ret
  }
}
/// Set to true before calling Windows APIs like TrackPopupMenu that enter a
/// modal message loop. Set to false after exiting the modal message loop.
#[allow(non_snake_case)]
pub fn cef_set_osmodal_loop(osModalLoop: bool, ) -> () {
  unsafe {
    let ret = cef_sys::cef_set_osmodal_loop(if osModalLoop { 1 } else { 0 },);
    ret
  }
}
/// Call during process startup to enable High-DPI support on Windows 7 or newer.
/// Older versions of Windows should be left DPI-unaware because they do not
/// support DirectWrite and GDI fonts are kerned very badly.
#[allow(non_snake_case)]
pub fn cef_enable_highdpi_support() -> () {
  unsafe {
    let ret = cef_sys::cef_enable_highdpi_support();
    ret
  }
}
