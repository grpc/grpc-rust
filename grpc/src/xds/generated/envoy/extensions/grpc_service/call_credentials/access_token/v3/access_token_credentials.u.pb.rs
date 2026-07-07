const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__grpc_0service__call_0credentials__access_0token__v3__AccessTokenCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AccessTokenCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AccessTokenCredentials>
}

impl ::protobuf::Message for AccessTokenCredentials {
  type MessageView<'msg> = AccessTokenCredentialsView<'msg>;
  type MessageMut<'msg> = AccessTokenCredentialsMut<'msg>;
}

impl ::std::default::Default for AccessTokenCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AccessTokenCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AccessTokenCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `AccessTokenCredentialsMut`.
unsafe impl ::std::marker::Sync for AccessTokenCredentials {}

// SAFETY:
// - `AccessTokenCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AccessTokenCredentials {}

impl ::protobuf::Proxied for AccessTokenCredentials {
  type View<'msg> = AccessTokenCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AccessTokenCredentials {}

impl ::protobuf::MutProxied for AccessTokenCredentials {
  type Mut<'msg> = AccessTokenCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AccessTokenCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AccessTokenCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AccessTokenCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AccessTokenCredentialsView<'msg> {
  type Message = AccessTokenCredentials;
}

impl ::std::fmt::Debug for AccessTokenCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AccessTokenCredentialsView<'_> {
  fn default() -> AccessTokenCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AccessTokenCredentials>> for AccessTokenCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AccessTokenCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AccessTokenCredentialsView<'msg> {

  pub fn to_owned(&self) -> AccessTokenCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // token: optional string
  pub fn token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `AccessTokenCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AccessTokenCredentialsView<'_> {}

// SAFETY:
// - `AccessTokenCredentialsView` is `Send` because while its alive a `AccessTokenCredentialsMut` cannot.
// - `AccessTokenCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for AccessTokenCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for AccessTokenCredentialsView<'msg> {
  type Proxied = AccessTokenCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, AccessTokenCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AccessTokenCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> AccessTokenCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AccessTokenCredentials> for AccessTokenCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AccessTokenCredentials {
    let mut dst = AccessTokenCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AccessTokenCredentials> for AccessTokenCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AccessTokenCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AccessTokenCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AccessTokenCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AccessTokenCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AccessTokenCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessTokenCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AccessTokenCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AccessTokenCredentialsMut<'msg> {
  type Message = AccessTokenCredentials;
}

impl ::std::fmt::Debug for AccessTokenCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AccessTokenCredentials>> for AccessTokenCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessTokenCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AccessTokenCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessTokenCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AccessTokenCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // token: optional string
  pub fn token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `AccessTokenCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AccessTokenCredentialsMut<'_> {}

// SAFETY:
// - `AccessTokenCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AccessTokenCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for AccessTokenCredentialsMut<'msg> {
  type Proxied = AccessTokenCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, AccessTokenCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AccessTokenCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AccessTokenCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AccessTokenCredentialsMut<'msg> {
  type MutProxied = AccessTokenCredentials;
  fn as_mut(&mut self) -> AccessTokenCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AccessTokenCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> AccessTokenCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AccessTokenCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AccessTokenCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AccessTokenCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AccessTokenCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // token: optional string
  pub fn token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl AccessTokenCredentials

impl ::std::ops::Drop for AccessTokenCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AccessTokenCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AccessTokenCredentials {
  type Proxied = Self;
  fn as_view(&self) -> AccessTokenCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AccessTokenCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AccessTokenCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AccessTokenCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__grpc_0service__call_0credentials__access_0token__v3__AccessTokenCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__grpc_0service__call_0credentials__access_0token__v3__AccessTokenCredentials_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__grpc_0service__call_0credentials__access_0token__v3__AccessTokenCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AccessTokenCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AccessTokenCredentials {
  type Msg = AccessTokenCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessTokenCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessTokenCredentials {
  type Msg = AccessTokenCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessTokenCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AccessTokenCredentialsMut<'_> {
  type Msg = AccessTokenCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessTokenCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessTokenCredentialsMut<'_> {
  type Msg = AccessTokenCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessTokenCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessTokenCredentialsView<'_> {
  type Msg = AccessTokenCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessTokenCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AccessTokenCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



