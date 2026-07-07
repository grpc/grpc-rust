const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__grpc_0service__channel_0credentials__xds__v3__XdsCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct XdsCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<XdsCredentials>
}

impl ::protobuf::Message for XdsCredentials {
  type MessageView<'msg> = XdsCredentialsView<'msg>;
  type MessageMut<'msg> = XdsCredentialsMut<'msg>;
}

impl ::std::default::Default for XdsCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for XdsCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `XdsCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `XdsCredentialsMut`.
unsafe impl ::std::marker::Sync for XdsCredentials {}

// SAFETY:
// - `XdsCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for XdsCredentials {}

impl ::protobuf::Proxied for XdsCredentials {
  type View<'msg> = XdsCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for XdsCredentials {}

impl ::protobuf::MutProxied for XdsCredentials {
  type Mut<'msg> = XdsCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct XdsCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, XdsCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for XdsCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for XdsCredentialsView<'msg> {
  type Message = XdsCredentials;
}

impl ::std::fmt::Debug for XdsCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for XdsCredentialsView<'_> {
  fn default() -> XdsCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, XdsCredentials>> for XdsCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, XdsCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> XdsCredentialsView<'msg> {

  pub fn to_owned(&self) -> XdsCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // fallback_credentials: optional message google.protobuf.Any
  pub fn has_fallback_credentials(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn fallback_credentials_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_fallback_credentials().then(|| self.fallback_credentials())
  }
  pub fn fallback_credentials(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

}

// SAFETY:
// - `XdsCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for XdsCredentialsView<'_> {}

// SAFETY:
// - `XdsCredentialsView` is `Send` because while its alive a `XdsCredentialsMut` cannot.
// - `XdsCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for XdsCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for XdsCredentialsView<'msg> {
  type Proxied = XdsCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, XdsCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for XdsCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> XdsCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<XdsCredentials> for XdsCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> XdsCredentials {
    let mut dst = XdsCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<XdsCredentials> for XdsCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> XdsCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for XdsCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for XdsCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for XdsCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct XdsCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, XdsCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for XdsCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for XdsCredentialsMut<'msg> {
  type Message = XdsCredentials;
}

impl ::std::fmt::Debug for XdsCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, XdsCredentials>> for XdsCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, XdsCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> XdsCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, XdsCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> XdsCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // fallback_credentials: optional message google.protobuf.Any
  pub fn has_fallback_credentials(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_fallback_credentials(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn fallback_credentials_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_fallback_credentials().then(|| self.fallback_credentials())
  }
  pub fn fallback_credentials(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn fallback_credentials_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_fallback_credentials(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `XdsCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for XdsCredentialsMut<'_> {}

// SAFETY:
// - `XdsCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for XdsCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for XdsCredentialsMut<'msg> {
  type Proxied = XdsCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, XdsCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for XdsCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, XdsCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for XdsCredentialsMut<'msg> {
  type MutProxied = XdsCredentials;
  fn as_mut(&mut self) -> XdsCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for XdsCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> XdsCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl XdsCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, XdsCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> XdsCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> XdsCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // fallback_credentials: optional message google.protobuf.Any
  pub fn has_fallback_credentials(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_fallback_credentials(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn fallback_credentials_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_fallback_credentials().then(|| self.fallback_credentials())
  }
  pub fn fallback_credentials(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn fallback_credentials_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_fallback_credentials(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl XdsCredentials

impl ::std::ops::Drop for XdsCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for XdsCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for XdsCredentials {
  type Proxied = Self;
  fn as_view(&self) -> XdsCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for XdsCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> XdsCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for XdsCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__grpc_0service__channel_0credentials__xds__v3__XdsCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__grpc_0service__channel_0credentials__xds__v3__XdsCredentials_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__grpc_0service__channel_0credentials__xds__v3__XdsCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for XdsCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for XdsCredentials {
  type Msg = XdsCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<XdsCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for XdsCredentials {
  type Msg = XdsCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<XdsCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for XdsCredentialsMut<'_> {
  type Msg = XdsCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<XdsCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for XdsCredentialsMut<'_> {
  type Msg = XdsCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<XdsCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for XdsCredentialsView<'_> {
  type Msg = XdsCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<XdsCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for XdsCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



