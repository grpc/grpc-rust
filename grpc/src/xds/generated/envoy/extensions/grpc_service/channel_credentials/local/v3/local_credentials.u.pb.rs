const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__grpc_0service__channel_0credentials__local__v3__LocalCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LocalCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LocalCredentials>
}

impl ::protobuf::Message for LocalCredentials {
  type MessageView<'msg> = LocalCredentialsView<'msg>;
  type MessageMut<'msg> = LocalCredentialsMut<'msg>;
}

impl ::std::default::Default for LocalCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LocalCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LocalCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `LocalCredentialsMut`.
unsafe impl ::std::marker::Sync for LocalCredentials {}

// SAFETY:
// - `LocalCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LocalCredentials {}

impl ::protobuf::Proxied for LocalCredentials {
  type View<'msg> = LocalCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LocalCredentials {}

impl ::protobuf::MutProxied for LocalCredentials {
  type Mut<'msg> = LocalCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LocalCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LocalCredentialsView<'msg> {
  type Message = LocalCredentials;
}

impl ::std::fmt::Debug for LocalCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LocalCredentialsView<'_> {
  fn default() -> LocalCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LocalCredentials>> for LocalCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalCredentialsView<'msg> {

  pub fn to_owned(&self) -> LocalCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `LocalCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LocalCredentialsView<'_> {}

// SAFETY:
// - `LocalCredentialsView` is `Send` because while its alive a `LocalCredentialsMut` cannot.
// - `LocalCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for LocalCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for LocalCredentialsView<'msg> {
  type Proxied = LocalCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, LocalCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> LocalCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalCredentials> for LocalCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalCredentials {
    let mut dst = LocalCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalCredentials> for LocalCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LocalCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LocalCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LocalCredentialsMut<'msg> {
  type Message = LocalCredentials;
}

impl ::std::fmt::Debug for LocalCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LocalCredentials>> for LocalCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LocalCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `LocalCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LocalCredentialsMut<'_> {}

// SAFETY:
// - `LocalCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LocalCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for LocalCredentialsMut<'msg> {
  type Proxied = LocalCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, LocalCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LocalCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LocalCredentialsMut<'msg> {
  type MutProxied = LocalCredentials;
  fn as_mut(&mut self) -> LocalCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LocalCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> LocalCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LocalCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LocalCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LocalCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LocalCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl LocalCredentials

impl ::std::ops::Drop for LocalCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LocalCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LocalCredentials {
  type Proxied = Self;
  fn as_view(&self) -> LocalCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LocalCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LocalCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LocalCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__grpc_0service__channel_0credentials__local__v3__LocalCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__grpc_0service__channel_0credentials__local__v3__LocalCredentials_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__grpc_0service__channel_0credentials__local__v3__LocalCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalCredentials {
  type Msg = LocalCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalCredentials {
  type Msg = LocalCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalCredentialsMut<'_> {
  type Msg = LocalCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalCredentialsMut<'_> {
  type Msg = LocalCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalCredentialsView<'_> {
  type Msg = LocalCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



