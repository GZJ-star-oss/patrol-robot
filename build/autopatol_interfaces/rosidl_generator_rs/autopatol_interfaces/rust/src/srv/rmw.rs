#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "autopatol_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__autopatol_interfaces__srv__SpeechText_Request() -> *const std::ffi::c_void;
}

#[link(name = "autopatol_interfaces__rosidl_generator_c")]
extern "C" {
    fn autopatol_interfaces__srv__SpeechText_Request__init(msg: *mut SpeechText_Request) -> bool;
    fn autopatol_interfaces__srv__SpeechText_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeechText_Request>, size: usize) -> bool;
    fn autopatol_interfaces__srv__SpeechText_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeechText_Request>);
    fn autopatol_interfaces__srv__SpeechText_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeechText_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeechText_Request>) -> bool;
}

// Corresponds to autopatol_interfaces__srv__SpeechText_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeechText_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub text: rosidl_runtime_rs::String,

}



impl Default for SpeechText_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !autopatol_interfaces__srv__SpeechText_Request__init(&mut msg as *mut _) {
        panic!("Call to autopatol_interfaces__srv__SpeechText_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeechText_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { autopatol_interfaces__srv__SpeechText_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { autopatol_interfaces__srv__SpeechText_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { autopatol_interfaces__srv__SpeechText_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeechText_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeechText_Request where Self: Sized {
  const TYPE_NAME: &'static str = "autopatol_interfaces/srv/SpeechText_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__autopatol_interfaces__srv__SpeechText_Request() }
  }
}


#[link(name = "autopatol_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__autopatol_interfaces__srv__SpeechText_Response() -> *const std::ffi::c_void;
}

#[link(name = "autopatol_interfaces__rosidl_generator_c")]
extern "C" {
    fn autopatol_interfaces__srv__SpeechText_Response__init(msg: *mut SpeechText_Response) -> bool;
    fn autopatol_interfaces__srv__SpeechText_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeechText_Response>, size: usize) -> bool;
    fn autopatol_interfaces__srv__SpeechText_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeechText_Response>);
    fn autopatol_interfaces__srv__SpeechText_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeechText_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeechText_Response>) -> bool;
}

// Corresponds to autopatol_interfaces__srv__SpeechText_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeechText_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for SpeechText_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !autopatol_interfaces__srv__SpeechText_Response__init(&mut msg as *mut _) {
        panic!("Call to autopatol_interfaces__srv__SpeechText_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeechText_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { autopatol_interfaces__srv__SpeechText_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { autopatol_interfaces__srv__SpeechText_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { autopatol_interfaces__srv__SpeechText_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeechText_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeechText_Response where Self: Sized {
  const TYPE_NAME: &'static str = "autopatol_interfaces/srv/SpeechText_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__autopatol_interfaces__srv__SpeechText_Response() }
  }
}






#[link(name = "autopatol_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__autopatol_interfaces__srv__SpeechText() -> *const std::ffi::c_void;
}

// Corresponds to autopatol_interfaces__srv__SpeechText
#[allow(missing_docs, non_camel_case_types)]
pub struct SpeechText;

impl rosidl_runtime_rs::Service for SpeechText {
    type Request = SpeechText_Request;
    type Response = SpeechText_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__autopatol_interfaces__srv__SpeechText() }
    }
}


