const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut udpa__annotations__FieldSecurityAnnotation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FieldSecurityAnnotation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FieldSecurityAnnotation>
}

impl ::protobuf::Message for FieldSecurityAnnotation {
  type MessageView<'msg> = FieldSecurityAnnotationView<'msg>;
  type MessageMut<'msg> = FieldSecurityAnnotationMut<'msg>;
}

impl ::std::default::Default for FieldSecurityAnnotation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FieldSecurityAnnotation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FieldSecurityAnnotation` is `Sync` because it does not implement interior mutability.
//    Neither does `FieldSecurityAnnotationMut`.
unsafe impl ::std::marker::Sync for FieldSecurityAnnotation {}

// SAFETY:
// - `FieldSecurityAnnotation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FieldSecurityAnnotation {}

impl ::protobuf::Proxied for FieldSecurityAnnotation {
  type View<'msg> = FieldSecurityAnnotationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FieldSecurityAnnotation {}

impl ::protobuf::MutProxied for FieldSecurityAnnotation {
  type Mut<'msg> = FieldSecurityAnnotationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FieldSecurityAnnotationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldSecurityAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldSecurityAnnotationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FieldSecurityAnnotationView<'msg> {
  type Message = FieldSecurityAnnotation;
}

impl ::std::fmt::Debug for FieldSecurityAnnotationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FieldSecurityAnnotationView<'_> {
  fn default() -> FieldSecurityAnnotationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FieldSecurityAnnotation>> for FieldSecurityAnnotationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldSecurityAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldSecurityAnnotationView<'msg> {

  pub fn to_owned(&self) -> FieldSecurityAnnotation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // configure_for_untrusted_downstream: optional bool
  pub fn configure_for_untrusted_downstream(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }

  // configure_for_untrusted_upstream: optional bool
  pub fn configure_for_untrusted_upstream(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `FieldSecurityAnnotationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FieldSecurityAnnotationView<'_> {}

// SAFETY:
// - `FieldSecurityAnnotationView` is `Send` because while its alive a `FieldSecurityAnnotationMut` cannot.
// - `FieldSecurityAnnotationView` does not use thread-local data.
unsafe impl ::std::marker::Send for FieldSecurityAnnotationView<'_> {}

impl<'msg> ::protobuf::AsView for FieldSecurityAnnotationView<'msg> {
  type Proxied = FieldSecurityAnnotation;
  fn as_view(&self) -> ::protobuf::View<'msg, FieldSecurityAnnotation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldSecurityAnnotationView<'msg> {
  fn into_view<'shorter>(self) -> FieldSecurityAnnotationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldSecurityAnnotation> for FieldSecurityAnnotationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldSecurityAnnotation {
    let mut dst = FieldSecurityAnnotation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldSecurityAnnotation> for FieldSecurityAnnotationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldSecurityAnnotation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FieldSecurityAnnotation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FieldSecurityAnnotationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FieldSecurityAnnotationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FieldSecurityAnnotationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldSecurityAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldSecurityAnnotationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FieldSecurityAnnotationMut<'msg> {
  type Message = FieldSecurityAnnotation;
}

impl ::std::fmt::Debug for FieldSecurityAnnotationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FieldSecurityAnnotation>> for FieldSecurityAnnotationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldSecurityAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldSecurityAnnotationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldSecurityAnnotation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FieldSecurityAnnotation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // configure_for_untrusted_downstream: optional bool
  pub fn configure_for_untrusted_downstream(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_configure_for_untrusted_downstream(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // configure_for_untrusted_upstream: optional bool
  pub fn configure_for_untrusted_upstream(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_configure_for_untrusted_upstream(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `FieldSecurityAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FieldSecurityAnnotationMut<'_> {}

// SAFETY:
// - `FieldSecurityAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FieldSecurityAnnotationMut<'_> {}

impl<'msg> ::protobuf::AsView for FieldSecurityAnnotationMut<'msg> {
  type Proxied = FieldSecurityAnnotation;
  fn as_view(&self) -> ::protobuf::View<'_, FieldSecurityAnnotation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldSecurityAnnotationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FieldSecurityAnnotation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FieldSecurityAnnotationMut<'msg> {
  type MutProxied = FieldSecurityAnnotation;
  fn as_mut(&mut self) -> FieldSecurityAnnotationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FieldSecurityAnnotationMut<'msg> {
  fn into_mut<'shorter>(self) -> FieldSecurityAnnotationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FieldSecurityAnnotation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FieldSecurityAnnotation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FieldSecurityAnnotationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FieldSecurityAnnotationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // configure_for_untrusted_downstream: optional bool
  pub fn configure_for_untrusted_downstream(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_configure_for_untrusted_downstream(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // configure_for_untrusted_upstream: optional bool
  pub fn configure_for_untrusted_upstream(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_configure_for_untrusted_upstream(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

}  // impl FieldSecurityAnnotation

impl ::std::ops::Drop for FieldSecurityAnnotation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FieldSecurityAnnotation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FieldSecurityAnnotation {
  type Proxied = Self;
  fn as_view(&self) -> FieldSecurityAnnotationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FieldSecurityAnnotation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FieldSecurityAnnotationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FieldSecurityAnnotation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::udpa__annotations__FieldSecurityAnnotation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::udpa__annotations__FieldSecurityAnnotation_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::udpa__annotations__FieldSecurityAnnotation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldSecurityAnnotation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldSecurityAnnotation {
  type Msg = FieldSecurityAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldSecurityAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldSecurityAnnotation {
  type Msg = FieldSecurityAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldSecurityAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldSecurityAnnotationMut<'_> {
  type Msg = FieldSecurityAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldSecurityAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldSecurityAnnotationMut<'_> {
  type Msg = FieldSecurityAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldSecurityAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldSecurityAnnotationView<'_> {
  type Msg = FieldSecurityAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldSecurityAnnotation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldSecurityAnnotationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}




