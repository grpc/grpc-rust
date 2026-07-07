const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0proc__v3__ProcessingMode_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ProcessingMode {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ProcessingMode>
}

impl ::protobuf::Message for ProcessingMode {
  type MessageView<'msg> = ProcessingModeView<'msg>;
  type MessageMut<'msg> = ProcessingModeMut<'msg>;
}

impl ::std::default::Default for ProcessingMode {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ProcessingMode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ProcessingMode` is `Sync` because it does not implement interior mutability.
//    Neither does `ProcessingModeMut`.
unsafe impl ::std::marker::Sync for ProcessingMode {}

// SAFETY:
// - `ProcessingMode` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ProcessingMode {}

impl ::protobuf::Proxied for ProcessingMode {
  type View<'msg> = ProcessingModeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ProcessingMode {}

impl ::protobuf::MutProxied for ProcessingMode {
  type Mut<'msg> = ProcessingModeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProcessingModeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProcessingMode>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProcessingModeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ProcessingModeView<'msg> {
  type Message = ProcessingMode;
}

impl ::std::fmt::Debug for ProcessingModeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ProcessingModeView<'_> {
  fn default() -> ProcessingModeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ProcessingMode>> for ProcessingModeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProcessingMode>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProcessingModeView<'msg> {

  pub fn to_owned(&self) -> ProcessingMode {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // request_header_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn request_header_mode(self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }

  // response_header_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn response_header_mode(self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }

  // request_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn request_body_mode(self) -> super::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }

  // response_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn response_body_mode(self) -> super::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }

  // request_trailer_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn request_trailer_mode(self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }

  // response_trailer_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn response_trailer_mode(self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ProcessingModeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ProcessingModeView<'_> {}

// SAFETY:
// - `ProcessingModeView` is `Send` because while its alive a `ProcessingModeMut` cannot.
// - `ProcessingModeView` does not use thread-local data.
unsafe impl ::std::marker::Send for ProcessingModeView<'_> {}

impl<'msg> ::protobuf::AsView for ProcessingModeView<'msg> {
  type Proxied = ProcessingMode;
  fn as_view(&self) -> ::protobuf::View<'msg, ProcessingMode> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProcessingModeView<'msg> {
  fn into_view<'shorter>(self) -> ProcessingModeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ProcessingMode> for ProcessingModeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProcessingMode {
    let mut dst = ProcessingMode::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ProcessingMode> for ProcessingModeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProcessingMode {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ProcessingMode {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProcessingModeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProcessingModeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProcessingModeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingMode>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProcessingModeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ProcessingModeMut<'msg> {
  type Message = ProcessingMode;
}

impl ::std::fmt::Debug for ProcessingModeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingMode>> for ProcessingModeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingMode>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProcessingModeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingMode> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ProcessingMode {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // request_header_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn request_header_mode(&self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_header_mode(&mut self, val: super::processing_mode::HeaderSendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // response_header_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn response_header_mode(&self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_header_mode(&mut self, val: super::processing_mode::HeaderSendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // request_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn request_body_mode(&self) -> super::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_body_mode(&mut self, val: super::processing_mode::BodySendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // response_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn response_body_mode(&self) -> super::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_body_mode(&mut self, val: super::processing_mode::BodySendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // request_trailer_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn request_trailer_mode(&self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_trailer_mode(&mut self, val: super::processing_mode::HeaderSendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // response_trailer_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn response_trailer_mode(&self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_trailer_mode(&mut self, val: super::processing_mode::HeaderSendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

}

// SAFETY:
// - `ProcessingModeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ProcessingModeMut<'_> {}

// SAFETY:
// - `ProcessingModeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ProcessingModeMut<'_> {}

impl<'msg> ::protobuf::AsView for ProcessingModeMut<'msg> {
  type Proxied = ProcessingMode;
  fn as_view(&self) -> ::protobuf::View<'_, ProcessingMode> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProcessingModeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ProcessingMode>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ProcessingModeMut<'msg> {
  type MutProxied = ProcessingMode;
  fn as_mut(&mut self) -> ProcessingModeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ProcessingModeMut<'msg> {
  fn into_mut<'shorter>(self) -> ProcessingModeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ProcessingMode {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ProcessingMode> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ProcessingModeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ProcessingModeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // request_header_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn request_header_mode(&self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_header_mode(&mut self, val: super::processing_mode::HeaderSendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // response_header_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn response_header_mode(&self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_header_mode(&mut self, val: super::processing_mode::HeaderSendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // request_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn request_body_mode(&self) -> super::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_body_mode(&mut self, val: super::processing_mode::BodySendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // response_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn response_body_mode(&self) -> super::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_body_mode(&mut self, val: super::processing_mode::BodySendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // request_trailer_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn request_trailer_mode(&self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_trailer_mode(&mut self, val: super::processing_mode::HeaderSendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // response_trailer_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.HeaderSendMode
  pub fn response_trailer_mode(&self) -> super::processing_mode::HeaderSendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::processing_mode::HeaderSendMode::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_trailer_mode(&mut self, val: super::processing_mode::HeaderSendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

}  // impl ProcessingMode

impl ::std::ops::Drop for ProcessingMode {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ProcessingMode {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ProcessingMode {
  type Proxied = Self;
  fn as_view(&self) -> ProcessingModeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ProcessingMode {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ProcessingModeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ProcessingMode {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0proc__v3__ProcessingMode_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P.P.P.P.P.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0proc__v3__ProcessingMode_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0proc__v3__ProcessingMode_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProcessingMode {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProcessingMode {
  type Msg = ProcessingMode;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingMode> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProcessingMode {
  type Msg = ProcessingMode;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingMode> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProcessingModeMut<'_> {
  type Msg = ProcessingMode;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingMode> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProcessingModeMut<'_> {
  type Msg = ProcessingMode;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingMode> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProcessingModeView<'_> {
  type Msg = ProcessingMode;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingMode> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProcessingModeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod processing_mode {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HeaderSendMode(i32);

#[allow(non_upper_case_globals)]
impl HeaderSendMode {
  pub const Default: HeaderSendMode = HeaderSendMode(0);
  pub const Send: HeaderSendMode = HeaderSendMode(1);
  pub const Skip: HeaderSendMode = HeaderSendMode(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Default",
      1 => "Send",
      2 => "Skip",
      _ => return None
    })
  }
}

impl ::std::convert::From<HeaderSendMode> for i32 {
  fn from(val: HeaderSendMode) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for HeaderSendMode {
  fn from(val: i32) -> HeaderSendMode {
    Self(val)
  }
}

impl ::std::default::Default for HeaderSendMode {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for HeaderSendMode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "HeaderSendMode::{}", constant_name)
    } else {
      write!(f, "HeaderSendMode::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for HeaderSendMode {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for HeaderSendMode {}

impl ::protobuf::Proxied for HeaderSendMode {
  type View<'a> = HeaderSendMode;
}

impl ::protobuf::AsView for HeaderSendMode {
  type Proxied = HeaderSendMode;

  fn as_view(&self) -> HeaderSendMode {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderSendMode {
  fn into_view<'shorter>(self) -> HeaderSendMode where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for HeaderSendMode {
  const NAME: &'static str = "HeaderSendMode";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for HeaderSendMode {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BodySendMode(i32);

#[allow(non_upper_case_globals)]
impl BodySendMode {
  pub const None: BodySendMode = BodySendMode(0);
  pub const Streamed: BodySendMode = BodySendMode(1);
  pub const Buffered: BodySendMode = BodySendMode(2);
  pub const BufferedPartial: BodySendMode = BodySendMode(3);
  pub const FullDuplexStreamed: BodySendMode = BodySendMode(4);
  pub const Grpc: BodySendMode = BodySendMode(5);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "None",
      1 => "Streamed",
      2 => "Buffered",
      3 => "BufferedPartial",
      4 => "FullDuplexStreamed",
      5 => "Grpc",
      _ => return None
    })
  }
}

impl ::std::convert::From<BodySendMode> for i32 {
  fn from(val: BodySendMode) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for BodySendMode {
  fn from(val: i32) -> BodySendMode {
    Self(val)
  }
}

impl ::std::default::Default for BodySendMode {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for BodySendMode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "BodySendMode::{}", constant_name)
    } else {
      write!(f, "BodySendMode::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for BodySendMode {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for BodySendMode {}

impl ::protobuf::Proxied for BodySendMode {
  type View<'a> = BodySendMode;
}

impl ::protobuf::AsView for BodySendMode {
  type Proxied = BodySendMode;

  fn as_view(&self) -> BodySendMode {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BodySendMode {
  fn into_view<'shorter>(self) -> BodySendMode where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for BodySendMode {
  const NAME: &'static str = "BodySendMode";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5)
  }
}

impl ::protobuf::__internal::EntityType for BodySendMode {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod processing_mode


