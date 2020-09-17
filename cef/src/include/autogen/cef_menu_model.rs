pub type CefMenuModel = crate::include::base::CefProxy<cef_sys::cef_menu_model_t>;
#[allow(non_snake_case)]
impl CefMenuModel {
  /// Create a new MenuModel with the specified |delegate|.
  #[allow(non_snake_case)]
  pub fn create_menu_model(delegate: crate::include::CefMenuModelDelegate, ) -> Option<crate::include::CefMenuModel> {
    unsafe {
      let ret = cef_sys::cef_menu_model_create(crate::include::CefMenuModelDelegate::to_cef_own(delegate),);
      crate::include::CefMenuModel::from_cef_own(ret)
    }
  }
  /// Returns true if this menu is a submenu.
  pub fn is_sub_menu(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_sub_menu {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Clears the menu. Returns true on success.
  pub fn clear(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().clear {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the number of items in this menu.
  pub fn get_count(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Add a separator to the menu. Returns true on success.
  pub fn add_separator(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_separator {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Add an item to the menu. Returns true on success.
  pub fn add_item(&mut self, command_id: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_item {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Add a check item to the menu. Returns true on success.
  pub fn add_check_item(&mut self, command_id: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_check_item {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Add a radio item to the menu. Only a single item with the specified
  /// |group_id| can be checked at a time. Returns true on success.
  pub fn add_radio_item(&mut self, command_id: i32, label: &crate::include::internal::CefString, group_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_radio_item {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(label),group_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Add a sub-menu to the menu. The new sub-menu is returned.
  pub fn add_sub_menu(&mut self, command_id: i32, label: &crate::include::internal::CefString) -> Option<crate::include::CefMenuModel> {
    unsafe {
      let ret = match self.raw.as_ref().add_sub_menu {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      crate::include::CefMenuModel::from_cef_own(ret)
    }
  }
  /// Insert a separator in the menu at the specified |index|. Returns true on
  /// success.
  pub fn insert_separator_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().insert_separator_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Insert an item in the menu at the specified |index|. Returns true on
  /// success.
  pub fn insert_item_at(&mut self, index: i32, command_id: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().insert_item_at {
        Some(f) => f(self.raw.as_ptr(),index,command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Insert a check item in the menu at the specified |index|. Returns true on
  /// success.
  pub fn insert_check_item_at(&mut self, index: i32, command_id: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().insert_check_item_at {
        Some(f) => f(self.raw.as_ptr(),index,command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Insert a radio item in the menu at the specified |index|. Only a single
  /// item with the specified |group_id| can be checked at a time. Returns true
  /// on success.
  pub fn insert_radio_item_at(&mut self, index: i32, command_id: i32, label: &crate::include::internal::CefString, group_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().insert_radio_item_at {
        Some(f) => f(self.raw.as_ptr(),index,command_id,crate::include::internal::IntoCef::into_cef(label),group_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Insert a sub-menu in the menu at the specified |index|. The new sub-menu
  /// is returned.
  pub fn insert_sub_menu_at(&mut self, index: i32, command_id: i32, label: &crate::include::internal::CefString) -> Option<crate::include::CefMenuModel> {
    unsafe {
      let ret = match self.raw.as_ref().insert_sub_menu_at {
        Some(f) => f(self.raw.as_ptr(),index,command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      crate::include::CefMenuModel::from_cef_own(ret)
    }
  }
  /// Removes the item with the specified |command_id|. Returns true on success.
  pub fn remove(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Removes the item at the specified |index|. Returns true on success.
  pub fn remove_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the index associated with the specified |command_id| or -1 if not
  /// found due to the command id not existing in the menu.
  pub fn get_index_of(&mut self, command_id: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_index_of {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the command id at the specified |index| or -1 if not found due to
  /// invalid range or the index being a separator.
  pub fn get_command_id_at(&mut self, index: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_command_id_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret
    }
  }
  /// Sets the command id at the specified |index|. Returns true on success.
  pub fn set_command_id_at(&mut self, index: i32, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_command_id_at {
        Some(f) => f(self.raw.as_ptr(),index,command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the label for the specified |command_id| or empty if not found.
  pub fn get_label(&mut self, command_id: i32) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_label {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the label at the specified |index| or empty if not found due to
  /// invalid range or the index being a separator.
  pub fn get_label_at(&mut self, index: i32) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_label_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Sets the label for the specified |command_id|. Returns true on success.
  pub fn set_label(&mut self, command_id: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_label {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Set the label at the specified |index|. Returns true on success.
  pub fn set_label_at(&mut self, index: i32, label: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_label_at {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::internal::IntoCef::into_cef(label),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the item type for the specified |command_id|.
  pub fn get_type(&mut self, command_id: i32) -> crate::include::internal::CefMenuItemType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the item type at the specified |index|.
  pub fn get_type_at(&mut self, index: i32) -> crate::include::internal::CefMenuItemType {
    unsafe {
      let ret = match self.raw.as_ref().get_type_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the group id for the specified |command_id| or -1 if invalid.
  pub fn get_group_id(&mut self, command_id: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_group_id {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the group id at the specified |index| or -1 if invalid.
  pub fn get_group_id_at(&mut self, index: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_group_id_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret
    }
  }
  /// Sets the group id for the specified |command_id|. Returns true on success.
  pub fn set_group_id(&mut self, command_id: i32, group_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_group_id {
        Some(f) => f(self.raw.as_ptr(),command_id,group_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the group id at the specified |index|. Returns true on success.
  pub fn set_group_id_at(&mut self, index: i32, group_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_group_id_at {
        Some(f) => f(self.raw.as_ptr(),index,group_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the submenu for the specified |command_id| or empty if invalid.
  pub fn get_sub_menu(&mut self, command_id: i32) -> Option<crate::include::CefMenuModel> {
    unsafe {
      let ret = match self.raw.as_ref().get_sub_menu {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      crate::include::CefMenuModel::from_cef_own(ret)
    }
  }
  /// Returns the submenu at the specified |index| or empty if invalid.
  pub fn get_sub_menu_at(&mut self, index: i32) -> Option<crate::include::CefMenuModel> {
    unsafe {
      let ret = match self.raw.as_ref().get_sub_menu_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefMenuModel::from_cef_own(ret)
    }
  }
  /// Returns true if the specified |command_id| is visible.
  pub fn is_visible(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_visible {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the specified |index| is visible.
  pub fn is_visible_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_visible_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Change the visibility of the specified |command_id|. Returns true on
  /// success.
  pub fn set_visible(&mut self, command_id: i32, visible: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_visible {
        Some(f) => f(self.raw.as_ptr(),command_id,if visible { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Change the visibility at the specified |index|. Returns true on success.
  pub fn set_visible_at(&mut self, index: i32, visible: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_visible_at {
        Some(f) => f(self.raw.as_ptr(),index,if visible { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the specified |command_id| is enabled.
  pub fn is_enabled(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_enabled {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the specified |index| is enabled.
  pub fn is_enabled_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_enabled_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Change the enabled status of the specified |command_id|. Returns true on
  /// success.
  pub fn set_enabled(&mut self, command_id: i32, enabled: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_enabled {
        Some(f) => f(self.raw.as_ptr(),command_id,if enabled { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Change the enabled status at the specified |index|. Returns true on
  /// success.
  pub fn set_enabled_at(&mut self, index: i32, enabled: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_enabled_at {
        Some(f) => f(self.raw.as_ptr(),index,if enabled { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the specified |command_id| is checked. Only applies to
  /// check and radio items.
  pub fn is_checked(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_checked {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the specified |index| is checked. Only applies to check
  /// and radio items.
  pub fn is_checked_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_checked_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Check the specified |command_id|. Only applies to check and radio items.
  /// Returns true on success.
  pub fn set_checked(&mut self, command_id: i32, checked: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_checked {
        Some(f) => f(self.raw.as_ptr(),command_id,if checked { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Check the specified |index|. Only applies to check and radio items. Returns
  /// true on success.
  pub fn set_checked_at(&mut self, index: i32, checked: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_checked_at {
        Some(f) => f(self.raw.as_ptr(),index,if checked { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the specified |command_id| has a keyboard accelerator
  /// assigned.
  pub fn has_accelerator(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_accelerator {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the specified |index| has a keyboard accelerator assigned.
  pub fn has_accelerator_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_accelerator_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Set the keyboard accelerator for the specified |command_id|. |key_code| can
  /// be any virtual key or character value. Returns true on success.
  pub fn set_accelerator(&mut self, command_id: i32, key_code: i32, shift_pressed: bool, ctrl_pressed: bool, alt_pressed: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_accelerator {
        Some(f) => f(self.raw.as_ptr(),command_id,key_code,if shift_pressed { 1 } else { 0 },if ctrl_pressed { 1 } else { 0 },if alt_pressed { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Set the keyboard accelerator at the specified |index|. |key_code| can be
  /// any virtual key or character value. Returns true on success.
  pub fn set_accelerator_at(&mut self, index: i32, key_code: i32, shift_pressed: bool, ctrl_pressed: bool, alt_pressed: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_accelerator_at {
        Some(f) => f(self.raw.as_ptr(),index,key_code,if shift_pressed { 1 } else { 0 },if ctrl_pressed { 1 } else { 0 },if alt_pressed { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Remove the keyboard accelerator for the specified |command_id|. Returns
  /// true on success.
  pub fn remove_accelerator(&mut self, command_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove_accelerator {
        Some(f) => f(self.raw.as_ptr(),command_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Remove the keyboard accelerator at the specified |index|. Returns true on
  /// success.
  pub fn remove_accelerator_at(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove_accelerator_at {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Retrieves the keyboard accelerator for the specified |command_id|. Returns
  /// true on success.
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
  /// Retrieves the keyboard accelerator for the specified |index|. Returns true
  /// on success.
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
  /// Set the explicit color for |command_id| and |color_type| to |color|.
  /// Specify a |color| value of 0 to remove the explicit color. If no explicit
  /// color or default color is set for |color_type| then the system color will
  /// be used. Returns true on success.
  pub fn set_color(&mut self, command_id: i32, color_type: crate::include::internal::CefMenuColorType, color: crate::include::internal::CefColor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_color {
        Some(f) => f(self.raw.as_ptr(),command_id,color_type.into(),color.into(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Set the explicit color for |command_id| and |index| to |color|. Specify a
  /// |color| value of 0 to remove the explicit color. Specify an |index| value
  /// of -1 to set the default color for items that do not have an explicit
  /// color set. If no explicit color or default color is set for |color_type|
  /// then the system color will be used. Returns true on success.
  pub fn set_color_at(&mut self, index: i32, color_type: crate::include::internal::CefMenuColorType, color: crate::include::internal::CefColor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_color_at {
        Some(f) => f(self.raw.as_ptr(),index,color_type.into(),color.into(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns in |color| the color that was explicitly set for |command_id| and
  /// |color_type|. If a color was not set then 0 will be returned in |color|.
  /// Returns true on success.
  pub fn get_color(&mut self, command_id: i32, color_type: crate::include::internal::CefMenuColorType, color: &mut crate::include::internal::CefColor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_color {
        Some(f) => f(self.raw.as_ptr(),command_id,color_type.into(),color as *mut _ as *mut _,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns in |color| the color that was explicitly set for |command_id| and
  /// |color_type|. Specify an |index| value of -1 to return the default color
  /// in |color|. If a color was not set then 0 will be returned in |color|.
  /// Returns true on success.
  pub fn get_color_at(&mut self, index: i32, color_type: crate::include::internal::CefMenuColorType, color: &mut crate::include::internal::CefColor) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_color_at {
        Some(f) => f(self.raw.as_ptr(),index,color_type.into(),color as *mut _ as *mut _,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the font list for the specified |command_id|. If |font_list| is empty
  /// the system font will be used. Returns true on success. The format is
  /// "<FONT_FAMILY_LIST>,[STYLES] <SIZE>", where:
  /// - FONT_FAMILY_LIST is a comma-separated list of font family names,
  /// - STYLES is an optional space-separated list of style names (case-sensitive
  /// "Bold" and "Italic" are supported), and
  /// - SIZE is an integer font size in pixels with the suffix "px".
  /// 
  /// Here are examples of valid font description strings:
  /// - "Arial, Helvetica, Bold Italic 14px"
  /// - "Arial, 14px"
  pub fn set_font_list(&mut self, command_id: i32, font_list: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_font_list {
        Some(f) => f(self.raw.as_ptr(),command_id,crate::include::internal::IntoCef::into_cef(font_list),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the font list for the specified |index|. Specify an |index| value of
  /// -1 to set the default font. If |font_list| is empty the system font will
  /// be used. Returns true on success. The format is
  /// "<FONT_FAMILY_LIST>,[STYLES] <SIZE>", where:
  /// - FONT_FAMILY_LIST is a comma-separated list of font family names,
  /// - STYLES is an optional space-separated list of style names (case-sensitive
  /// "Bold" and "Italic" are supported), and
  /// - SIZE is an integer font size in pixels with the suffix "px".
  /// 
  /// Here are examples of valid font description strings:
  /// - "Arial, Helvetica, Bold Italic 14px"
  /// - "Arial, 14px"
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
