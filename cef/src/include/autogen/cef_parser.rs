/// Parse the specified |url| into its component parts.
/// Returns false if the URL is empty or invalid.
#[allow(non_snake_case)]
pub fn cef_parse_url(url: &crate::include::internal::CefString, parts: &mut crate::include::internal::CefURLParts, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_parse_url(crate::include::internal::IntoCef::into_cef(url),parts as *mut _ as *mut _,);
    if ret == 0 { false } else { true }
  }
}
/// Creates a URL from the specified |parts|, which must contain a non-empty
/// spec or a non-empty host and path (at a minimum), but not both.
/// Returns false if |parts| isn't initialized as described.
#[allow(non_snake_case)]
pub fn cef_create_url(parts: &crate::include::internal::CefURLParts, url: &mut crate::include::internal::CefString, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_create_url(parts as *const _ as *const _,crate::include::internal::IntoCef::into_cef(url),);
    if ret == 0 { false } else { true }
  }
}
/// This is a convenience function for formatting a URL in a concise and human-
/// friendly way to help users make security-related decisions (or in other
/// circumstances when people need to distinguish sites, origins, or otherwise-
/// simplified URLs from each other). Internationalized domain names (IDN) may be
/// presented in Unicode if the conversion is considered safe. The returned value
/// will (a) omit the path for standard schemes, excepting file and filesystem,
/// and (b) omit the port if it is the default for the scheme. Do not use this
/// for URLs which will be parsed or sent to other applications.
#[allow(non_snake_case)]
pub fn cef_format_url_for_security_display(origin_url: &crate::include::internal::CefString, ) -> crate::include::internal::CefString {
  unsafe {
    let ret = cef_sys::cef_format_url_for_security_display(crate::include::internal::IntoCef::into_cef(origin_url),);
    crate::include::internal::CefString::userfree(ret)
  }
}
/// Returns the mime type for the specified file extension or an empty string if
/// unknown.
#[allow(non_snake_case)]
pub fn cef_get_mime_type(extension: &crate::include::internal::CefString, ) -> crate::include::internal::CefString {
  unsafe {
    let ret = cef_sys::cef_get_mime_type(crate::include::internal::IntoCef::into_cef(extension),);
    crate::include::internal::CefString::userfree(ret)
  }
}
/// Decodes the base64 encoded string |data|. The returned value will be NULL if
/// the decoding fails.
#[allow(non_snake_case)]
pub fn cef_base64decode(data: &crate::include::internal::CefString, ) -> Option<crate::include::CefBinaryValue> {
  unsafe {
    let ret = cef_sys::cef_base64decode(crate::include::internal::IntoCef::into_cef(data),);
    crate::include::CefBinaryValue::from_cef_own(ret)
  }
}
/// Escapes characters in |text| which are unsuitable for use as a query
/// parameter value. Everything except alphanumerics and -_.!~*'() will be
/// converted to "%XX". If |use_plus| is true spaces will change to "+". The
/// result is basically the same as encodeURIComponent in Javacript.
#[allow(non_snake_case)]
pub fn cef_uriencode(text: &crate::include::internal::CefString, use_plus: bool, ) -> crate::include::internal::CefString {
  unsafe {
    let ret = cef_sys::cef_uriencode(crate::include::internal::IntoCef::into_cef(text),if use_plus { 1 } else { 0 },);
    crate::include::internal::CefString::userfree(ret)
  }
}
/// Unescapes |text| and returns the result. Unescaping consists of looking for
/// the exact pattern "%XX" where each X is a hex digit and converting to the
/// character with the numerical value of those digits (e.g. "i%20=%203%3b"
/// unescapes to "i = 3;"). If |convert_to_utf8| is true this function will
/// attempt to interpret the initial decoded result as UTF-8. If the result is
/// convertable into UTF-8 it will be returned as converted. Otherwise the
/// initial decoded result will be returned.  The |unescape_rule| parameter
/// supports further customization the decoding process.
#[allow(non_snake_case)]
pub fn cef_uridecode(text: &crate::include::internal::CefString, convert_to_utf8: bool, unescape_rule: crate::include::internal::CefUriUnescapeRule, ) -> crate::include::internal::CefString {
  unsafe {
    let ret = cef_sys::cef_uridecode(crate::include::internal::IntoCef::into_cef(text),if convert_to_utf8 { 1 } else { 0 },unescape_rule.into(),);
    crate::include::internal::CefString::userfree(ret)
  }
}
/// Parses the specified |json_string| and returns a dictionary or list
/// representation. If JSON parsing fails this method returns NULL.
#[allow(non_snake_case)]
pub fn cef_parse_json(json_string: &crate::include::internal::CefString, options: crate::include::internal::CefJsonParserOptions, ) -> Option<crate::include::CefValue> {
  unsafe {
    let ret = cef_sys::cef_parse_json(crate::include::internal::IntoCef::into_cef(json_string),options.into(),);
    crate::include::CefValue::from_cef_own(ret)
  }
}
/// Parses the specified |json_string| and returns a dictionary or list
/// representation. If JSON parsing fails this method returns NULL and populates
/// |error_code_out| and |error_msg_out| with an error code and a formatted error
/// message respectively.
#[allow(non_snake_case)]
pub fn cef_parse_jsonand_return_error(json_string: &crate::include::internal::CefString, options: crate::include::internal::CefJsonParserOptions, error_code_out: &mut crate::include::internal::CefJsonParserError, error_msg_out: &mut crate::include::internal::CefString, ) -> Option<crate::include::CefValue> {
  unsafe {
    let ret = cef_sys::cef_parse_jsonand_return_error(crate::include::internal::IntoCef::into_cef(json_string),options.into(),error_code_out as *mut _ as *mut _,crate::include::internal::IntoCef::into_cef(error_msg_out),);
    crate::include::CefValue::from_cef_own(ret)
  }
}
/// Generates a JSON string from the specified root |node| which should be a
/// dictionary or list value. Returns an empty string on failure. This method
/// requires exclusive access to |node| including any underlying data.
#[allow(non_snake_case)]
pub fn cef_write_json(node: crate::include::CefValue, options: crate::include::internal::CefJsonWriterOptions, ) -> crate::include::internal::CefString {
  unsafe {
    let ret = cef_sys::cef_write_json(crate::include::CefValue::to_cef_own(node),options.into(),);
    crate::include::internal::CefString::userfree(ret)
  }
}
