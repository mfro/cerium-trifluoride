pub type CefMediaRouter = crate::include::base::CefProxy<cef_sys::cef_media_router_t>;
#[allow(non_snake_case)]
impl CefMediaRouter {
  pub fn add_observer(&mut self, observer: crate::include::CefMediaObserver) -> Option<crate::include::CefRegistration> {
    unsafe {
      let ret = match self.raw.as_ref().add_observer {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefMediaObserver::to_cef_own(observer),),
        None => panic!(),
      };
      crate::include::CefRegistration::from_cef_own(ret)
    }
  }
  pub fn get_source(&mut self, urn: &crate::include::internal::CefString) -> Option<crate::include::CefMediaSource> {
    unsafe {
      let ret = match self.raw.as_ref().get_source {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(urn),),
        None => panic!(),
      };
      crate::include::CefMediaSource::from_cef_own(ret)
    }
  }
  pub fn notify_current_sinks(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().notify_current_sinks {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn create_route(&mut self, source: crate::include::CefMediaSource, sink: crate::include::CefMediaSink, callback: crate::include::CefMediaRouteCreateCallback) -> () {
    unsafe {
      let ret = match self.raw.as_ref().create_route {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefMediaSource::to_cef_own(source),crate::include::CefMediaSink::to_cef_own(sink),crate::include::CefMediaRouteCreateCallback::to_cef_own(callback),),
        None => panic!(),
      };
      ret
    }
  }
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
pub trait MediaObserver {
  fn on_route_state_changed(&mut self, route: crate::include::CefMediaRoute, state: crate::include::internal::CefMediaRouteConnectionState) -> () { Default::default() }
}
define_refcounted!(MediaObserver, media_observer, on_route_state_changed,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_media_observer_t_on_route_state_changed(_self: *mut cef_sys::cef_media_observer_t, route: *mut cef_sys::cef_media_route_t, state: cef_sys::cef_media_route_connection_state_t) -> () {
  let ret = CefMediaObserver::from_cef(_self, true).get().on_route_state_changed(crate::include::CefMediaRoute::from_cef_own(route).unwrap(),state.into(),);
  ret
}
pub type CefMediaRoute = crate::include::base::CefProxy<cef_sys::cef_media_route_t>;
#[allow(non_snake_case)]
impl CefMediaRoute {
  pub fn get_id(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_id {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_source(&mut self) -> Option<crate::include::CefMediaSource> {
    unsafe {
      let ret = match self.raw.as_ref().get_source {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefMediaSource::from_cef_own(ret)
    }
  }
  pub fn get_sink(&mut self) -> Option<crate::include::CefMediaSink> {
    unsafe {
      let ret = match self.raw.as_ref().get_sink {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefMediaSink::from_cef_own(ret)
    }
  }
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
pub trait MediaRouteCreateCallback {
  fn on_media_route_create_finished(&mut self, result: crate::include::internal::CefMediaRouteCreateResult, error: &crate::include::internal::CefString, route: crate::include::CefMediaRoute) -> () { Default::default() }
}
define_refcounted!(MediaRouteCreateCallback, media_route_create_callback, on_media_route_create_finished,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_media_route_create_callback_t_on_media_route_create_finished(_self: *mut cef_sys::cef_media_route_create_callback_t, result: cef_sys::cef_media_route_create_result_t, error: *const cef_sys::cef_string_t, route: *mut cef_sys::cef_media_route_t) -> () {
  let ret = CefMediaRouteCreateCallback::from_cef(_self, true).get().on_media_route_create_finished(result.into(),&crate::include::internal::CefString::from_cef(error).unwrap(),crate::include::CefMediaRoute::from_cef_own(route).unwrap(),);
  ret
}
pub type CefMediaSink = crate::include::base::CefProxy<cef_sys::cef_media_sink_t>;
#[allow(non_snake_case)]
impl CefMediaSink {
  pub fn get_id(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_id {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_description(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_description {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_icon_type(&mut self) -> crate::include::internal::CefMediaSinkIconType {
    unsafe {
      let ret = match self.raw.as_ref().get_icon_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_device_info(&mut self, callback: crate::include::CefMediaSinkDeviceInfoCallback) -> () {
    unsafe {
      let ret = match self.raw.as_ref().get_device_info {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefMediaSinkDeviceInfoCallback::to_cef_own(callback),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_cast_sink(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_cast_sink {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_dial_sink(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_dial_sink {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
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
pub trait MediaSinkDeviceInfoCallback {
  fn on_media_sink_device_info(&mut self, device_info: &crate::include::internal::CefMediaSinkDeviceInfo) -> () { Default::default() }
}
define_refcounted!(MediaSinkDeviceInfoCallback, media_sink_device_info_callback, on_media_sink_device_info,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_media_sink_device_info_callback_t_on_media_sink_device_info(_self: *mut cef_sys::cef_media_sink_device_info_callback_t, device_info: *const cef_sys::cef_media_sink_device_info_t) -> () {
  let ret = CefMediaSinkDeviceInfoCallback::from_cef(_self, true).get().on_media_sink_device_info(&*(device_info as *const _),);
  ret
}
pub type CefMediaSource = crate::include::base::CefProxy<cef_sys::cef_media_source_t>;
#[allow(non_snake_case)]
impl CefMediaSource {
  pub fn get_id(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_id {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn is_cast_source(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_cast_source {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
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
