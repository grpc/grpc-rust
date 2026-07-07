const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__grpc_0service__channel_0credentials__tls__v3__TlsCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TlsCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TlsCredentials>
}

impl ::protobuf::Message for TlsCredentials {
  type MessageView<'msg> = TlsCredentialsView<'msg>;
  type MessageMut<'msg> = TlsCredentialsMut<'msg>;
}

impl ::std::default::Default for TlsCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TlsCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TlsCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `TlsCredentialsMut`.
unsafe impl ::std::marker::Sync for TlsCredentials {}

// SAFETY:
// - `TlsCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TlsCredentials {}

impl ::protobuf::Proxied for TlsCredentials {
  type View<'msg> = TlsCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TlsCredentials {}

impl ::protobuf::MutProxied for TlsCredentials {
  type Mut<'msg> = TlsCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TlsCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TlsCredentialsView<'msg> {
  type Message = TlsCredentials;
}

impl ::std::fmt::Debug for TlsCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TlsCredentialsView<'_> {
  fn default() -> TlsCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TlsCredentials>> for TlsCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsCredentialsView<'msg> {

  pub fn to_owned(&self) -> TlsCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // root_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_root_certificate_provider(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn root_certificate_provider_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'msg>> {
    self.has_root_certificate_provider().then(|| self.root_certificate_provider())
  }
  pub fn root_certificate_provider(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView::default())
  }

  // identity_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_identity_certificate_provider(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn identity_certificate_provider_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'msg>> {
    self.has_identity_certificate_provider().then(|| self.identity_certificate_provider())
  }
  pub fn identity_certificate_provider(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView::default())
  }

}

// SAFETY:
// - `TlsCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TlsCredentialsView<'_> {}

// SAFETY:
// - `TlsCredentialsView` is `Send` because while its alive a `TlsCredentialsMut` cannot.
// - `TlsCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for TlsCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for TlsCredentialsView<'msg> {
  type Proxied = TlsCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, TlsCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> TlsCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsCredentials> for TlsCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsCredentials {
    let mut dst = TlsCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsCredentials> for TlsCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TlsCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TlsCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TlsCredentialsMut<'msg> {
  type Message = TlsCredentials;
}

impl ::std::fmt::Debug for TlsCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TlsCredentials>> for TlsCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TlsCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // root_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_root_certificate_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_root_certificate_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn root_certificate_provider_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'_>> {
    self.has_root_certificate_provider().then(|| self.root_certificate_provider())
  }
  pub fn root_certificate_provider(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView::default())
  }
  pub fn root_certificate_provider_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceMut<'_> {
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
  pub fn set_root_certificate_provider(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // identity_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_identity_certificate_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_identity_certificate_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn identity_certificate_provider_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'_>> {
    self.has_identity_certificate_provider().then(|| self.identity_certificate_provider())
  }
  pub fn identity_certificate_provider(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView::default())
  }
  pub fn identity_certificate_provider_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceMut<'_> {
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
  pub fn set_identity_certificate_provider(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `TlsCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TlsCredentialsMut<'_> {}

// SAFETY:
// - `TlsCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TlsCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for TlsCredentialsMut<'msg> {
  type Proxied = TlsCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, TlsCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TlsCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TlsCredentialsMut<'msg> {
  type MutProxied = TlsCredentials;
  fn as_mut(&mut self) -> TlsCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TlsCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> TlsCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TlsCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TlsCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TlsCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TlsCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // root_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_root_certificate_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_root_certificate_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn root_certificate_provider_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'_>> {
    self.has_root_certificate_provider().then(|| self.root_certificate_provider())
  }
  pub fn root_certificate_provider(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView::default())
  }
  pub fn root_certificate_provider_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceMut<'_> {
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
  pub fn set_root_certificate_provider(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // identity_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_identity_certificate_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_identity_certificate_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn identity_certificate_provider_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'_>> {
    self.has_identity_certificate_provider().then(|| self.identity_certificate_provider())
  }
  pub fn identity_certificate_provider(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceView::default())
  }
  pub fn identity_certificate_provider_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstanceMut<'_> {
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
  pub fn set_identity_certificate_provider(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl TlsCredentials

impl ::std::ops::Drop for TlsCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TlsCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TlsCredentials {
  type Proxied = Self;
  fn as_view(&self) -> TlsCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TlsCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TlsCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TlsCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__grpc_0service__channel_0credentials__tls__v3__TlsCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__grpc_0service__channel_0credentials__tls__v3__TlsCredentials_msg_init.0, &[<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstance as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::tls::common_tls_context::CertificateProviderInstance as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__grpc_0service__channel_0credentials__tls__v3__TlsCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsCredentials {
  type Msg = TlsCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsCredentials {
  type Msg = TlsCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsCredentialsMut<'_> {
  type Msg = TlsCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsCredentialsMut<'_> {
  type Msg = TlsCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsCredentialsView<'_> {
  type Msg = TlsCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



