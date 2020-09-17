pub type CefMediaRouter = crate::include::base::CefProxy<cef_sys::cef_media_router_t>;
#[allow(non_snake_case)]
impl CefMediaRouter {
  /// Returns the MediaRouter object associated with the global request context.
  /// Equivalent to calling
  /// CefRequestContext::GetGlobalContext()->GetMediaRouter().
  #[allow(non_snake_case)]
  pub fn get_global_media_router() -> Option<crate::include::CefMediaRouter> {
    unsafe {
      let ret = cef_sys::cef_media_router_get_global();
      crate::include::CefMediaRouter::from_cef_own(ret)
    }
  }
  /// Add an observer for MediaRouter events. The observer will remain registered
  /// until the returned Registration object is destroyed.
  pub fn add_observer(&mut self, observer: crate::include::CefMediaObserver) -> Option<crate::include::CefRegistration> {
    unsafe {
      let ret = match self.raw.as_ref().add_observer {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefMediaObserver::to_cef_own(observer),),
        None => panic!(),
      };
      crate::include::CefRegistration::from_cef_own(ret)
    }
  }
  /// Returns a MediaSource object for the specified media source URN. Supported
  /// URN schemes include "cast:" and "dial:", and will be already known by the
  /// client application (e.g. "cast:<appId>?clientId=<clientId>").
  pub fn get_source(&mut self, urn: &crate::include::internal::CefString) -> Option<crate::include::CefMediaSource> {
    unsafe {
      let ret = match self.raw.as_ref().get_source {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(urn),),
        None => panic!(),
      };
      crate::include::CefMediaSource::from_cef_own(ret)
    }
  }
  /// Trigger an asynchronous call to CefMediaObserver::OnSinks on all
  /// registered observers.
  pub fn notify_current_sinks(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().notify_current_sinks {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Create a new route between |source| and |sink|. Source and sink must be
  /// valid, compatible (as reported by CefMediaSink::IsCompatibleWith), and a
  /// route between them must not already exist. |callback| will be executed
  /// on success or failure. If route creation succeeds it will also trigger an
  /// asynchronous call to CefMediaObserver::OnRoutes on all registered
  /// observers.
  pub fn create_route(&mut self, source: crate::include::CefMediaSource, sink: crate::include::CefMediaSink, callback: crate::include::CefMediaRouteCreateCallback) -> () {
    unsafe {
      let ret = match self.raw.as_ref().create_route {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefMediaSource::to_cef_own(source),crate::include::CefMediaSink::to_cef_own(sink),crate::include::CefMediaRouteCreateCallback::to_cef_own(callback),),
        None => panic!(),
      };
      ret
    }
  }
  /// Trigger an asynchronous call to CefMediaObserver::OnRoutes on all
  /// registered observers.
  pub fn notify_current_routes(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().notify_current_routes {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Implemented by the client to observe MediaRouter events and registered via
/// CefMediaRouter::AddObserver. The methods of this class will be called on the
/// browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait MediaObserver {
  /// The connection state of |route| has changed.
  fn on_route_state_changed(&mut self, route: crate::include::CefMediaRoute, state: crate::include::internal::CefMediaRouteConnectionState) -> () { Default::default() }
}
define_refcounted!(MediaObserver, CefMediaObserver, cef_media_observer_t, on_route_state_changed: cef_media_observer_t_on_route_state_changed,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_media_observer_t_on_route_state_changed(_self: *mut cef_sys::cef_media_observer_t, route: *mut cef_sys::cef_media_route_t, state: cef_sys::cef_media_route_connection_state_t) -> () {
  let ret = CefMediaObserver::from_cef(_self, true).get().on_route_state_changed(crate::include::CefMediaRoute::from_cef_own(route).unwrap(),state.into(),);
  ret
}
pub type CefMediaRoute = crate::include::base::CefProxy<cef_sys::cef_media_route_t>;
#[allow(non_snake_case)]
impl CefMediaRoute {
  /// Returns the ID for this route.
  pub fn get_id(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_id {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the source associated with this route.
  pub fn get_source(&mut self) -> Option<crate::include::CefMediaSource> {
    unsafe {
      let ret = match self.raw.as_ref().get_source {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefMediaSource::from_cef_own(ret)
    }
  }
  /// Returns the sink associated with this route.
  pub fn get_sink(&mut self) -> Option<crate::include::CefMediaSink> {
    unsafe {
      let ret = match self.raw.as_ref().get_sink {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefMediaSink::from_cef_own(ret)
    }
  }
  /// Terminate this route. Will result in an asynchronous call to
  /// CefMediaObserver::OnRoutes on all registered observers.
  pub fn terminate(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().terminate {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Callback interface for CefMediaRouter::CreateRoute. The methods of this
/// class will be called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait MediaRouteCreateCallback {
  /// Method that will be executed when the route creation has finished. |result|
  /// will be CEF_MRCR_OK if the route creation succeeded. |error| will be a
  /// description of the error if the route creation failed. |route| is the
  /// resulting route, or empty if the route creation failed.
  fn on_media_route_create_finished(&mut self, result: crate::include::internal::CefMediaRouteCreateResult, error: Option<&crate::include::internal::CefString>, route: Option<crate::include::CefMediaRoute>) -> () { Default::default() }
}
define_refcounted!(MediaRouteCreateCallback, CefMediaRouteCreateCallback, cef_media_route_create_callback_t, on_media_route_create_finished: cef_media_route_create_callback_t_on_media_route_create_finished,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_media_route_create_callback_t_on_media_route_create_finished(_self: *mut cef_sys::cef_media_route_create_callback_t, result: cef_sys::cef_media_route_create_result_t, error: *const cef_sys::cef_string_t, route: *mut cef_sys::cef_media_route_t) -> () {
  let ret = CefMediaRouteCreateCallback::from_cef(_self, true).get().on_media_route_create_finished(result.into(),match &crate::include::internal::CefString::from_cef(error) { Some(ref x) => Some(x), None => None },crate::include::CefMediaRoute::from_cef_own(route),);
  ret
}
pub type CefMediaSink = crate::include::base::CefProxy<cef_sys::cef_media_sink_t>;
#[allow(non_snake_case)]
impl CefMediaSink {
  /// Returns the ID for this sink.
  pub fn get_id(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_id {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the name of this sink.
  pub fn get_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the description of this sink.
  pub fn get_description(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_description {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the icon type for this sink.
  pub fn get_icon_type(&mut self) -> crate::include::internal::CefMediaSinkIconType {
    unsafe {
      let ret = match self.raw.as_ref().get_icon_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Asynchronously retrieves device info.
  pub fn get_device_info(&mut self, callback: crate::include::CefMediaSinkDeviceInfoCallback) -> () {
    unsafe {
      let ret = match self.raw.as_ref().get_device_info {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefMediaSinkDeviceInfoCallback::to_cef_own(callback),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if this sink accepts content via Cast.
  pub fn is_cast_sink(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_cast_sink {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this sink accepts content via DIAL.
  pub fn is_dial_sink(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_dial_sink {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this sink is compatible with |source|.
  pub fn is_compatible_with(&mut self, source: crate::include::CefMediaSource) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_compatible_with {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefMediaSource::to_cef_own(source),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
/// Callback interface for CefMediaSink::GetDeviceInfo. The methods of this
/// class will be called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait MediaSinkDeviceInfoCallback {
  /// Method that will be executed asyncronously once device information has been
  /// retrieved.
  fn on_media_sink_device_info(&mut self, device_info: &crate::include::internal::CefMediaSinkDeviceInfo) -> () { Default::default() }
}
define_refcounted!(MediaSinkDeviceInfoCallback, CefMediaSinkDeviceInfoCallback, cef_media_sink_device_info_callback_t, on_media_sink_device_info: cef_media_sink_device_info_callback_t_on_media_sink_device_info,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_media_sink_device_info_callback_t_on_media_sink_device_info(_self: *mut cef_sys::cef_media_sink_device_info_callback_t, device_info: *const cef_sys::cef_media_sink_device_info_t) -> () {
  let ret = CefMediaSinkDeviceInfoCallback::from_cef(_self, true).get().on_media_sink_device_info(&*(device_info as *const _),);
  ret
}
pub type CefMediaSource = crate::include::base::CefProxy<cef_sys::cef_media_source_t>;
#[allow(non_snake_case)]
impl CefMediaSource {
  /// Returns the ID (media source URN or URL) for this source.
  pub fn get_id(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_id {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns true if this source outputs its content via Cast.
  pub fn is_cast_source(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_cast_source {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this source outputs its content via DIAL.
  pub fn is_dial_source(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_dial_source {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
