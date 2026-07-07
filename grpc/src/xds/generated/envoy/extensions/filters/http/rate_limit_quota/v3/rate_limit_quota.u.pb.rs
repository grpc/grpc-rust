const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaFilterConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RateLimitQuotaFilterConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RateLimitQuotaFilterConfig>
}

impl ::protobuf::Message for RateLimitQuotaFilterConfig {
  type MessageView<'msg> = RateLimitQuotaFilterConfigView<'msg>;
  type MessageMut<'msg> = RateLimitQuotaFilterConfigMut<'msg>;
}

impl ::std::default::Default for RateLimitQuotaFilterConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RateLimitQuotaFilterConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RateLimitQuotaFilterConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `RateLimitQuotaFilterConfigMut`.
unsafe impl ::std::marker::Sync for RateLimitQuotaFilterConfig {}

// SAFETY:
// - `RateLimitQuotaFilterConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitQuotaFilterConfig {}

impl ::protobuf::Proxied for RateLimitQuotaFilterConfig {
  type View<'msg> = RateLimitQuotaFilterConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RateLimitQuotaFilterConfig {}

impl ::protobuf::MutProxied for RateLimitQuotaFilterConfig {
  type Mut<'msg> = RateLimitQuotaFilterConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RateLimitQuotaFilterConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaFilterConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitQuotaFilterConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RateLimitQuotaFilterConfigView<'msg> {
  type Message = RateLimitQuotaFilterConfig;
}

impl ::std::fmt::Debug for RateLimitQuotaFilterConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RateLimitQuotaFilterConfigView<'_> {
  fn default() -> RateLimitQuotaFilterConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaFilterConfig>> for RateLimitQuotaFilterConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaFilterConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitQuotaFilterConfigView<'msg> {

  pub fn to_owned(&self) -> RateLimitQuotaFilterConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // rlqs_server: optional message envoy.config.core.v3.GrpcService
  pub fn has_rlqs_server(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn rlqs_server_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'msg>> {
    self.has_rlqs_server().then(|| self.rlqs_server())
  }
  pub fn rlqs_server(self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }

  // domain: optional string
  pub fn domain(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // bucket_matchers: optional message xds.type.matcher.v3.Matcher
  pub fn has_bucket_matchers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn bucket_matchers_opt(self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg>> {
    self.has_bucket_matchers().then(|| self.bucket_matchers())
  }
  pub fn bucket_matchers(self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }

  // filter_enabled: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_filter_enabled(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn filter_enabled_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'msg>> {
    self.has_filter_enabled().then(|| self.filter_enabled())
  }
  pub fn filter_enabled(self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }

  // filter_enforced: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_filter_enforced(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn filter_enforced_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'msg>> {
    self.has_filter_enforced().then(|| self.filter_enforced())
  }
  pub fn filter_enforced(self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }

  // request_headers_to_add_when_not_enforced: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add_when_not_enforced(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `RateLimitQuotaFilterConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RateLimitQuotaFilterConfigView<'_> {}

// SAFETY:
// - `RateLimitQuotaFilterConfigView` is `Send` because while its alive a `RateLimitQuotaFilterConfigMut` cannot.
// - `RateLimitQuotaFilterConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitQuotaFilterConfigView<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitQuotaFilterConfigView<'msg> {
  type Proxied = RateLimitQuotaFilterConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, RateLimitQuotaFilterConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitQuotaFilterConfigView<'msg> {
  fn into_view<'shorter>(self) -> RateLimitQuotaFilterConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitQuotaFilterConfig> for RateLimitQuotaFilterConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitQuotaFilterConfig {
    let mut dst = RateLimitQuotaFilterConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitQuotaFilterConfig> for RateLimitQuotaFilterConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitQuotaFilterConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RateLimitQuotaFilterConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitQuotaFilterConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitQuotaFilterConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RateLimitQuotaFilterConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaFilterConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitQuotaFilterConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RateLimitQuotaFilterConfigMut<'msg> {
  type Message = RateLimitQuotaFilterConfig;
}

impl ::std::fmt::Debug for RateLimitQuotaFilterConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaFilterConfig>> for RateLimitQuotaFilterConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaFilterConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitQuotaFilterConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaFilterConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RateLimitQuotaFilterConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // rlqs_server: optional message envoy.config.core.v3.GrpcService
  pub fn has_rlqs_server(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_rlqs_server(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn rlqs_server_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_>> {
    self.has_rlqs_server().then(|| self.rlqs_server())
  }
  pub fn rlqs_server(&self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }
  pub fn rlqs_server_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceMut<'_> {
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
  pub fn set_rlqs_server(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // domain: optional string
  pub fn domain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_domain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // bucket_matchers: optional message xds.type.matcher.v3.Matcher
  pub fn has_bucket_matchers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_bucket_matchers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn bucket_matchers_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_bucket_matchers().then(|| self.bucket_matchers())
  }
  pub fn bucket_matchers(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn bucket_matchers_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_bucket_matchers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // filter_enabled: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_filter_enabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_filter_enabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn filter_enabled_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_>> {
    self.has_filter_enabled().then(|| self.filter_enabled())
  }
  pub fn filter_enabled(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }
  pub fn filter_enabled_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentMut<'_> {
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
  pub fn set_filter_enabled(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // filter_enforced: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_filter_enforced(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_filter_enforced(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn filter_enforced_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_>> {
    self.has_filter_enforced().then(|| self.filter_enforced())
  }
  pub fn filter_enforced(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }
  pub fn filter_enforced_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_filter_enforced(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // request_headers_to_add_when_not_enforced: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add_when_not_enforced(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_headers_to_add_when_not_enforced_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_request_headers_to_add_when_not_enforced(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}

// SAFETY:
// - `RateLimitQuotaFilterConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RateLimitQuotaFilterConfigMut<'_> {}

// SAFETY:
// - `RateLimitQuotaFilterConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RateLimitQuotaFilterConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitQuotaFilterConfigMut<'msg> {
  type Proxied = RateLimitQuotaFilterConfig;
  fn as_view(&self) -> ::protobuf::View<'_, RateLimitQuotaFilterConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitQuotaFilterConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RateLimitQuotaFilterConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RateLimitQuotaFilterConfigMut<'msg> {
  type MutProxied = RateLimitQuotaFilterConfig;
  fn as_mut(&mut self) -> RateLimitQuotaFilterConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RateLimitQuotaFilterConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> RateLimitQuotaFilterConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RateLimitQuotaFilterConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RateLimitQuotaFilterConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RateLimitQuotaFilterConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RateLimitQuotaFilterConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // rlqs_server: optional message envoy.config.core.v3.GrpcService
  pub fn has_rlqs_server(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_rlqs_server(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn rlqs_server_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_>> {
    self.has_rlqs_server().then(|| self.rlqs_server())
  }
  pub fn rlqs_server(&self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }
  pub fn rlqs_server_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceMut<'_> {
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
  pub fn set_rlqs_server(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // domain: optional string
  pub fn domain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_domain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // bucket_matchers: optional message xds.type.matcher.v3.Matcher
  pub fn has_bucket_matchers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_bucket_matchers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn bucket_matchers_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_bucket_matchers().then(|| self.bucket_matchers())
  }
  pub fn bucket_matchers(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn bucket_matchers_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_bucket_matchers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // filter_enabled: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_filter_enabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_filter_enabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn filter_enabled_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_>> {
    self.has_filter_enabled().then(|| self.filter_enabled())
  }
  pub fn filter_enabled(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }
  pub fn filter_enabled_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentMut<'_> {
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
  pub fn set_filter_enabled(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // filter_enforced: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_filter_enforced(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_filter_enforced(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn filter_enforced_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_>> {
    self.has_filter_enforced().then(|| self.filter_enforced())
  }
  pub fn filter_enforced(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }
  pub fn filter_enforced_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_filter_enforced(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // request_headers_to_add_when_not_enforced: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add_when_not_enforced(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_headers_to_add_when_not_enforced_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_request_headers_to_add_when_not_enforced(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}  // impl RateLimitQuotaFilterConfig

impl ::std::ops::Drop for RateLimitQuotaFilterConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RateLimitQuotaFilterConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RateLimitQuotaFilterConfig {
  type Proxied = Self;
  fn as_view(&self) -> RateLimitQuotaFilterConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RateLimitQuotaFilterConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RateLimitQuotaFilterConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RateLimitQuotaFilterConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaFilterConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X333G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaFilterConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaFilterConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitQuotaFilterConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitQuotaFilterConfig {
  type Msg = RateLimitQuotaFilterConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaFilterConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaFilterConfig {
  type Msg = RateLimitQuotaFilterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaFilterConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitQuotaFilterConfigMut<'_> {
  type Msg = RateLimitQuotaFilterConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaFilterConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaFilterConfigMut<'_> {
  type Msg = RateLimitQuotaFilterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaFilterConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaFilterConfigView<'_> {
  type Msg = RateLimitQuotaFilterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaFilterConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitQuotaFilterConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaOverride_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RateLimitQuotaOverride {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RateLimitQuotaOverride>
}

impl ::protobuf::Message for RateLimitQuotaOverride {
  type MessageView<'msg> = RateLimitQuotaOverrideView<'msg>;
  type MessageMut<'msg> = RateLimitQuotaOverrideMut<'msg>;
}

impl ::std::default::Default for RateLimitQuotaOverride {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RateLimitQuotaOverride {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RateLimitQuotaOverride` is `Sync` because it does not implement interior mutability.
//    Neither does `RateLimitQuotaOverrideMut`.
unsafe impl ::std::marker::Sync for RateLimitQuotaOverride {}

// SAFETY:
// - `RateLimitQuotaOverride` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitQuotaOverride {}

impl ::protobuf::Proxied for RateLimitQuotaOverride {
  type View<'msg> = RateLimitQuotaOverrideView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RateLimitQuotaOverride {}

impl ::protobuf::MutProxied for RateLimitQuotaOverride {
  type Mut<'msg> = RateLimitQuotaOverrideMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RateLimitQuotaOverrideView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaOverride>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitQuotaOverrideView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RateLimitQuotaOverrideView<'msg> {
  type Message = RateLimitQuotaOverride;
}

impl ::std::fmt::Debug for RateLimitQuotaOverrideView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RateLimitQuotaOverrideView<'_> {
  fn default() -> RateLimitQuotaOverrideView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaOverride>> for RateLimitQuotaOverrideView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaOverride>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitQuotaOverrideView<'msg> {

  pub fn to_owned(&self) -> RateLimitQuotaOverride {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // domain: optional string
  pub fn domain(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // bucket_matchers: optional message xds.type.matcher.v3.Matcher
  pub fn has_bucket_matchers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn bucket_matchers_opt(self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg>> {
    self.has_bucket_matchers().then(|| self.bucket_matchers())
  }
  pub fn bucket_matchers(self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }

}

// SAFETY:
// - `RateLimitQuotaOverrideView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RateLimitQuotaOverrideView<'_> {}

// SAFETY:
// - `RateLimitQuotaOverrideView` is `Send` because while its alive a `RateLimitQuotaOverrideMut` cannot.
// - `RateLimitQuotaOverrideView` does not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitQuotaOverrideView<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitQuotaOverrideView<'msg> {
  type Proxied = RateLimitQuotaOverride;
  fn as_view(&self) -> ::protobuf::View<'msg, RateLimitQuotaOverride> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitQuotaOverrideView<'msg> {
  fn into_view<'shorter>(self) -> RateLimitQuotaOverrideView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitQuotaOverride> for RateLimitQuotaOverrideView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitQuotaOverride {
    let mut dst = RateLimitQuotaOverride::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitQuotaOverride> for RateLimitQuotaOverrideMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitQuotaOverride {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RateLimitQuotaOverride {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitQuotaOverrideView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitQuotaOverrideMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RateLimitQuotaOverrideMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaOverride>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitQuotaOverrideMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RateLimitQuotaOverrideMut<'msg> {
  type Message = RateLimitQuotaOverride;
}

impl ::std::fmt::Debug for RateLimitQuotaOverrideMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaOverride>> for RateLimitQuotaOverrideMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaOverride>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitQuotaOverrideMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaOverride> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RateLimitQuotaOverride {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // domain: optional string
  pub fn domain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_domain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // bucket_matchers: optional message xds.type.matcher.v3.Matcher
  pub fn has_bucket_matchers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_bucket_matchers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn bucket_matchers_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_bucket_matchers().then(|| self.bucket_matchers())
  }
  pub fn bucket_matchers(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn bucket_matchers_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_bucket_matchers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

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
// - `RateLimitQuotaOverrideMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RateLimitQuotaOverrideMut<'_> {}

// SAFETY:
// - `RateLimitQuotaOverrideMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RateLimitQuotaOverrideMut<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitQuotaOverrideMut<'msg> {
  type Proxied = RateLimitQuotaOverride;
  fn as_view(&self) -> ::protobuf::View<'_, RateLimitQuotaOverride> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitQuotaOverrideMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RateLimitQuotaOverride>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RateLimitQuotaOverrideMut<'msg> {
  type MutProxied = RateLimitQuotaOverride;
  fn as_mut(&mut self) -> RateLimitQuotaOverrideMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RateLimitQuotaOverrideMut<'msg> {
  fn into_mut<'shorter>(self) -> RateLimitQuotaOverrideMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RateLimitQuotaOverride {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RateLimitQuotaOverride> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RateLimitQuotaOverrideView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RateLimitQuotaOverrideMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // domain: optional string
  pub fn domain(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_domain(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // bucket_matchers: optional message xds.type.matcher.v3.Matcher
  pub fn has_bucket_matchers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_bucket_matchers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn bucket_matchers_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_bucket_matchers().then(|| self.bucket_matchers())
  }
  pub fn bucket_matchers(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn bucket_matchers_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_bucket_matchers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl RateLimitQuotaOverride

impl ::std::ops::Drop for RateLimitQuotaOverride {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RateLimitQuotaOverride {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RateLimitQuotaOverride {
  type Proxied = Self;
  fn as_view(&self) -> RateLimitQuotaOverrideView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RateLimitQuotaOverride {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RateLimitQuotaOverrideMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RateLimitQuotaOverride {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaOverride_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaOverride_msg_init.0, &[<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaOverride_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitQuotaOverride {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitQuotaOverride {
  type Msg = RateLimitQuotaOverride;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaOverride> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaOverride {
  type Msg = RateLimitQuotaOverride;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaOverride> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitQuotaOverrideMut<'_> {
  type Msg = RateLimitQuotaOverride;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaOverride> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaOverrideMut<'_> {
  type Msg = RateLimitQuotaOverride;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaOverride> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaOverrideView<'_> {
  type Msg = RateLimitQuotaOverride;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaOverride> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitQuotaOverrideMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RateLimitQuotaBucketSettings {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RateLimitQuotaBucketSettings>
}

impl ::protobuf::Message for RateLimitQuotaBucketSettings {
  type MessageView<'msg> = RateLimitQuotaBucketSettingsView<'msg>;
  type MessageMut<'msg> = RateLimitQuotaBucketSettingsMut<'msg>;
}

impl ::std::default::Default for RateLimitQuotaBucketSettings {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RateLimitQuotaBucketSettings {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RateLimitQuotaBucketSettings` is `Sync` because it does not implement interior mutability.
//    Neither does `RateLimitQuotaBucketSettingsMut`.
unsafe impl ::std::marker::Sync for RateLimitQuotaBucketSettings {}

// SAFETY:
// - `RateLimitQuotaBucketSettings` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitQuotaBucketSettings {}

impl ::protobuf::Proxied for RateLimitQuotaBucketSettings {
  type View<'msg> = RateLimitQuotaBucketSettingsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RateLimitQuotaBucketSettings {}

impl ::protobuf::MutProxied for RateLimitQuotaBucketSettings {
  type Mut<'msg> = RateLimitQuotaBucketSettingsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RateLimitQuotaBucketSettingsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaBucketSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitQuotaBucketSettingsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RateLimitQuotaBucketSettingsView<'msg> {
  type Message = RateLimitQuotaBucketSettings;
}

impl ::std::fmt::Debug for RateLimitQuotaBucketSettingsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RateLimitQuotaBucketSettingsView<'_> {
  fn default() -> RateLimitQuotaBucketSettingsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaBucketSettings>> for RateLimitQuotaBucketSettingsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitQuotaBucketSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitQuotaBucketSettingsView<'msg> {

  pub fn to_owned(&self) -> RateLimitQuotaBucketSettings {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bucket_id_builder: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder
  pub fn has_bucket_id_builder(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn bucket_id_builder_opt(self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::BucketIdBuilderView<'msg>> {
    self.has_bucket_id_builder().then(|| self.bucket_id_builder())
  }
  pub fn bucket_id_builder(self) -> super::rate_limit_quota_bucket_settings::BucketIdBuilderView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::BucketIdBuilderView::default())
  }

  // reporting_interval: optional message google.protobuf.Duration
  pub fn has_reporting_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn reporting_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_reporting_interval().then(|| self.reporting_interval())
  }
  pub fn reporting_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // deny_response_settings: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.DenyResponseSettings
  pub fn has_deny_response_settings(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn deny_response_settings_opt(self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::DenyResponseSettingsView<'msg>> {
    self.has_deny_response_settings().then(|| self.deny_response_settings())
  }
  pub fn deny_response_settings(self) -> super::rate_limit_quota_bucket_settings::DenyResponseSettingsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::DenyResponseSettingsView::default())
  }

  // no_assignment_behavior: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.NoAssignmentBehavior
  pub fn has_no_assignment_behavior(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn no_assignment_behavior_opt(self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::NoAssignmentBehaviorView<'msg>> {
    self.has_no_assignment_behavior().then(|| self.no_assignment_behavior())
  }
  pub fn no_assignment_behavior(self) -> super::rate_limit_quota_bucket_settings::NoAssignmentBehaviorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::NoAssignmentBehaviorView::default())
  }

  // expired_assignment_behavior: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior
  pub fn has_expired_assignment_behavior(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn expired_assignment_behavior_opt(self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehaviorView<'msg>> {
    self.has_expired_assignment_behavior().then(|| self.expired_assignment_behavior())
  }
  pub fn expired_assignment_behavior(self) -> super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehaviorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehaviorView::default())
  }

}

// SAFETY:
// - `RateLimitQuotaBucketSettingsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RateLimitQuotaBucketSettingsView<'_> {}

// SAFETY:
// - `RateLimitQuotaBucketSettingsView` is `Send` because while its alive a `RateLimitQuotaBucketSettingsMut` cannot.
// - `RateLimitQuotaBucketSettingsView` does not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitQuotaBucketSettingsView<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitQuotaBucketSettingsView<'msg> {
  type Proxied = RateLimitQuotaBucketSettings;
  fn as_view(&self) -> ::protobuf::View<'msg, RateLimitQuotaBucketSettings> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitQuotaBucketSettingsView<'msg> {
  fn into_view<'shorter>(self) -> RateLimitQuotaBucketSettingsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitQuotaBucketSettings> for RateLimitQuotaBucketSettingsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitQuotaBucketSettings {
    let mut dst = RateLimitQuotaBucketSettings::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitQuotaBucketSettings> for RateLimitQuotaBucketSettingsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitQuotaBucketSettings {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RateLimitQuotaBucketSettings {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitQuotaBucketSettingsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitQuotaBucketSettingsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RateLimitQuotaBucketSettingsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaBucketSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitQuotaBucketSettingsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RateLimitQuotaBucketSettingsMut<'msg> {
  type Message = RateLimitQuotaBucketSettings;
}

impl ::std::fmt::Debug for RateLimitQuotaBucketSettingsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaBucketSettings>> for RateLimitQuotaBucketSettingsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaBucketSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitQuotaBucketSettingsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitQuotaBucketSettings> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RateLimitQuotaBucketSettings {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bucket_id_builder: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder
  pub fn has_bucket_id_builder(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_bucket_id_builder(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn bucket_id_builder_opt(&self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::BucketIdBuilderView<'_>> {
    self.has_bucket_id_builder().then(|| self.bucket_id_builder())
  }
  pub fn bucket_id_builder(&self) -> super::rate_limit_quota_bucket_settings::BucketIdBuilderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::BucketIdBuilderView::default())
  }
  pub fn bucket_id_builder_mut(&mut self) -> super::rate_limit_quota_bucket_settings::BucketIdBuilderMut<'_> {
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
  pub fn set_bucket_id_builder(&mut self,
    val: impl ::protobuf::IntoProxied<super::rate_limit_quota_bucket_settings::BucketIdBuilder>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // reporting_interval: optional message google.protobuf.Duration
  pub fn has_reporting_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_reporting_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn reporting_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_reporting_interval().then(|| self.reporting_interval())
  }
  pub fn reporting_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn reporting_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_reporting_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // deny_response_settings: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.DenyResponseSettings
  pub fn has_deny_response_settings(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_deny_response_settings(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn deny_response_settings_opt(&self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::DenyResponseSettingsView<'_>> {
    self.has_deny_response_settings().then(|| self.deny_response_settings())
  }
  pub fn deny_response_settings(&self) -> super::rate_limit_quota_bucket_settings::DenyResponseSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::DenyResponseSettingsView::default())
  }
  pub fn deny_response_settings_mut(&mut self) -> super::rate_limit_quota_bucket_settings::DenyResponseSettingsMut<'_> {
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
  pub fn set_deny_response_settings(&mut self,
    val: impl ::protobuf::IntoProxied<super::rate_limit_quota_bucket_settings::DenyResponseSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // no_assignment_behavior: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.NoAssignmentBehavior
  pub fn has_no_assignment_behavior(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_no_assignment_behavior(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn no_assignment_behavior_opt(&self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::NoAssignmentBehaviorView<'_>> {
    self.has_no_assignment_behavior().then(|| self.no_assignment_behavior())
  }
  pub fn no_assignment_behavior(&self) -> super::rate_limit_quota_bucket_settings::NoAssignmentBehaviorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::NoAssignmentBehaviorView::default())
  }
  pub fn no_assignment_behavior_mut(&mut self) -> super::rate_limit_quota_bucket_settings::NoAssignmentBehaviorMut<'_> {
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
  pub fn set_no_assignment_behavior(&mut self,
    val: impl ::protobuf::IntoProxied<super::rate_limit_quota_bucket_settings::NoAssignmentBehavior>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // expired_assignment_behavior: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior
  pub fn has_expired_assignment_behavior(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_expired_assignment_behavior(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn expired_assignment_behavior_opt(&self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehaviorView<'_>> {
    self.has_expired_assignment_behavior().then(|| self.expired_assignment_behavior())
  }
  pub fn expired_assignment_behavior(&self) -> super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehaviorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehaviorView::default())
  }
  pub fn expired_assignment_behavior_mut(&mut self) -> super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehaviorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_expired_assignment_behavior(&mut self,
    val: impl ::protobuf::IntoProxied<super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehavior>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}

// SAFETY:
// - `RateLimitQuotaBucketSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RateLimitQuotaBucketSettingsMut<'_> {}

// SAFETY:
// - `RateLimitQuotaBucketSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RateLimitQuotaBucketSettingsMut<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitQuotaBucketSettingsMut<'msg> {
  type Proxied = RateLimitQuotaBucketSettings;
  fn as_view(&self) -> ::protobuf::View<'_, RateLimitQuotaBucketSettings> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitQuotaBucketSettingsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RateLimitQuotaBucketSettings>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RateLimitQuotaBucketSettingsMut<'msg> {
  type MutProxied = RateLimitQuotaBucketSettings;
  fn as_mut(&mut self) -> RateLimitQuotaBucketSettingsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RateLimitQuotaBucketSettingsMut<'msg> {
  fn into_mut<'shorter>(self) -> RateLimitQuotaBucketSettingsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RateLimitQuotaBucketSettings {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RateLimitQuotaBucketSettings> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RateLimitQuotaBucketSettingsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RateLimitQuotaBucketSettingsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bucket_id_builder: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder
  pub fn has_bucket_id_builder(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_bucket_id_builder(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn bucket_id_builder_opt(&self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::BucketIdBuilderView<'_>> {
    self.has_bucket_id_builder().then(|| self.bucket_id_builder())
  }
  pub fn bucket_id_builder(&self) -> super::rate_limit_quota_bucket_settings::BucketIdBuilderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::BucketIdBuilderView::default())
  }
  pub fn bucket_id_builder_mut(&mut self) -> super::rate_limit_quota_bucket_settings::BucketIdBuilderMut<'_> {
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
  pub fn set_bucket_id_builder(&mut self,
    val: impl ::protobuf::IntoProxied<super::rate_limit_quota_bucket_settings::BucketIdBuilder>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // reporting_interval: optional message google.protobuf.Duration
  pub fn has_reporting_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_reporting_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn reporting_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_reporting_interval().then(|| self.reporting_interval())
  }
  pub fn reporting_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn reporting_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_reporting_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // deny_response_settings: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.DenyResponseSettings
  pub fn has_deny_response_settings(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_deny_response_settings(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn deny_response_settings_opt(&self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::DenyResponseSettingsView<'_>> {
    self.has_deny_response_settings().then(|| self.deny_response_settings())
  }
  pub fn deny_response_settings(&self) -> super::rate_limit_quota_bucket_settings::DenyResponseSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::DenyResponseSettingsView::default())
  }
  pub fn deny_response_settings_mut(&mut self) -> super::rate_limit_quota_bucket_settings::DenyResponseSettingsMut<'_> {
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
  pub fn set_deny_response_settings(&mut self,
    val: impl ::protobuf::IntoProxied<super::rate_limit_quota_bucket_settings::DenyResponseSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // no_assignment_behavior: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.NoAssignmentBehavior
  pub fn has_no_assignment_behavior(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_no_assignment_behavior(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn no_assignment_behavior_opt(&self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::NoAssignmentBehaviorView<'_>> {
    self.has_no_assignment_behavior().then(|| self.no_assignment_behavior())
  }
  pub fn no_assignment_behavior(&self) -> super::rate_limit_quota_bucket_settings::NoAssignmentBehaviorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::NoAssignmentBehaviorView::default())
  }
  pub fn no_assignment_behavior_mut(&mut self) -> super::rate_limit_quota_bucket_settings::NoAssignmentBehaviorMut<'_> {
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
  pub fn set_no_assignment_behavior(&mut self,
    val: impl ::protobuf::IntoProxied<super::rate_limit_quota_bucket_settings::NoAssignmentBehavior>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // expired_assignment_behavior: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior
  pub fn has_expired_assignment_behavior(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_expired_assignment_behavior(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn expired_assignment_behavior_opt(&self) -> ::std::option::Option<super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehaviorView<'_>> {
    self.has_expired_assignment_behavior().then(|| self.expired_assignment_behavior())
  }
  pub fn expired_assignment_behavior(&self) -> super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehaviorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehaviorView::default())
  }
  pub fn expired_assignment_behavior_mut(&mut self) -> super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehaviorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_expired_assignment_behavior(&mut self,
    val: impl ::protobuf::IntoProxied<super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehavior>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}  // impl RateLimitQuotaBucketSettings

impl ::std::ops::Drop for RateLimitQuotaBucketSettings {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RateLimitQuotaBucketSettings {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RateLimitQuotaBucketSettings {
  type Proxied = Self;
  fn as_view(&self) -> RateLimitQuotaBucketSettingsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RateLimitQuotaBucketSettings {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RateLimitQuotaBucketSettingsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RateLimitQuotaBucketSettings {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings_msg_init.0, &[<super::rate_limit_quota_bucket_settings::BucketIdBuilder as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::rate_limit_quota_bucket_settings::DenyResponseSettings as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::rate_limit_quota_bucket_settings::NoAssignmentBehavior as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::rate_limit_quota_bucket_settings::ExpiredAssignmentBehavior as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitQuotaBucketSettings {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitQuotaBucketSettings {
  type Msg = RateLimitQuotaBucketSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaBucketSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaBucketSettings {
  type Msg = RateLimitQuotaBucketSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaBucketSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitQuotaBucketSettingsMut<'_> {
  type Msg = RateLimitQuotaBucketSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaBucketSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaBucketSettingsMut<'_> {
  type Msg = RateLimitQuotaBucketSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaBucketSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitQuotaBucketSettingsView<'_> {
  type Msg = RateLimitQuotaBucketSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitQuotaBucketSettings> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitQuotaBucketSettingsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod rate_limit_quota_bucket_settings {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__NoAssignmentBehavior_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NoAssignmentBehavior {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NoAssignmentBehavior>
}

impl ::protobuf::Message for NoAssignmentBehavior {
  type MessageView<'msg> = NoAssignmentBehaviorView<'msg>;
  type MessageMut<'msg> = NoAssignmentBehaviorMut<'msg>;
}

impl ::std::default::Default for NoAssignmentBehavior {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NoAssignmentBehavior {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NoAssignmentBehavior` is `Sync` because it does not implement interior mutability.
//    Neither does `NoAssignmentBehaviorMut`.
unsafe impl ::std::marker::Sync for NoAssignmentBehavior {}

// SAFETY:
// - `NoAssignmentBehavior` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for NoAssignmentBehavior {}

impl ::protobuf::Proxied for NoAssignmentBehavior {
  type View<'msg> = NoAssignmentBehaviorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NoAssignmentBehavior {}

impl ::protobuf::MutProxied for NoAssignmentBehavior {
  type Mut<'msg> = NoAssignmentBehaviorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NoAssignmentBehaviorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NoAssignmentBehavior>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NoAssignmentBehaviorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NoAssignmentBehaviorView<'msg> {
  type Message = NoAssignmentBehavior;
}

impl ::std::fmt::Debug for NoAssignmentBehaviorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NoAssignmentBehaviorView<'_> {
  fn default() -> NoAssignmentBehaviorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NoAssignmentBehavior>> for NoAssignmentBehaviorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NoAssignmentBehavior>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NoAssignmentBehaviorView<'msg> {

  pub fn to_owned(&self) -> NoAssignmentBehavior {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // fallback_rate_limit: optional message envoy.type.v3.RateLimitStrategy
  pub fn has_fallback_rate_limit(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn fallback_rate_limit_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'msg>> {
    self.has_fallback_rate_limit().then(|| self.fallback_rate_limit())
  }
  pub fn fallback_rate_limit(self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView::default())
  }

  pub fn no_assignment_behavior(self) -> super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorOneof<'msg> {
    match self.no_assignment_behavior_case() {
      super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorCase::FallbackRateLimit =>
          super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorOneof::FallbackRateLimit(self.fallback_rate_limit()),
      _ => super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn no_assignment_behavior_case(self) -> super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `NoAssignmentBehaviorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for NoAssignmentBehaviorView<'_> {}

// SAFETY:
// - `NoAssignmentBehaviorView` is `Send` because while its alive a `NoAssignmentBehaviorMut` cannot.
// - `NoAssignmentBehaviorView` does not use thread-local data.
unsafe impl ::std::marker::Send for NoAssignmentBehaviorView<'_> {}

impl<'msg> ::protobuf::AsView for NoAssignmentBehaviorView<'msg> {
  type Proxied = NoAssignmentBehavior;
  fn as_view(&self) -> ::protobuf::View<'msg, NoAssignmentBehavior> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NoAssignmentBehaviorView<'msg> {
  fn into_view<'shorter>(self) -> NoAssignmentBehaviorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NoAssignmentBehavior> for NoAssignmentBehaviorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NoAssignmentBehavior {
    let mut dst = NoAssignmentBehavior::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NoAssignmentBehavior> for NoAssignmentBehaviorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NoAssignmentBehavior {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for NoAssignmentBehavior {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NoAssignmentBehaviorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NoAssignmentBehaviorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NoAssignmentBehaviorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NoAssignmentBehavior>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NoAssignmentBehaviorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NoAssignmentBehaviorMut<'msg> {
  type Message = NoAssignmentBehavior;
}

impl ::std::fmt::Debug for NoAssignmentBehaviorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NoAssignmentBehavior>> for NoAssignmentBehaviorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NoAssignmentBehavior>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NoAssignmentBehaviorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NoAssignmentBehavior> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> NoAssignmentBehavior {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // fallback_rate_limit: optional message envoy.type.v3.RateLimitStrategy
  pub fn has_fallback_rate_limit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_fallback_rate_limit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn fallback_rate_limit_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_>> {
    self.has_fallback_rate_limit().then(|| self.fallback_rate_limit())
  }
  pub fn fallback_rate_limit(&self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView::default())
  }
  pub fn fallback_rate_limit_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyMut<'_> {
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
  pub fn set_fallback_rate_limit(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  pub fn no_assignment_behavior(&self) -> super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorOneof<'_> {
    match &self.no_assignment_behavior_case() {
      super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorCase::FallbackRateLimit =>
          super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorOneof::FallbackRateLimit(self.fallback_rate_limit()),
      _ => super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn no_assignment_behavior_case(&self) -> super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `NoAssignmentBehaviorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for NoAssignmentBehaviorMut<'_> {}

// SAFETY:
// - `NoAssignmentBehaviorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for NoAssignmentBehaviorMut<'_> {}

impl<'msg> ::protobuf::AsView for NoAssignmentBehaviorMut<'msg> {
  type Proxied = NoAssignmentBehavior;
  fn as_view(&self) -> ::protobuf::View<'_, NoAssignmentBehavior> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NoAssignmentBehaviorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NoAssignmentBehavior>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for NoAssignmentBehaviorMut<'msg> {
  type MutProxied = NoAssignmentBehavior;
  fn as_mut(&mut self) -> NoAssignmentBehaviorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NoAssignmentBehaviorMut<'msg> {
  fn into_mut<'shorter>(self) -> NoAssignmentBehaviorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NoAssignmentBehavior {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NoAssignmentBehavior> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NoAssignmentBehaviorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NoAssignmentBehaviorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // fallback_rate_limit: optional message envoy.type.v3.RateLimitStrategy
  pub fn has_fallback_rate_limit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_fallback_rate_limit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn fallback_rate_limit_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_>> {
    self.has_fallback_rate_limit().then(|| self.fallback_rate_limit())
  }
  pub fn fallback_rate_limit(&self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView::default())
  }
  pub fn fallback_rate_limit_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyMut<'_> {
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
  pub fn set_fallback_rate_limit(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  pub fn no_assignment_behavior(&self) -> super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorOneof<'_> {
    match &self.no_assignment_behavior_case() {
      super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorCase::FallbackRateLimit =>
          super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorOneof::FallbackRateLimit(self.fallback_rate_limit()),
      _ => super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn no_assignment_behavior_case(&self) -> super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehaviorCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl NoAssignmentBehavior

impl ::std::ops::Drop for NoAssignmentBehavior {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NoAssignmentBehavior {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NoAssignmentBehavior {
  type Proxied = Self;
  fn as_view(&self) -> NoAssignmentBehaviorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NoAssignmentBehavior {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NoAssignmentBehaviorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NoAssignmentBehavior {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__NoAssignmentBehavior_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3^!");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__NoAssignmentBehavior_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__NoAssignmentBehavior_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NoAssignmentBehavior {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NoAssignmentBehavior {
  type Msg = NoAssignmentBehavior;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NoAssignmentBehavior> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NoAssignmentBehavior {
  type Msg = NoAssignmentBehavior;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NoAssignmentBehavior> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NoAssignmentBehaviorMut<'_> {
  type Msg = NoAssignmentBehavior;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NoAssignmentBehavior> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NoAssignmentBehaviorMut<'_> {
  type Msg = NoAssignmentBehavior;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NoAssignmentBehavior> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NoAssignmentBehaviorView<'_> {
  type Msg = NoAssignmentBehavior;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NoAssignmentBehavior> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NoAssignmentBehaviorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod no_assignment_behavior {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum NoAssignmentBehaviorOneof<'msg> {
  FallbackRateLimit(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategy>) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum NoAssignmentBehaviorCase {
  FallbackRateLimit = 1,

  not_set = 0
}

impl NoAssignmentBehaviorCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<NoAssignmentBehaviorCase> {
    match v {
      0 => Some(NoAssignmentBehaviorCase::not_set),
      1 => Some(NoAssignmentBehaviorCase::FallbackRateLimit),
      _ => None
    }
  }
}
}  // pub mod no_assignment_behavior

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__ExpiredAssignmentBehavior_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExpiredAssignmentBehavior {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExpiredAssignmentBehavior>
}

impl ::protobuf::Message for ExpiredAssignmentBehavior {
  type MessageView<'msg> = ExpiredAssignmentBehaviorView<'msg>;
  type MessageMut<'msg> = ExpiredAssignmentBehaviorMut<'msg>;
}

impl ::std::default::Default for ExpiredAssignmentBehavior {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExpiredAssignmentBehavior {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExpiredAssignmentBehavior` is `Sync` because it does not implement interior mutability.
//    Neither does `ExpiredAssignmentBehaviorMut`.
unsafe impl ::std::marker::Sync for ExpiredAssignmentBehavior {}

// SAFETY:
// - `ExpiredAssignmentBehavior` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExpiredAssignmentBehavior {}

impl ::protobuf::Proxied for ExpiredAssignmentBehavior {
  type View<'msg> = ExpiredAssignmentBehaviorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExpiredAssignmentBehavior {}

impl ::protobuf::MutProxied for ExpiredAssignmentBehavior {
  type Mut<'msg> = ExpiredAssignmentBehaviorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExpiredAssignmentBehaviorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExpiredAssignmentBehavior>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExpiredAssignmentBehaviorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExpiredAssignmentBehaviorView<'msg> {
  type Message = ExpiredAssignmentBehavior;
}

impl ::std::fmt::Debug for ExpiredAssignmentBehaviorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExpiredAssignmentBehaviorView<'_> {
  fn default() -> ExpiredAssignmentBehaviorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExpiredAssignmentBehavior>> for ExpiredAssignmentBehaviorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExpiredAssignmentBehavior>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExpiredAssignmentBehaviorView<'msg> {

  pub fn to_owned(&self) -> ExpiredAssignmentBehavior {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // expired_assignment_behavior_timeout: optional message google.protobuf.Duration
  pub fn has_expired_assignment_behavior_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn expired_assignment_behavior_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_expired_assignment_behavior_timeout().then(|| self.expired_assignment_behavior_timeout())
  }
  pub fn expired_assignment_behavior_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // fallback_rate_limit: optional message envoy.type.v3.RateLimitStrategy
  pub fn has_fallback_rate_limit(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn fallback_rate_limit_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'msg>> {
    self.has_fallback_rate_limit().then(|| self.fallback_rate_limit())
  }
  pub fn fallback_rate_limit(self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView::default())
  }

  // reuse_last_assignment: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior.ReuseLastAssignment
  pub fn has_reuse_last_assignment(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn reuse_last_assignment_opt(self) -> ::std::option::Option<super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignmentView<'msg>> {
    self.has_reuse_last_assignment().then(|| self.reuse_last_assignment())
  }
  pub fn reuse_last_assignment(self) -> super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignmentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignmentView::default())
  }

  pub fn expired_assignment_behavior(self) -> super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof<'msg> {
    match self.expired_assignment_behavior_case() {
      super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase::FallbackRateLimit =>
          super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof::FallbackRateLimit(self.fallback_rate_limit()),
      super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase::ReuseLastAssignment =>
          super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof::ReuseLastAssignment(self.reuse_last_assignment()),
      _ => super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn expired_assignment_behavior_case(self) -> super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExpiredAssignmentBehaviorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExpiredAssignmentBehaviorView<'_> {}

// SAFETY:
// - `ExpiredAssignmentBehaviorView` is `Send` because while its alive a `ExpiredAssignmentBehaviorMut` cannot.
// - `ExpiredAssignmentBehaviorView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExpiredAssignmentBehaviorView<'_> {}

impl<'msg> ::protobuf::AsView for ExpiredAssignmentBehaviorView<'msg> {
  type Proxied = ExpiredAssignmentBehavior;
  fn as_view(&self) -> ::protobuf::View<'msg, ExpiredAssignmentBehavior> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExpiredAssignmentBehaviorView<'msg> {
  fn into_view<'shorter>(self) -> ExpiredAssignmentBehaviorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExpiredAssignmentBehavior> for ExpiredAssignmentBehaviorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExpiredAssignmentBehavior {
    let mut dst = ExpiredAssignmentBehavior::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExpiredAssignmentBehavior> for ExpiredAssignmentBehaviorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExpiredAssignmentBehavior {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExpiredAssignmentBehavior {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExpiredAssignmentBehaviorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExpiredAssignmentBehaviorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExpiredAssignmentBehaviorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpiredAssignmentBehavior>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExpiredAssignmentBehaviorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExpiredAssignmentBehaviorMut<'msg> {
  type Message = ExpiredAssignmentBehavior;
}

impl ::std::fmt::Debug for ExpiredAssignmentBehaviorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExpiredAssignmentBehavior>> for ExpiredAssignmentBehaviorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpiredAssignmentBehavior>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExpiredAssignmentBehaviorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpiredAssignmentBehavior> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExpiredAssignmentBehavior {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // expired_assignment_behavior_timeout: optional message google.protobuf.Duration
  pub fn has_expired_assignment_behavior_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_expired_assignment_behavior_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn expired_assignment_behavior_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_expired_assignment_behavior_timeout().then(|| self.expired_assignment_behavior_timeout())
  }
  pub fn expired_assignment_behavior_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn expired_assignment_behavior_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_expired_assignment_behavior_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // fallback_rate_limit: optional message envoy.type.v3.RateLimitStrategy
  pub fn has_fallback_rate_limit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_fallback_rate_limit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn fallback_rate_limit_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_>> {
    self.has_fallback_rate_limit().then(|| self.fallback_rate_limit())
  }
  pub fn fallback_rate_limit(&self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView::default())
  }
  pub fn fallback_rate_limit_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyMut<'_> {
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
  pub fn set_fallback_rate_limit(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // reuse_last_assignment: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior.ReuseLastAssignment
  pub fn has_reuse_last_assignment(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_reuse_last_assignment(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn reuse_last_assignment_opt(&self) -> ::std::option::Option<super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignmentView<'_>> {
    self.has_reuse_last_assignment().then(|| self.reuse_last_assignment())
  }
  pub fn reuse_last_assignment(&self) -> super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignmentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignmentView::default())
  }
  pub fn reuse_last_assignment_mut(&mut self) -> super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignmentMut<'_> {
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
  pub fn set_reuse_last_assignment(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignment>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn expired_assignment_behavior(&self) -> super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof<'_> {
    match &self.expired_assignment_behavior_case() {
      super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase::FallbackRateLimit =>
          super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof::FallbackRateLimit(self.fallback_rate_limit()),
      super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase::ReuseLastAssignment =>
          super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof::ReuseLastAssignment(self.reuse_last_assignment()),
      _ => super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn expired_assignment_behavior_case(&self) -> super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExpiredAssignmentBehaviorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExpiredAssignmentBehaviorMut<'_> {}

// SAFETY:
// - `ExpiredAssignmentBehaviorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExpiredAssignmentBehaviorMut<'_> {}

impl<'msg> ::protobuf::AsView for ExpiredAssignmentBehaviorMut<'msg> {
  type Proxied = ExpiredAssignmentBehavior;
  fn as_view(&self) -> ::protobuf::View<'_, ExpiredAssignmentBehavior> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExpiredAssignmentBehaviorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExpiredAssignmentBehavior>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExpiredAssignmentBehaviorMut<'msg> {
  type MutProxied = ExpiredAssignmentBehavior;
  fn as_mut(&mut self) -> ExpiredAssignmentBehaviorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExpiredAssignmentBehaviorMut<'msg> {
  fn into_mut<'shorter>(self) -> ExpiredAssignmentBehaviorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExpiredAssignmentBehavior {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExpiredAssignmentBehavior> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExpiredAssignmentBehaviorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExpiredAssignmentBehaviorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // expired_assignment_behavior_timeout: optional message google.protobuf.Duration
  pub fn has_expired_assignment_behavior_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_expired_assignment_behavior_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn expired_assignment_behavior_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_expired_assignment_behavior_timeout().then(|| self.expired_assignment_behavior_timeout())
  }
  pub fn expired_assignment_behavior_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn expired_assignment_behavior_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_expired_assignment_behavior_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // fallback_rate_limit: optional message envoy.type.v3.RateLimitStrategy
  pub fn has_fallback_rate_limit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_fallback_rate_limit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn fallback_rate_limit_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_>> {
    self.has_fallback_rate_limit().then(|| self.fallback_rate_limit())
  }
  pub fn fallback_rate_limit(&self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyView::default())
  }
  pub fn fallback_rate_limit_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategyMut<'_> {
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
  pub fn set_fallback_rate_limit(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // reuse_last_assignment: optional message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior.ReuseLastAssignment
  pub fn has_reuse_last_assignment(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_reuse_last_assignment(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn reuse_last_assignment_opt(&self) -> ::std::option::Option<super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignmentView<'_>> {
    self.has_reuse_last_assignment().then(|| self.reuse_last_assignment())
  }
  pub fn reuse_last_assignment(&self) -> super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignmentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignmentView::default())
  }
  pub fn reuse_last_assignment_mut(&mut self) -> super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignmentMut<'_> {
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
  pub fn set_reuse_last_assignment(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignment>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn expired_assignment_behavior(&self) -> super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof<'_> {
    match &self.expired_assignment_behavior_case() {
      super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase::FallbackRateLimit =>
          super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof::FallbackRateLimit(self.fallback_rate_limit()),
      super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase::ReuseLastAssignment =>
          super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof::ReuseLastAssignment(self.reuse_last_assignment()),
      _ => super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn expired_assignment_behavior_case(&self) -> super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehaviorCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ExpiredAssignmentBehavior

impl ::std::ops::Drop for ExpiredAssignmentBehavior {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExpiredAssignmentBehavior {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExpiredAssignmentBehavior {
  type Proxied = Self;
  fn as_view(&self) -> ExpiredAssignmentBehaviorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExpiredAssignmentBehavior {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExpiredAssignmentBehaviorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExpiredAssignmentBehavior {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__ExpiredAssignmentBehavior_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__ExpiredAssignmentBehavior_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignment as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__ExpiredAssignmentBehavior_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExpiredAssignmentBehavior {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExpiredAssignmentBehavior {
  type Msg = ExpiredAssignmentBehavior;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpiredAssignmentBehavior> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpiredAssignmentBehavior {
  type Msg = ExpiredAssignmentBehavior;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpiredAssignmentBehavior> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExpiredAssignmentBehaviorMut<'_> {
  type Msg = ExpiredAssignmentBehavior;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpiredAssignmentBehavior> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpiredAssignmentBehaviorMut<'_> {
  type Msg = ExpiredAssignmentBehavior;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpiredAssignmentBehavior> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpiredAssignmentBehaviorView<'_> {
  type Msg = ExpiredAssignmentBehavior;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpiredAssignmentBehavior> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExpiredAssignmentBehaviorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod expired_assignment_behavior {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__ExpiredAssignmentBehavior__ReuseLastAssignment_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ReuseLastAssignment {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ReuseLastAssignment>
}

impl ::protobuf::Message for ReuseLastAssignment {
  type MessageView<'msg> = ReuseLastAssignmentView<'msg>;
  type MessageMut<'msg> = ReuseLastAssignmentMut<'msg>;
}

impl ::std::default::Default for ReuseLastAssignment {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ReuseLastAssignment {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ReuseLastAssignment` is `Sync` because it does not implement interior mutability.
//    Neither does `ReuseLastAssignmentMut`.
unsafe impl ::std::marker::Sync for ReuseLastAssignment {}

// SAFETY:
// - `ReuseLastAssignment` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ReuseLastAssignment {}

impl ::protobuf::Proxied for ReuseLastAssignment {
  type View<'msg> = ReuseLastAssignmentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ReuseLastAssignment {}

impl ::protobuf::MutProxied for ReuseLastAssignment {
  type Mut<'msg> = ReuseLastAssignmentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReuseLastAssignmentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ReuseLastAssignment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReuseLastAssignmentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ReuseLastAssignmentView<'msg> {
  type Message = ReuseLastAssignment;
}

impl ::std::fmt::Debug for ReuseLastAssignmentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ReuseLastAssignmentView<'_> {
  fn default() -> ReuseLastAssignmentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ReuseLastAssignment>> for ReuseLastAssignmentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ReuseLastAssignment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReuseLastAssignmentView<'msg> {

  pub fn to_owned(&self) -> ReuseLastAssignment {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ReuseLastAssignmentView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ReuseLastAssignmentView<'_> {}

// SAFETY:
// - `ReuseLastAssignmentView` is `Send` because while its alive a `ReuseLastAssignmentMut` cannot.
// - `ReuseLastAssignmentView` does not use thread-local data.
unsafe impl ::std::marker::Send for ReuseLastAssignmentView<'_> {}

impl<'msg> ::protobuf::AsView for ReuseLastAssignmentView<'msg> {
  type Proxied = ReuseLastAssignment;
  fn as_view(&self) -> ::protobuf::View<'msg, ReuseLastAssignment> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReuseLastAssignmentView<'msg> {
  fn into_view<'shorter>(self) -> ReuseLastAssignmentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ReuseLastAssignment> for ReuseLastAssignmentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReuseLastAssignment {
    let mut dst = ReuseLastAssignment::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ReuseLastAssignment> for ReuseLastAssignmentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReuseLastAssignment {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ReuseLastAssignment {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ReuseLastAssignmentView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ReuseLastAssignmentMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReuseLastAssignmentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ReuseLastAssignment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReuseLastAssignmentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ReuseLastAssignmentMut<'msg> {
  type Message = ReuseLastAssignment;
}

impl ::std::fmt::Debug for ReuseLastAssignmentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ReuseLastAssignment>> for ReuseLastAssignmentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ReuseLastAssignment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReuseLastAssignmentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ReuseLastAssignment> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ReuseLastAssignment {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ReuseLastAssignmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ReuseLastAssignmentMut<'_> {}

// SAFETY:
// - `ReuseLastAssignmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ReuseLastAssignmentMut<'_> {}

impl<'msg> ::protobuf::AsView for ReuseLastAssignmentMut<'msg> {
  type Proxied = ReuseLastAssignment;
  fn as_view(&self) -> ::protobuf::View<'_, ReuseLastAssignment> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReuseLastAssignmentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ReuseLastAssignment>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ReuseLastAssignmentMut<'msg> {
  type MutProxied = ReuseLastAssignment;
  fn as_mut(&mut self) -> ReuseLastAssignmentMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ReuseLastAssignmentMut<'msg> {
  fn into_mut<'shorter>(self) -> ReuseLastAssignmentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ReuseLastAssignment {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ReuseLastAssignment> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ReuseLastAssignmentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ReuseLastAssignmentMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl ReuseLastAssignment

impl ::std::ops::Drop for ReuseLastAssignment {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ReuseLastAssignment {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ReuseLastAssignment {
  type Proxied = Self;
  fn as_view(&self) -> ReuseLastAssignmentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ReuseLastAssignment {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ReuseLastAssignmentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ReuseLastAssignment {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__ExpiredAssignmentBehavior__ReuseLastAssignment_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__ExpiredAssignmentBehavior__ReuseLastAssignment_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__ExpiredAssignmentBehavior__ReuseLastAssignment_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ReuseLastAssignment {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ReuseLastAssignment {
  type Msg = ReuseLastAssignment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReuseLastAssignment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReuseLastAssignment {
  type Msg = ReuseLastAssignment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReuseLastAssignment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ReuseLastAssignmentMut<'_> {
  type Msg = ReuseLastAssignment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReuseLastAssignment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReuseLastAssignmentMut<'_> {
  type Msg = ReuseLastAssignment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReuseLastAssignment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReuseLastAssignmentView<'_> {
  type Msg = ReuseLastAssignment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReuseLastAssignment> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ReuseLastAssignmentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ExpiredAssignmentBehaviorOneof<'msg> {
  FallbackRateLimit(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::v3::ratelimit_strategy::RateLimitStrategy>) = 2,
  ReuseLastAssignment(::protobuf::View<'msg, super::super::super::rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignment>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ExpiredAssignmentBehaviorCase {
  FallbackRateLimit = 2,
  ReuseLastAssignment = 3,

  not_set = 0
}

impl ExpiredAssignmentBehaviorCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ExpiredAssignmentBehaviorCase> {
    match v {
      0 => Some(ExpiredAssignmentBehaviorCase::not_set),
      2 => Some(ExpiredAssignmentBehaviorCase::FallbackRateLimit),
      3 => Some(ExpiredAssignmentBehaviorCase::ReuseLastAssignment),
      _ => None
    }
  }
}
}  // pub mod expired_assignment_behavior

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__DenyResponseSettings_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DenyResponseSettings {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DenyResponseSettings>
}

impl ::protobuf::Message for DenyResponseSettings {
  type MessageView<'msg> = DenyResponseSettingsView<'msg>;
  type MessageMut<'msg> = DenyResponseSettingsMut<'msg>;
}

impl ::std::default::Default for DenyResponseSettings {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DenyResponseSettings {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DenyResponseSettings` is `Sync` because it does not implement interior mutability.
//    Neither does `DenyResponseSettingsMut`.
unsafe impl ::std::marker::Sync for DenyResponseSettings {}

// SAFETY:
// - `DenyResponseSettings` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DenyResponseSettings {}

impl ::protobuf::Proxied for DenyResponseSettings {
  type View<'msg> = DenyResponseSettingsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DenyResponseSettings {}

impl ::protobuf::MutProxied for DenyResponseSettings {
  type Mut<'msg> = DenyResponseSettingsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DenyResponseSettingsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DenyResponseSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DenyResponseSettingsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DenyResponseSettingsView<'msg> {
  type Message = DenyResponseSettings;
}

impl ::std::fmt::Debug for DenyResponseSettingsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DenyResponseSettingsView<'_> {
  fn default() -> DenyResponseSettingsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DenyResponseSettings>> for DenyResponseSettingsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DenyResponseSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DenyResponseSettingsView<'msg> {

  pub fn to_owned(&self) -> DenyResponseSettings {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // http_status: optional message envoy.type.v3.HttpStatus
  pub fn has_http_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn http_status_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'msg>> {
    self.has_http_status().then(|| self.http_status())
  }
  pub fn http_status(self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }

  // http_body: optional message google.protobuf.BytesValue
  pub fn has_http_body(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn http_body_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BytesValueView<'msg>> {
    self.has_http_body().then(|| self.http_body())
  }
  pub fn http_body(self) -> ::protobuf_well_known_types::BytesValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BytesValueView::default())
  }

  // grpc_status: optional message google.rpc.Status
  pub fn has_grpc_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn grpc_status_opt(self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'msg>> {
    self.has_grpc_status().then(|| self.grpc_status())
  }
  pub fn grpc_status(self) -> crate::xds::generated::google::rpc::status::StatusView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }

  // response_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn response_headers_to_add(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `DenyResponseSettingsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DenyResponseSettingsView<'_> {}

// SAFETY:
// - `DenyResponseSettingsView` is `Send` because while its alive a `DenyResponseSettingsMut` cannot.
// - `DenyResponseSettingsView` does not use thread-local data.
unsafe impl ::std::marker::Send for DenyResponseSettingsView<'_> {}

impl<'msg> ::protobuf::AsView for DenyResponseSettingsView<'msg> {
  type Proxied = DenyResponseSettings;
  fn as_view(&self) -> ::protobuf::View<'msg, DenyResponseSettings> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DenyResponseSettingsView<'msg> {
  fn into_view<'shorter>(self) -> DenyResponseSettingsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DenyResponseSettings> for DenyResponseSettingsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DenyResponseSettings {
    let mut dst = DenyResponseSettings::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DenyResponseSettings> for DenyResponseSettingsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DenyResponseSettings {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DenyResponseSettings {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DenyResponseSettingsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DenyResponseSettingsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DenyResponseSettingsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DenyResponseSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DenyResponseSettingsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DenyResponseSettingsMut<'msg> {
  type Message = DenyResponseSettings;
}

impl ::std::fmt::Debug for DenyResponseSettingsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DenyResponseSettings>> for DenyResponseSettingsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DenyResponseSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DenyResponseSettingsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DenyResponseSettings> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DenyResponseSettings {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // http_status: optional message envoy.type.v3.HttpStatus
  pub fn has_http_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_status_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_>> {
    self.has_http_status().then(|| self.http_status())
  }
  pub fn http_status(&self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }
  pub fn http_status_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusMut<'_> {
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
  pub fn set_http_status(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // http_body: optional message google.protobuf.BytesValue
  pub fn has_http_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_http_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn http_body_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BytesValueView<'_>> {
    self.has_http_body().then(|| self.http_body())
  }
  pub fn http_body(&self) -> ::protobuf_well_known_types::BytesValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BytesValueView::default())
  }
  pub fn http_body_mut(&mut self) -> ::protobuf_well_known_types::BytesValueMut<'_> {
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
  pub fn set_http_body(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BytesValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // grpc_status: optional message google.rpc.Status
  pub fn has_grpc_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_grpc_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn grpc_status_opt(&self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'_>> {
    self.has_grpc_status().then(|| self.grpc_status())
  }
  pub fn grpc_status(&self) -> crate::xds::generated::google::rpc::status::StatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }
  pub fn grpc_status_mut(&mut self) -> crate::xds::generated::google::rpc::status::StatusMut<'_> {
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
  pub fn set_grpc_status(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::rpc::status::Status>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // response_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn response_headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn response_headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_response_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `DenyResponseSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DenyResponseSettingsMut<'_> {}

// SAFETY:
// - `DenyResponseSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DenyResponseSettingsMut<'_> {}

impl<'msg> ::protobuf::AsView for DenyResponseSettingsMut<'msg> {
  type Proxied = DenyResponseSettings;
  fn as_view(&self) -> ::protobuf::View<'_, DenyResponseSettings> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DenyResponseSettingsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DenyResponseSettings>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DenyResponseSettingsMut<'msg> {
  type MutProxied = DenyResponseSettings;
  fn as_mut(&mut self) -> DenyResponseSettingsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DenyResponseSettingsMut<'msg> {
  fn into_mut<'shorter>(self) -> DenyResponseSettingsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DenyResponseSettings {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DenyResponseSettings> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DenyResponseSettingsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DenyResponseSettingsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // http_status: optional message envoy.type.v3.HttpStatus
  pub fn has_http_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_status_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_>> {
    self.has_http_status().then(|| self.http_status())
  }
  pub fn http_status(&self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }
  pub fn http_status_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusMut<'_> {
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
  pub fn set_http_status(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // http_body: optional message google.protobuf.BytesValue
  pub fn has_http_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_http_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn http_body_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BytesValueView<'_>> {
    self.has_http_body().then(|| self.http_body())
  }
  pub fn http_body(&self) -> ::protobuf_well_known_types::BytesValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BytesValueView::default())
  }
  pub fn http_body_mut(&mut self) -> ::protobuf_well_known_types::BytesValueMut<'_> {
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
  pub fn set_http_body(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BytesValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // grpc_status: optional message google.rpc.Status
  pub fn has_grpc_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_grpc_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn grpc_status_opt(&self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'_>> {
    self.has_grpc_status().then(|| self.grpc_status())
  }
  pub fn grpc_status(&self) -> crate::xds::generated::google::rpc::status::StatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }
  pub fn grpc_status_mut(&mut self) -> crate::xds::generated::google::rpc::status::StatusMut<'_> {
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
  pub fn set_grpc_status(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::rpc::status::Status>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // response_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn response_headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn response_headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_response_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl DenyResponseSettings

impl ::std::ops::Drop for DenyResponseSettings {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DenyResponseSettings {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DenyResponseSettings {
  type Proxied = Self;
  fn as_view(&self) -> DenyResponseSettingsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DenyResponseSettings {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DenyResponseSettingsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DenyResponseSettings {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__DenyResponseSettings_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__DenyResponseSettings_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BytesValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::google::rpc::status::Status as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__DenyResponseSettings_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DenyResponseSettings {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DenyResponseSettings {
  type Msg = DenyResponseSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DenyResponseSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DenyResponseSettings {
  type Msg = DenyResponseSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DenyResponseSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DenyResponseSettingsMut<'_> {
  type Msg = DenyResponseSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DenyResponseSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DenyResponseSettingsMut<'_> {
  type Msg = DenyResponseSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DenyResponseSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DenyResponseSettingsView<'_> {
  type Msg = DenyResponseSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DenyResponseSettings> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DenyResponseSettingsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BucketIdBuilder {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BucketIdBuilder>
}

impl ::protobuf::Message for BucketIdBuilder {
  type MessageView<'msg> = BucketIdBuilderView<'msg>;
  type MessageMut<'msg> = BucketIdBuilderMut<'msg>;
}

impl ::std::default::Default for BucketIdBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BucketIdBuilder {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BucketIdBuilder` is `Sync` because it does not implement interior mutability.
//    Neither does `BucketIdBuilderMut`.
unsafe impl ::std::marker::Sync for BucketIdBuilder {}

// SAFETY:
// - `BucketIdBuilder` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BucketIdBuilder {}

impl ::protobuf::Proxied for BucketIdBuilder {
  type View<'msg> = BucketIdBuilderView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BucketIdBuilder {}

impl ::protobuf::MutProxied for BucketIdBuilder {
  type Mut<'msg> = BucketIdBuilderMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BucketIdBuilderView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BucketIdBuilder>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BucketIdBuilderView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BucketIdBuilderView<'msg> {
  type Message = BucketIdBuilder;
}

impl ::std::fmt::Debug for BucketIdBuilderView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BucketIdBuilderView<'_> {
  fn default() -> BucketIdBuilderView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BucketIdBuilder>> for BucketIdBuilderView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BucketIdBuilder>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BucketIdBuilderView<'msg> {

  pub fn to_owned(&self) -> BucketIdBuilder {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bucket_id_builder: repeated message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder.BucketIdBuilderEntry
  pub fn bucket_id_builder(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, super::super::rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::super::rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `BucketIdBuilderView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BucketIdBuilderView<'_> {}

// SAFETY:
// - `BucketIdBuilderView` is `Send` because while its alive a `BucketIdBuilderMut` cannot.
// - `BucketIdBuilderView` does not use thread-local data.
unsafe impl ::std::marker::Send for BucketIdBuilderView<'_> {}

impl<'msg> ::protobuf::AsView for BucketIdBuilderView<'msg> {
  type Proxied = BucketIdBuilder;
  fn as_view(&self) -> ::protobuf::View<'msg, BucketIdBuilder> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BucketIdBuilderView<'msg> {
  fn into_view<'shorter>(self) -> BucketIdBuilderView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BucketIdBuilder> for BucketIdBuilderView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BucketIdBuilder {
    let mut dst = BucketIdBuilder::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BucketIdBuilder> for BucketIdBuilderMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BucketIdBuilder {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BucketIdBuilder {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BucketIdBuilderView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BucketIdBuilderMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BucketIdBuilderMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketIdBuilder>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BucketIdBuilderMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BucketIdBuilderMut<'msg> {
  type Message = BucketIdBuilder;
}

impl ::std::fmt::Debug for BucketIdBuilderMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BucketIdBuilder>> for BucketIdBuilderMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketIdBuilder>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BucketIdBuilderMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BucketIdBuilder> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BucketIdBuilder {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bucket_id_builder: repeated message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder.BucketIdBuilderEntry
  pub fn bucket_id_builder(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::super::rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::super::rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn bucket_id_builder_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::super::rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_bucket_id_builder(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::super::rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `BucketIdBuilderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BucketIdBuilderMut<'_> {}

// SAFETY:
// - `BucketIdBuilderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BucketIdBuilderMut<'_> {}

impl<'msg> ::protobuf::AsView for BucketIdBuilderMut<'msg> {
  type Proxied = BucketIdBuilder;
  fn as_view(&self) -> ::protobuf::View<'_, BucketIdBuilder> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BucketIdBuilderMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BucketIdBuilder>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BucketIdBuilderMut<'msg> {
  type MutProxied = BucketIdBuilder;
  fn as_mut(&mut self) -> BucketIdBuilderMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BucketIdBuilderMut<'msg> {
  fn into_mut<'shorter>(self) -> BucketIdBuilderMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BucketIdBuilder {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BucketIdBuilder> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BucketIdBuilderView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BucketIdBuilderMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bucket_id_builder: repeated message envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder.BucketIdBuilderEntry
  pub fn bucket_id_builder(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::super::rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::super::rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn bucket_id_builder_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::super::rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_bucket_id_builder(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::super::rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl BucketIdBuilder

impl ::std::ops::Drop for BucketIdBuilder {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BucketIdBuilder {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BucketIdBuilder {
  type Proxied = Self;
  fn as_view(&self) -> BucketIdBuilderView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BucketIdBuilder {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BucketIdBuilderMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BucketIdBuilder {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder_msg_init.0, &[<super::super::rate_limit_quota_bucket_settings::bucket_id_builder::BucketIdBuilderEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::rate_limit_quota_bucket_settings::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BucketIdBuilder {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BucketIdBuilder {
  type Msg = BucketIdBuilder;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketIdBuilder> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketIdBuilder {
  type Msg = BucketIdBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketIdBuilder> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BucketIdBuilderMut<'_> {
  type Msg = BucketIdBuilder;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketIdBuilder> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketIdBuilderMut<'_> {
  type Msg = BucketIdBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketIdBuilder> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BucketIdBuilderView<'_> {
  type Msg = BucketIdBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BucketIdBuilder> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BucketIdBuilderMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod bucket_id_builder {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder__ValueBuilder_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ValueBuilder {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ValueBuilder>
}

impl ::protobuf::Message for ValueBuilder {
  type MessageView<'msg> = ValueBuilderView<'msg>;
  type MessageMut<'msg> = ValueBuilderMut<'msg>;
}

impl ::std::default::Default for ValueBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ValueBuilder {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ValueBuilder` is `Sync` because it does not implement interior mutability.
//    Neither does `ValueBuilderMut`.
unsafe impl ::std::marker::Sync for ValueBuilder {}

// SAFETY:
// - `ValueBuilder` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ValueBuilder {}

impl ::protobuf::Proxied for ValueBuilder {
  type View<'msg> = ValueBuilderView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ValueBuilder {}

impl ::protobuf::MutProxied for ValueBuilder {
  type Mut<'msg> = ValueBuilderMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ValueBuilderView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValueBuilder>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValueBuilderView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ValueBuilderView<'msg> {
  type Message = ValueBuilder;
}

impl ::std::fmt::Debug for ValueBuilderView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ValueBuilderView<'_> {
  fn default() -> ValueBuilderView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ValueBuilder>> for ValueBuilderView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValueBuilder>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValueBuilderView<'msg> {

  pub fn to_owned(&self) -> ValueBuilder {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // string_value: optional string
  pub fn has_string_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn string_value_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_string_value().then(|| self.string_value())
  }
  pub fn string_value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // custom_value: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn custom_value_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_custom_value().then(|| self.custom_value())
  }
  pub fn custom_value(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  pub fn value_specifier(self) -> super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof<'msg> {
    match self.value_specifier_case() {
      super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase::StringValue =>
          super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof::StringValue(self.string_value()),
      super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase::CustomValue =>
          super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof::CustomValue(self.custom_value()),
      _ => super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_specifier_case(self) -> super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ValueBuilderView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ValueBuilderView<'_> {}

// SAFETY:
// - `ValueBuilderView` is `Send` because while its alive a `ValueBuilderMut` cannot.
// - `ValueBuilderView` does not use thread-local data.
unsafe impl ::std::marker::Send for ValueBuilderView<'_> {}

impl<'msg> ::protobuf::AsView for ValueBuilderView<'msg> {
  type Proxied = ValueBuilder;
  fn as_view(&self) -> ::protobuf::View<'msg, ValueBuilder> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValueBuilderView<'msg> {
  fn into_view<'shorter>(self) -> ValueBuilderView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ValueBuilder> for ValueBuilderView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValueBuilder {
    let mut dst = ValueBuilder::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ValueBuilder> for ValueBuilderMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValueBuilder {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ValueBuilder {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ValueBuilderView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ValueBuilderMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ValueBuilderMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValueBuilder>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValueBuilderMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ValueBuilderMut<'msg> {
  type Message = ValueBuilder;
}

impl ::std::fmt::Debug for ValueBuilderMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ValueBuilder>> for ValueBuilderMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValueBuilder>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValueBuilderMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ValueBuilder> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ValueBuilder {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // string_value: optional string
  pub fn has_string_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_string_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn string_value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_string_value().then(|| self.string_value())
  }
  pub fn string_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_string_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // custom_value: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_custom_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn custom_value_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_value().then(|| self.custom_value())
  }
  pub fn custom_value(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_value_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn value_specifier(&self) -> super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof<'_> {
    match &self.value_specifier_case() {
      super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase::StringValue =>
          super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof::StringValue(self.string_value()),
      super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase::CustomValue =>
          super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof::CustomValue(self.custom_value()),
      _ => super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_specifier_case(&self) -> super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ValueBuilderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ValueBuilderMut<'_> {}

// SAFETY:
// - `ValueBuilderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ValueBuilderMut<'_> {}

impl<'msg> ::protobuf::AsView for ValueBuilderMut<'msg> {
  type Proxied = ValueBuilder;
  fn as_view(&self) -> ::protobuf::View<'_, ValueBuilder> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValueBuilderMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ValueBuilder>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ValueBuilderMut<'msg> {
  type MutProxied = ValueBuilder;
  fn as_mut(&mut self) -> ValueBuilderMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ValueBuilderMut<'msg> {
  fn into_mut<'shorter>(self) -> ValueBuilderMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ValueBuilder {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ValueBuilder> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ValueBuilderView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ValueBuilderMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // string_value: optional string
  pub fn has_string_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_string_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn string_value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_string_value().then(|| self.string_value())
  }
  pub fn string_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_string_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // custom_value: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_custom_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn custom_value_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_value().then(|| self.custom_value())
  }
  pub fn custom_value(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_value_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn value_specifier(&self) -> super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof<'_> {
    match &self.value_specifier_case() {
      super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase::StringValue =>
          super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof::StringValue(self.string_value()),
      super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase::CustomValue =>
          super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof::CustomValue(self.custom_value()),
      _ => super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_specifier_case(&self) -> super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ValueBuilder

impl ::std::ops::Drop for ValueBuilder {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ValueBuilder {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ValueBuilder {
  type Proxied = Self;
  fn as_view(&self) -> ValueBuilderView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ValueBuilder {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ValueBuilderMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ValueBuilder {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder__ValueBuilder_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T3^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder__ValueBuilder_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder__ValueBuilder_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValueBuilder {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValueBuilder {
  type Msg = ValueBuilder;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValueBuilder> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValueBuilder {
  type Msg = ValueBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValueBuilder> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValueBuilderMut<'_> {
  type Msg = ValueBuilder;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValueBuilder> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValueBuilderMut<'_> {
  type Msg = ValueBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValueBuilder> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValueBuilderView<'_> {
  type Msg = ValueBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValueBuilder> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValueBuilderMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod value_builder {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ValueSpecifierOneof<'msg> {
  StringValue(&'msg ::protobuf::ProtoStr) = 1,
  CustomValue(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ValueSpecifierCase {
  StringValue = 1,
  CustomValue = 2,

  not_set = 0
}

impl ValueSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ValueSpecifierCase> {
    match v {
      0 => Some(ValueSpecifierCase::not_set),
      1 => Some(ValueSpecifierCase::StringValue),
      2 => Some(ValueSpecifierCase::CustomValue),
      _ => None
    }
  }
}
}  // pub mod value_builder

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder__BucketIdBuilderEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct BucketIdBuilderEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BucketIdBuilderEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder__BucketIdBuilderEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder__BucketIdBuilderEntry_msg_init.0, &[<super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::rate_limit_quota_bucket_settings::bucket_id_builder::envoy__extensions__filters__http__rate_0limit_0quota__v3__RateLimitQuotaBucketSettings__BucketIdBuilder__BucketIdBuilderEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod bucket_id_builder


}  // pub mod rate_limit_quota_bucket_settings


