const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__gcp_0authn__v3__GcpAuthnFilterConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GcpAuthnFilterConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GcpAuthnFilterConfig>
}

impl ::protobuf::Message for GcpAuthnFilterConfig {
  type MessageView<'msg> = GcpAuthnFilterConfigView<'msg>;
  type MessageMut<'msg> = GcpAuthnFilterConfigMut<'msg>;
}

impl ::std::default::Default for GcpAuthnFilterConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GcpAuthnFilterConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GcpAuthnFilterConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `GcpAuthnFilterConfigMut`.
unsafe impl ::std::marker::Sync for GcpAuthnFilterConfig {}

// SAFETY:
// - `GcpAuthnFilterConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GcpAuthnFilterConfig {}

impl ::protobuf::Proxied for GcpAuthnFilterConfig {
  type View<'msg> = GcpAuthnFilterConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GcpAuthnFilterConfig {}

impl ::protobuf::MutProxied for GcpAuthnFilterConfig {
  type Mut<'msg> = GcpAuthnFilterConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GcpAuthnFilterConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GcpAuthnFilterConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GcpAuthnFilterConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GcpAuthnFilterConfigView<'msg> {
  type Message = GcpAuthnFilterConfig;
}

impl ::std::fmt::Debug for GcpAuthnFilterConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GcpAuthnFilterConfigView<'_> {
  fn default() -> GcpAuthnFilterConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GcpAuthnFilterConfig>> for GcpAuthnFilterConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GcpAuthnFilterConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GcpAuthnFilterConfigView<'msg> {

  pub fn to_owned(&self) -> GcpAuthnFilterConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // http_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_http_uri(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn http_uri_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'msg>> {
    self.has_http_uri().then(|| self.http_uri())
  }
  pub fn http_uri(self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn retry_policy_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'msg>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }

  // cache_config: optional message envoy.extensions.filters.http.gcp_authn.v3.TokenCacheConfig
  pub fn has_cache_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn cache_config_opt(self) -> ::std::option::Option<super::TokenCacheConfigView<'msg>> {
    self.has_cache_config().then(|| self.cache_config())
  }
  pub fn cache_config(self) -> super::TokenCacheConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TokenCacheConfigView::default())
  }

  // token_header: optional message envoy.extensions.filters.http.gcp_authn.v3.TokenHeader
  pub fn has_token_header(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn token_header_opt(self) -> ::std::option::Option<super::TokenHeaderView<'msg>> {
    self.has_token_header().then(|| self.token_header())
  }
  pub fn token_header(self) -> super::TokenHeaderView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TokenHeaderView::default())
  }

  // cluster: optional string
  pub fn cluster(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `GcpAuthnFilterConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GcpAuthnFilterConfigView<'_> {}

// SAFETY:
// - `GcpAuthnFilterConfigView` is `Send` because while its alive a `GcpAuthnFilterConfigMut` cannot.
// - `GcpAuthnFilterConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for GcpAuthnFilterConfigView<'_> {}

impl<'msg> ::protobuf::AsView for GcpAuthnFilterConfigView<'msg> {
  type Proxied = GcpAuthnFilterConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, GcpAuthnFilterConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GcpAuthnFilterConfigView<'msg> {
  fn into_view<'shorter>(self) -> GcpAuthnFilterConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GcpAuthnFilterConfig> for GcpAuthnFilterConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GcpAuthnFilterConfig {
    let mut dst = GcpAuthnFilterConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GcpAuthnFilterConfig> for GcpAuthnFilterConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GcpAuthnFilterConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GcpAuthnFilterConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GcpAuthnFilterConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GcpAuthnFilterConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GcpAuthnFilterConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GcpAuthnFilterConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GcpAuthnFilterConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GcpAuthnFilterConfigMut<'msg> {
  type Message = GcpAuthnFilterConfig;
}

impl ::std::fmt::Debug for GcpAuthnFilterConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GcpAuthnFilterConfig>> for GcpAuthnFilterConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GcpAuthnFilterConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GcpAuthnFilterConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GcpAuthnFilterConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GcpAuthnFilterConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // http_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_http_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_uri_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_>> {
    self.has_http_uri().then(|| self.http_uri())
  }
  pub fn http_uri(&self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }
  pub fn http_uri_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriMut<'_> {
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
  pub fn set_http_uri(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(&self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RetryPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cache_config: optional message envoy.extensions.filters.http.gcp_authn.v3.TokenCacheConfig
  pub fn has_cache_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cache_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cache_config_opt(&self) -> ::std::option::Option<super::TokenCacheConfigView<'_>> {
    self.has_cache_config().then(|| self.cache_config())
  }
  pub fn cache_config(&self) -> super::TokenCacheConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TokenCacheConfigView::default())
  }
  pub fn cache_config_mut(&mut self) -> super::TokenCacheConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cache_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::TokenCacheConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // token_header: optional message envoy.extensions.filters.http.gcp_authn.v3.TokenHeader
  pub fn has_token_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_token_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn token_header_opt(&self) -> ::std::option::Option<super::TokenHeaderView<'_>> {
    self.has_token_header().then(|| self.token_header())
  }
  pub fn token_header(&self) -> super::TokenHeaderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TokenHeaderView::default())
  }
  pub fn token_header_mut(&mut self) -> super::TokenHeaderMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_token_header(&mut self,
    val: impl ::protobuf::IntoProxied<super::TokenHeader>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // cluster: optional string
  pub fn cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}

// SAFETY:
// - `GcpAuthnFilterConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GcpAuthnFilterConfigMut<'_> {}

// SAFETY:
// - `GcpAuthnFilterConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GcpAuthnFilterConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for GcpAuthnFilterConfigMut<'msg> {
  type Proxied = GcpAuthnFilterConfig;
  fn as_view(&self) -> ::protobuf::View<'_, GcpAuthnFilterConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GcpAuthnFilterConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GcpAuthnFilterConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GcpAuthnFilterConfigMut<'msg> {
  type MutProxied = GcpAuthnFilterConfig;
  fn as_mut(&mut self) -> GcpAuthnFilterConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GcpAuthnFilterConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> GcpAuthnFilterConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GcpAuthnFilterConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GcpAuthnFilterConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GcpAuthnFilterConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GcpAuthnFilterConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // http_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_http_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_uri_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_>> {
    self.has_http_uri().then(|| self.http_uri())
  }
  pub fn http_uri(&self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }
  pub fn http_uri_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriMut<'_> {
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
  pub fn set_http_uri(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(&self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RetryPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cache_config: optional message envoy.extensions.filters.http.gcp_authn.v3.TokenCacheConfig
  pub fn has_cache_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cache_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cache_config_opt(&self) -> ::std::option::Option<super::TokenCacheConfigView<'_>> {
    self.has_cache_config().then(|| self.cache_config())
  }
  pub fn cache_config(&self) -> super::TokenCacheConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TokenCacheConfigView::default())
  }
  pub fn cache_config_mut(&mut self) -> super::TokenCacheConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cache_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::TokenCacheConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // token_header: optional message envoy.extensions.filters.http.gcp_authn.v3.TokenHeader
  pub fn has_token_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_token_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn token_header_opt(&self) -> ::std::option::Option<super::TokenHeaderView<'_>> {
    self.has_token_header().then(|| self.token_header())
  }
  pub fn token_header(&self) -> super::TokenHeaderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TokenHeaderView::default())
  }
  pub fn token_header_mut(&mut self) -> super::TokenHeaderMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_token_header(&mut self,
    val: impl ::protobuf::IntoProxied<super::TokenHeader>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // cluster: optional string
  pub fn cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}  // impl GcpAuthnFilterConfig

impl ::std::ops::Drop for GcpAuthnFilterConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GcpAuthnFilterConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GcpAuthnFilterConfig {
  type Proxied = Self;
  fn as_view(&self) -> GcpAuthnFilterConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GcpAuthnFilterConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GcpAuthnFilterConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GcpAuthnFilterConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__gcp_0authn__v3__GcpAuthnFilterConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33331X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__gcp_0authn__v3__GcpAuthnFilterConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RetryPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TokenCacheConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TokenHeader as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__gcp_0authn__v3__GcpAuthnFilterConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GcpAuthnFilterConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GcpAuthnFilterConfig {
  type Msg = GcpAuthnFilterConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GcpAuthnFilterConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GcpAuthnFilterConfig {
  type Msg = GcpAuthnFilterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GcpAuthnFilterConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GcpAuthnFilterConfigMut<'_> {
  type Msg = GcpAuthnFilterConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GcpAuthnFilterConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GcpAuthnFilterConfigMut<'_> {
  type Msg = GcpAuthnFilterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GcpAuthnFilterConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GcpAuthnFilterConfigView<'_> {
  type Msg = GcpAuthnFilterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GcpAuthnFilterConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GcpAuthnFilterConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__gcp_0authn__v3__Audience_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Audience {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Audience>
}

impl ::protobuf::Message for Audience {
  type MessageView<'msg> = AudienceView<'msg>;
  type MessageMut<'msg> = AudienceMut<'msg>;
}

impl ::std::default::Default for Audience {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Audience {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Audience` is `Sync` because it does not implement interior mutability.
//    Neither does `AudienceMut`.
unsafe impl ::std::marker::Sync for Audience {}

// SAFETY:
// - `Audience` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Audience {}

impl ::protobuf::Proxied for Audience {
  type View<'msg> = AudienceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Audience {}

impl ::protobuf::MutProxied for Audience {
  type Mut<'msg> = AudienceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AudienceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Audience>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AudienceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AudienceView<'msg> {
  type Message = Audience;
}

impl ::std::fmt::Debug for AudienceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AudienceView<'_> {
  fn default() -> AudienceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Audience>> for AudienceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Audience>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AudienceView<'msg> {

  pub fn to_owned(&self) -> Audience {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // url: optional string
  pub fn url(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `AudienceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AudienceView<'_> {}

// SAFETY:
// - `AudienceView` is `Send` because while its alive a `AudienceMut` cannot.
// - `AudienceView` does not use thread-local data.
unsafe impl ::std::marker::Send for AudienceView<'_> {}

impl<'msg> ::protobuf::AsView for AudienceView<'msg> {
  type Proxied = Audience;
  fn as_view(&self) -> ::protobuf::View<'msg, Audience> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AudienceView<'msg> {
  fn into_view<'shorter>(self) -> AudienceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Audience> for AudienceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Audience {
    let mut dst = Audience::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Audience> for AudienceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Audience {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Audience {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AudienceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AudienceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AudienceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Audience>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AudienceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AudienceMut<'msg> {
  type Message = Audience;
}

impl ::std::fmt::Debug for AudienceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Audience>> for AudienceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Audience>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AudienceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Audience> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Audience {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // url: optional string
  pub fn url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `AudienceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AudienceMut<'_> {}

// SAFETY:
// - `AudienceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AudienceMut<'_> {}

impl<'msg> ::protobuf::AsView for AudienceMut<'msg> {
  type Proxied = Audience;
  fn as_view(&self) -> ::protobuf::View<'_, Audience> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AudienceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Audience>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AudienceMut<'msg> {
  type MutProxied = Audience;
  fn as_mut(&mut self) -> AudienceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AudienceMut<'msg> {
  fn into_mut<'shorter>(self) -> AudienceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Audience {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Audience> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AudienceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AudienceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // url: optional string
  pub fn url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl Audience

impl ::std::ops::Drop for Audience {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Audience {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Audience {
  type Proxied = Self;
  fn as_view(&self) -> AudienceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Audience {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AudienceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Audience {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__gcp_0authn__v3__Audience_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__gcp_0authn__v3__Audience_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__gcp_0authn__v3__Audience_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Audience {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Audience {
  type Msg = Audience;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Audience> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Audience {
  type Msg = Audience;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Audience> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AudienceMut<'_> {
  type Msg = Audience;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Audience> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AudienceMut<'_> {
  type Msg = Audience;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Audience> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AudienceView<'_> {
  type Msg = Audience;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Audience> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AudienceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__gcp_0authn__v3__TokenCacheConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TokenCacheConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TokenCacheConfig>
}

impl ::protobuf::Message for TokenCacheConfig {
  type MessageView<'msg> = TokenCacheConfigView<'msg>;
  type MessageMut<'msg> = TokenCacheConfigMut<'msg>;
}

impl ::std::default::Default for TokenCacheConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TokenCacheConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TokenCacheConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `TokenCacheConfigMut`.
unsafe impl ::std::marker::Sync for TokenCacheConfig {}

// SAFETY:
// - `TokenCacheConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TokenCacheConfig {}

impl ::protobuf::Proxied for TokenCacheConfig {
  type View<'msg> = TokenCacheConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TokenCacheConfig {}

impl ::protobuf::MutProxied for TokenCacheConfig {
  type Mut<'msg> = TokenCacheConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TokenCacheConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TokenCacheConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TokenCacheConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TokenCacheConfigView<'msg> {
  type Message = TokenCacheConfig;
}

impl ::std::fmt::Debug for TokenCacheConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TokenCacheConfigView<'_> {
  fn default() -> TokenCacheConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TokenCacheConfig>> for TokenCacheConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TokenCacheConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TokenCacheConfigView<'msg> {

  pub fn to_owned(&self) -> TokenCacheConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // cache_size: optional message google.protobuf.UInt64Value
  pub fn has_cache_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn cache_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_cache_size().then(|| self.cache_size())
  }
  pub fn cache_size(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

}

// SAFETY:
// - `TokenCacheConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TokenCacheConfigView<'_> {}

// SAFETY:
// - `TokenCacheConfigView` is `Send` because while its alive a `TokenCacheConfigMut` cannot.
// - `TokenCacheConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for TokenCacheConfigView<'_> {}

impl<'msg> ::protobuf::AsView for TokenCacheConfigView<'msg> {
  type Proxied = TokenCacheConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, TokenCacheConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TokenCacheConfigView<'msg> {
  fn into_view<'shorter>(self) -> TokenCacheConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TokenCacheConfig> for TokenCacheConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TokenCacheConfig {
    let mut dst = TokenCacheConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TokenCacheConfig> for TokenCacheConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TokenCacheConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TokenCacheConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TokenCacheConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TokenCacheConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TokenCacheConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TokenCacheConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TokenCacheConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TokenCacheConfigMut<'msg> {
  type Message = TokenCacheConfig;
}

impl ::std::fmt::Debug for TokenCacheConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TokenCacheConfig>> for TokenCacheConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TokenCacheConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TokenCacheConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TokenCacheConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TokenCacheConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // cache_size: optional message google.protobuf.UInt64Value
  pub fn has_cache_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_cache_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn cache_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_cache_size().then(|| self.cache_size())
  }
  pub fn cache_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn cache_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_cache_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

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
// - `TokenCacheConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TokenCacheConfigMut<'_> {}

// SAFETY:
// - `TokenCacheConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TokenCacheConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for TokenCacheConfigMut<'msg> {
  type Proxied = TokenCacheConfig;
  fn as_view(&self) -> ::protobuf::View<'_, TokenCacheConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TokenCacheConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TokenCacheConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TokenCacheConfigMut<'msg> {
  type MutProxied = TokenCacheConfig;
  fn as_mut(&mut self) -> TokenCacheConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TokenCacheConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> TokenCacheConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TokenCacheConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TokenCacheConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TokenCacheConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TokenCacheConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // cache_size: optional message google.protobuf.UInt64Value
  pub fn has_cache_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_cache_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn cache_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_cache_size().then(|| self.cache_size())
  }
  pub fn cache_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn cache_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_cache_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl TokenCacheConfig

impl ::std::ops::Drop for TokenCacheConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TokenCacheConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TokenCacheConfig {
  type Proxied = Self;
  fn as_view(&self) -> TokenCacheConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TokenCacheConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TokenCacheConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TokenCacheConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__gcp_0authn__v3__TokenCacheConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__gcp_0authn__v3__TokenCacheConfig_msg_init.0, &[<::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__gcp_0authn__v3__TokenCacheConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TokenCacheConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TokenCacheConfig {
  type Msg = TokenCacheConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenCacheConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TokenCacheConfig {
  type Msg = TokenCacheConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenCacheConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TokenCacheConfigMut<'_> {
  type Msg = TokenCacheConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenCacheConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TokenCacheConfigMut<'_> {
  type Msg = TokenCacheConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenCacheConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TokenCacheConfigView<'_> {
  type Msg = TokenCacheConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenCacheConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TokenCacheConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__gcp_0authn__v3__TokenHeader_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TokenHeader {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TokenHeader>
}

impl ::protobuf::Message for TokenHeader {
  type MessageView<'msg> = TokenHeaderView<'msg>;
  type MessageMut<'msg> = TokenHeaderMut<'msg>;
}

impl ::std::default::Default for TokenHeader {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TokenHeader {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TokenHeader` is `Sync` because it does not implement interior mutability.
//    Neither does `TokenHeaderMut`.
unsafe impl ::std::marker::Sync for TokenHeader {}

// SAFETY:
// - `TokenHeader` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TokenHeader {}

impl ::protobuf::Proxied for TokenHeader {
  type View<'msg> = TokenHeaderView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TokenHeader {}

impl ::protobuf::MutProxied for TokenHeader {
  type Mut<'msg> = TokenHeaderMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TokenHeaderView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TokenHeader>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TokenHeaderView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TokenHeaderView<'msg> {
  type Message = TokenHeader;
}

impl ::std::fmt::Debug for TokenHeaderView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TokenHeaderView<'_> {
  fn default() -> TokenHeaderView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TokenHeader>> for TokenHeaderView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TokenHeader>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TokenHeaderView<'msg> {

  pub fn to_owned(&self) -> TokenHeader {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // value_prefix: optional string
  pub fn value_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `TokenHeaderView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TokenHeaderView<'_> {}

// SAFETY:
// - `TokenHeaderView` is `Send` because while its alive a `TokenHeaderMut` cannot.
// - `TokenHeaderView` does not use thread-local data.
unsafe impl ::std::marker::Send for TokenHeaderView<'_> {}

impl<'msg> ::protobuf::AsView for TokenHeaderView<'msg> {
  type Proxied = TokenHeader;
  fn as_view(&self) -> ::protobuf::View<'msg, TokenHeader> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TokenHeaderView<'msg> {
  fn into_view<'shorter>(self) -> TokenHeaderView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TokenHeader> for TokenHeaderView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TokenHeader {
    let mut dst = TokenHeader::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TokenHeader> for TokenHeaderMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TokenHeader {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TokenHeader {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TokenHeaderView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TokenHeaderMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TokenHeaderMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TokenHeader>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TokenHeaderMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TokenHeaderMut<'msg> {
  type Message = TokenHeader;
}

impl ::std::fmt::Debug for TokenHeaderMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TokenHeader>> for TokenHeaderMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TokenHeader>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TokenHeaderMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TokenHeader> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TokenHeader {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value_prefix: optional string
  pub fn value_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `TokenHeaderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TokenHeaderMut<'_> {}

// SAFETY:
// - `TokenHeaderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TokenHeaderMut<'_> {}

impl<'msg> ::protobuf::AsView for TokenHeaderMut<'msg> {
  type Proxied = TokenHeader;
  fn as_view(&self) -> ::protobuf::View<'_, TokenHeader> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TokenHeaderMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TokenHeader>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TokenHeaderMut<'msg> {
  type MutProxied = TokenHeader;
  fn as_mut(&mut self) -> TokenHeaderMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TokenHeaderMut<'msg> {
  fn into_mut<'shorter>(self) -> TokenHeaderMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TokenHeader {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TokenHeader> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TokenHeaderView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TokenHeaderMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value_prefix: optional string
  pub fn value_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl TokenHeader

impl ::std::ops::Drop for TokenHeader {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TokenHeader {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TokenHeader {
  type Proxied = Self;
  fn as_view(&self) -> TokenHeaderView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TokenHeader {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TokenHeaderMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TokenHeader {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__gcp_0authn__v3__TokenHeader_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__gcp_0authn__v3__TokenHeader_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__gcp_0authn__v3__TokenHeader_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TokenHeader {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TokenHeader {
  type Msg = TokenHeader;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenHeader> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TokenHeader {
  type Msg = TokenHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenHeader> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TokenHeaderMut<'_> {
  type Msg = TokenHeader;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenHeader> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TokenHeaderMut<'_> {
  type Msg = TokenHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenHeader> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TokenHeaderView<'_> {
  type Msg = TokenHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TokenHeader> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TokenHeaderMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



