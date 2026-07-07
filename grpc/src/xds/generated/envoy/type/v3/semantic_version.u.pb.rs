const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__v3__SemanticVersion_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SemanticVersion {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SemanticVersion>
}

impl ::protobuf::Message for SemanticVersion {
  type MessageView<'msg> = SemanticVersionView<'msg>;
  type MessageMut<'msg> = SemanticVersionMut<'msg>;
}

impl ::std::default::Default for SemanticVersion {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SemanticVersion {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SemanticVersion` is `Sync` because it does not implement interior mutability.
//    Neither does `SemanticVersionMut`.
unsafe impl ::std::marker::Sync for SemanticVersion {}

// SAFETY:
// - `SemanticVersion` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SemanticVersion {}

impl ::protobuf::Proxied for SemanticVersion {
  type View<'msg> = SemanticVersionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SemanticVersion {}

impl ::protobuf::MutProxied for SemanticVersion {
  type Mut<'msg> = SemanticVersionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SemanticVersionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SemanticVersion>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SemanticVersionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SemanticVersionView<'msg> {
  type Message = SemanticVersion;
}

impl ::std::fmt::Debug for SemanticVersionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SemanticVersionView<'_> {
  fn default() -> SemanticVersionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SemanticVersion>> for SemanticVersionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SemanticVersion>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SemanticVersionView<'msg> {

  pub fn to_owned(&self) -> SemanticVersion {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // major_number: optional uint32
  pub fn major_number(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // minor_number: optional uint32
  pub fn minor_number(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // patch: optional uint32
  pub fn patch(self) -> u32 {
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
// - `SemanticVersionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SemanticVersionView<'_> {}

// SAFETY:
// - `SemanticVersionView` is `Send` because while its alive a `SemanticVersionMut` cannot.
// - `SemanticVersionView` does not use thread-local data.
unsafe impl ::std::marker::Send for SemanticVersionView<'_> {}

impl<'msg> ::protobuf::AsView for SemanticVersionView<'msg> {
  type Proxied = SemanticVersion;
  fn as_view(&self) -> ::protobuf::View<'msg, SemanticVersion> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SemanticVersionView<'msg> {
  fn into_view<'shorter>(self) -> SemanticVersionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SemanticVersion> for SemanticVersionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SemanticVersion {
    let mut dst = SemanticVersion::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SemanticVersion> for SemanticVersionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SemanticVersion {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SemanticVersion {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SemanticVersionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SemanticVersionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SemanticVersionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SemanticVersion>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SemanticVersionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SemanticVersionMut<'msg> {
  type Message = SemanticVersion;
}

impl ::std::fmt::Debug for SemanticVersionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SemanticVersion>> for SemanticVersionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SemanticVersion>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SemanticVersionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SemanticVersion> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SemanticVersion {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // major_number: optional uint32
  pub fn major_number(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_major_number(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // minor_number: optional uint32
  pub fn minor_number(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_minor_number(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

  // patch: optional uint32
  pub fn patch(&self) -> u32 {
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
  pub fn set_patch(&mut self, val: u32) {
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
// - `SemanticVersionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SemanticVersionMut<'_> {}

// SAFETY:
// - `SemanticVersionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SemanticVersionMut<'_> {}

impl<'msg> ::protobuf::AsView for SemanticVersionMut<'msg> {
  type Proxied = SemanticVersion;
  fn as_view(&self) -> ::protobuf::View<'_, SemanticVersion> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SemanticVersionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SemanticVersion>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SemanticVersionMut<'msg> {
  type MutProxied = SemanticVersion;
  fn as_mut(&mut self) -> SemanticVersionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SemanticVersionMut<'msg> {
  fn into_mut<'shorter>(self) -> SemanticVersionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SemanticVersion {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SemanticVersion> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SemanticVersionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SemanticVersionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // major_number: optional uint32
  pub fn major_number(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_major_number(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // minor_number: optional uint32
  pub fn minor_number(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_minor_number(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

  // patch: optional uint32
  pub fn patch(&self) -> u32 {
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
  pub fn set_patch(&mut self, val: u32) {
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

}  // impl SemanticVersion

impl ::std::ops::Drop for SemanticVersion {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SemanticVersion {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SemanticVersion {
  type Proxied = Self;
  fn as_view(&self) -> SemanticVersionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SemanticVersion {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SemanticVersionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SemanticVersion {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__v3__SemanticVersion_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P)P)P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__v3__SemanticVersion_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__v3__SemanticVersion_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SemanticVersion {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SemanticVersion {
  type Msg = SemanticVersion;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SemanticVersion> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SemanticVersion {
  type Msg = SemanticVersion;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SemanticVersion> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SemanticVersionMut<'_> {
  type Msg = SemanticVersion;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SemanticVersion> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SemanticVersionMut<'_> {
  type Msg = SemanticVersion;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SemanticVersion> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SemanticVersionView<'_> {
  type Msg = SemanticVersion;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SemanticVersion> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SemanticVersionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



