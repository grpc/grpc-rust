const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__AdsDummy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AdsDummy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AdsDummy>
}

impl ::protobuf::Message for AdsDummy {
  type MessageView<'msg> = AdsDummyView<'msg>;
  type MessageMut<'msg> = AdsDummyMut<'msg>;
}

impl ::std::default::Default for AdsDummy {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AdsDummy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AdsDummy` is `Sync` because it does not implement interior mutability.
//    Neither does `AdsDummyMut`.
unsafe impl ::std::marker::Sync for AdsDummy {}

// SAFETY:
// - `AdsDummy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AdsDummy {}

impl ::protobuf::Proxied for AdsDummy {
  type View<'msg> = AdsDummyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AdsDummy {}

impl ::protobuf::MutProxied for AdsDummy {
  type Mut<'msg> = AdsDummyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AdsDummyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AdsDummy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdsDummyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AdsDummyView<'msg> {
  type Message = AdsDummy;
}

impl ::std::fmt::Debug for AdsDummyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AdsDummyView<'_> {
  fn default() -> AdsDummyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AdsDummy>> for AdsDummyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AdsDummy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdsDummyView<'msg> {

  pub fn to_owned(&self) -> AdsDummy {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `AdsDummyView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AdsDummyView<'_> {}

// SAFETY:
// - `AdsDummyView` is `Send` because while its alive a `AdsDummyMut` cannot.
// - `AdsDummyView` does not use thread-local data.
unsafe impl ::std::marker::Send for AdsDummyView<'_> {}

impl<'msg> ::protobuf::AsView for AdsDummyView<'msg> {
  type Proxied = AdsDummy;
  fn as_view(&self) -> ::protobuf::View<'msg, AdsDummy> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdsDummyView<'msg> {
  fn into_view<'shorter>(self) -> AdsDummyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AdsDummy> for AdsDummyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AdsDummy {
    let mut dst = AdsDummy::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AdsDummy> for AdsDummyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AdsDummy {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AdsDummy {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AdsDummyView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AdsDummyMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AdsDummyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AdsDummy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdsDummyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AdsDummyMut<'msg> {
  type Message = AdsDummy;
}

impl ::std::fmt::Debug for AdsDummyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AdsDummy>> for AdsDummyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AdsDummy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdsDummyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AdsDummy> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AdsDummy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `AdsDummyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AdsDummyMut<'_> {}

// SAFETY:
// - `AdsDummyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AdsDummyMut<'_> {}

impl<'msg> ::protobuf::AsView for AdsDummyMut<'msg> {
  type Proxied = AdsDummy;
  fn as_view(&self) -> ::protobuf::View<'_, AdsDummy> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdsDummyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AdsDummy>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AdsDummyMut<'msg> {
  type MutProxied = AdsDummy;
  fn as_mut(&mut self) -> AdsDummyMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AdsDummyMut<'msg> {
  fn into_mut<'shorter>(self) -> AdsDummyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AdsDummy {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AdsDummy> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AdsDummyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AdsDummyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl AdsDummy

impl ::std::ops::Drop for AdsDummy {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AdsDummy {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AdsDummy {
  type Proxied = Self;
  fn as_view(&self) -> AdsDummyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AdsDummy {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AdsDummyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AdsDummy {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__discovery__v3__AdsDummy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__discovery__v3__AdsDummy_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__discovery__v3__AdsDummy_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AdsDummy {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AdsDummy {
  type Msg = AdsDummy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdsDummy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdsDummy {
  type Msg = AdsDummy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdsDummy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AdsDummyMut<'_> {
  type Msg = AdsDummy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdsDummy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdsDummyMut<'_> {
  type Msg = AdsDummy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdsDummy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdsDummyView<'_> {
  type Msg = AdsDummy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdsDummy> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AdsDummyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



