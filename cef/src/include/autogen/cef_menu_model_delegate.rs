/// Implement this interface to handle menu model events. The methods of this
/// class will be called on the browser process UI thread unless otherwise
/// indicated.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait MenuModelDelegate {
  /// Perform the action associated with the specified |command_id| and
  /// optional |event_flags|.
  fn execute_command(&mut self, menu_model: crate::include::CefMenuModel, command_id: i32, event_flags: crate::include::internal::CefEventFlags) -> () { Default::default() }
  /// Called when the user moves the mouse outside the menu and over the owning
  /// window.
  fn mouse_outside_menu(&mut self, menu_model: crate::include::CefMenuModel, screen_point: &crate::include::internal::CefPoint) -> () { Default::default() }
  /// Called on unhandled open submenu keyboard commands. |is_rtl| will be true
  /// if the menu is displaying a right-to-left language.
  fn unhandled_open_submenu(&mut self, menu_model: crate::include::CefMenuModel, is_rtl: bool) -> () { Default::default() }
  /// Called on unhandled close submenu keyboard commands. |is_rtl| will be true
  /// if the menu is displaying a right-to-left language.
  fn unhandled_close_submenu(&mut self, menu_model: crate::include::CefMenuModel, is_rtl: bool) -> () { Default::default() }
  /// The menu is about to show.
  fn menu_will_show(&mut self, menu_model: crate::include::CefMenuModel) -> () { Default::default() }
  /// The menu has closed.
  fn menu_closed(&mut self, menu_model: crate::include::CefMenuModel) -> () { Default::default() }
  /// Optionally modify a menu item label. Return true if |label| was modified.
  fn format_label(&mut self, menu_model: crate::include::CefMenuModel, label: &mut crate::include::internal::CefString) -> bool { Default::default() }
}
define_refcounted!(MenuModelDelegate, CefMenuModelDelegate, cef_menu_model_delegate_t, execute_command: cef_menu_model_delegate_t_execute_command,mouse_outside_menu: cef_menu_model_delegate_t_mouse_outside_menu,unhandled_open_submenu: cef_menu_model_delegate_t_unhandled_open_submenu,unhandled_close_submenu: cef_menu_model_delegate_t_unhandled_close_submenu,menu_will_show: cef_menu_model_delegate_t_menu_will_show,menu_closed: cef_menu_model_delegate_t_menu_closed,format_label: cef_menu_model_delegate_t_format_label,);
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
  let ret = CefMenuModelDelegate::from_cef(_self, true).get().format_label(crate::include::CefMenuModel::from_cef_own(menu_model).unwrap(),&mut *(label as *mut _),);
  if ret { 1 } else { 0 }
}
