const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__grpc_0service__channel_0credentials__insecure__v3__InsecureCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct InsecureCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<InsecureCredentials>
}

impl ::protobuf::Message for InsecureCredentials {
  type MessageView<'msg> = InsecureCredentialsView<'msg>;
  type MessageMut<'msg> = InsecureCredentialsMut<'msg>;
}

impl ::std::default::Default for InsecureCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for InsecureCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `InsecureCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `InsecureCredentialsMut`.
unsafe impl ::std::marker::Sync for InsecureCredentials {}

// SAFETY:
// - `InsecureCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for InsecureCredentials {}

impl ::protobuf::Proxied for InsecureCredentials {
  type View<'msg> = InsecureCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for InsecureCredentials {}

impl ::protobuf::MutProxied for InsecureCredentials {
  type Mut<'msg> = InsecureCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct InsecureCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, InsecureCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InsecureCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for InsecureCredentialsView<'msg> {
  type Message = InsecureCredentials;
}

impl ::std::fmt::Debug for InsecureCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for InsecureCredentialsView<'_> {
  fn default() -> InsecureCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, InsecureCredentials>> for InsecureCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, InsecureCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InsecureCredentialsView<'msg> {

  pub fn to_owned(&self) -> InsecureCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `InsecureCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for InsecureCredentialsView<'_> {}

// SAFETY:
// - `InsecureCredentialsView` is `Send` because while its alive a `InsecureCredentialsMut` cannot.
// - `InsecureCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for InsecureCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for InsecureCredentialsView<'msg> {
  type Proxied = InsecureCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, InsecureCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InsecureCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> InsecureCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<InsecureCredentials> for InsecureCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> InsecureCredentials {
    let mut dst = InsecureCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<InsecureCredentials> for InsecureCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> InsecureCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for InsecureCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for InsecureCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for InsecureCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct InsecureCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, InsecureCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InsecureCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for InsecureCredentialsMut<'msg> {
  type Message = InsecureCredentials;
}

impl ::std::fmt::Debug for InsecureCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, InsecureCredentials>> for InsecureCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, InsecureCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InsecureCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, InsecureCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> InsecureCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `InsecureCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for InsecureCredentialsMut<'_> {}

// SAFETY:
// - `InsecureCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for InsecureCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for InsecureCredentialsMut<'msg> {
  type Proxied = InsecureCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, InsecureCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InsecureCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, InsecureCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for InsecureCredentialsMut<'msg> {
  type MutProxied = InsecureCredentials;
  fn as_mut(&mut self) -> InsecureCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for InsecureCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> InsecureCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl InsecureCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, InsecureCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> InsecureCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> InsecureCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl InsecureCredentials

impl ::std::ops::Drop for InsecureCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for InsecureCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for InsecureCredentials {
  type Proxied = Self;
  fn as_view(&self) -> InsecureCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for InsecureCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> InsecureCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for InsecureCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__grpc_0service__channel_0credentials__insecure__v3__InsecureCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__grpc_0service__channel_0credentials__insecure__v3__InsecureCredentials_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__grpc_0service__channel_0credentials__insecure__v3__InsecureCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for InsecureCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for InsecureCredentials {
  type Msg = InsecureCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InsecureCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InsecureCredentials {
  type Msg = InsecureCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InsecureCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for InsecureCredentialsMut<'_> {
  type Msg = InsecureCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InsecureCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InsecureCredentialsMut<'_> {
  type Msg = InsecureCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InsecureCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InsecureCredentialsView<'_> {
  type Msg = InsecureCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InsecureCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for InsecureCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



