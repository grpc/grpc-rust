const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__SocketCmsgHeaders_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SocketCmsgHeaders {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SocketCmsgHeaders>
}

impl ::protobuf::Message for SocketCmsgHeaders {
  type MessageView<'msg> = SocketCmsgHeadersView<'msg>;
  type MessageMut<'msg> = SocketCmsgHeadersMut<'msg>;
}

impl ::std::default::Default for SocketCmsgHeaders {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SocketCmsgHeaders {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SocketCmsgHeaders` is `Sync` because it does not implement interior mutability.
//    Neither does `SocketCmsgHeadersMut`.
unsafe impl ::std::marker::Sync for SocketCmsgHeaders {}

// SAFETY:
// - `SocketCmsgHeaders` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SocketCmsgHeaders {}

impl ::protobuf::Proxied for SocketCmsgHeaders {
  type View<'msg> = SocketCmsgHeadersView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SocketCmsgHeaders {}

impl ::protobuf::MutProxied for SocketCmsgHeaders {
  type Mut<'msg> = SocketCmsgHeadersMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SocketCmsgHeadersView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SocketCmsgHeaders>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SocketCmsgHeadersView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SocketCmsgHeadersView<'msg> {
  type Message = SocketCmsgHeaders;
}

impl ::std::fmt::Debug for SocketCmsgHeadersView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SocketCmsgHeadersView<'_> {
  fn default() -> SocketCmsgHeadersView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SocketCmsgHeaders>> for SocketCmsgHeadersView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SocketCmsgHeaders>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SocketCmsgHeadersView<'msg> {

  pub fn to_owned(&self) -> SocketCmsgHeaders {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // level: optional message google.protobuf.UInt32Value
  pub fn has_level(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn level_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_level().then(|| self.level())
  }
  pub fn level(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // type: optional message google.protobuf.UInt32Value
  pub fn has_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn type_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // expected_size: optional uint32
  pub fn expected_size(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SocketCmsgHeadersView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SocketCmsgHeadersView<'_> {}

// SAFETY:
// - `SocketCmsgHeadersView` is `Send` because while its alive a `SocketCmsgHeadersMut` cannot.
// - `SocketCmsgHeadersView` does not use thread-local data.
unsafe impl ::std::marker::Send for SocketCmsgHeadersView<'_> {}

impl<'msg> ::protobuf::AsView for SocketCmsgHeadersView<'msg> {
  type Proxied = SocketCmsgHeaders;
  fn as_view(&self) -> ::protobuf::View<'msg, SocketCmsgHeaders> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SocketCmsgHeadersView<'msg> {
  fn into_view<'shorter>(self) -> SocketCmsgHeadersView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SocketCmsgHeaders> for SocketCmsgHeadersView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SocketCmsgHeaders {
    let mut dst = SocketCmsgHeaders::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SocketCmsgHeaders> for SocketCmsgHeadersMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SocketCmsgHeaders {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SocketCmsgHeaders {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SocketCmsgHeadersView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SocketCmsgHeadersMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SocketCmsgHeadersMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketCmsgHeaders>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SocketCmsgHeadersMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SocketCmsgHeadersMut<'msg> {
  type Message = SocketCmsgHeaders;
}

impl ::std::fmt::Debug for SocketCmsgHeadersMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SocketCmsgHeaders>> for SocketCmsgHeadersMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketCmsgHeaders>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SocketCmsgHeadersMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketCmsgHeaders> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SocketCmsgHeaders {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // level: optional message google.protobuf.UInt32Value
  pub fn has_level(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_level(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn level_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_level().then(|| self.level())
  }
  pub fn level(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn level_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_level(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // type: optional message google.protobuf.UInt32Value
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn type_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn type_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // expected_size: optional uint32
  pub fn expected_size(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_expected_size(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `SocketCmsgHeadersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SocketCmsgHeadersMut<'_> {}

// SAFETY:
// - `SocketCmsgHeadersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SocketCmsgHeadersMut<'_> {}

impl<'msg> ::protobuf::AsView for SocketCmsgHeadersMut<'msg> {
  type Proxied = SocketCmsgHeaders;
  fn as_view(&self) -> ::protobuf::View<'_, SocketCmsgHeaders> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SocketCmsgHeadersMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SocketCmsgHeaders>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SocketCmsgHeadersMut<'msg> {
  type MutProxied = SocketCmsgHeaders;
  fn as_mut(&mut self) -> SocketCmsgHeadersMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SocketCmsgHeadersMut<'msg> {
  fn into_mut<'shorter>(self) -> SocketCmsgHeadersMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SocketCmsgHeaders {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SocketCmsgHeaders> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SocketCmsgHeadersView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SocketCmsgHeadersMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // level: optional message google.protobuf.UInt32Value
  pub fn has_level(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_level(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn level_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_level().then(|| self.level())
  }
  pub fn level(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn level_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_level(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // type: optional message google.protobuf.UInt32Value
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn type_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn type_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_type(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // expected_size: optional uint32
  pub fn expected_size(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_expected_size(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

}  // impl SocketCmsgHeaders

impl ::std::ops::Drop for SocketCmsgHeaders {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SocketCmsgHeaders {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SocketCmsgHeaders {
  type Proxied = Self;
  fn as_view(&self) -> SocketCmsgHeadersView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SocketCmsgHeaders {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SocketCmsgHeadersMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SocketCmsgHeaders {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__SocketCmsgHeaders_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33)P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__SocketCmsgHeaders_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__SocketCmsgHeaders_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SocketCmsgHeaders {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SocketCmsgHeaders {
  type Msg = SocketCmsgHeaders;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketCmsgHeaders> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketCmsgHeaders {
  type Msg = SocketCmsgHeaders;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketCmsgHeaders> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SocketCmsgHeadersMut<'_> {
  type Msg = SocketCmsgHeaders;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketCmsgHeaders> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketCmsgHeadersMut<'_> {
  type Msg = SocketCmsgHeaders;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketCmsgHeaders> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketCmsgHeadersView<'_> {
  type Msg = SocketCmsgHeaders;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketCmsgHeaders> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SocketCmsgHeadersMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



