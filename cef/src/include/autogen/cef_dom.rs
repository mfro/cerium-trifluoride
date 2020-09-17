/// Interface to implement for visiting the DOM. The methods of this class will
/// be called on the render process main thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait DOMVisitor {
  /// Method executed for visiting the DOM. The document object passed to this
  /// method represents a snapshot of the DOM at the time this method is
  /// executed. DOM objects are only valid for the scope of this method. Do not
  /// keep references to or attempt to access any DOM objects outside the scope
  /// of this method.
  fn visit(&mut self, document: crate::include::CefDOMDocument) -> () { Default::default() }
}
define_refcounted!(DOMVisitor, CefDOMVisitor, cef_domvisitor_t, visit: cef_domvisitor_t_visit,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_domvisitor_t_visit(_self: *mut cef_sys::cef_domvisitor_t, document: *mut cef_sys::cef_domdocument_t) -> () {
  let ret = CefDOMVisitor::from_cef(_self, true).get().visit(crate::include::CefDOMDocument::from_cef_own(document).unwrap(),);
  ret
}
pub type CefDOMDocument = crate::include::refcounting::CefProxy<cef_sys::cef_domdocument_t>;
#[allow(non_snake_case)]
impl CefDOMDocument {
  /// Returns the document type.
  pub fn get_type(&mut self) -> crate::include::internal::CefDomDocumentType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the root document node.
  pub fn get_document(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_document {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  /// Returns the BODY node of an HTML document.
  pub fn get_body(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_body {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  /// Returns the HEAD node of an HTML document.
  pub fn get_head(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_head {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  /// Returns the title of an HTML document.
  pub fn get_title(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_title {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the document element with the specified ID value.
  pub fn get_element_by_id(&mut self, id: &crate::include::internal::CefString) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_element_by_id {
        Some(f) => f(self.raw.as_ptr(),id as *const _ as *const _,),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  /// Returns the node that currently has keyboard focus.
  pub fn get_focused_node(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_focused_node {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  /// Returns true if a portion of the document is selected.
  pub fn has_selection(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_selection {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the selection offset within the start node.
  pub fn get_selection_start_offset(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_selection_start_offset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the selection offset within the end node.
  pub fn get_selection_end_offset(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_selection_end_offset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the contents of this selection as markup.
  pub fn get_selection_as_markup(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_selection_as_markup {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the contents of this selection as text.
  pub fn get_selection_as_text(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_selection_as_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the base URL for the document.
  pub fn get_base_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_base_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns a complete URL based on the document base URL and the specified
  /// partial URL.
  pub fn get_complete_url(&mut self, partialURL: &crate::include::internal::CefString) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_complete_url {
        Some(f) => f(self.raw.as_ptr(),partialURL as *const _ as *const _,),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
}
pub type CefDOMNode = crate::include::refcounting::CefProxy<cef_sys::cef_domnode_t>;
#[allow(non_snake_case)]
impl CefDOMNode {
  /// Returns the type for this node.
  pub fn get_type(&mut self) -> crate::include::internal::CefDomNodeType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns true if this is a text node.
  pub fn is_text(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this is an element node.
  pub fn is_element(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_element {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this is an editable node.
  pub fn is_editable(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_editable {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this is a form control element node.
  pub fn is_form_control_element(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_form_control_element {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the type of this form control element node.
  pub fn get_form_control_element_type(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_form_control_element_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns true if this object is pointing to the same handle as |that|
  /// object.
  pub fn is_same(&mut self, that: crate::include::CefDOMNode) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDOMNode::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the name of this node.
  pub fn get_name(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the value of this node.
  pub fn get_value(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Set the value of this node. Returns true on success.
  pub fn set_value(&mut self, value: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value {
        Some(f) => f(self.raw.as_ptr(),value as *const _ as *const _,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the contents of this node as markup.
  pub fn get_as_markup(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_as_markup {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the document associated with this node.
  pub fn get_document(&mut self) -> Option<crate::include::CefDOMDocument> {
    unsafe {
      let ret = match self.raw.as_ref().get_document {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMDocument::from_cef_own(ret)
    }
  }
  /// Returns the parent node.
  pub fn get_parent(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_parent {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  /// Returns the previous sibling node.
  pub fn get_previous_sibling(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_previous_sibling {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  /// Returns the next sibling node.
  pub fn get_next_sibling(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_next_sibling {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  /// Returns true if this node has child nodes.
  pub fn has_children(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_children {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Return the first child node.
  pub fn get_first_child(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_first_child {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  /// Returns the last child node.
  pub fn get_last_child(&mut self) -> Option<crate::include::CefDOMNode> {
    unsafe {
      let ret = match self.raw.as_ref().get_last_child {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDOMNode::from_cef_own(ret)
    }
  }
  /// The following methods are valid only for element nodes.
  /// Returns the tag name of this element.
  pub fn get_element_tag_name(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_element_tag_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns true if this element has attributes.
  pub fn has_element_attributes(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_element_attributes {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this element has an attribute named |attrName|.
  pub fn has_element_attribute(&mut self, attrName: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_element_attribute {
        Some(f) => f(self.raw.as_ptr(),attrName as *const _ as *const _,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the element attribute named |attrName|.
  pub fn get_element_attribute(&mut self, attrName: &crate::include::internal::CefString) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_element_attribute {
        Some(f) => f(self.raw.as_ptr(),attrName as *const _ as *const _,),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Set the value for the element attribute named |attrName|. Returns true on
  /// success.
  pub fn set_element_attribute(&mut self, attrName: &crate::include::internal::CefString, value: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_element_attribute {
        Some(f) => f(self.raw.as_ptr(),attrName as *const _ as *const _,value as *const _ as *const _,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the inner text of the element.
  pub fn get_element_inner_text(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_element_inner_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the bounds of the element.
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
