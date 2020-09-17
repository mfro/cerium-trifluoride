/// Implement this interface to filter resource response content. The methods of
/// this class will be called on the browser process IO thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait ResponseFilter {
  /// Initialize the response filter. Will only be called a single time. The
  /// filter will not be installed if this method returns false.
  fn init_filter(&mut self) -> bool { Default::default() }
  /// Called to filter a chunk of data. Expected usage is as follows:
  /// 
  /// A. Read input data from |data_in| and set |data_in_read| to the number of
  /// bytes that were read up to a maximum of |data_in_size|. |data_in| will
  /// be NULL if |data_in_size| is zero.
  /// B. Write filtered output data to |data_out| and set |data_out_written| to
  /// the number of bytes that were written up to a maximum of
  /// |data_out_size|. If no output data was written then all data must be
  /// read from |data_in| (user must set |data_in_read| = |data_in_size|).
  /// C. Return RESPONSE_FILTER_DONE if all output data was written or
  /// RESPONSE_FILTER_NEED_MORE_DATA if output data is still pending.
  /// 
  /// This method will be called repeatedly until the input buffer has been
  /// fully read (user sets |data_in_read| = |data_in_size|) and there is no
  /// more input data to filter (the resource response is complete). This method
  /// may then be called an additional time with an empty input buffer if the
  /// user filled the output buffer (set |data_out_written| = |data_out_size|)
  /// and returned RESPONSE_FILTER_NEED_MORE_DATA to indicate that output data is
  /// still pending.
  /// 
  /// Calls to this method will stop when one of the following conditions is met:
  /// 
  /// A. There is no more input data to filter (the resource response is
  /// complete) and the user sets |data_out_written| = 0 or returns
  /// RESPONSE_FILTER_DONE to indicate that all data has been written, or;
  /// B. The user returns RESPONSE_FILTER_ERROR to indicate an error.
  /// 
  /// Do not keep a reference to the buffers passed to this method.
  fn filter(&mut self, data_in: &mut [u8], data_in_read: &mut u64, data_out: &mut [u8], data_out_written: &mut u64) -> crate::include::internal::CefResponseFilterStatus { Default::default() }
}
define_refcounted!(ResponseFilter, CefResponseFilter, cef_response_filter_t, init_filter: cef_response_filter_t_init_filter,filter: cef_response_filter_t_filter,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_response_filter_t_init_filter(_self: *mut cef_sys::cef_response_filter_t) -> i32 {
  let ret = CefResponseFilter::from_cef(_self, true).get().init_filter();
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_response_filter_t_filter(_self: *mut cef_sys::cef_response_filter_t, data_in0: *mut std::os::raw::c_void, data_in1: u64, data_in_read: *mut u64, data_out0: *mut std::os::raw::c_void, data_out1: u64, data_out_written: *mut u64) -> cef_sys::cef_response_filter_status_t {
  let ret = CefResponseFilter::from_cef(_self, true).get().filter(std::slice::from_raw_parts_mut(data_in0 as *mut _, data_in1 as _),&mut *data_in_read,std::slice::from_raw_parts_mut(data_out0 as *mut _, data_out1 as _),&mut *data_out_written,);
  ret.into()
}
