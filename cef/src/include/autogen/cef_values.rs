pub type CefValue = crate::include::base::CefProxy<cef_sys::cef_value_t>;
#[allow(non_snake_case)]
impl CefValue {
  /// Creates a new object.
  #[allow(non_snake_case)]
  pub fn create() -> Option<crate::include::CefValue> {
    unsafe {
      let ret = cef_sys::cef_value_create();
      crate::include::CefValue::from_cef_own(ret)
    }
  }
  /// Returns true if the underlying data is valid. This will always be true for
  /// simple types. For complex types (binary, dictionary and list) the
  /// underlying data may become invalid if owned by another object (e.g. list or
  /// dictionary) and that other object is then modified or destroyed. This value
  /// object can be re-used by calling Set*() even if the underlying data is
  /// invalid.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the underlying data is owned by another object.
  pub fn is_owned(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_owned {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the underlying data is read-only. Some APIs may expose
  /// read-only objects.
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object and |that| object have the same underlying
  /// data. If true modifications to this object will also affect |that| object
  /// and vice-versa.
  pub fn is_same(&mut self, that: crate::include::CefValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object and |that| object have an equivalent underlying
  /// value but are not necessarily the same object.
  pub fn is_equal(&mut self, that: crate::include::CefValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_equal {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns a copy of this object. The underlying data will also be copied.
  pub fn copy(&mut self) -> Option<crate::include::CefValue> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefValue::from_cef_own(ret)
    }
  }
  /// Returns the underlying value type.
  pub fn get_type(&mut self) -> crate::include::internal::CefValueType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the underlying value as type bool.
  pub fn get_bool(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_bool {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the underlying value as type int.
  pub fn get_int(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_int {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the underlying value as type double.
  pub fn get_double(&mut self) -> f64 {
    unsafe {
      let ret = match self.raw.as_ref().get_double {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the underlying value as type string.
  pub fn get_string(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_string {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the underlying value as type binary. The returned reference may
  /// become invalid if the value is owned by another object or if ownership is
  /// transferred to another object in the future. To maintain a reference to
  /// the value after assigning ownership to a dictionary or list pass this
  /// object to the SetValue() method instead of passing the returned reference
  /// to SetBinary().
  pub fn get_binary(&mut self) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_binary {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  /// Returns the underlying value as type dictionary. The returned reference may
  /// become invalid if the value is owned by another object or if ownership is
  /// transferred to another object in the future. To maintain a reference to
  /// the value after assigning ownership to a dictionary or list pass this
  /// object to the SetValue() method instead of passing the returned reference
  /// to SetDictionary().
  pub fn get_dictionary(&mut self) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_dictionary {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  /// Returns the underlying value as type list. The returned reference may
  /// become invalid if the value is owned by another object or if ownership is
  /// transferred to another object in the future. To maintain a reference to
  /// the value after assigning ownership to a dictionary or list pass this
  /// object to the SetValue() method instead of passing the returned reference
  /// to SetList().
  pub fn get_list(&mut self) -> Option<crate::include::CefListValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_list {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefListValue::from_cef_own(ret)
    }
  }
  /// Sets the underlying value as type null. Returns true if the value was set
  /// successfully.
  pub fn set_null(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_null {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the underlying value as type bool. Returns true if the value was set
  /// successfully.
  pub fn set_bool(&mut self, value: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_bool {
        Some(f) => f(self.raw.as_ptr(),if value { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the underlying value as type int. Returns true if the value was set
  /// successfully.
  pub fn set_int(&mut self, value: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_int {
        Some(f) => f(self.raw.as_ptr(),value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the underlying value as type double. Returns true if the value was set
  /// successfully.
  pub fn set_double(&mut self, value: f64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_double {
        Some(f) => f(self.raw.as_ptr(),value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the underlying value as type string. Returns true if the value was set
  /// successfully.
  pub fn set_string(&mut self, value: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_string {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the underlying value as type binary. Returns true if the value was set
  /// successfully. This object keeps a reference to |value| and ownership of the
  /// underlying data remains unchanged.
  pub fn set_binary(&mut self, value: crate::include::CefBinaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_binary {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefBinaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the underlying value as type dict. Returns true if the value was set
  /// successfully. This object keeps a reference to |value| and ownership of the
  /// underlying data remains unchanged.
  pub fn set_dictionary(&mut self, value: crate::include::CefDictionaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_dictionary {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDictionaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the underlying value as type list. Returns true if the value was set
  /// successfully. This object keeps a reference to |value| and ownership of the
  /// underlying data remains unchanged.
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
  /// Returns true if this object is valid. This object may become invalid if
  /// the underlying data is owned by another object (e.g. list or dictionary)
  /// and that other object is then modified or destroyed. Do not call any other
  /// methods if this method returns false.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object is currently owned by another object.
  pub fn is_owned(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_owned {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object and |that| object have the same underlying
  /// data.
  pub fn is_same(&mut self, that: crate::include::CefBinaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefBinaryValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object and |that| object have an equivalent underlying
  /// value but are not necessarily the same object.
  pub fn is_equal(&mut self, that: crate::include::CefBinaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_equal {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefBinaryValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns a copy of this object. The data in this object will also be copied.
  pub fn copy(&mut self) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  /// Returns the data size.
  pub fn get_size(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_size {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Read up to |buffer_size| number of bytes into |buffer|. Reading begins at
  /// the specified byte |data_offset|. Returns the number of bytes read.
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
  /// Creates a new object that is not owned by any other object.
  #[allow(non_snake_case)]
  pub fn create() -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = cef_sys::cef_dictionary_value_create();
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  /// Returns true if this object is valid. This object may become invalid if
  /// the underlying data is owned by another object (e.g. list or dictionary)
  /// and that other object is then modified or destroyed. Do not call any other
  /// methods if this method returns false.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object is currently owned by another object.
  pub fn is_owned(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_owned {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the values of this object are read-only. Some APIs may
  /// expose read-only objects.
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object and |that| object have the same underlying
  /// data. If true modifications to this object will also affect |that| object
  /// and vice-versa.
  pub fn is_same(&mut self, that: crate::include::CefDictionaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDictionaryValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object and |that| object have an equivalent underlying
  /// value but are not necessarily the same object.
  pub fn is_equal(&mut self, that: crate::include::CefDictionaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_equal {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDictionaryValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns a writable copy of this object. If |exclude_empty_children| is true
  /// any empty dictionaries or lists will be excluded from the copy.
  pub fn copy(&mut self, exclude_empty_children: bool) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),if exclude_empty_children { 1 } else { 0 },),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  /// Returns the number of values.
  pub fn get_size(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_size {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Removes all values. Returns true on success.
  pub fn clear(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().clear {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the current dictionary has a value for the given key.
  pub fn has_key(&mut self, key: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_key {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Removes the value at the specified key. Returns true is the value was
  /// removed successfully.
  pub fn remove(&mut self, key: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the value type for the specified key.
  pub fn get_type(&mut self, key: &crate::include::internal::CefString) -> crate::include::internal::CefValueType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the value at the specified key. For simple types the returned
  /// value will copy existing data and modifications to the value will not
  /// modify this object. For complex types (binary, dictionary and list) the
  /// returned value will reference existing data and modifications to the value
  /// will modify this object.
  pub fn get_value(&mut self, key: &crate::include::internal::CefString) -> Option<crate::include::CefValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_value {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      crate::include::CefValue::from_cef_own(ret)
    }
  }
  /// Returns the value at the specified key as type bool.
  pub fn get_bool(&mut self, key: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_bool {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the value at the specified key as type int.
  pub fn get_int(&mut self, key: &crate::include::internal::CefString) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_int {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the value at the specified key as type double.
  pub fn get_double(&mut self, key: &crate::include::internal::CefString) -> f64 {
    unsafe {
      let ret = match self.raw.as_ref().get_double {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the value at the specified key as type string.
  pub fn get_string(&mut self, key: &crate::include::internal::CefString) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_string {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the value at the specified key as type binary. The returned
  /// value will reference existing data.
  pub fn get_binary(&mut self, key: &crate::include::internal::CefString) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_binary {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  /// Returns the value at the specified key as type dictionary. The returned
  /// value will reference existing data and modifications to the value will
  /// modify this object.
  pub fn get_dictionary(&mut self, key: &crate::include::internal::CefString) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_dictionary {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  /// Returns the value at the specified key as type list. The returned value
  /// will reference existing data and modifications to the value will modify
  /// this object.
  pub fn get_list(&mut self, key: &crate::include::internal::CefString) -> Option<crate::include::CefListValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_list {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      crate::include::CefListValue::from_cef_own(ret)
    }
  }
  /// Sets the value at the specified key. Returns true if the value was set
  /// successfully. If |value| represents simple data then the underlying data
  /// will be copied and modifications to |value| will not modify this object. If
  /// |value| represents complex data (binary, dictionary or list) then the
  /// underlying data will be referenced and modifications to |value| will modify
  /// this object.
  pub fn set_value(&mut self, key: &crate::include::internal::CefString, value: crate::include::CefValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),crate::include::CefValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified key as type null. Returns true if the
  /// value was set successfully.
  pub fn set_null(&mut self, key: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_null {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified key as type bool. Returns true if the
  /// value was set successfully.
  pub fn set_bool(&mut self, key: &crate::include::internal::CefString, value: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_bool {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),if value { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified key as type int. Returns true if the
  /// value was set successfully.
  pub fn set_int(&mut self, key: &crate::include::internal::CefString, value: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_int {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified key as type double. Returns true if the
  /// value was set successfully.
  pub fn set_double(&mut self, key: &crate::include::internal::CefString, value: f64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_double {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified key as type string. Returns true if the
  /// value was set successfully.
  pub fn set_string(&mut self, key: &crate::include::internal::CefString, value: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_string {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),crate::include::internal::IntoCef::into_cef(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified key as type binary. Returns true if the
  /// value was set successfully. If |value| is currently owned by another object
  /// then the value will be copied and the |value| reference will not change.
  /// Otherwise, ownership will be transferred to this object and the |value|
  /// reference will be invalidated.
  pub fn set_binary(&mut self, key: &crate::include::internal::CefString, value: crate::include::CefBinaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_binary {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),crate::include::CefBinaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified key as type dict. Returns true if the
  /// value was set successfully. If |value| is currently owned by another object
  /// then the value will be copied and the |value| reference will not change.
  /// Otherwise, ownership will be transferred to this object and the |value|
  /// reference will be invalidated.
  pub fn set_dictionary(&mut self, key: &crate::include::internal::CefString, value: crate::include::CefDictionaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_dictionary {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),crate::include::CefDictionaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified key as type list. Returns true if the
  /// value was set successfully. If |value| is currently owned by another object
  /// then the value will be copied and the |value| reference will not change.
  /// Otherwise, ownership will be transferred to this object and the |value|
  /// reference will be invalidated.
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
  /// Creates a new object that is not owned by any other object.
  #[allow(non_snake_case)]
  pub fn create() -> Option<crate::include::CefListValue> {
    unsafe {
      let ret = cef_sys::cef_list_value_create();
      crate::include::CefListValue::from_cef_own(ret)
    }
  }
  /// Returns true if this object is valid. This object may become invalid if
  /// the underlying data is owned by another object (e.g. list or dictionary)
  /// and that other object is then modified or destroyed. Do not call any other
  /// methods if this method returns false.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object is currently owned by another object.
  pub fn is_owned(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_owned {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the values of this object are read-only. Some APIs may
  /// expose read-only objects.
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object and |that| object have the same underlying
  /// data. If true modifications to this object will also affect |that| object
  /// and vice-versa.
  pub fn is_same(&mut self, that: crate::include::CefListValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefListValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object and |that| object have an equivalent underlying
  /// value but are not necessarily the same object.
  pub fn is_equal(&mut self, that: crate::include::CefListValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_equal {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefListValue::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns a writable copy of this object.
  pub fn copy(&mut self) -> Option<crate::include::CefListValue> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefListValue::from_cef_own(ret)
    }
  }
  /// Sets the number of values. If the number of values is expanded all
  /// new value slots will default to type null. Returns true on success.
  pub fn set_size(&mut self, size: u64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_size {
        Some(f) => f(self.raw.as_ptr(),size,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the number of values.
  pub fn get_size(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_size {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Removes all values. Returns true on success.
  pub fn clear(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().clear {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Removes the value at the specified index.
  pub fn remove(&mut self, index: u64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the value type at the specified index.
  pub fn get_type(&mut self, index: u64) -> crate::include::internal::CefValueType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the value at the specified index. For simple types the returned
  /// value will copy existing data and modifications to the value will not
  /// modify this object. For complex types (binary, dictionary and list) the
  /// returned value will reference existing data and modifications to the value
  /// will modify this object.
  pub fn get_value(&mut self, index: u64) -> Option<crate::include::CefValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_value {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefValue::from_cef_own(ret)
    }
  }
  /// Returns the value at the specified index as type bool.
  pub fn get_bool(&mut self, index: u64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_bool {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the value at the specified index as type int.
  pub fn get_int(&mut self, index: u64) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_int {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the value at the specified index as type double.
  pub fn get_double(&mut self, index: u64) -> f64 {
    unsafe {
      let ret = match self.raw.as_ref().get_double {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the value at the specified index as type string.
  pub fn get_string(&mut self, index: u64) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_string {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the value at the specified index as type binary. The returned
  /// value will reference existing data.
  pub fn get_binary(&mut self, index: u64) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_binary {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  /// Returns the value at the specified index as type dictionary. The returned
  /// value will reference existing data and modifications to the value will
  /// modify this object.
  pub fn get_dictionary(&mut self, index: u64) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_dictionary {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  /// Returns the value at the specified index as type list. The returned
  /// value will reference existing data and modifications to the value will
  /// modify this object.
  pub fn get_list(&mut self, index: u64) -> Option<crate::include::CefListValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_list {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefListValue::from_cef_own(ret)
    }
  }
  /// Sets the value at the specified index. Returns true if the value was set
  /// successfully. If |value| represents simple data then the underlying data
  /// will be copied and modifications to |value| will not modify this object. If
  /// |value| represents complex data (binary, dictionary or list) then the
  /// underlying data will be referenced and modifications to |value| will modify
  /// this object.
  pub fn set_value(&mut self, index: u64, value: crate::include::CefValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::CefValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified index as type null. Returns true if the
  /// value was set successfully.
  pub fn set_null(&mut self, index: u64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_null {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified index as type bool. Returns true if the
  /// value was set successfully.
  pub fn set_bool(&mut self, index: u64, value: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_bool {
        Some(f) => f(self.raw.as_ptr(),index,if value { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified index as type int. Returns true if the
  /// value was set successfully.
  pub fn set_int(&mut self, index: u64, value: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_int {
        Some(f) => f(self.raw.as_ptr(),index,value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified index as type double. Returns true if the
  /// value was set successfully.
  pub fn set_double(&mut self, index: u64, value: f64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_double {
        Some(f) => f(self.raw.as_ptr(),index,value,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified index as type string. Returns true if the
  /// value was set successfully.
  pub fn set_string(&mut self, index: u64, value: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_string {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::internal::IntoCef::into_cef(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified index as type binary. Returns true if the
  /// value was set successfully. If |value| is currently owned by another object
  /// then the value will be copied and the |value| reference will not change.
  /// Otherwise, ownership will be transferred to this object and the |value|
  /// reference will be invalidated.
  pub fn set_binary(&mut self, index: u64, value: crate::include::CefBinaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_binary {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::CefBinaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified index as type dict. Returns true if the
  /// value was set successfully. If |value| is currently owned by another object
  /// then the value will be copied and the |value| reference will not change.
  /// Otherwise, ownership will be transferred to this object and the |value|
  /// reference will be invalidated.
  pub fn set_dictionary(&mut self, index: u64, value: crate::include::CefDictionaryValue) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_dictionary {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::CefDictionaryValue::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Sets the value at the specified index as type list. Returns true if the
  /// value was set successfully. If |value| is currently owned by another object
  /// then the value will be copied and the |value| reference will not change.
  /// Otherwise, ownership will be transferred to this object and the |value|
  /// reference will be invalidated.
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
