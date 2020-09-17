pub type CefXmlReader = crate::include::refcounting::CefProxy<cef_sys::cef_xml_reader_t>;
#[allow(non_snake_case)]
impl CefXmlReader {
  /// Create a new CefXmlReader object. The returned object's methods can only
  /// be called from the thread that created the object.
  #[allow(non_snake_case)]
  pub fn create(stream: crate::include::CefStreamReader, encodingType: crate::include::internal::CefXmlEncodingType, URI: &crate::include::internal::CefString, ) -> Option<crate::include::CefXmlReader> {
    unsafe {
      let ret = cef_sys::cef_xml_reader_create(crate::include::CefStreamReader::to_cef_own(stream),encodingType.into(),URI as *const _ as *const _,);
      crate::include::CefXmlReader::from_cef_own(ret)
    }
  }
  /// Moves the cursor to the next node in the document. This method must be
  /// called at least once to set the current cursor position. Returns true if
  /// the cursor position was set successfully.
  pub fn move_to_next_node(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_next_node {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Close the document. This should be called directly to ensure that cleanup
  /// occurs on the correct thread.
  pub fn close(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().close {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if an error has been reported by the XML parser.
  pub fn has_error(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_error {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the error string.
  pub fn get_error(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_error {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// The below methods retrieve data for the node at the current cursor
  /// position.
  /// Returns the node type.
  pub fn get_type(&mut self) -> crate::include::internal::CefXmlNodeType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the node depth. Depth starts at 0 for the root node.
  pub fn get_depth(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_depth {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the local name. See
  /// http://www.w3.org/TR/REC-xml-names/#NT-LocalPart for additional details.
  pub fn get_local_name(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_local_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the namespace prefix. See http://www.w3.org/TR/REC-xml-names/ for
  /// additional details.
  pub fn get_prefix(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_prefix {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the qualified name, equal to (Prefix:)LocalName. See
  /// http://www.w3.org/TR/REC-xml-names/#ns-qualnames for additional details.
  pub fn get_qualified_name(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_qualified_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the URI defining the namespace associated with the node. See
  /// http://www.w3.org/TR/REC-xml-names/ for additional details.
  pub fn get_namespace_uri(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_namespace_uri {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the base URI of the node. See http://www.w3.org/TR/xmlbase/ for
  /// additional details.
  pub fn get_base_uri(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_base_uri {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the xml:lang scope within which the node resides. See
  /// http://www.w3.org/TR/REC-xml/#sec-lang-tag for additional details.
  pub fn get_xml_lang(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_xml_lang {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns true if the node represents an empty element. <a/> is considered
  /// empty but <a></a> is not.
  pub fn is_empty_element(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_empty_element {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the node has a text value.
  pub fn has_value(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the text value.
  pub fn get_value(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns true if the node has attributes.
  pub fn has_attributes(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_attributes {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the number of attributes.
  pub fn get_attribute_count(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_attribute_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the value of the attribute at the specified 0-based index.
  pub fn get_attribute_byindex(&mut self, index: i32) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_attribute_byindex {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the value of the attribute with the specified qualified name.
  pub fn get_attribute_byqname(&mut self, qualifiedName: &crate::include::internal::CefString) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_attribute_byqname {
        Some(f) => f(self.raw.as_ptr(),qualifiedName as *const _ as *const _,),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the value of the attribute with the specified local name and
  /// namespace URI.
  pub fn get_attribute_bylname(&mut self, localName: &crate::include::internal::CefString, namespaceURI: &crate::include::internal::CefString) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_attribute_bylname {
        Some(f) => f(self.raw.as_ptr(),localName as *const _ as *const _,namespaceURI as *const _ as *const _,),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns an XML representation of the current node's children.
  pub fn get_inner_xml(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_inner_xml {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns an XML representation of the current node including its children.
  pub fn get_outer_xml(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_outer_xml {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the line number for the current node.
  pub fn get_line_number(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_line_number {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Attribute nodes are not traversed by default. The below methods can be
  /// used to move the cursor to an attribute node. MoveToCarryingElement() can
  /// be called afterwards to return the cursor to the carrying element. The
  /// depth of an attribute node will be 1 + the depth of the carrying element.
  /// Moves the cursor to the attribute at the specified 0-based index. Returns
  /// true if the cursor position was set successfully.
  pub fn move_to_attribute_byindex(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_attribute_byindex {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Moves the cursor to the attribute with the specified qualified name.
  /// Returns true if the cursor position was set successfully.
  pub fn move_to_attribute_byqname(&mut self, qualifiedName: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_attribute_byqname {
        Some(f) => f(self.raw.as_ptr(),qualifiedName as *const _ as *const _,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Moves the cursor to the attribute with the specified local name and
  /// namespace URI. Returns true if the cursor position was set successfully.
  pub fn move_to_attribute_bylname(&mut self, localName: &crate::include::internal::CefString, namespaceURI: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_attribute_bylname {
        Some(f) => f(self.raw.as_ptr(),localName as *const _ as *const _,namespaceURI as *const _ as *const _,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Moves the cursor to the first attribute in the current element. Returns
  /// true if the cursor position was set successfully.
  pub fn move_to_first_attribute(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_first_attribute {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Moves the cursor to the next attribute in the current element. Returns
  /// true if the cursor position was set successfully.
  pub fn move_to_next_attribute(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_next_attribute {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Moves the cursor back to the carrying element. Returns true if the cursor
  /// position was set successfully.
  pub fn move_to_carrying_element(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_carrying_element {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
