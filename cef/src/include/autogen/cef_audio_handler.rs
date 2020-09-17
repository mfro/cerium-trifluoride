/// Implement this interface to handle audio events.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait AudioHandler {
  /// Called on the UI thread to allow configuration of audio stream parameters.
  /// Return true to proceed with audio stream capture, or false to cancel it.
  /// All members of |params| can optionally be configured here, but they are
  /// also pre-filled with some sensible defaults.
  fn get_audio_parameters(&mut self, browser: crate::include::CefBrowser, params: &mut crate::include::internal::CefAudioParameters) -> bool { Default::default() }
  /// Called on a browser audio capture thread when the browser starts
  /// streaming audio. OnAudioSteamStopped will always be called after
  /// OnAudioStreamStarted; both methods may be called multiple times
  /// for the same browser. |params| contains the audio parameters like
  /// sample rate and channel layout. |channels| is the number of channels.
  fn on_audio_stream_started(&mut self, browser: crate::include::CefBrowser, params: &crate::include::internal::CefAudioParameters, channels: i32) -> () { Default::default() }
  /// Called on the UI thread when the stream has stopped. OnAudioSteamStopped
  /// will always be called after OnAudioStreamStarted; both methods may be
  /// called multiple times for the same stream.
  fn on_audio_stream_stopped(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  /// Called on the UI or audio stream thread when an error occurred. During the
  /// stream creation phase this callback will be called on the UI thread while
  /// in the capturing phase it will be called on the audio stream thread. The
  /// stream will be stopped immediately.
  fn on_audio_stream_error(&mut self, browser: crate::include::CefBrowser, message: &crate::include::internal::CefString) -> () { Default::default() }
}
define_refcounted!(AudioHandler, CefAudioHandler, cef_audio_handler_t, get_audio_parameters: cef_audio_handler_t_get_audio_parameters,on_audio_stream_started: cef_audio_handler_t_on_audio_stream_started,on_audio_stream_stopped: cef_audio_handler_t_on_audio_stream_stopped,on_audio_stream_error: cef_audio_handler_t_on_audio_stream_error,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_audio_handler_t_get_audio_parameters(_self: *mut cef_sys::cef_audio_handler_t, browser: *mut cef_sys::cef_browser_t, params: *mut cef_sys::cef_audio_parameters_t) -> i32 {
  let ret = CefAudioHandler::from_cef(_self, true).get().get_audio_parameters(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&mut *(params as *mut _),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_audio_handler_t_on_audio_stream_started(_self: *mut cef_sys::cef_audio_handler_t, browser: *mut cef_sys::cef_browser_t, params: *const cef_sys::cef_audio_parameters_t, channels: i32) -> () {
  let ret = CefAudioHandler::from_cef(_self, true).get().on_audio_stream_started(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&*(params as *const _),channels,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_audio_handler_t_on_audio_stream_stopped(_self: *mut cef_sys::cef_audio_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefAudioHandler::from_cef(_self, true).get().on_audio_stream_stopped(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_audio_handler_t_on_audio_stream_error(_self: *mut cef_sys::cef_audio_handler_t, browser: *mut cef_sys::cef_browser_t, message: *const cef_sys::cef_string_t) -> () {
  let ret = CefAudioHandler::from_cef(_self, true).get().on_audio_stream_error(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&*(message as *const _),);
  ret
}
