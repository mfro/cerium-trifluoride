pub type CefValue = crate::include::base::CefProxy<cef_sys::cef_value_t>;
#[allow(non_snake_case)]
impl CefValue {
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_owned(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_owned {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_same(&mut self, that: crate::include::CefValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_equal(&mut self, that: crate::include::CefValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_equal {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn copy(&mut self) -> Option<crate::include::CefValue> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefValue::from_cef_own(ret)
    }
  }
  pub fn get_type(&mut self) -> crate::include::internal::CefValueType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_bool(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_bool {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_int(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_int {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_double(&mut self) -> f64 {
    unsafe {
      let ret = match self.raw.as_ref().get_double {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_string(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_string {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_binary(&mut self) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_binary {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  pub fn get_dictionary(&mut self) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_dictionary {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  pub fn get_list(&mut self) -> Option<crate::include::CefListValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_list {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefListValue::from_cef_own(ret)
    }
  }
  pub fn set_null(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_null {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_bool(&mut self, value: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_bool {
        Some(f) => f(self.raw.as_ptr(),if value { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_int(&mut self, value: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_int {
        Some(f) => f(self.raw.as_ptr(),value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_double(&mut self, value: f64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_double {
        Some(f) => f(self.raw.as_ptr(),value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_string(&mut self, value: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_string {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_binary(&mut self, value: crate::include::CefBinaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_binary {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefBinaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_dictionary(&mut self, value: crate::include::CefDictionaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_dictionary {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDictionaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_list(&mut self, value: crate::include::CefListValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_list {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefListValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
pub type CefBinaryValue = crate::include::base::CefProxy<cef_sys::cef_binary_value_t>;
#[allow(non_snake_case)]
impl CefBinaryValue {
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_owned(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_owned {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_same(&mut self, that: crate::include::CefBinaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefBinaryValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_equal(&mut self, that: crate::include::CefBinaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_equal {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefBinaryValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn copy(&mut self) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  pub fn get_size(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_size {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_data(&mut self, buffer: &mut std::os::raw::c_void, buffer_size: u64, data_offset: u64) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_data {
        Some(f) => f(self.raw.as_ptr(),buffer,buffer_size,data_offset,),
        None => panic!(),
      };
      ret
    }
  }
}
pub type CefDictionaryValue = crate::include::base::CefProxy<cef_sys::cef_dictionary_value_t>;
#[allow(non_snake_case)]
impl CefDictionaryValue {
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_owned(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_owned {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_same(&mut self, that: crate::include::CefDictionaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDictionaryValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_equal(&mut self, that: crate::include::CefDictionaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_equal {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDictionaryValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn copy(&mut self, exclude_empty_children: bool) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),if exclude_empty_children { 1 } else { 0 },),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  pub fn get_size(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_size {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
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
  pub fn has_key(&mut self, key: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_key {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn remove(&mut self, key: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_type(&mut self, key: &crate::include::internal::CefString) -> crate::include::internal::CefValueType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_value(&mut self, key: &crate::include::internal::CefString) -> Option<crate::include::CefValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_value {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      crate::include::CefValue::from_cef_own(ret)
    }
  }
  pub fn get_bool(&mut self, key: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_bool {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_int(&mut self, key: &crate::include::internal::CefString) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_int {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_double(&mut self, key: &crate::include::internal::CefString) -> f64 {
    unsafe {
      let ret = match self.raw.as_ref().get_double {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_string(&mut self, key: &crate::include::internal::CefString) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_string {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_binary(&mut self, key: &crate::include::internal::CefString) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_binary {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  pub fn get_dictionary(&mut self, key: &crate::include::internal::CefString) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_dictionary {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  pub fn get_list(&mut self, key: &crate::include::internal::CefString) -> Option<crate::include::CefListValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_list {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      crate::include::CefListValue::from_cef_own(ret)
    }
  }
  pub fn set_value(&mut self, key: &crate::include::internal::CefString, value: crate::include::CefValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),crate::include::CefValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_null(&mut self, key: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_null {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_bool(&mut self, key: &crate::include::internal::CefString, value: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_bool {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),if value { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_int(&mut self, key: &crate::include::internal::CefString, value: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_int {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_double(&mut self, key: &crate::include::internal::CefString, value: f64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_double {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_string(&mut self, key: &crate::include::internal::CefString, value: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_string {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),crate::include::internal::IntoCef::into_cef(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_binary(&mut self, key: &crate::include::internal::CefString, value: crate::include::CefBinaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_binary {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),crate::include::CefBinaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_dictionary(&mut self, key: &crate::include::internal::CefString, value: crate::include::CefDictionaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_dictionary {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),crate::include::CefDictionaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_list(&mut self, key: &crate::include::internal::CefString, value: crate::include::CefListValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_list {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),crate::include::CefListValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
pub type CefListValue = crate::include::base::CefProxy<cef_sys::cef_list_value_t>;
#[allow(non_snake_case)]
impl CefListValue {
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_owned(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_owned {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_same(&mut self, that: crate::include::CefListValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefListValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_equal(&mut self, that: crate::include::CefListValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_equal {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefListValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn copy(&mut self) -> Option<crate::include::CefListValue> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefListValue::from_cef_own(ret)
    }
  }
  pub fn set_size(&mut self, size: u64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_size {
        Some(f) => f(self.raw.as_ptr(),size,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_size(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_size {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
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
  pub fn remove(&mut self, index: u64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_type(&mut self, index: u64) -> crate::include::internal::CefValueType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_value(&mut self, index: u64) -> Option<crate::include::CefValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_value {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefValue::from_cef_own(ret)
    }
  }
  pub fn get_bool(&mut self, index: u64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_bool {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_int(&mut self, index: u64) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_int {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_double(&mut self, index: u64) -> f64 {
    unsafe {
      let ret = match self.raw.as_ref().get_double {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_string(&mut self, index: u64) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_string {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_binary(&mut self, index: u64) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_binary {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  pub fn get_dictionary(&mut self, index: u64) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_dictionary {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  pub fn get_list(&mut self, index: u64) -> Option<crate::include::CefListValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_list {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefListValue::from_cef_own(ret)
    }
  }
  pub fn set_value(&mut self, index: u64, value: crate::include::CefValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::CefValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_null(&mut self, index: u64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_null {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_bool(&mut self, index: u64, value: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_bool {
        Some(f) => f(self.raw.as_ptr(),index,if value { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_int(&mut self, index: u64, value: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_int {
        Some(f) => f(self.raw.as_ptr(),index,value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_double(&mut self, index: u64, value: f64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_double {
        Some(f) => f(self.raw.as_ptr(),index,value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_string(&mut self, index: u64, value: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_string {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::internal::IntoCef::into_cef(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_binary(&mut self, index: u64, value: crate::include::CefBinaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_binary {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::CefBinaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_dictionary(&mut self, index: u64, value: crate::include::CefDictionaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_dictionary {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::CefDictionaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_list(&mut self, index: u64, value: crate::include::CefListValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_list {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::CefListValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
