const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__grpc_0service__channel_0credentials__google_0default__v3__GoogleDefaultCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GoogleDefaultCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GoogleDefaultCredentials>
}

impl ::protobuf::Message for GoogleDefaultCredentials {
  type MessageView<'msg> = GoogleDefaultCredentialsView<'msg>;
  type MessageMut<'msg> = GoogleDefaultCredentialsMut<'msg>;
}

impl ::std::default::Default for GoogleDefaultCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GoogleDefaultCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GoogleDefaultCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `GoogleDefaultCredentialsMut`.
unsafe impl ::std::marker::Sync for GoogleDefaultCredentials {}

// SAFETY:
// - `GoogleDefaultCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GoogleDefaultCredentials {}

impl ::protobuf::Proxied for GoogleDefaultCredentials {
  type View<'msg> = GoogleDefaultCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GoogleDefaultCredentials {}

impl ::protobuf::MutProxied for GoogleDefaultCredentials {
  type Mut<'msg> = GoogleDefaultCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GoogleDefaultCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleDefaultCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleDefaultCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GoogleDefaultCredentialsView<'msg> {
  type Message = GoogleDefaultCredentials;
}

impl ::std::fmt::Debug for GoogleDefaultCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GoogleDefaultCredentialsView<'_> {
  fn default() -> GoogleDefaultCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleDefaultCredentials>> for GoogleDefaultCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleDefaultCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleDefaultCredentialsView<'msg> {

  pub fn to_owned(&self) -> GoogleDefaultCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GoogleDefaultCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GoogleDefaultCredentialsView<'_> {}

// SAFETY:
// - `GoogleDefaultCredentialsView` is `Send` because while its alive a `GoogleDefaultCredentialsMut` cannot.
// - `GoogleDefaultCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for GoogleDefaultCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for GoogleDefaultCredentialsView<'msg> {
  type Proxied = GoogleDefaultCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, GoogleDefaultCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleDefaultCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> GoogleDefaultCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleDefaultCredentials> for GoogleDefaultCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleDefaultCredentials {
    let mut dst = GoogleDefaultCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleDefaultCredentials> for GoogleDefaultCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleDefaultCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GoogleDefaultCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GoogleDefaultCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GoogleDefaultCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GoogleDefaultCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleDefaultCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleDefaultCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GoogleDefaultCredentialsMut<'msg> {
  type Message = GoogleDefaultCredentials;
}

impl ::std::fmt::Debug for GoogleDefaultCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleDefaultCredentials>> for GoogleDefaultCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleDefaultCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleDefaultCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleDefaultCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GoogleDefaultCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `GoogleDefaultCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GoogleDefaultCredentialsMut<'_> {}

// SAFETY:
// - `GoogleDefaultCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GoogleDefaultCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for GoogleDefaultCredentialsMut<'msg> {
  type Proxied = GoogleDefaultCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, GoogleDefaultCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleDefaultCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GoogleDefaultCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GoogleDefaultCredentialsMut<'msg> {
  type MutProxied = GoogleDefaultCredentials;
  fn as_mut(&mut self) -> GoogleDefaultCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GoogleDefaultCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> GoogleDefaultCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GoogleDefaultCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GoogleDefaultCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GoogleDefaultCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GoogleDefaultCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl GoogleDefaultCredentials

impl ::std::ops::Drop for GoogleDefaultCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GoogleDefaultCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GoogleDefaultCredentials {
  type Proxied = Self;
  fn as_view(&self) -> GoogleDefaultCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GoogleDefaultCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GoogleDefaultCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GoogleDefaultCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__grpc_0service__channel_0credentials__google_0default__v3__GoogleDefaultCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__grpc_0service__channel_0credentials__google_0default__v3__GoogleDefaultCredentials_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__grpc_0service__channel_0credentials__google_0default__v3__GoogleDefaultCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleDefaultCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleDefaultCredentials {
  type Msg = GoogleDefaultCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleDefaultCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleDefaultCredentials {
  type Msg = GoogleDefaultCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleDefaultCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleDefaultCredentialsMut<'_> {
  type Msg = GoogleDefaultCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleDefaultCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleDefaultCredentialsMut<'_> {
  type Msg = GoogleDefaultCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleDefaultCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleDefaultCredentialsView<'_> {
  type Msg = GoogleDefaultCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleDefaultCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleDefaultCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



