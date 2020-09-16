/// Interface to implement for visiting the DOM. The methods of this class will
/// be called on the render process main thread.
#[allow(non_snake_case)]
pub trait DOMVisitor {
  fn visit(&mut self, document: crate::include::CefDOMDocument) -> () { Default::default() }
}
define_refcounted!(DOMVisitor, domvisitor, visit,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_domvisitor_t_visit(_self: *mut cef_sys::cef_domvisitor_t, document: *mut cef_sys::cef_domdocument_t) -> () {
  let ret = CefDOMVisitor::from_cef(_self, true).get().visit(crate::include::CefDOMDocument::from_cef_own(document).unwrap(),);
  ret
}
pub type CefDOMDocument = crate::include::base::CefProxy<cef_sys::cef_domdocument_t>;
#[allow(non_snake_case)]
impl CefDOMDocument {
  pub fn get_type(&mut self) -> crate::include::internal::CefDomDocumentType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_document(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_document {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  pub fn get_body(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_body {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  pub fn get_head(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_head {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  pub fn get_title(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_title {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_element_by_id(&mut self, id: &crate::include::internal::CefString) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_element_by_id {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(id),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  pub fn get_focused_node(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_focused_node {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  pub fn has_selection(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_selection {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_selection_start_offset(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_selection_start_offset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_selection_end_offset(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_selection_end_offset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_selection_as_markup(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_selection_as_markup {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_selection_as_text(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_selection_as_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_base_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_base_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_complete_url(&mut self, partialURL: &crate::include::internal::CefString) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_complete_url {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(partialURL),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
}
pub type CefDOMNode = crate::include::base::CefProxy<cef_sys::cef_domnode_t>;
#[allow(non_snake_case)]
impl CefDOMNode {
  pub fn get_type(&mut self) -> crate::include::internal::CefDomNodeType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn is_text(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_element(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_element {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_editable(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_editable {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_form_control_element(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_form_control_element {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_form_control_element_type(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_form_control_element_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn is_same(&mut self, that: crate::include::CefDOMNode) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDOMNode::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_value(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_value(&mut self, value: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_as_markup(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_as_markup {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_document(&mut self) -> Option<crate::include::CefDOMDocument> {
    unsafe {
      let ret = match self.raw.as_ref().get_document {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMDocument::from_cef_own(ret)
    }
  }
  pub fn get_parent(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_parent {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  pub fn get_previous_sibling(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_previous_sibling {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  pub fn get_next_sibling(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_next_sibling {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  pub fn has_children(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_children {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_first_child(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_first_child {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  pub fn get_last_child(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_last_child {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  pub fn get_element_tag_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_element_tag_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn has_element_attributes(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_element_attributes {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_element_attribute(&mut self, attrName: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_element_attribute {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(attrName),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_element_attribute(&mut self, attrName: &crate::include::internal::CefString) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_element_attribute {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(attrName),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_element_attribute(&mut self, attrName: &crate::include::internal::CefString, value: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_element_attribute {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(attrName),crate::include::internal::IntoCef::into_cef(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_element_inner_text(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_element_inner_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_element_bounds(&mut self) -> crate::include::internal::CefRect {
    unsafe {
      let ret = match self.raw.as_ref().get_element_bounds {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
}
