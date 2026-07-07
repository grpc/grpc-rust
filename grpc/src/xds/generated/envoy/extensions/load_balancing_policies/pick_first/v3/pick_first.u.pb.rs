const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__pick_0first__v3__PickFirst_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PickFirst {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PickFirst>
}

impl ::protobuf::Message for PickFirst {
  type MessageView<'msg> = PickFirstView<'msg>;
  type MessageMut<'msg> = PickFirstMut<'msg>;
}

impl ::std::default::Default for PickFirst {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PickFirst {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PickFirst` is `Sync` because it does not implement interior mutability.
//    Neither does `PickFirstMut`.
unsafe impl ::std::marker::Sync for PickFirst {}

// SAFETY:
// - `PickFirst` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PickFirst {}

impl ::protobuf::Proxied for PickFirst {
  type View<'msg> = PickFirstView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PickFirst {}

impl ::protobuf::MutProxied for PickFirst {
  type Mut<'msg> = PickFirstMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PickFirstView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PickFirst>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PickFirstView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PickFirstView<'msg> {
  type Message = PickFirst;
}

impl ::std::fmt::Debug for PickFirstView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PickFirstView<'_> {
  fn default() -> PickFirstView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PickFirst>> for PickFirstView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PickFirst>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PickFirstView<'msg> {

  pub fn to_owned(&self) -> PickFirst {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // shuffle_address_list: optional bool
  pub fn shuffle_address_list(self) -> bool {
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

}

// SAFETY:
// - `PickFirstView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PickFirstView<'_> {}

// SAFETY:
// - `PickFirstView` is `Send` because while its alive a `PickFirstMut` cannot.
// - `PickFirstView` does not use thread-local data.
unsafe impl ::std::marker::Send for PickFirstView<'_> {}

impl<'msg> ::protobuf::AsView for PickFirstView<'msg> {
  type Proxied = PickFirst;
  fn as_view(&self) -> ::protobuf::View<'msg, PickFirst> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PickFirstView<'msg> {
  fn into_view<'shorter>(self) -> PickFirstView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PickFirst> for PickFirstView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PickFirst {
    let mut dst = PickFirst::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PickFirst> for PickFirstMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PickFirst {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PickFirst {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PickFirstView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PickFirstMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PickFirstMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PickFirst>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PickFirstMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PickFirstMut<'msg> {
  type Message = PickFirst;
}

impl ::std::fmt::Debug for PickFirstMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PickFirst>> for PickFirstMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PickFirst>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PickFirstMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PickFirst> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PickFirst {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // shuffle_address_list: optional bool
  pub fn shuffle_address_list(&self) -> bool {
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
  pub fn set_shuffle_address_list(&mut self, val: bool) {
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

}

// SAFETY:
// - `PickFirstMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PickFirstMut<'_> {}

// SAFETY:
// - `PickFirstMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PickFirstMut<'_> {}

impl<'msg> ::protobuf::AsView for PickFirstMut<'msg> {
  type Proxied = PickFirst;
  fn as_view(&self) -> ::protobuf::View<'_, PickFirst> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PickFirstMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PickFirst>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PickFirstMut<'msg> {
  type MutProxied = PickFirst;
  fn as_mut(&mut self) -> PickFirstMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PickFirstMut<'msg> {
  fn into_mut<'shorter>(self) -> PickFirstMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PickFirst {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PickFirst> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PickFirstView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PickFirstMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // shuffle_address_list: optional bool
  pub fn shuffle_address_list(&self) -> bool {
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
  pub fn set_shuffle_address_list(&mut self, val: bool) {
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

}  // impl PickFirst

impl ::std::ops::Drop for PickFirst {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PickFirst {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PickFirst {
  type Proxied = Self;
  fn as_view(&self) -> PickFirstView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PickFirst {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PickFirstMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PickFirst {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__load_0balancing_0policies__pick_0first__v3__PickFirst_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__load_0balancing_0policies__pick_0first__v3__PickFirst_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__load_0balancing_0policies__pick_0first__v3__PickFirst_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PickFirst {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PickFirst {
  type Msg = PickFirst;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PickFirst> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PickFirst {
  type Msg = PickFirst;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PickFirst> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PickFirstMut<'_> {
  type Msg = PickFirst;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PickFirst> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PickFirstMut<'_> {
  type Msg = PickFirst;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PickFirst> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PickFirstView<'_> {
  type Msg = PickFirst;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PickFirst> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PickFirstMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



