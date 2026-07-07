const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__SocketOption_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SocketOption {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SocketOption>
}

impl ::protobuf::Message for SocketOption {
  type MessageView<'msg> = SocketOptionView<'msg>;
  type MessageMut<'msg> = SocketOptionMut<'msg>;
}

impl ::std::default::Default for SocketOption {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SocketOption {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SocketOption` is `Sync` because it does not implement interior mutability.
//    Neither does `SocketOptionMut`.
unsafe impl ::std::marker::Sync for SocketOption {}

// SAFETY:
// - `SocketOption` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SocketOption {}

impl ::protobuf::Proxied for SocketOption {
  type View<'msg> = SocketOptionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SocketOption {}

impl ::protobuf::MutProxied for SocketOption {
  type Mut<'msg> = SocketOptionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SocketOptionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SocketOption>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SocketOptionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SocketOptionView<'msg> {
  type Message = SocketOption;
}

impl ::std::fmt::Debug for SocketOptionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SocketOptionView<'_> {
  fn default() -> SocketOptionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SocketOption>> for SocketOptionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SocketOption>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SocketOptionView<'msg> {

  pub fn to_owned(&self) -> SocketOption {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // description: optional string
  pub fn description(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // level: optional int64
  pub fn level(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // name: optional int64
  pub fn name(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // int_value: optional int64
  pub fn has_int_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn int_value_opt(self) -> ::std::option::Option<i64> {
    self.has_int_value().then(|| self.int_value())
  }
  pub fn int_value(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // buf_value: optional bytes
  pub fn has_buf_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn buf_value_opt(self) -> ::std::option::Option<&'msg [u8]> {
    self.has_buf_value().then(|| self.buf_value())
  }
  pub fn buf_value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // state: optional enum envoy.config.core.v3.SocketOption.SocketState
  pub fn state(self) -> super::socket_option::SocketState {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::socket_option::SocketState::StatePrebind).into()
      ).try_into().unwrap()
    }
  }

  // type: optional message envoy.config.core.v3.SocketOption.SocketType
  pub fn has_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn type_opt(self) -> ::std::option::Option<super::socket_option::SocketTypeView<'msg>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(self) -> super::socket_option::SocketTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::socket_option::SocketTypeView::default())
  }

  pub fn value(self) -> super::socket_option::ValueOneof<'msg> {
    match self.value_case() {
      super::socket_option::ValueCase::IntValue =>
          super::socket_option::ValueOneof::IntValue(self.int_value()),
      super::socket_option::ValueCase::BufValue =>
          super::socket_option::ValueOneof::BufValue(self.buf_value()),
      _ => super::socket_option::ValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_case(self) -> super::socket_option::ValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::socket_option::ValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SocketOptionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SocketOptionView<'_> {}

// SAFETY:
// - `SocketOptionView` is `Send` because while its alive a `SocketOptionMut` cannot.
// - `SocketOptionView` does not use thread-local data.
unsafe impl ::std::marker::Send for SocketOptionView<'_> {}

impl<'msg> ::protobuf::AsView for SocketOptionView<'msg> {
  type Proxied = SocketOption;
  fn as_view(&self) -> ::protobuf::View<'msg, SocketOption> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SocketOptionView<'msg> {
  fn into_view<'shorter>(self) -> SocketOptionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SocketOption> for SocketOptionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SocketOption {
    let mut dst = SocketOption::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SocketOption> for SocketOptionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SocketOption {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SocketOption {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SocketOptionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SocketOptionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SocketOptionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketOption>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SocketOptionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SocketOptionMut<'msg> {
  type Message = SocketOption;
}

impl ::std::fmt::Debug for SocketOptionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SocketOption>> for SocketOptionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketOption>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SocketOptionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketOption> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SocketOption {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // description: optional string
  pub fn description(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_description(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // level: optional int64
  pub fn level(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_level(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  // name: optional int64
  pub fn name(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_name(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // int_value: optional int64
  pub fn has_int_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_int_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn int_value_opt(&self) -> ::std::option::Option<i64> {
    self.has_int_value().then(|| self.int_value())
  }
  pub fn int_value(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_int_value(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // buf_value: optional bytes
  pub fn has_buf_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_buf_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn buf_value_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_buf_value().then(|| self.buf_value())
  }
  pub fn buf_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_buf_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // state: optional enum envoy.config.core.v3.SocketOption.SocketState
  pub fn state(&self) -> super::socket_option::SocketState {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::socket_option::SocketState::StatePrebind).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_state(&mut self, val: super::socket_option::SocketState) {
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

  // type: optional message envoy.config.core.v3.SocketOption.SocketType
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn type_opt(&self) -> ::std::option::Option<super::socket_option::SocketTypeView<'_>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(&self) -> super::socket_option::SocketTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::socket_option::SocketTypeView::default())
  }
  pub fn type_mut(&mut self) -> super::socket_option::SocketTypeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::socket_option::SocketType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn value(&self) -> super::socket_option::ValueOneof<'_> {
    match &self.value_case() {
      super::socket_option::ValueCase::IntValue =>
          super::socket_option::ValueOneof::IntValue(self.int_value()),
      super::socket_option::ValueCase::BufValue =>
          super::socket_option::ValueOneof::BufValue(self.buf_value()),
      _ => super::socket_option::ValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_case(&self) -> super::socket_option::ValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::socket_option::ValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SocketOptionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SocketOptionMut<'_> {}

// SAFETY:
// - `SocketOptionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SocketOptionMut<'_> {}

impl<'msg> ::protobuf::AsView for SocketOptionMut<'msg> {
  type Proxied = SocketOption;
  fn as_view(&self) -> ::protobuf::View<'_, SocketOption> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SocketOptionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SocketOption>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SocketOptionMut<'msg> {
  type MutProxied = SocketOption;
  fn as_mut(&mut self) -> SocketOptionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SocketOptionMut<'msg> {
  fn into_mut<'shorter>(self) -> SocketOptionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SocketOption {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SocketOption> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SocketOptionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SocketOptionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // description: optional string
  pub fn description(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_description(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // level: optional int64
  pub fn level(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_level(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  // name: optional int64
  pub fn name(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_name(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // int_value: optional int64
  pub fn has_int_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_int_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn int_value_opt(&self) -> ::std::option::Option<i64> {
    self.has_int_value().then(|| self.int_value())
  }
  pub fn int_value(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_int_value(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // buf_value: optional bytes
  pub fn has_buf_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_buf_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn buf_value_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_buf_value().then(|| self.buf_value())
  }
  pub fn buf_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_buf_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // state: optional enum envoy.config.core.v3.SocketOption.SocketState
  pub fn state(&self) -> super::socket_option::SocketState {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::socket_option::SocketState::StatePrebind).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_state(&mut self, val: super::socket_option::SocketState) {
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

  // type: optional message envoy.config.core.v3.SocketOption.SocketType
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn type_opt(&self) -> ::std::option::Option<super::socket_option::SocketTypeView<'_>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(&self) -> super::socket_option::SocketTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::socket_option::SocketTypeView::default())
  }
  pub fn type_mut(&mut self) -> super::socket_option::SocketTypeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::socket_option::SocketType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn value(&self) -> super::socket_option::ValueOneof<'_> {
    match &self.value_case() {
      super::socket_option::ValueCase::IntValue =>
          super::socket_option::ValueOneof::IntValue(self.int_value()),
      super::socket_option::ValueCase::BufValue =>
          super::socket_option::ValueOneof::BufValue(self.buf_value()),
      _ => super::socket_option::ValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_case(&self) -> super::socket_option::ValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::socket_option::ValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl SocketOption

impl ::std::ops::Drop for SocketOption {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SocketOption {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SocketOption {
  type Proxied = Self;
  fn as_view(&self) -> SocketOptionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SocketOption {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SocketOptionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SocketOption {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__SocketOption_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X+P+P+0.P3^%|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__SocketOption_msg_init.0, &[<super::socket_option::SocketType as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__SocketOption_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SocketOption {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SocketOption {
  type Msg = SocketOption;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketOption> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketOption {
  type Msg = SocketOption;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketOption> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SocketOptionMut<'_> {
  type Msg = SocketOption;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketOption> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketOptionMut<'_> {
  type Msg = SocketOption;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketOption> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketOptionView<'_> {
  type Msg = SocketOption;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketOption> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SocketOptionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod socket_option {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__SocketOption__SocketType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SocketType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SocketType>
}

impl ::protobuf::Message for SocketType {
  type MessageView<'msg> = SocketTypeView<'msg>;
  type MessageMut<'msg> = SocketTypeMut<'msg>;
}

impl ::std::default::Default for SocketType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SocketType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SocketType` is `Sync` because it does not implement interior mutability.
//    Neither does `SocketTypeMut`.
unsafe impl ::std::marker::Sync for SocketType {}

// SAFETY:
// - `SocketType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SocketType {}

impl ::protobuf::Proxied for SocketType {
  type View<'msg> = SocketTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SocketType {}

impl ::protobuf::MutProxied for SocketType {
  type Mut<'msg> = SocketTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SocketTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SocketType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SocketTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SocketTypeView<'msg> {
  type Message = SocketType;
}

impl ::std::fmt::Debug for SocketTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SocketTypeView<'_> {
  fn default() -> SocketTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SocketType>> for SocketTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SocketType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SocketTypeView<'msg> {

  pub fn to_owned(&self) -> SocketType {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // stream: optional message envoy.config.core.v3.SocketOption.SocketType.Stream
  pub fn has_stream(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn stream_opt(self) -> ::std::option::Option<super::super::socket_option::socket_type::StreamView<'msg>> {
    self.has_stream().then(|| self.stream())
  }
  pub fn stream(self) -> super::super::socket_option::socket_type::StreamView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::socket_option::socket_type::StreamView::default())
  }

  // datagram: optional message envoy.config.core.v3.SocketOption.SocketType.Datagram
  pub fn has_datagram(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn datagram_opt(self) -> ::std::option::Option<super::super::socket_option::socket_type::DatagramView<'msg>> {
    self.has_datagram().then(|| self.datagram())
  }
  pub fn datagram(self) -> super::super::socket_option::socket_type::DatagramView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::socket_option::socket_type::DatagramView::default())
  }

}

// SAFETY:
// - `SocketTypeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SocketTypeView<'_> {}

// SAFETY:
// - `SocketTypeView` is `Send` because while its alive a `SocketTypeMut` cannot.
// - `SocketTypeView` does not use thread-local data.
unsafe impl ::std::marker::Send for SocketTypeView<'_> {}

impl<'msg> ::protobuf::AsView for SocketTypeView<'msg> {
  type Proxied = SocketType;
  fn as_view(&self) -> ::protobuf::View<'msg, SocketType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SocketTypeView<'msg> {
  fn into_view<'shorter>(self) -> SocketTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SocketType> for SocketTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SocketType {
    let mut dst = SocketType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SocketType> for SocketTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SocketType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SocketType {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SocketTypeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SocketTypeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SocketTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SocketTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SocketTypeMut<'msg> {
  type Message = SocketType;
}

impl ::std::fmt::Debug for SocketTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SocketType>> for SocketTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SocketTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketType> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SocketType {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // stream: optional message envoy.config.core.v3.SocketOption.SocketType.Stream
  pub fn has_stream(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_stream(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn stream_opt(&self) -> ::std::option::Option<super::super::socket_option::socket_type::StreamView<'_>> {
    self.has_stream().then(|| self.stream())
  }
  pub fn stream(&self) -> super::super::socket_option::socket_type::StreamView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::socket_option::socket_type::StreamView::default())
  }
  pub fn stream_mut(&mut self) -> super::super::socket_option::socket_type::StreamMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stream(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::socket_option::socket_type::Stream>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // datagram: optional message envoy.config.core.v3.SocketOption.SocketType.Datagram
  pub fn has_datagram(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_datagram(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn datagram_opt(&self) -> ::std::option::Option<super::super::socket_option::socket_type::DatagramView<'_>> {
    self.has_datagram().then(|| self.datagram())
  }
  pub fn datagram(&self) -> super::super::socket_option::socket_type::DatagramView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::socket_option::socket_type::DatagramView::default())
  }
  pub fn datagram_mut(&mut self) -> super::super::socket_option::socket_type::DatagramMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_datagram(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::socket_option::socket_type::Datagram>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `SocketTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SocketTypeMut<'_> {}

// SAFETY:
// - `SocketTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SocketTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for SocketTypeMut<'msg> {
  type Proxied = SocketType;
  fn as_view(&self) -> ::protobuf::View<'_, SocketType> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SocketTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SocketType>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SocketTypeMut<'msg> {
  type MutProxied = SocketType;
  fn as_mut(&mut self) -> SocketTypeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SocketTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> SocketTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SocketType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SocketType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SocketTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SocketTypeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // stream: optional message envoy.config.core.v3.SocketOption.SocketType.Stream
  pub fn has_stream(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_stream(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn stream_opt(&self) -> ::std::option::Option<super::super::socket_option::socket_type::StreamView<'_>> {
    self.has_stream().then(|| self.stream())
  }
  pub fn stream(&self) -> super::super::socket_option::socket_type::StreamView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::socket_option::socket_type::StreamView::default())
  }
  pub fn stream_mut(&mut self) -> super::super::socket_option::socket_type::StreamMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stream(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::socket_option::socket_type::Stream>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // datagram: optional message envoy.config.core.v3.SocketOption.SocketType.Datagram
  pub fn has_datagram(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_datagram(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn datagram_opt(&self) -> ::std::option::Option<super::super::socket_option::socket_type::DatagramView<'_>> {
    self.has_datagram().then(|| self.datagram())
  }
  pub fn datagram(&self) -> super::super::socket_option::socket_type::DatagramView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::socket_option::socket_type::DatagramView::default())
  }
  pub fn datagram_mut(&mut self) -> super::super::socket_option::socket_type::DatagramMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_datagram(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::socket_option::socket_type::Datagram>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl SocketType

impl ::std::ops::Drop for SocketType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SocketType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SocketType {
  type Proxied = Self;
  fn as_view(&self) -> SocketTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SocketType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SocketTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SocketType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::socket_option::envoy__config__core__v3__SocketOption__SocketType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::socket_option::envoy__config__core__v3__SocketOption__SocketType_msg_init.0, &[<super::super::socket_option::socket_type::Stream as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::socket_option::socket_type::Datagram as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::socket_option::envoy__config__core__v3__SocketOption__SocketType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SocketType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SocketType {
  type Msg = SocketType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketType {
  type Msg = SocketType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SocketTypeMut<'_> {
  type Msg = SocketType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketTypeMut<'_> {
  type Msg = SocketType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketTypeView<'_> {
  type Msg = SocketType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SocketTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod socket_type {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__SocketOption__SocketType__Stream_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Stream {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Stream>
}

impl ::protobuf::Message for Stream {
  type MessageView<'msg> = StreamView<'msg>;
  type MessageMut<'msg> = StreamMut<'msg>;
}

impl ::std::default::Default for Stream {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Stream {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Stream` is `Sync` because it does not implement interior mutability.
//    Neither does `StreamMut`.
unsafe impl ::std::marker::Sync for Stream {}

// SAFETY:
// - `Stream` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Stream {}

impl ::protobuf::Proxied for Stream {
  type View<'msg> = StreamView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Stream {}

impl ::protobuf::MutProxied for Stream {
  type Mut<'msg> = StreamMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StreamView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Stream>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StreamView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StreamView<'msg> {
  type Message = Stream;
}

impl ::std::fmt::Debug for StreamView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StreamView<'_> {
  fn default() -> StreamView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Stream>> for StreamView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Stream>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StreamView<'msg> {

  pub fn to_owned(&self) -> Stream {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `StreamView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StreamView<'_> {}

// SAFETY:
// - `StreamView` is `Send` because while its alive a `StreamMut` cannot.
// - `StreamView` does not use thread-local data.
unsafe impl ::std::marker::Send for StreamView<'_> {}

impl<'msg> ::protobuf::AsView for StreamView<'msg> {
  type Proxied = Stream;
  fn as_view(&self) -> ::protobuf::View<'msg, Stream> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StreamView<'msg> {
  fn into_view<'shorter>(self) -> StreamView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Stream> for StreamView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Stream {
    let mut dst = Stream::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Stream> for StreamMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Stream {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Stream {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StreamView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StreamMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StreamMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Stream>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StreamMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StreamMut<'msg> {
  type Message = Stream;
}

impl ::std::fmt::Debug for StreamMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Stream>> for StreamMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Stream>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StreamMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Stream> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Stream {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `StreamMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StreamMut<'_> {}

// SAFETY:
// - `StreamMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StreamMut<'_> {}

impl<'msg> ::protobuf::AsView for StreamMut<'msg> {
  type Proxied = Stream;
  fn as_view(&self) -> ::protobuf::View<'_, Stream> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StreamMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Stream>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StreamMut<'msg> {
  type MutProxied = Stream;
  fn as_mut(&mut self) -> StreamMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StreamMut<'msg> {
  fn into_mut<'shorter>(self) -> StreamMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Stream {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Stream> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StreamView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StreamMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Stream

impl ::std::ops::Drop for Stream {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Stream {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Stream {
  type Proxied = Self;
  fn as_view(&self) -> StreamView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Stream {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StreamMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Stream {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::socket_option::socket_type::envoy__config__core__v3__SocketOption__SocketType__Stream_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::socket_option::socket_type::envoy__config__core__v3__SocketOption__SocketType__Stream_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::socket_option::socket_type::envoy__config__core__v3__SocketOption__SocketType__Stream_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Stream {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Stream {
  type Msg = Stream;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Stream> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Stream {
  type Msg = Stream;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Stream> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StreamMut<'_> {
  type Msg = Stream;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Stream> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamMut<'_> {
  type Msg = Stream;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Stream> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamView<'_> {
  type Msg = Stream;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Stream> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StreamMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__SocketOption__SocketType__Datagram_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Datagram {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Datagram>
}

impl ::protobuf::Message for Datagram {
  type MessageView<'msg> = DatagramView<'msg>;
  type MessageMut<'msg> = DatagramMut<'msg>;
}

impl ::std::default::Default for Datagram {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Datagram {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Datagram` is `Sync` because it does not implement interior mutability.
//    Neither does `DatagramMut`.
unsafe impl ::std::marker::Sync for Datagram {}

// SAFETY:
// - `Datagram` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Datagram {}

impl ::protobuf::Proxied for Datagram {
  type View<'msg> = DatagramView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Datagram {}

impl ::protobuf::MutProxied for Datagram {
  type Mut<'msg> = DatagramMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DatagramView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Datagram>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DatagramView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DatagramView<'msg> {
  type Message = Datagram;
}

impl ::std::fmt::Debug for DatagramView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DatagramView<'_> {
  fn default() -> DatagramView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Datagram>> for DatagramView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Datagram>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DatagramView<'msg> {

  pub fn to_owned(&self) -> Datagram {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `DatagramView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DatagramView<'_> {}

// SAFETY:
// - `DatagramView` is `Send` because while its alive a `DatagramMut` cannot.
// - `DatagramView` does not use thread-local data.
unsafe impl ::std::marker::Send for DatagramView<'_> {}

impl<'msg> ::protobuf::AsView for DatagramView<'msg> {
  type Proxied = Datagram;
  fn as_view(&self) -> ::protobuf::View<'msg, Datagram> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DatagramView<'msg> {
  fn into_view<'shorter>(self) -> DatagramView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Datagram> for DatagramView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Datagram {
    let mut dst = Datagram::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Datagram> for DatagramMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Datagram {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Datagram {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DatagramView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DatagramMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DatagramMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Datagram>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DatagramMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DatagramMut<'msg> {
  type Message = Datagram;
}

impl ::std::fmt::Debug for DatagramMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Datagram>> for DatagramMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Datagram>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DatagramMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Datagram> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Datagram {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `DatagramMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DatagramMut<'_> {}

// SAFETY:
// - `DatagramMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DatagramMut<'_> {}

impl<'msg> ::protobuf::AsView for DatagramMut<'msg> {
  type Proxied = Datagram;
  fn as_view(&self) -> ::protobuf::View<'_, Datagram> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DatagramMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Datagram>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DatagramMut<'msg> {
  type MutProxied = Datagram;
  fn as_mut(&mut self) -> DatagramMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DatagramMut<'msg> {
  fn into_mut<'shorter>(self) -> DatagramMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Datagram {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Datagram> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DatagramView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DatagramMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Datagram

impl ::std::ops::Drop for Datagram {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Datagram {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Datagram {
  type Proxied = Self;
  fn as_view(&self) -> DatagramView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Datagram {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DatagramMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Datagram {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::socket_option::socket_type::envoy__config__core__v3__SocketOption__SocketType__Datagram_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::socket_option::socket_type::envoy__config__core__v3__SocketOption__SocketType__Datagram_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::socket_option::socket_type::envoy__config__core__v3__SocketOption__SocketType__Datagram_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Datagram {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Datagram {
  type Msg = Datagram;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Datagram> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Datagram {
  type Msg = Datagram;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Datagram> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DatagramMut<'_> {
  type Msg = Datagram;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Datagram> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DatagramMut<'_> {
  type Msg = Datagram;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Datagram> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DatagramView<'_> {
  type Msg = Datagram;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Datagram> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DatagramMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod socket_type

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SocketState(i32);

#[allow(non_upper_case_globals)]
impl SocketState {
  pub const StatePrebind: SocketState = SocketState(0);
  pub const StateBound: SocketState = SocketState(1);
  pub const StateListening: SocketState = SocketState(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "StatePrebind",
      1 => "StateBound",
      2 => "StateListening",
      _ => return None
    })
  }
}

impl ::std::convert::From<SocketState> for i32 {
  fn from(val: SocketState) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for SocketState {
  fn from(val: i32) -> SocketState {
    Self(val)
  }
}

impl ::std::default::Default for SocketState {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for SocketState {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "SocketState::{}", constant_name)
    } else {
      write!(f, "SocketState::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for SocketState {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for SocketState {}

impl ::protobuf::Proxied for SocketState {
  type View<'a> = SocketState;
}

impl ::protobuf::AsView for SocketState {
  type Proxied = SocketState;

  fn as_view(&self) -> SocketState {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SocketState {
  fn into_view<'shorter>(self) -> SocketState where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for SocketState {
  const NAME: &'static str = "SocketState";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for SocketState {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ValueOneof<'msg> {
  IntValue(i64) = 4,
  BufValue(&'msg [u8]) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ValueCase {
  IntValue = 4,
  BufValue = 5,

  not_set = 0
}

impl ValueCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ValueCase> {
    match v {
      0 => Some(ValueCase::not_set),
      4 => Some(ValueCase::IntValue),
      5 => Some(ValueCase::BufValue),
      _ => None
    }
  }
}
}  // pub mod socket_option


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__SocketOptionsOverride_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SocketOptionsOverride {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SocketOptionsOverride>
}

impl ::protobuf::Message for SocketOptionsOverride {
  type MessageView<'msg> = SocketOptionsOverrideView<'msg>;
  type MessageMut<'msg> = SocketOptionsOverrideMut<'msg>;
}

impl ::std::default::Default for SocketOptionsOverride {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SocketOptionsOverride {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SocketOptionsOverride` is `Sync` because it does not implement interior mutability.
//    Neither does `SocketOptionsOverrideMut`.
unsafe impl ::std::marker::Sync for SocketOptionsOverride {}

// SAFETY:
// - `SocketOptionsOverride` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SocketOptionsOverride {}

impl ::protobuf::Proxied for SocketOptionsOverride {
  type View<'msg> = SocketOptionsOverrideView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SocketOptionsOverride {}

impl ::protobuf::MutProxied for SocketOptionsOverride {
  type Mut<'msg> = SocketOptionsOverrideMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SocketOptionsOverrideView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SocketOptionsOverride>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SocketOptionsOverrideView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SocketOptionsOverrideView<'msg> {
  type Message = SocketOptionsOverride;
}

impl ::std::fmt::Debug for SocketOptionsOverrideView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SocketOptionsOverrideView<'_> {
  fn default() -> SocketOptionsOverrideView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SocketOptionsOverride>> for SocketOptionsOverrideView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SocketOptionsOverride>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SocketOptionsOverrideView<'msg> {

  pub fn to_owned(&self) -> SocketOptionsOverride {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(self) -> ::protobuf::RepeatedView<'msg, super::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `SocketOptionsOverrideView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SocketOptionsOverrideView<'_> {}

// SAFETY:
// - `SocketOptionsOverrideView` is `Send` because while its alive a `SocketOptionsOverrideMut` cannot.
// - `SocketOptionsOverrideView` does not use thread-local data.
unsafe impl ::std::marker::Send for SocketOptionsOverrideView<'_> {}

impl<'msg> ::protobuf::AsView for SocketOptionsOverrideView<'msg> {
  type Proxied = SocketOptionsOverride;
  fn as_view(&self) -> ::protobuf::View<'msg, SocketOptionsOverride> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SocketOptionsOverrideView<'msg> {
  fn into_view<'shorter>(self) -> SocketOptionsOverrideView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SocketOptionsOverride> for SocketOptionsOverrideView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SocketOptionsOverride {
    let mut dst = SocketOptionsOverride::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SocketOptionsOverride> for SocketOptionsOverrideMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SocketOptionsOverride {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SocketOptionsOverride {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SocketOptionsOverrideView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SocketOptionsOverrideMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SocketOptionsOverrideMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketOptionsOverride>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SocketOptionsOverrideMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SocketOptionsOverrideMut<'msg> {
  type Message = SocketOptionsOverride;
}

impl ::std::fmt::Debug for SocketOptionsOverrideMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SocketOptionsOverride>> for SocketOptionsOverrideMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketOptionsOverride>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SocketOptionsOverrideMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketOptionsOverride> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SocketOptionsOverride {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(&self) -> ::protobuf::RepeatedView<'_, super::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn socket_options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::SocketOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_socket_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::SocketOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `SocketOptionsOverrideMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SocketOptionsOverrideMut<'_> {}

// SAFETY:
// - `SocketOptionsOverrideMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SocketOptionsOverrideMut<'_> {}

impl<'msg> ::protobuf::AsView for SocketOptionsOverrideMut<'msg> {
  type Proxied = SocketOptionsOverride;
  fn as_view(&self) -> ::protobuf::View<'_, SocketOptionsOverride> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SocketOptionsOverrideMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SocketOptionsOverride>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SocketOptionsOverrideMut<'msg> {
  type MutProxied = SocketOptionsOverride;
  fn as_mut(&mut self) -> SocketOptionsOverrideMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SocketOptionsOverrideMut<'msg> {
  fn into_mut<'shorter>(self) -> SocketOptionsOverrideMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SocketOptionsOverride {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SocketOptionsOverride> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SocketOptionsOverrideView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SocketOptionsOverrideMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(&self) -> ::protobuf::RepeatedView<'_, super::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn socket_options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::SocketOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_socket_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::SocketOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl SocketOptionsOverride

impl ::std::ops::Drop for SocketOptionsOverride {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SocketOptionsOverride {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SocketOptionsOverride {
  type Proxied = Self;
  fn as_view(&self) -> SocketOptionsOverrideView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SocketOptionsOverride {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SocketOptionsOverrideMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SocketOptionsOverride {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__SocketOptionsOverride_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__SocketOptionsOverride_msg_init.0, &[<super::SocketOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__SocketOptionsOverride_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SocketOptionsOverride {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SocketOptionsOverride {
  type Msg = SocketOptionsOverride;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketOptionsOverride> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketOptionsOverride {
  type Msg = SocketOptionsOverride;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketOptionsOverride> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SocketOptionsOverrideMut<'_> {
  type Msg = SocketOptionsOverride;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketOptionsOverride> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketOptionsOverrideMut<'_> {
  type Msg = SocketOptionsOverride;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketOptionsOverride> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketOptionsOverrideView<'_> {
  type Msg = SocketOptionsOverride;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketOptionsOverride> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SocketOptionsOverrideMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



