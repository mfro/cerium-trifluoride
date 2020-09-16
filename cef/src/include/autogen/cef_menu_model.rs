pub type CefMenuModel = crate::include::base::CefProxy<cef_sys::cef_menu_model_t>;
#[allow(non_snake_case)]
impl CefMenuModel {
  pub fn is_sub_menu(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_sub_menu {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn clear(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().clear {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_count(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn add_separator(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_separator {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn add_item(&mut self, command_id: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_item {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn add_check_item(&mut self, command_id: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_check_item {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn add_radio_item(&mut self, command_id: i32, label: &crate::include::internal::CefString, group_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_radio_item {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(label),group_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn add_sub_menu(&mut self, command_id: i32, label: &crate::include::internal::CefString) -> Option<crate::include::CefMenuModel> {
    unsafe {
      let ret = match self.raw.as_ref().add_sub_menu {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      crate::include::CefMenuModel::from_cef_own(ret)
    }
  }
  pub fn insert_separator_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().insert_separator_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn insert_item_at(&mut self, index: i32, command_id: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().insert_item_at {
        Some(f) => f(self.raw.as_ptr(),index,command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn insert_check_item_at(&mut self, index: i32, command_id: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().insert_check_item_at {
        Some(f) => f(self.raw.as_ptr(),index,command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn insert_radio_item_at(&mut self, index: i32, command_id: i32, label: &crate::include::internal::CefString, group_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().insert_radio_item_at {
        Some(f) => f(self.raw.as_ptr(),index,command_id,crate::include::internal::IntoCef::into_cef(label),group_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn insert_sub_menu_at(&mut self, index: i32, command_id: i32, label: &crate::include::internal::CefString) -> Option<crate::include::CefMenuModel> {
    unsafe {
      let ret = match self.raw.as_ref().insert_sub_menu_at {
        Some(f) => f(self.raw.as_ptr(),index,command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      crate::include::CefMenuModel::from_cef_own(ret)
    }
  }
  pub fn remove(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn remove_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_index_of(&mut self, command_id: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_index_of {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_command_id_at(&mut self, index: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_command_id_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_command_id_at(&mut self, index: i32, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_command_id_at {
        Some(f) => f(self.raw.as_ptr(),index,command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_label(&mut self, command_id: i32) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_label {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_label_at(&mut self, index: i32) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_label_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_label(&mut self, command_id: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_label {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_label_at(&mut self, index: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_label_at {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_type(&mut self, command_id: i32) -> crate::include::internal::CefMenuItemType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_type_at(&mut self, index: i32) -> crate::include::internal::CefMenuItemType {
    unsafe {
      let ret = match self.raw.as_ref().get_type_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_group_id(&mut self, command_id: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_group_id {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_group_id_at(&mut self, index: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_group_id_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_group_id(&mut self, command_id: i32, group_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_group_id {
        Some(f) => f(self.raw.as_ptr(),command_id,group_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_group_id_at(&mut self, index: i32, group_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_group_id_at {
        Some(f) => f(self.raw.as_ptr(),index,group_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_sub_menu(&mut self, command_id: i32) -> Option<crate::include::CefMenuModel> {
    unsafe {
      let ret = match self.raw.as_ref().get_sub_menu {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      crate::include::CefMenuModel::from_cef_own(ret)
    }
  }
  pub fn get_sub_menu_at(&mut self, index: i32) -> Option<crate::include::CefMenuModel> {
    unsafe {
      let ret = match self.raw.as_ref().get_sub_menu_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefMenuModel::from_cef_own(ret)
    }
  }
  pub fn is_visible(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_visible {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_visible_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_visible_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_visible(&mut self, command_id: i32, visible: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_visible {
        Some(f) => f(self.raw.as_ptr(),command_id,if visible { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_visible_at(&mut self, index: i32, visible: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_visible_at {
        Some(f) => f(self.raw.as_ptr(),index,if visible { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_enabled(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_enabled {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_enabled_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_enabled_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_enabled(&mut self, command_id: i32, enabled: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_enabled {
        Some(f) => f(self.raw.as_ptr(),command_id,if enabled { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_enabled_at(&mut self, index: i32, enabled: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_enabled_at {
        Some(f) => f(self.raw.as_ptr(),index,if enabled { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_checked(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_checked {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_checked_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_checked_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_checked(&mut self, command_id: i32, checked: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_checked {
        Some(f) => f(self.raw.as_ptr(),command_id,if checked { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_checked_at(&mut self, index: i32, checked: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_checked_at {
        Some(f) => f(self.raw.as_ptr(),index,if checked { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_accelerator(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_accelerator {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_accelerator_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_accelerator_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_accelerator(&mut self, command_id: i32, key_code: i32, shift_pressed: bool, ctrl_pressed: bool, alt_pressed: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_accelerator {
        Some(f) => f(self.raw.as_ptr(),command_id,key_code,if shift_pressed { 1 } else { 0 },if ctrl_pressed { 1 } else { 0 },if alt_pressed { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_accelerator_at(&mut self, index: i32, key_code: i32, shift_pressed: bool, ctrl_pressed: bool, alt_pressed: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_accelerator_at {
        Some(f) => f(self.raw.as_ptr(),index,key_code,if shift_pressed { 1 } else { 0 },if ctrl_pressed { 1 } else { 0 },if alt_pressed { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn remove_accelerator(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove_accelerator {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn remove_accelerator_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove_accelerator_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_accelerator(&mut self, command_id: i32, key_code: &mut i32, shift_pressed: &mut bool, ctrl_pressed: &mut bool, alt_pressed: &mut bool) -> bool {
    unsafe {
      let mut shift_pressed__tmp = if *shift_pressed { 1 } else { 0 };
      let mut ctrl_pressed__tmp = if *ctrl_pressed { 1 } else { 0 };
      let mut alt_pressed__tmp = if *alt_pressed { 1 } else { 0 };
      let ret = match self.raw.as_ref().get_accelerator {
        Some(f) => f(self.raw.as_ptr(),command_id,key_code,&mut shift_pressed__tmp,&mut ctrl_pressed__tmp,&mut alt_pressed__tmp,),
        None => panic!(),
      };
      *shift_pressed = if shift_pressed__tmp == 0 { false } else { true };
      *ctrl_pressed = if ctrl_pressed__tmp == 0 { false } else { true };
      *alt_pressed = if alt_pressed__tmp == 0 { false } else { true };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_accelerator_at(&mut self, index: i32, key_code: &mut i32, shift_pressed: &mut bool, ctrl_pressed: &mut bool, alt_pressed: &mut bool) -> bool {
    unsafe {
      let mut shift_pressed__tmp = if *shift_pressed { 1 } else { 0 };
      let mut ctrl_pressed__tmp = if *ctrl_pressed { 1 } else { 0 };
      let mut alt_pressed__tmp = if *alt_pressed { 1 } else { 0 };
      let ret = match self.raw.as_ref().get_accelerator_at {
        Some(f) => f(self.raw.as_ptr(),index,key_code,&mut shift_pressed__tmp,&mut ctrl_pressed__tmp,&mut alt_pressed__tmp,),
        None => panic!(),
      };
      *shift_pressed = if shift_pressed__tmp == 0 { false } else { true };
      *ctrl_pressed = if ctrl_pressed__tmp == 0 { false } else { true };
      *alt_pressed = if alt_pressed__tmp == 0 { false } else { true };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_color(&mut self, command_id: i32, color_type: crate::include::internal::CefMenuColorType, color: crate::include::internal::CefColor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_color {
        Some(f) => f(self.raw.as_ptr(),command_id,color_type.into(),color.into(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_color_at(&mut self, index: i32, color_type: crate::include::internal::CefMenuColorType, color: crate::include::internal::CefColor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_color_at {
        Some(f) => f(self.raw.as_ptr(),index,color_type.into(),color.into(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_color(&mut self, command_id: i32, color_type: crate::include::internal::CefMenuColorType, color: &mut crate::include::internal::CefColor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_color {
        Some(f) => f(self.raw.as_ptr(),command_id,color_type.into(),&mut color.raw,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_color_at(&mut self, index: i32, color_type: crate::include::internal::CefMenuColorType, color: &mut crate::include::internal::CefColor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_color_at {
        Some(f) => f(self.raw.as_ptr(),index,color_type.into(),&mut color.raw,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_font_list(&mut self, command_id: i32, font_list: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_font_list {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(font_list),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_font_list_at(&mut self, index: i32, font_list: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_font_list_at {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::internal::IntoCef::into_cef(font_list),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
