/// Implement this interface to handle menu model events. The methods of this
/// class will be called on the browser process UI thread unless otherwise
/// indicated.
#[allow(non_snake_case)]
pub trait MenuModelDelegate {
  fn execute_command(&mut self, menu_model: crate::include::CefMenuModel, command_id: i32, event_flags: crate::include::internal::CefEventFlags) -> () { Default::default() }
  fn mouse_outside_menu(&mut self, menu_model: crate::include::CefMenuModel, screen_point: &crate::include::internal::CefPoint) -> () { Default::default() }
  fn unhandled_open_submenu(&mut self, menu_model: crate::include::CefMenuModel, is_rtl: bool) -> () { Default::default() }
  fn unhandled_close_submenu(&mut self, menu_model: crate::include::CefMenuModel, is_rtl: bool) -> () { Default::default() }
  fn menu_will_show(&mut self, menu_model: crate::include::CefMenuModel) -> () { Default::default() }
  fn menu_closed(&mut self, menu_model: crate::include::CefMenuModel) -> () { Default::default() }
  fn format_label(&mut self, menu_model: crate::include::CefMenuModel, label: &mut crate::include::internal::CefString) -> bool { Default::default() }
}
define_refcounted!(MenuModelDelegate, menu_model_delegate, execute_command,mouse_outside_menu,unhandled_open_submenu,unhandled_close_submenu,menu_will_show,menu_closed,format_label,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_menu_model_delegate_t_execute_command(_self: *mut cef_sys::cef_menu_model_delegate_t, menu_model: *mut cef_sys::cef_menu_model_t, command_id: i32, event_flags: cef_sys::cef_event_flags_t) -> () {
  let ret = CefMenuModelDelegate::from_cef(_self, true).get().execute_command(crate::include::CefMenuModel::from_cef_own(menu_model).unwrap(),command_id,event_flags.into(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_menu_model_delegate_t_mouse_outside_menu(_self: *mut cef_sys::cef_menu_model_delegate_t, menu_model: *mut cef_sys::cef_menu_model_t, screen_point: *const cef_sys::cef_point_t) -> () {
  let ret = CefMenuModelDelegate::from_cef(_self, true).get().mouse_outside_menu(crate::include::CefMenuModel::from_cef_own(menu_model).unwrap(),&*(screen_point as *const _),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_menu_model_delegate_t_unhandled_open_submenu(_self: *mut cef_sys::cef_menu_model_delegate_t, menu_model: *mut cef_sys::cef_menu_model_t, is_rtl: i32) -> () {
  let ret = CefMenuModelDelegate::from_cef(_self, true).get().unhandled_open_submenu(crate::include::CefMenuModel::from_cef_own(menu_model).unwrap(),if is_rtl == 0 { false } else { true },);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_menu_model_delegate_t_unhandled_close_submenu(_self: *mut cef_sys::cef_menu_model_delegate_t, menu_model: *mut cef_sys::cef_menu_model_t, is_rtl: i32) -> () {
  let ret = CefMenuModelDelegate::from_cef(_self, true).get().unhandled_close_submenu(crate::include::CefMenuModel::from_cef_own(menu_model).unwrap(),if is_rtl == 0 { false } else { true },);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_menu_model_delegate_t_menu_will_show(_self: *mut cef_sys::cef_menu_model_delegate_t, menu_model: *mut cef_sys::cef_menu_model_t) -> () {
  let ret = CefMenuModelDelegate::from_cef(_self, true).get().menu_will_show(crate::include::CefMenuModel::from_cef_own(menu_model).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_menu_model_delegate_t_menu_closed(_self: *mut cef_sys::cef_menu_model_delegate_t, menu_model: *mut cef_sys::cef_menu_model_t) -> () {
  let ret = CefMenuModelDelegate::from_cef(_self, true).get().menu_closed(crate::include::CefMenuModel::from_cef_own(menu_model).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_menu_model_delegate_t_format_label(_self: *mut cef_sys::cef_menu_model_delegate_t, menu_model: *mut cef_sys::cef_menu_model_t, label: *mut cef_sys::cef_string_t) -> i32 {
  let ret = CefMenuModelDelegate::from_cef(_self, true).get().format_label(crate::include::CefMenuModel::from_cef_own(menu_model).unwrap(),&mut crate::include::internal::CefString::from_cef(label).unwrap(),);
  if ret { 1 } else { 0 }
}
