/// Implement this interface to filter resource response content. The methods of
/// this class will be called on the browser process IO thread.
#[allow(non_snake_case)]
pub trait ResponseFilter {
  fn init_filter(&mut self) -> bool { Default::default() }
  fn filter(&mut self, data_in: &mut std::os::raw::c_void, data_in_size: u64, data_in_read: &mut u64, data_out: &mut std::os::raw::c_void, data_out_size: u64, data_out_written: &mut u64) -> crate::include::internal::CefResponseFilterStatus { Default::default() }
}
define_refcounted!(ResponseFilter, response_filter, init_filter,filter,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_response_filter_t_init_filter(_self: *mut cef_sys::cef_response_filter_t) -> i32 {
  let ret = CefResponseFilter::from_cef(_self, true).get().init_filter();
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_response_filter_t_filter(_self: *mut cef_sys::cef_response_filter_t, data_in: *mut std::os::raw::c_void, data_in_size: u64, data_in_read: *mut u64, data_out: *mut std::os::raw::c_void, data_out_size: u64, data_out_written: *mut u64) -> cef_sys::cef_response_filter_status_t {
  let ret = CefResponseFilter::from_cef(_self, true).get().filter(&mut *data_in,data_in_size,&mut *data_in_read,&mut *data_out,data_out_size,&mut *data_out_written,);
  ret.into()
}
