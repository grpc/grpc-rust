const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0authz__v3__ExtAuthz_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExtAuthz {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExtAuthz>
}

impl ::protobuf::Message for ExtAuthz {
  type MessageView<'msg> = ExtAuthzView<'msg>;
  type MessageMut<'msg> = ExtAuthzMut<'msg>;
}

impl ::std::default::Default for ExtAuthz {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExtAuthz {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExtAuthz` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtAuthzMut`.
unsafe impl ::std::marker::Sync for ExtAuthz {}

// SAFETY:
// - `ExtAuthz` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExtAuthz {}

impl ::protobuf::Proxied for ExtAuthz {
  type View<'msg> = ExtAuthzView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExtAuthz {}

impl ::protobuf::MutProxied for ExtAuthz {
  type Mut<'msg> = ExtAuthzMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtAuthzView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtAuthz>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtAuthzView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtAuthzView<'msg> {
  type Message = ExtAuthz;
}

impl ::std::fmt::Debug for ExtAuthzView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtAuthzView<'_> {
  fn default() -> ExtAuthzView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExtAuthz>> for ExtAuthzView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtAuthz>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtAuthzView<'msg> {

  pub fn to_owned(&self) -> ExtAuthz {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // grpc_service: optional message envoy.config.core.v3.GrpcService
  pub fn has_grpc_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn grpc_service_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'msg>> {
    self.has_grpc_service().then(|| self.grpc_service())
  }
  pub fn grpc_service(self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }

  // http_service: optional message envoy.extensions.filters.http.ext_authz.v3.HttpService
  pub fn has_http_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn http_service_opt(self) -> ::std::option::Option<super::HttpServiceView<'msg>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(self) -> super::HttpServiceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpServiceView::default())
  }

  // transport_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn transport_api_version(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        10, (crate::xds::generated::envoy::config::core::v3::config_source::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }

  // failure_mode_allow: optional bool
  pub fn failure_mode_allow(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

  // failure_mode_allow_header_add: optional bool
  pub fn failure_mode_allow_header_add(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }

  // with_request_body: optional message envoy.extensions.filters.http.ext_authz.v3.BufferSettings
  pub fn has_with_request_body(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn with_request_body_opt(self) -> ::std::option::Option<super::BufferSettingsView<'msg>> {
    self.has_with_request_body().then(|| self.with_request_body())
  }
  pub fn with_request_body(self) -> super::BufferSettingsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BufferSettingsView::default())
  }

  // clear_route_cache: optional bool
  pub fn clear_route_cache(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }

  // status_on_error: optional message envoy.type.v3.HttpStatus
  pub fn has_status_on_error(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn status_on_error_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'msg>> {
    self.has_status_on_error().then(|| self.status_on_error())
  }
  pub fn status_on_error(self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }

  // validate_mutations: optional bool
  pub fn validate_mutations(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        22, (false).into()
      ).try_into().unwrap()
    }
  }

  // metadata_context_namespaces: repeated string
  pub fn metadata_context_namespaces(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // typed_metadata_context_namespaces: repeated string
  pub fn typed_metadata_context_namespaces(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        14
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // route_metadata_context_namespaces: repeated string
  pub fn route_metadata_context_namespaces(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        19
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // route_typed_metadata_context_namespaces: repeated string
  pub fn route_typed_metadata_context_namespaces(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // filter_enabled: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_filter_enabled(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn filter_enabled_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'msg>> {
    self.has_filter_enabled().then(|| self.filter_enabled())
  }
  pub fn filter_enabled(self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }

  // filter_enabled_metadata: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_filter_enabled_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn filter_enabled_metadata_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'msg>> {
    self.has_filter_enabled_metadata().then(|| self.filter_enabled_metadata())
  }
  pub fn filter_enabled_metadata(self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }

  // deny_at_disable: optional message envoy.config.core.v3.RuntimeFeatureFlag
  pub fn has_deny_at_disable(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn deny_at_disable_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlagView<'msg>> {
    self.has_deny_at_disable().then(|| self.deny_at_disable())
  }
  pub fn deny_at_disable(self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlagView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlagView::default())
  }

  // include_peer_certificate: optional bool
  pub fn include_peer_certificate(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // bootstrap_metadata_labels_key: optional string
  pub fn bootstrap_metadata_labels_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        13, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // allowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn allowed_headers_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg>> {
    self.has_allowed_headers().then(|| self.allowed_headers())
  }
  pub fn allowed_headers(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }

  // disallowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_disallowed_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn disallowed_headers_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg>> {
    self.has_disallowed_headers().then(|| self.disallowed_headers())
  }
  pub fn disallowed_headers(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }

  // include_tls_session: optional bool
  pub fn include_tls_session(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        16, (false).into()
      ).try_into().unwrap()
    }
  }

  // charge_cluster_response_stats: optional message google.protobuf.BoolValue
  pub fn has_charge_cluster_response_stats(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn charge_cluster_response_stats_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_charge_cluster_response_stats().then(|| self.charge_cluster_response_stats())
  }
  pub fn charge_cluster_response_stats(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // encode_raw_headers: optional bool
  pub fn encode_raw_headers(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        21, (false).into()
      ).try_into().unwrap()
    }
  }

  // decoder_header_mutation_rules: optional message envoy.config.common.mutation_rules.v3.HeaderMutationRules
  pub fn has_decoder_header_mutation_rules(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn decoder_header_mutation_rules_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'msg>> {
    self.has_decoder_header_mutation_rules().then(|| self.decoder_header_mutation_rules())
  }
  pub fn decoder_header_mutation_rules(self) -> crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView::default())
  }

  // enable_dynamic_metadata_ingestion: optional message google.protobuf.BoolValue
  pub fn has_enable_dynamic_metadata_ingestion(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn enable_dynamic_metadata_ingestion_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_enable_dynamic_metadata_ingestion().then(|| self.enable_dynamic_metadata_ingestion())
  }
  pub fn enable_dynamic_metadata_ingestion(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // filter_metadata: optional message google.protobuf.Struct
  pub fn has_filter_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn filter_metadata_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_filter_metadata().then(|| self.filter_metadata())
  }
  pub fn filter_metadata(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(26)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // emit_filter_state_stats: optional bool
  pub fn emit_filter_state_stats(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        27, (false).into()
      ).try_into().unwrap()
    }
  }

  // max_denied_response_body_bytes: optional uint32
  pub fn max_denied_response_body_bytes(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        28, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // enforce_response_header_limits: optional bool
  pub fn enforce_response_header_limits(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        29, (false).into()
      ).try_into().unwrap()
    }
  }

  pub fn services(self) -> super::ext_authz::ServicesOneof<'msg> {
    match self.services_case() {
      super::ext_authz::ServicesCase::GrpcService =>
          super::ext_authz::ServicesOneof::GrpcService(self.grpc_service()),
      super::ext_authz::ServicesCase::HttpService =>
          super::ext_authz::ServicesOneof::HttpService(self.http_service()),
      _ => super::ext_authz::ServicesOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn services_case(self) -> super::ext_authz::ServicesCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::ext_authz::ServicesCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExtAuthzView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtAuthzView<'_> {}

// SAFETY:
// - `ExtAuthzView` is `Send` because while its alive a `ExtAuthzMut` cannot.
// - `ExtAuthzView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtAuthzView<'_> {}

impl<'msg> ::protobuf::AsView for ExtAuthzView<'msg> {
  type Proxied = ExtAuthz;
  fn as_view(&self) -> ::protobuf::View<'msg, ExtAuthz> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtAuthzView<'msg> {
  fn into_view<'shorter>(self) -> ExtAuthzView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtAuthz> for ExtAuthzView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtAuthz {
    let mut dst = ExtAuthz::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtAuthz> for ExtAuthzMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtAuthz {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExtAuthz {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtAuthzView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtAuthzMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtAuthzMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtAuthz>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtAuthzMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtAuthzMut<'msg> {
  type Message = ExtAuthz;
}

impl ::std::fmt::Debug for ExtAuthzMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExtAuthz>> for ExtAuthzMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtAuthz>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtAuthzMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtAuthz> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExtAuthz {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // grpc_service: optional message envoy.config.core.v3.GrpcService
  pub fn has_grpc_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_grpc_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn grpc_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_>> {
    self.has_grpc_service().then(|| self.grpc_service())
  }
  pub fn grpc_service(&self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }
  pub fn grpc_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceMut<'_> {
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
  pub fn set_grpc_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // http_service: optional message envoy.extensions.filters.http.ext_authz.v3.HttpService
  pub fn has_http_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_http_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn http_service_opt(&self) -> ::std::option::Option<super::HttpServiceView<'_>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(&self) -> super::HttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpServiceView::default())
  }
  pub fn http_service_mut(&mut self) -> super::HttpServiceMut<'_> {
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
  pub fn set_http_service(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // transport_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn transport_api_version(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        10, (crate::xds::generated::envoy::config::core::v3::config_source::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_transport_api_version(&mut self, val: crate::xds::generated::envoy::config::core::v3::config_source::ApiVersion) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        10, val.into()
      )
    }
  }

  // failure_mode_allow: optional bool
  pub fn failure_mode_allow(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_failure_mode_allow(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // failure_mode_allow_header_add: optional bool
  pub fn failure_mode_allow_header_add(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_failure_mode_allow_header_add(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        17, val.into()
      )
    }
  }

  // with_request_body: optional message envoy.extensions.filters.http.ext_authz.v3.BufferSettings
  pub fn has_with_request_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_with_request_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn with_request_body_opt(&self) -> ::std::option::Option<super::BufferSettingsView<'_>> {
    self.has_with_request_body().then(|| self.with_request_body())
  }
  pub fn with_request_body(&self) -> super::BufferSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BufferSettingsView::default())
  }
  pub fn with_request_body_mut(&mut self) -> super::BufferSettingsMut<'_> {
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
  pub fn set_with_request_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::BufferSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // clear_route_cache: optional bool
  pub fn clear_route_cache(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_clear_route_cache(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

  // status_on_error: optional message envoy.type.v3.HttpStatus
  pub fn has_status_on_error(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_status_on_error(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn status_on_error_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_>> {
    self.has_status_on_error().then(|| self.status_on_error())
  }
  pub fn status_on_error(&self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }
  pub fn status_on_error_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusMut<'_> {
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
  pub fn set_status_on_error(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // validate_mutations: optional bool
  pub fn validate_mutations(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        22, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_validate_mutations(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        22, val.into()
      )
    }
  }

  // metadata_context_namespaces: repeated string
  pub fn metadata_context_namespaces(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn metadata_context_namespaces_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
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
  pub fn set_metadata_context_namespaces(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // typed_metadata_context_namespaces: repeated string
  pub fn typed_metadata_context_namespaces(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        14
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn typed_metadata_context_namespaces_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        14,
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
  pub fn set_typed_metadata_context_namespaces(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        src);
    }
  }

  // route_metadata_context_namespaces: repeated string
  pub fn route_metadata_context_namespaces(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        19
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn route_metadata_context_namespaces_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        19,
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
  pub fn set_route_metadata_context_namespaces(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        src);
    }
  }

  // route_typed_metadata_context_namespaces: repeated string
  pub fn route_typed_metadata_context_namespaces(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn route_typed_metadata_context_namespaces_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        20,
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
  pub fn set_route_typed_metadata_context_namespaces(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        src);
    }
  }

  // filter_enabled: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_filter_enabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_filter_enabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn filter_enabled_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_>> {
    self.has_filter_enabled().then(|| self.filter_enabled())
  }
  pub fn filter_enabled(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }
  pub fn filter_enabled_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
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
        7,
        val
      );
    }
  }

  // filter_enabled_metadata: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_filter_enabled_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_filter_enabled_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn filter_enabled_metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_>> {
    self.has_filter_enabled_metadata().then(|| self.filter_enabled_metadata())
  }
  pub fn filter_enabled_metadata(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }
  pub fn filter_enabled_metadata_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_filter_enabled_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // deny_at_disable: optional message envoy.config.core.v3.RuntimeFeatureFlag
  pub fn has_deny_at_disable(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_deny_at_disable(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn deny_at_disable_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlagView<'_>> {
    self.has_deny_at_disable().then(|| self.deny_at_disable())
  }
  pub fn deny_at_disable(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlagView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlagView::default())
  }
  pub fn deny_at_disable_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlagMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_deny_at_disable(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlag>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // include_peer_certificate: optional bool
  pub fn include_peer_certificate(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_include_peer_certificate(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        8, val.into()
      )
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val);
    }
  }

  // bootstrap_metadata_labels_key: optional string
  pub fn bootstrap_metadata_labels_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        13, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_bootstrap_metadata_labels_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val);
    }
  }

  // allowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_allowed_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn allowed_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_headers().then(|| self.allowed_headers())
  }
  pub fn allowed_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         15, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_allowed_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // disallowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_disallowed_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_disallowed_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn disallowed_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_disallowed_headers().then(|| self.disallowed_headers())
  }
  pub fn disallowed_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn disallowed_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         23, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_disallowed_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val
      );
    }
  }

  // include_tls_session: optional bool
  pub fn include_tls_session(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        16, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_include_tls_session(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        16, val.into()
      )
    }
  }

  // charge_cluster_response_stats: optional message google.protobuf.BoolValue
  pub fn has_charge_cluster_response_stats(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_charge_cluster_response_stats(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn charge_cluster_response_stats_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_charge_cluster_response_stats().then(|| self.charge_cluster_response_stats())
  }
  pub fn charge_cluster_response_stats(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn charge_cluster_response_stats_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         18, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_charge_cluster_response_stats(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
    }
  }

  // encode_raw_headers: optional bool
  pub fn encode_raw_headers(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        21, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_encode_raw_headers(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        21, val.into()
      )
    }
  }

  // decoder_header_mutation_rules: optional message envoy.config.common.mutation_rules.v3.HeaderMutationRules
  pub fn has_decoder_header_mutation_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_decoder_header_mutation_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn decoder_header_mutation_rules_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'_>> {
    self.has_decoder_header_mutation_rules().then(|| self.decoder_header_mutation_rules())
  }
  pub fn decoder_header_mutation_rules(&self) -> crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView::default())
  }
  pub fn decoder_header_mutation_rules_mut(&mut self) -> crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         24, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_decoder_header_mutation_rules(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // enable_dynamic_metadata_ingestion: optional message google.protobuf.BoolValue
  pub fn has_enable_dynamic_metadata_ingestion(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn clear_enable_dynamic_metadata_ingestion(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        25
      );
    }
  }
  pub fn enable_dynamic_metadata_ingestion_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enable_dynamic_metadata_ingestion().then(|| self.enable_dynamic_metadata_ingestion())
  }
  pub fn enable_dynamic_metadata_ingestion(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enable_dynamic_metadata_ingestion_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         25, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_enable_dynamic_metadata_ingestion(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val
      );
    }
  }

  // filter_metadata: optional message google.protobuf.Struct
  pub fn has_filter_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn clear_filter_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        26
      );
    }
  }
  pub fn filter_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_filter_metadata().then(|| self.filter_metadata())
  }
  pub fn filter_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(26)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn filter_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         26, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_filter_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        26,
        val
      );
    }
  }

  // emit_filter_state_stats: optional bool
  pub fn emit_filter_state_stats(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        27, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_emit_filter_state_stats(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        27, val.into()
      )
    }
  }

  // max_denied_response_body_bytes: optional uint32
  pub fn max_denied_response_body_bytes(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        28, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_denied_response_body_bytes(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        28, val.into()
      )
    }
  }

  // enforce_response_header_limits: optional bool
  pub fn enforce_response_header_limits(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        29, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforce_response_header_limits(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        29, val.into()
      )
    }
  }

  pub fn services(&self) -> super::ext_authz::ServicesOneof<'_> {
    match &self.services_case() {
      super::ext_authz::ServicesCase::GrpcService =>
          super::ext_authz::ServicesOneof::GrpcService(self.grpc_service()),
      super::ext_authz::ServicesCase::HttpService =>
          super::ext_authz::ServicesOneof::HttpService(self.http_service()),
      _ => super::ext_authz::ServicesOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn services_case(&self) -> super::ext_authz::ServicesCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::ext_authz::ServicesCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExtAuthzMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtAuthzMut<'_> {}

// SAFETY:
// - `ExtAuthzMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtAuthzMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtAuthzMut<'msg> {
  type Proxied = ExtAuthz;
  fn as_view(&self) -> ::protobuf::View<'_, ExtAuthz> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtAuthzMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExtAuthz>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtAuthzMut<'msg> {
  type MutProxied = ExtAuthz;
  fn as_mut(&mut self) -> ExtAuthzMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtAuthzMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtAuthzMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExtAuthz {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExtAuthz> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtAuthzView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtAuthzMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // grpc_service: optional message envoy.config.core.v3.GrpcService
  pub fn has_grpc_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_grpc_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn grpc_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_>> {
    self.has_grpc_service().then(|| self.grpc_service())
  }
  pub fn grpc_service(&self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }
  pub fn grpc_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceMut<'_> {
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
  pub fn set_grpc_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // http_service: optional message envoy.extensions.filters.http.ext_authz.v3.HttpService
  pub fn has_http_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_http_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn http_service_opt(&self) -> ::std::option::Option<super::HttpServiceView<'_>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(&self) -> super::HttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpServiceView::default())
  }
  pub fn http_service_mut(&mut self) -> super::HttpServiceMut<'_> {
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
  pub fn set_http_service(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // transport_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn transport_api_version(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        10, (crate::xds::generated::envoy::config::core::v3::config_source::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_transport_api_version(&mut self, val: crate::xds::generated::envoy::config::core::v3::config_source::ApiVersion) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        10, val.into()
      )
    }
  }

  // failure_mode_allow: optional bool
  pub fn failure_mode_allow(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_failure_mode_allow(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // failure_mode_allow_header_add: optional bool
  pub fn failure_mode_allow_header_add(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_failure_mode_allow_header_add(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        17, val.into()
      )
    }
  }

  // with_request_body: optional message envoy.extensions.filters.http.ext_authz.v3.BufferSettings
  pub fn has_with_request_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_with_request_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn with_request_body_opt(&self) -> ::std::option::Option<super::BufferSettingsView<'_>> {
    self.has_with_request_body().then(|| self.with_request_body())
  }
  pub fn with_request_body(&self) -> super::BufferSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BufferSettingsView::default())
  }
  pub fn with_request_body_mut(&mut self) -> super::BufferSettingsMut<'_> {
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
  pub fn set_with_request_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::BufferSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // clear_route_cache: optional bool
  pub fn clear_route_cache(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_clear_route_cache(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

  // status_on_error: optional message envoy.type.v3.HttpStatus
  pub fn has_status_on_error(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_status_on_error(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn status_on_error_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_>> {
    self.has_status_on_error().then(|| self.status_on_error())
  }
  pub fn status_on_error(&self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }
  pub fn status_on_error_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusMut<'_> {
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
  pub fn set_status_on_error(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // validate_mutations: optional bool
  pub fn validate_mutations(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        22, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_validate_mutations(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        22, val.into()
      )
    }
  }

  // metadata_context_namespaces: repeated string
  pub fn metadata_context_namespaces(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn metadata_context_namespaces_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
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
  pub fn set_metadata_context_namespaces(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // typed_metadata_context_namespaces: repeated string
  pub fn typed_metadata_context_namespaces(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        14
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn typed_metadata_context_namespaces_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        14,
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
  pub fn set_typed_metadata_context_namespaces(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        src);
    }
  }

  // route_metadata_context_namespaces: repeated string
  pub fn route_metadata_context_namespaces(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        19
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn route_metadata_context_namespaces_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        19,
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
  pub fn set_route_metadata_context_namespaces(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        src);
    }
  }

  // route_typed_metadata_context_namespaces: repeated string
  pub fn route_typed_metadata_context_namespaces(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn route_typed_metadata_context_namespaces_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        20,
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
  pub fn set_route_typed_metadata_context_namespaces(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        src);
    }
  }

  // filter_enabled: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_filter_enabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_filter_enabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn filter_enabled_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_>> {
    self.has_filter_enabled().then(|| self.filter_enabled())
  }
  pub fn filter_enabled(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }
  pub fn filter_enabled_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
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
        7,
        val
      );
    }
  }

  // filter_enabled_metadata: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_filter_enabled_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_filter_enabled_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn filter_enabled_metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_>> {
    self.has_filter_enabled_metadata().then(|| self.filter_enabled_metadata())
  }
  pub fn filter_enabled_metadata(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }
  pub fn filter_enabled_metadata_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_filter_enabled_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // deny_at_disable: optional message envoy.config.core.v3.RuntimeFeatureFlag
  pub fn has_deny_at_disable(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_deny_at_disable(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn deny_at_disable_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlagView<'_>> {
    self.has_deny_at_disable().then(|| self.deny_at_disable())
  }
  pub fn deny_at_disable(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlagView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlagView::default())
  }
  pub fn deny_at_disable_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlagMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_deny_at_disable(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlag>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // include_peer_certificate: optional bool
  pub fn include_peer_certificate(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_include_peer_certificate(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        8, val.into()
      )
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val);
    }
  }

  // bootstrap_metadata_labels_key: optional string
  pub fn bootstrap_metadata_labels_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        13, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_bootstrap_metadata_labels_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val);
    }
  }

  // allowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_allowed_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn allowed_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_headers().then(|| self.allowed_headers())
  }
  pub fn allowed_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         15, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_allowed_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // disallowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_disallowed_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_disallowed_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn disallowed_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_disallowed_headers().then(|| self.disallowed_headers())
  }
  pub fn disallowed_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn disallowed_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         23, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_disallowed_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val
      );
    }
  }

  // include_tls_session: optional bool
  pub fn include_tls_session(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        16, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_include_tls_session(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        16, val.into()
      )
    }
  }

  // charge_cluster_response_stats: optional message google.protobuf.BoolValue
  pub fn has_charge_cluster_response_stats(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_charge_cluster_response_stats(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn charge_cluster_response_stats_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_charge_cluster_response_stats().then(|| self.charge_cluster_response_stats())
  }
  pub fn charge_cluster_response_stats(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn charge_cluster_response_stats_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         18, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_charge_cluster_response_stats(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
    }
  }

  // encode_raw_headers: optional bool
  pub fn encode_raw_headers(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        21, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_encode_raw_headers(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        21, val.into()
      )
    }
  }

  // decoder_header_mutation_rules: optional message envoy.config.common.mutation_rules.v3.HeaderMutationRules
  pub fn has_decoder_header_mutation_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_decoder_header_mutation_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn decoder_header_mutation_rules_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'_>> {
    self.has_decoder_header_mutation_rules().then(|| self.decoder_header_mutation_rules())
  }
  pub fn decoder_header_mutation_rules(&self) -> crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView::default())
  }
  pub fn decoder_header_mutation_rules_mut(&mut self) -> crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         24, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_decoder_header_mutation_rules(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // enable_dynamic_metadata_ingestion: optional message google.protobuf.BoolValue
  pub fn has_enable_dynamic_metadata_ingestion(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn clear_enable_dynamic_metadata_ingestion(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        25
      );
    }
  }
  pub fn enable_dynamic_metadata_ingestion_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enable_dynamic_metadata_ingestion().then(|| self.enable_dynamic_metadata_ingestion())
  }
  pub fn enable_dynamic_metadata_ingestion(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enable_dynamic_metadata_ingestion_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         25, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_enable_dynamic_metadata_ingestion(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val
      );
    }
  }

  // filter_metadata: optional message google.protobuf.Struct
  pub fn has_filter_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn clear_filter_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        26
      );
    }
  }
  pub fn filter_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_filter_metadata().then(|| self.filter_metadata())
  }
  pub fn filter_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(26)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn filter_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         26, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_filter_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        26,
        val
      );
    }
  }

  // emit_filter_state_stats: optional bool
  pub fn emit_filter_state_stats(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        27, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_emit_filter_state_stats(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        27, val.into()
      )
    }
  }

  // max_denied_response_body_bytes: optional uint32
  pub fn max_denied_response_body_bytes(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        28, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_denied_response_body_bytes(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        28, val.into()
      )
    }
  }

  // enforce_response_header_limits: optional bool
  pub fn enforce_response_header_limits(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        29, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enforce_response_header_limits(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        29, val.into()
      )
    }
  }

  pub fn services(&self) -> super::ext_authz::ServicesOneof<'_> {
    match &self.services_case() {
      super::ext_authz::ServicesCase::GrpcService =>
          super::ext_authz::ServicesOneof::GrpcService(self.grpc_service()),
      super::ext_authz::ServicesCase::HttpService =>
          super::ext_authz::ServicesOneof::HttpService(self.http_service()),
      _ => super::ext_authz::ServicesOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn services_case(&self) -> super::ext_authz::ServicesCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::ext_authz::ServicesCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ExtAuthz

impl ::std::ops::Drop for ExtAuthz {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExtAuthz {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExtAuthz {
  type Proxied = Self;
  fn as_view(&self) -> ExtAuthzView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExtAuthz {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtAuthzMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExtAuthz {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0authz__v3__ExtAuthz_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3/P3a3/P3ET3/P3.P1X31XET3/P/P3ETET/P/P3333/P)P/P^!|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0authz__v3__ExtAuthz_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::BufferSettings as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RuntimeFeatureFlag as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0authz__v3__ExtAuthz_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtAuthz {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtAuthz {
  type Msg = ExtAuthz;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtAuthz> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtAuthz {
  type Msg = ExtAuthz;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtAuthz> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtAuthzMut<'_> {
  type Msg = ExtAuthz;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtAuthz> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtAuthzMut<'_> {
  type Msg = ExtAuthz;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtAuthz> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtAuthzView<'_> {
  type Msg = ExtAuthz;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtAuthz> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtAuthzMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod ext_authz {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ServicesOneof<'msg> {
  GrpcService(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>) = 1,
  HttpService(::protobuf::View<'msg, super::super::HttpService>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ServicesCase {
  GrpcService = 1,
  HttpService = 3,

  not_set = 0
}

impl ServicesCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ServicesCase> {
    match v {
      0 => Some(ServicesCase::not_set),
      1 => Some(ServicesCase::GrpcService),
      3 => Some(ServicesCase::HttpService),
      _ => None
    }
  }
}
}  // pub mod ext_authz


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0authz__v3__BufferSettings_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BufferSettings {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BufferSettings>
}

impl ::protobuf::Message for BufferSettings {
  type MessageView<'msg> = BufferSettingsView<'msg>;
  type MessageMut<'msg> = BufferSettingsMut<'msg>;
}

impl ::std::default::Default for BufferSettings {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BufferSettings {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BufferSettings` is `Sync` because it does not implement interior mutability.
//    Neither does `BufferSettingsMut`.
unsafe impl ::std::marker::Sync for BufferSettings {}

// SAFETY:
// - `BufferSettings` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BufferSettings {}

impl ::protobuf::Proxied for BufferSettings {
  type View<'msg> = BufferSettingsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BufferSettings {}

impl ::protobuf::MutProxied for BufferSettings {
  type Mut<'msg> = BufferSettingsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BufferSettingsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BufferSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BufferSettingsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BufferSettingsView<'msg> {
  type Message = BufferSettings;
}

impl ::std::fmt::Debug for BufferSettingsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BufferSettingsView<'_> {
  fn default() -> BufferSettingsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BufferSettings>> for BufferSettingsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BufferSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BufferSettingsView<'msg> {

  pub fn to_owned(&self) -> BufferSettings {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // max_request_bytes: optional uint32
  pub fn max_request_bytes(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // allow_partial_message: optional bool
  pub fn allow_partial_message(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

  // pack_as_bytes: optional bool
  pub fn pack_as_bytes(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `BufferSettingsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BufferSettingsView<'_> {}

// SAFETY:
// - `BufferSettingsView` is `Send` because while its alive a `BufferSettingsMut` cannot.
// - `BufferSettingsView` does not use thread-local data.
unsafe impl ::std::marker::Send for BufferSettingsView<'_> {}

impl<'msg> ::protobuf::AsView for BufferSettingsView<'msg> {
  type Proxied = BufferSettings;
  fn as_view(&self) -> ::protobuf::View<'msg, BufferSettings> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BufferSettingsView<'msg> {
  fn into_view<'shorter>(self) -> BufferSettingsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BufferSettings> for BufferSettingsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BufferSettings {
    let mut dst = BufferSettings::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BufferSettings> for BufferSettingsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BufferSettings {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BufferSettings {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BufferSettingsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BufferSettingsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BufferSettingsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BufferSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BufferSettingsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BufferSettingsMut<'msg> {
  type Message = BufferSettings;
}

impl ::std::fmt::Debug for BufferSettingsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BufferSettings>> for BufferSettingsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BufferSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BufferSettingsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BufferSettings> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BufferSettings {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // max_request_bytes: optional uint32
  pub fn max_request_bytes(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_request_bytes(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // allow_partial_message: optional bool
  pub fn allow_partial_message(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_allow_partial_message(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // pack_as_bytes: optional bool
  pub fn pack_as_bytes(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_pack_as_bytes(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `BufferSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BufferSettingsMut<'_> {}

// SAFETY:
// - `BufferSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BufferSettingsMut<'_> {}

impl<'msg> ::protobuf::AsView for BufferSettingsMut<'msg> {
  type Proxied = BufferSettings;
  fn as_view(&self) -> ::protobuf::View<'_, BufferSettings> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BufferSettingsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BufferSettings>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BufferSettingsMut<'msg> {
  type MutProxied = BufferSettings;
  fn as_mut(&mut self) -> BufferSettingsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BufferSettingsMut<'msg> {
  fn into_mut<'shorter>(self) -> BufferSettingsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BufferSettings {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BufferSettings> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BufferSettingsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BufferSettingsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // max_request_bytes: optional uint32
  pub fn max_request_bytes(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_request_bytes(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // allow_partial_message: optional bool
  pub fn allow_partial_message(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_allow_partial_message(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // pack_as_bytes: optional bool
  pub fn pack_as_bytes(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_pack_as_bytes(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

}  // impl BufferSettings

impl ::std::ops::Drop for BufferSettings {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BufferSettings {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BufferSettings {
  type Proxied = Self;
  fn as_view(&self) -> BufferSettingsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BufferSettings {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BufferSettingsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BufferSettings {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0authz__v3__BufferSettings_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0authz__v3__BufferSettings_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0authz__v3__BufferSettings_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BufferSettings {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BufferSettings {
  type Msg = BufferSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BufferSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BufferSettings {
  type Msg = BufferSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BufferSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BufferSettingsMut<'_> {
  type Msg = BufferSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BufferSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BufferSettingsMut<'_> {
  type Msg = BufferSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BufferSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BufferSettingsView<'_> {
  type Msg = BufferSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BufferSettings> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BufferSettingsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0authz__v3__HttpService_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpService {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpService>
}

impl ::protobuf::Message for HttpService {
  type MessageView<'msg> = HttpServiceView<'msg>;
  type MessageMut<'msg> = HttpServiceMut<'msg>;
}

impl ::std::default::Default for HttpService {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpService {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpService` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpServiceMut`.
unsafe impl ::std::marker::Sync for HttpService {}

// SAFETY:
// - `HttpService` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpService {}

impl ::protobuf::Proxied for HttpService {
  type View<'msg> = HttpServiceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpService {}

impl ::protobuf::MutProxied for HttpService {
  type Mut<'msg> = HttpServiceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpServiceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpService>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpServiceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpServiceView<'msg> {
  type Message = HttpService;
}

impl ::std::fmt::Debug for HttpServiceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpServiceView<'_> {
  fn default() -> HttpServiceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpService>> for HttpServiceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpService>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpServiceView<'msg> {

  pub fn to_owned(&self) -> HttpService {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // server_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_server_uri(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn server_uri_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'msg>> {
    self.has_server_uri().then(|| self.server_uri())
  }
  pub fn server_uri(self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }

  // path_prefix: optional string
  pub fn path_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // authorization_request: optional message envoy.extensions.filters.http.ext_authz.v3.AuthorizationRequest
  pub fn has_authorization_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn authorization_request_opt(self) -> ::std::option::Option<super::AuthorizationRequestView<'msg>> {
    self.has_authorization_request().then(|| self.authorization_request())
  }
  pub fn authorization_request(self) -> super::AuthorizationRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AuthorizationRequestView::default())
  }

  // authorization_response: optional message envoy.extensions.filters.http.ext_authz.v3.AuthorizationResponse
  pub fn has_authorization_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn authorization_response_opt(self) -> ::std::option::Option<super::AuthorizationResponseView<'msg>> {
    self.has_authorization_response().then(|| self.authorization_response())
  }
  pub fn authorization_response(self) -> super::AuthorizationResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AuthorizationResponseView::default())
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn retry_policy_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'msg>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }

}

// SAFETY:
// - `HttpServiceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpServiceView<'_> {}

// SAFETY:
// - `HttpServiceView` is `Send` because while its alive a `HttpServiceMut` cannot.
// - `HttpServiceView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpServiceView<'_> {}

impl<'msg> ::protobuf::AsView for HttpServiceView<'msg> {
  type Proxied = HttpService;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpService> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpServiceView<'msg> {
  fn into_view<'shorter>(self) -> HttpServiceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpService> for HttpServiceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpService {
    let mut dst = HttpService::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpService> for HttpServiceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpService {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpService {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpServiceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpServiceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpServiceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpService>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpServiceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpServiceMut<'msg> {
  type Message = HttpService;
}

impl ::std::fmt::Debug for HttpServiceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpService>> for HttpServiceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpService>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpServiceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpService> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpService {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // server_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_server_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_server_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn server_uri_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_>> {
    self.has_server_uri().then(|| self.server_uri())
  }
  pub fn server_uri(&self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }
  pub fn server_uri_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriMut<'_> {
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
  pub fn set_server_uri(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // path_prefix: optional string
  pub fn path_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // authorization_request: optional message envoy.extensions.filters.http.ext_authz.v3.AuthorizationRequest
  pub fn has_authorization_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_authorization_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn authorization_request_opt(&self) -> ::std::option::Option<super::AuthorizationRequestView<'_>> {
    self.has_authorization_request().then(|| self.authorization_request())
  }
  pub fn authorization_request(&self) -> super::AuthorizationRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AuthorizationRequestView::default())
  }
  pub fn authorization_request_mut(&mut self) -> super::AuthorizationRequestMut<'_> {
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
  pub fn set_authorization_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::AuthorizationRequest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // authorization_response: optional message envoy.extensions.filters.http.ext_authz.v3.AuthorizationResponse
  pub fn has_authorization_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_authorization_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn authorization_response_opt(&self) -> ::std::option::Option<super::AuthorizationResponseView<'_>> {
    self.has_authorization_response().then(|| self.authorization_response())
  }
  pub fn authorization_response(&self) -> super::AuthorizationResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AuthorizationResponseView::default())
  }
  pub fn authorization_response_mut(&mut self) -> super::AuthorizationResponseMut<'_> {
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
  pub fn set_authorization_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::AuthorizationResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(&self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RetryPolicy>) {

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
// - `HttpServiceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpServiceMut<'_> {}

// SAFETY:
// - `HttpServiceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpServiceMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpServiceMut<'msg> {
  type Proxied = HttpService;
  fn as_view(&self) -> ::protobuf::View<'_, HttpService> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpServiceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpService>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpServiceMut<'msg> {
  type MutProxied = HttpService;
  fn as_mut(&mut self) -> HttpServiceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpServiceMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpServiceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpService {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpService> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpServiceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpServiceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // server_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_server_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_server_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn server_uri_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_>> {
    self.has_server_uri().then(|| self.server_uri())
  }
  pub fn server_uri(&self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }
  pub fn server_uri_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriMut<'_> {
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
  pub fn set_server_uri(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // path_prefix: optional string
  pub fn path_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // authorization_request: optional message envoy.extensions.filters.http.ext_authz.v3.AuthorizationRequest
  pub fn has_authorization_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_authorization_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn authorization_request_opt(&self) -> ::std::option::Option<super::AuthorizationRequestView<'_>> {
    self.has_authorization_request().then(|| self.authorization_request())
  }
  pub fn authorization_request(&self) -> super::AuthorizationRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AuthorizationRequestView::default())
  }
  pub fn authorization_request_mut(&mut self) -> super::AuthorizationRequestMut<'_> {
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
  pub fn set_authorization_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::AuthorizationRequest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // authorization_response: optional message envoy.extensions.filters.http.ext_authz.v3.AuthorizationResponse
  pub fn has_authorization_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_authorization_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn authorization_response_opt(&self) -> ::std::option::Option<super::AuthorizationResponseView<'_>> {
    self.has_authorization_response().then(|| self.authorization_response())
  }
  pub fn authorization_response(&self) -> super::AuthorizationResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AuthorizationResponseView::default())
  }
  pub fn authorization_response_mut(&mut self) -> super::AuthorizationResponseMut<'_> {
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
  pub fn set_authorization_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::AuthorizationResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(&self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RetryPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}  // impl HttpService

impl ::std::ops::Drop for HttpService {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpService {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpService {
  type Proxied = Self;
  fn as_view(&self) -> HttpServiceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpService {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpServiceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpService {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0authz__v3__HttpService_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31Xd333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0authz__v3__HttpService_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::AuthorizationRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::AuthorizationResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RetryPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0authz__v3__HttpService_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpService {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpService {
  type Msg = HttpService;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpService> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpService {
  type Msg = HttpService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpService> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpServiceMut<'_> {
  type Msg = HttpService;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpService> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpServiceMut<'_> {
  type Msg = HttpService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpService> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpServiceView<'_> {
  type Msg = HttpService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpService> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpServiceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0authz__v3__AuthorizationRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AuthorizationRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AuthorizationRequest>
}

impl ::protobuf::Message for AuthorizationRequest {
  type MessageView<'msg> = AuthorizationRequestView<'msg>;
  type MessageMut<'msg> = AuthorizationRequestMut<'msg>;
}

impl ::std::default::Default for AuthorizationRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AuthorizationRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AuthorizationRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `AuthorizationRequestMut`.
unsafe impl ::std::marker::Sync for AuthorizationRequest {}

// SAFETY:
// - `AuthorizationRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AuthorizationRequest {}

impl ::protobuf::Proxied for AuthorizationRequest {
  type View<'msg> = AuthorizationRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AuthorizationRequest {}

impl ::protobuf::MutProxied for AuthorizationRequest {
  type Mut<'msg> = AuthorizationRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AuthorizationRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AuthorizationRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuthorizationRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AuthorizationRequestView<'msg> {
  type Message = AuthorizationRequest;
}

impl ::std::fmt::Debug for AuthorizationRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AuthorizationRequestView<'_> {
  fn default() -> AuthorizationRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AuthorizationRequest>> for AuthorizationRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AuthorizationRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuthorizationRequestView<'msg> {

  pub fn to_owned(&self) -> AuthorizationRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // allowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn allowed_headers_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg>> {
    self.has_allowed_headers().then(|| self.allowed_headers())
  }
  pub fn allowed_headers(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }

  // headers_to_add: repeated message envoy.config.core.v3.HeaderValue
  pub fn headers_to_add(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `AuthorizationRequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AuthorizationRequestView<'_> {}

// SAFETY:
// - `AuthorizationRequestView` is `Send` because while its alive a `AuthorizationRequestMut` cannot.
// - `AuthorizationRequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for AuthorizationRequestView<'_> {}

impl<'msg> ::protobuf::AsView for AuthorizationRequestView<'msg> {
  type Proxied = AuthorizationRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, AuthorizationRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuthorizationRequestView<'msg> {
  fn into_view<'shorter>(self) -> AuthorizationRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AuthorizationRequest> for AuthorizationRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AuthorizationRequest {
    let mut dst = AuthorizationRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AuthorizationRequest> for AuthorizationRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AuthorizationRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AuthorizationRequest {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuthorizationRequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuthorizationRequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AuthorizationRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AuthorizationRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuthorizationRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AuthorizationRequestMut<'msg> {
  type Message = AuthorizationRequest;
}

impl ::std::fmt::Debug for AuthorizationRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AuthorizationRequest>> for AuthorizationRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AuthorizationRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuthorizationRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AuthorizationRequest> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AuthorizationRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // allowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_allowed_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn allowed_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_headers().then(|| self.allowed_headers())
  }
  pub fn allowed_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_allowed_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // headers_to_add: repeated message envoy.config.core.v3.HeaderValue
  pub fn headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `AuthorizationRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AuthorizationRequestMut<'_> {}

// SAFETY:
// - `AuthorizationRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AuthorizationRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for AuthorizationRequestMut<'msg> {
  type Proxied = AuthorizationRequest;
  fn as_view(&self) -> ::protobuf::View<'_, AuthorizationRequest> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuthorizationRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AuthorizationRequest>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AuthorizationRequestMut<'msg> {
  type MutProxied = AuthorizationRequest;
  fn as_mut(&mut self) -> AuthorizationRequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AuthorizationRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> AuthorizationRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AuthorizationRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AuthorizationRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AuthorizationRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AuthorizationRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // allowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_allowed_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn allowed_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_headers().then(|| self.allowed_headers())
  }
  pub fn allowed_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_allowed_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // headers_to_add: repeated message envoy.config.core.v3.HeaderValue
  pub fn headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl AuthorizationRequest

impl ::std::ops::Drop for AuthorizationRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AuthorizationRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AuthorizationRequest {
  type Proxied = Self;
  fn as_view(&self) -> AuthorizationRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AuthorizationRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AuthorizationRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AuthorizationRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0authz__v3__AuthorizationRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0authz__v3__AuthorizationRequest_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0authz__v3__AuthorizationRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuthorizationRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuthorizationRequest {
  type Msg = AuthorizationRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthorizationRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthorizationRequest {
  type Msg = AuthorizationRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthorizationRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuthorizationRequestMut<'_> {
  type Msg = AuthorizationRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthorizationRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthorizationRequestMut<'_> {
  type Msg = AuthorizationRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthorizationRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthorizationRequestView<'_> {
  type Msg = AuthorizationRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthorizationRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuthorizationRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0authz__v3__AuthorizationResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AuthorizationResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AuthorizationResponse>
}

impl ::protobuf::Message for AuthorizationResponse {
  type MessageView<'msg> = AuthorizationResponseView<'msg>;
  type MessageMut<'msg> = AuthorizationResponseMut<'msg>;
}

impl ::std::default::Default for AuthorizationResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AuthorizationResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AuthorizationResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `AuthorizationResponseMut`.
unsafe impl ::std::marker::Sync for AuthorizationResponse {}

// SAFETY:
// - `AuthorizationResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AuthorizationResponse {}

impl ::protobuf::Proxied for AuthorizationResponse {
  type View<'msg> = AuthorizationResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AuthorizationResponse {}

impl ::protobuf::MutProxied for AuthorizationResponse {
  type Mut<'msg> = AuthorizationResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AuthorizationResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AuthorizationResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuthorizationResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AuthorizationResponseView<'msg> {
  type Message = AuthorizationResponse;
}

impl ::std::fmt::Debug for AuthorizationResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AuthorizationResponseView<'_> {
  fn default() -> AuthorizationResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AuthorizationResponse>> for AuthorizationResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AuthorizationResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuthorizationResponseView<'msg> {

  pub fn to_owned(&self) -> AuthorizationResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // allowed_upstream_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_upstream_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn allowed_upstream_headers_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg>> {
    self.has_allowed_upstream_headers().then(|| self.allowed_upstream_headers())
  }
  pub fn allowed_upstream_headers(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }

  // allowed_upstream_headers_to_append: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_upstream_headers_to_append(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn allowed_upstream_headers_to_append_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg>> {
    self.has_allowed_upstream_headers_to_append().then(|| self.allowed_upstream_headers_to_append())
  }
  pub fn allowed_upstream_headers_to_append(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }

  // allowed_client_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_client_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn allowed_client_headers_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg>> {
    self.has_allowed_client_headers().then(|| self.allowed_client_headers())
  }
  pub fn allowed_client_headers(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }

  // allowed_client_headers_on_success: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_client_headers_on_success(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn allowed_client_headers_on_success_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg>> {
    self.has_allowed_client_headers_on_success().then(|| self.allowed_client_headers_on_success())
  }
  pub fn allowed_client_headers_on_success(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }

  // dynamic_metadata_from_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_dynamic_metadata_from_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn dynamic_metadata_from_headers_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg>> {
    self.has_dynamic_metadata_from_headers().then(|| self.dynamic_metadata_from_headers())
  }
  pub fn dynamic_metadata_from_headers(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }

}

// SAFETY:
// - `AuthorizationResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AuthorizationResponseView<'_> {}

// SAFETY:
// - `AuthorizationResponseView` is `Send` because while its alive a `AuthorizationResponseMut` cannot.
// - `AuthorizationResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for AuthorizationResponseView<'_> {}

impl<'msg> ::protobuf::AsView for AuthorizationResponseView<'msg> {
  type Proxied = AuthorizationResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, AuthorizationResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuthorizationResponseView<'msg> {
  fn into_view<'shorter>(self) -> AuthorizationResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AuthorizationResponse> for AuthorizationResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AuthorizationResponse {
    let mut dst = AuthorizationResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AuthorizationResponse> for AuthorizationResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AuthorizationResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AuthorizationResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuthorizationResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuthorizationResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AuthorizationResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AuthorizationResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuthorizationResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AuthorizationResponseMut<'msg> {
  type Message = AuthorizationResponse;
}

impl ::std::fmt::Debug for AuthorizationResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AuthorizationResponse>> for AuthorizationResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AuthorizationResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuthorizationResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AuthorizationResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AuthorizationResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // allowed_upstream_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_upstream_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_allowed_upstream_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn allowed_upstream_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_upstream_headers().then(|| self.allowed_upstream_headers())
  }
  pub fn allowed_upstream_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_upstream_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_allowed_upstream_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // allowed_upstream_headers_to_append: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_upstream_headers_to_append(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_allowed_upstream_headers_to_append(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn allowed_upstream_headers_to_append_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_upstream_headers_to_append().then(|| self.allowed_upstream_headers_to_append())
  }
  pub fn allowed_upstream_headers_to_append(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_upstream_headers_to_append_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_allowed_upstream_headers_to_append(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // allowed_client_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_client_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_allowed_client_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn allowed_client_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_client_headers().then(|| self.allowed_client_headers())
  }
  pub fn allowed_client_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_client_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_allowed_client_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // allowed_client_headers_on_success: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_client_headers_on_success(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_allowed_client_headers_on_success(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn allowed_client_headers_on_success_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_client_headers_on_success().then(|| self.allowed_client_headers_on_success())
  }
  pub fn allowed_client_headers_on_success(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_client_headers_on_success_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_allowed_client_headers_on_success(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // dynamic_metadata_from_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_dynamic_metadata_from_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_dynamic_metadata_from_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn dynamic_metadata_from_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_dynamic_metadata_from_headers().then(|| self.dynamic_metadata_from_headers())
  }
  pub fn dynamic_metadata_from_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn dynamic_metadata_from_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_dynamic_metadata_from_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

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
// - `AuthorizationResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AuthorizationResponseMut<'_> {}

// SAFETY:
// - `AuthorizationResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AuthorizationResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for AuthorizationResponseMut<'msg> {
  type Proxied = AuthorizationResponse;
  fn as_view(&self) -> ::protobuf::View<'_, AuthorizationResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuthorizationResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AuthorizationResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AuthorizationResponseMut<'msg> {
  type MutProxied = AuthorizationResponse;
  fn as_mut(&mut self) -> AuthorizationResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AuthorizationResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> AuthorizationResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AuthorizationResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AuthorizationResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AuthorizationResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AuthorizationResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // allowed_upstream_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_upstream_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_allowed_upstream_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn allowed_upstream_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_upstream_headers().then(|| self.allowed_upstream_headers())
  }
  pub fn allowed_upstream_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_upstream_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_allowed_upstream_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // allowed_upstream_headers_to_append: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_upstream_headers_to_append(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_allowed_upstream_headers_to_append(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn allowed_upstream_headers_to_append_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_upstream_headers_to_append().then(|| self.allowed_upstream_headers_to_append())
  }
  pub fn allowed_upstream_headers_to_append(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_upstream_headers_to_append_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_allowed_upstream_headers_to_append(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // allowed_client_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_client_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_allowed_client_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn allowed_client_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_client_headers().then(|| self.allowed_client_headers())
  }
  pub fn allowed_client_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_client_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_allowed_client_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // allowed_client_headers_on_success: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_allowed_client_headers_on_success(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_allowed_client_headers_on_success(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn allowed_client_headers_on_success_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_allowed_client_headers_on_success().then(|| self.allowed_client_headers_on_success())
  }
  pub fn allowed_client_headers_on_success(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn allowed_client_headers_on_success_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_allowed_client_headers_on_success(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // dynamic_metadata_from_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_dynamic_metadata_from_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_dynamic_metadata_from_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn dynamic_metadata_from_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_dynamic_metadata_from_headers().then(|| self.dynamic_metadata_from_headers())
  }
  pub fn dynamic_metadata_from_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn dynamic_metadata_from_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_dynamic_metadata_from_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}  // impl AuthorizationResponse

impl ::std::ops::Drop for AuthorizationResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AuthorizationResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AuthorizationResponse {
  type Proxied = Self;
  fn as_view(&self) -> AuthorizationResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AuthorizationResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AuthorizationResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AuthorizationResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0authz__v3__AuthorizationResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0authz__v3__AuthorizationResponse_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0authz__v3__AuthorizationResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuthorizationResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuthorizationResponse {
  type Msg = AuthorizationResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthorizationResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthorizationResponse {
  type Msg = AuthorizationResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthorizationResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuthorizationResponseMut<'_> {
  type Msg = AuthorizationResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthorizationResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthorizationResponseMut<'_> {
  type Msg = AuthorizationResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthorizationResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthorizationResponseView<'_> {
  type Msg = AuthorizationResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AuthorizationResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuthorizationResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0authz__v3__ExtAuthzPerRoute_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExtAuthzPerRoute {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExtAuthzPerRoute>
}

impl ::protobuf::Message for ExtAuthzPerRoute {
  type MessageView<'msg> = ExtAuthzPerRouteView<'msg>;
  type MessageMut<'msg> = ExtAuthzPerRouteMut<'msg>;
}

impl ::std::default::Default for ExtAuthzPerRoute {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExtAuthzPerRoute {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExtAuthzPerRoute` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtAuthzPerRouteMut`.
unsafe impl ::std::marker::Sync for ExtAuthzPerRoute {}

// SAFETY:
// - `ExtAuthzPerRoute` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExtAuthzPerRoute {}

impl ::protobuf::Proxied for ExtAuthzPerRoute {
  type View<'msg> = ExtAuthzPerRouteView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExtAuthzPerRoute {}

impl ::protobuf::MutProxied for ExtAuthzPerRoute {
  type Mut<'msg> = ExtAuthzPerRouteMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtAuthzPerRouteView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtAuthzPerRoute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtAuthzPerRouteView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtAuthzPerRouteView<'msg> {
  type Message = ExtAuthzPerRoute;
}

impl ::std::fmt::Debug for ExtAuthzPerRouteView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtAuthzPerRouteView<'_> {
  fn default() -> ExtAuthzPerRouteView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExtAuthzPerRoute>> for ExtAuthzPerRouteView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtAuthzPerRoute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtAuthzPerRouteView<'msg> {

  pub fn to_owned(&self) -> ExtAuthzPerRoute {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // disabled: optional bool
  pub fn has_disabled(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn disabled_opt(self) -> ::std::option::Option<bool> {
    self.has_disabled().then(|| self.disabled())
  }
  pub fn disabled(self) -> bool {
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

  // check_settings: optional message envoy.extensions.filters.http.ext_authz.v3.CheckSettings
  pub fn has_check_settings(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn check_settings_opt(self) -> ::std::option::Option<super::CheckSettingsView<'msg>> {
    self.has_check_settings().then(|| self.check_settings())
  }
  pub fn check_settings(self) -> super::CheckSettingsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CheckSettingsView::default())
  }

  pub fn r#override(self) -> super::ext_authz_per_route::OverrideOneof<'msg> {
    match self.r#override_case() {
      super::ext_authz_per_route::OverrideCase::Disabled =>
          super::ext_authz_per_route::OverrideOneof::Disabled(self.disabled()),
      super::ext_authz_per_route::OverrideCase::CheckSettings =>
          super::ext_authz_per_route::OverrideOneof::CheckSettings(self.check_settings()),
      _ => super::ext_authz_per_route::OverrideOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#override_case(self) -> super::ext_authz_per_route::OverrideCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::ext_authz_per_route::OverrideCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExtAuthzPerRouteView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtAuthzPerRouteView<'_> {}

// SAFETY:
// - `ExtAuthzPerRouteView` is `Send` because while its alive a `ExtAuthzPerRouteMut` cannot.
// - `ExtAuthzPerRouteView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtAuthzPerRouteView<'_> {}

impl<'msg> ::protobuf::AsView for ExtAuthzPerRouteView<'msg> {
  type Proxied = ExtAuthzPerRoute;
  fn as_view(&self) -> ::protobuf::View<'msg, ExtAuthzPerRoute> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtAuthzPerRouteView<'msg> {
  fn into_view<'shorter>(self) -> ExtAuthzPerRouteView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtAuthzPerRoute> for ExtAuthzPerRouteView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtAuthzPerRoute {
    let mut dst = ExtAuthzPerRoute::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtAuthzPerRoute> for ExtAuthzPerRouteMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtAuthzPerRoute {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExtAuthzPerRoute {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtAuthzPerRouteView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtAuthzPerRouteMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtAuthzPerRouteMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtAuthzPerRoute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtAuthzPerRouteMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtAuthzPerRouteMut<'msg> {
  type Message = ExtAuthzPerRoute;
}

impl ::std::fmt::Debug for ExtAuthzPerRouteMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExtAuthzPerRoute>> for ExtAuthzPerRouteMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtAuthzPerRoute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtAuthzPerRouteMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtAuthzPerRoute> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExtAuthzPerRoute {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // disabled: optional bool
  pub fn has_disabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_disabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn disabled_opt(&self) -> ::std::option::Option<bool> {
    self.has_disabled().then(|| self.disabled())
  }
  pub fn disabled(&self) -> bool {
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
  pub fn set_disabled(&mut self, val: bool) {
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

  // check_settings: optional message envoy.extensions.filters.http.ext_authz.v3.CheckSettings
  pub fn has_check_settings(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_check_settings(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn check_settings_opt(&self) -> ::std::option::Option<super::CheckSettingsView<'_>> {
    self.has_check_settings().then(|| self.check_settings())
  }
  pub fn check_settings(&self) -> super::CheckSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CheckSettingsView::default())
  }
  pub fn check_settings_mut(&mut self) -> super::CheckSettingsMut<'_> {
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
  pub fn set_check_settings(&mut self,
    val: impl ::protobuf::IntoProxied<super::CheckSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn r#override(&self) -> super::ext_authz_per_route::OverrideOneof<'_> {
    match &self.r#override_case() {
      super::ext_authz_per_route::OverrideCase::Disabled =>
          super::ext_authz_per_route::OverrideOneof::Disabled(self.disabled()),
      super::ext_authz_per_route::OverrideCase::CheckSettings =>
          super::ext_authz_per_route::OverrideOneof::CheckSettings(self.check_settings()),
      _ => super::ext_authz_per_route::OverrideOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#override_case(&self) -> super::ext_authz_per_route::OverrideCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::ext_authz_per_route::OverrideCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExtAuthzPerRouteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtAuthzPerRouteMut<'_> {}

// SAFETY:
// - `ExtAuthzPerRouteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtAuthzPerRouteMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtAuthzPerRouteMut<'msg> {
  type Proxied = ExtAuthzPerRoute;
  fn as_view(&self) -> ::protobuf::View<'_, ExtAuthzPerRoute> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtAuthzPerRouteMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExtAuthzPerRoute>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtAuthzPerRouteMut<'msg> {
  type MutProxied = ExtAuthzPerRoute;
  fn as_mut(&mut self) -> ExtAuthzPerRouteMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtAuthzPerRouteMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtAuthzPerRouteMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExtAuthzPerRoute {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExtAuthzPerRoute> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtAuthzPerRouteView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtAuthzPerRouteMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // disabled: optional bool
  pub fn has_disabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_disabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn disabled_opt(&self) -> ::std::option::Option<bool> {
    self.has_disabled().then(|| self.disabled())
  }
  pub fn disabled(&self) -> bool {
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
  pub fn set_disabled(&mut self, val: bool) {
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

  // check_settings: optional message envoy.extensions.filters.http.ext_authz.v3.CheckSettings
  pub fn has_check_settings(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_check_settings(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn check_settings_opt(&self) -> ::std::option::Option<super::CheckSettingsView<'_>> {
    self.has_check_settings().then(|| self.check_settings())
  }
  pub fn check_settings(&self) -> super::CheckSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CheckSettingsView::default())
  }
  pub fn check_settings_mut(&mut self) -> super::CheckSettingsMut<'_> {
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
  pub fn set_check_settings(&mut self,
    val: impl ::protobuf::IntoProxied<super::CheckSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn r#override(&self) -> super::ext_authz_per_route::OverrideOneof<'_> {
    match &self.r#override_case() {
      super::ext_authz_per_route::OverrideCase::Disabled =>
          super::ext_authz_per_route::OverrideOneof::Disabled(self.disabled()),
      super::ext_authz_per_route::OverrideCase::CheckSettings =>
          super::ext_authz_per_route::OverrideOneof::CheckSettings(self.check_settings()),
      _ => super::ext_authz_per_route::OverrideOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#override_case(&self) -> super::ext_authz_per_route::OverrideCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::ext_authz_per_route::OverrideCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ExtAuthzPerRoute

impl ::std::ops::Drop for ExtAuthzPerRoute {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExtAuthzPerRoute {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExtAuthzPerRoute {
  type Proxied = Self;
  fn as_view(&self) -> ExtAuthzPerRouteView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExtAuthzPerRoute {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtAuthzPerRouteMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExtAuthzPerRoute {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0authz__v3__ExtAuthzPerRoute_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/3^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0authz__v3__ExtAuthzPerRoute_msg_init.0, &[<super::CheckSettings as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0authz__v3__ExtAuthzPerRoute_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtAuthzPerRoute {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtAuthzPerRoute {
  type Msg = ExtAuthzPerRoute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtAuthzPerRoute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtAuthzPerRoute {
  type Msg = ExtAuthzPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtAuthzPerRoute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtAuthzPerRouteMut<'_> {
  type Msg = ExtAuthzPerRoute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtAuthzPerRoute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtAuthzPerRouteMut<'_> {
  type Msg = ExtAuthzPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtAuthzPerRoute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtAuthzPerRouteView<'_> {
  type Msg = ExtAuthzPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtAuthzPerRoute> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtAuthzPerRouteMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod ext_authz_per_route {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum OverrideOneof<'msg> {
  Disabled(bool) = 1,
  CheckSettings(::protobuf::View<'msg, super::super::CheckSettings>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum OverrideCase {
  Disabled = 1,
  CheckSettings = 2,

  not_set = 0
}

impl OverrideCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<OverrideCase> {
    match v {
      0 => Some(OverrideCase::not_set),
      1 => Some(OverrideCase::Disabled),
      2 => Some(OverrideCase::CheckSettings),
      _ => None
    }
  }
}
}  // pub mod ext_authz_per_route


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0authz__v3__CheckSettings_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CheckSettings {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CheckSettings>
}

impl ::protobuf::Message for CheckSettings {
  type MessageView<'msg> = CheckSettingsView<'msg>;
  type MessageMut<'msg> = CheckSettingsMut<'msg>;
}

impl ::std::default::Default for CheckSettings {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CheckSettings {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CheckSettings` is `Sync` because it does not implement interior mutability.
//    Neither does `CheckSettingsMut`.
unsafe impl ::std::marker::Sync for CheckSettings {}

// SAFETY:
// - `CheckSettings` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CheckSettings {}

impl ::protobuf::Proxied for CheckSettings {
  type View<'msg> = CheckSettingsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CheckSettings {}

impl ::protobuf::MutProxied for CheckSettings {
  type Mut<'msg> = CheckSettingsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CheckSettingsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CheckSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckSettingsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CheckSettingsView<'msg> {
  type Message = CheckSettings;
}

impl ::std::fmt::Debug for CheckSettingsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CheckSettingsView<'_> {
  fn default() -> CheckSettingsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CheckSettings>> for CheckSettingsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CheckSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CheckSettingsView<'msg> {

  pub fn to_owned(&self) -> CheckSettings {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // context_extensions: repeated message envoy.extensions.filters.http.ext_authz.v3.CheckSettings.ContextExtensionsEntry
  pub fn context_extensions(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // disable_request_body_buffering: optional bool
  pub fn disable_request_body_buffering(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

  // with_request_body: optional message envoy.extensions.filters.http.ext_authz.v3.BufferSettings
  pub fn has_with_request_body(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn with_request_body_opt(self) -> ::std::option::Option<super::BufferSettingsView<'msg>> {
    self.has_with_request_body().then(|| self.with_request_body())
  }
  pub fn with_request_body(self) -> super::BufferSettingsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BufferSettingsView::default())
  }

  // grpc_service: optional message envoy.config.core.v3.GrpcService
  pub fn has_grpc_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn grpc_service_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'msg>> {
    self.has_grpc_service().then(|| self.grpc_service())
  }
  pub fn grpc_service(self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }

  // http_service: optional message envoy.extensions.filters.http.ext_authz.v3.HttpService
  pub fn has_http_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn http_service_opt(self) -> ::std::option::Option<super::HttpServiceView<'msg>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(self) -> super::HttpServiceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpServiceView::default())
  }

  pub fn service_override(self) -> super::check_settings::ServiceOverrideOneof<'msg> {
    match self.service_override_case() {
      super::check_settings::ServiceOverrideCase::GrpcService =>
          super::check_settings::ServiceOverrideOneof::GrpcService(self.grpc_service()),
      super::check_settings::ServiceOverrideCase::HttpService =>
          super::check_settings::ServiceOverrideOneof::HttpService(self.http_service()),
      _ => super::check_settings::ServiceOverrideOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn service_override_case(self) -> super::check_settings::ServiceOverrideCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::check_settings::ServiceOverrideCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CheckSettingsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CheckSettingsView<'_> {}

// SAFETY:
// - `CheckSettingsView` is `Send` because while its alive a `CheckSettingsMut` cannot.
// - `CheckSettingsView` does not use thread-local data.
unsafe impl ::std::marker::Send for CheckSettingsView<'_> {}

impl<'msg> ::protobuf::AsView for CheckSettingsView<'msg> {
  type Proxied = CheckSettings;
  fn as_view(&self) -> ::protobuf::View<'msg, CheckSettings> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckSettingsView<'msg> {
  fn into_view<'shorter>(self) -> CheckSettingsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckSettings> for CheckSettingsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckSettings {
    let mut dst = CheckSettings::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckSettings> for CheckSettingsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckSettings {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CheckSettings {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CheckSettingsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CheckSettingsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CheckSettingsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckSettingsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CheckSettingsMut<'msg> {
  type Message = CheckSettings;
}

impl ::std::fmt::Debug for CheckSettingsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CheckSettings>> for CheckSettingsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CheckSettingsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckSettings> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CheckSettings {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // context_extensions: repeated message envoy.extensions.filters.http.ext_authz.v3.CheckSettings.ContextExtensionsEntry
  pub fn context_extensions(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn context_extensions_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_context_extensions(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // disable_request_body_buffering: optional bool
  pub fn disable_request_body_buffering(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_disable_request_body_buffering(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // with_request_body: optional message envoy.extensions.filters.http.ext_authz.v3.BufferSettings
  pub fn has_with_request_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_with_request_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn with_request_body_opt(&self) -> ::std::option::Option<super::BufferSettingsView<'_>> {
    self.has_with_request_body().then(|| self.with_request_body())
  }
  pub fn with_request_body(&self) -> super::BufferSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BufferSettingsView::default())
  }
  pub fn with_request_body_mut(&mut self) -> super::BufferSettingsMut<'_> {
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
  pub fn set_with_request_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::BufferSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // grpc_service: optional message envoy.config.core.v3.GrpcService
  pub fn has_grpc_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_grpc_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn grpc_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_>> {
    self.has_grpc_service().then(|| self.grpc_service())
  }
  pub fn grpc_service(&self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }
  pub fn grpc_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceMut<'_> {
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
  pub fn set_grpc_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // http_service: optional message envoy.extensions.filters.http.ext_authz.v3.HttpService
  pub fn has_http_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_http_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn http_service_opt(&self) -> ::std::option::Option<super::HttpServiceView<'_>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(&self) -> super::HttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpServiceView::default())
  }
  pub fn http_service_mut(&mut self) -> super::HttpServiceMut<'_> {
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
  pub fn set_http_service(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn service_override(&self) -> super::check_settings::ServiceOverrideOneof<'_> {
    match &self.service_override_case() {
      super::check_settings::ServiceOverrideCase::GrpcService =>
          super::check_settings::ServiceOverrideOneof::GrpcService(self.grpc_service()),
      super::check_settings::ServiceOverrideCase::HttpService =>
          super::check_settings::ServiceOverrideOneof::HttpService(self.http_service()),
      _ => super::check_settings::ServiceOverrideOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn service_override_case(&self) -> super::check_settings::ServiceOverrideCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::check_settings::ServiceOverrideCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CheckSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CheckSettingsMut<'_> {}

// SAFETY:
// - `CheckSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CheckSettingsMut<'_> {}

impl<'msg> ::protobuf::AsView for CheckSettingsMut<'msg> {
  type Proxied = CheckSettings;
  fn as_view(&self) -> ::protobuf::View<'_, CheckSettings> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckSettingsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CheckSettings>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CheckSettingsMut<'msg> {
  type MutProxied = CheckSettings;
  fn as_mut(&mut self) -> CheckSettingsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CheckSettingsMut<'msg> {
  fn into_mut<'shorter>(self) -> CheckSettingsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CheckSettings {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CheckSettings> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CheckSettingsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CheckSettingsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // context_extensions: repeated message envoy.extensions.filters.http.ext_authz.v3.CheckSettings.ContextExtensionsEntry
  pub fn context_extensions(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn context_extensions_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_context_extensions(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // disable_request_body_buffering: optional bool
  pub fn disable_request_body_buffering(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_disable_request_body_buffering(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // with_request_body: optional message envoy.extensions.filters.http.ext_authz.v3.BufferSettings
  pub fn has_with_request_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_with_request_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn with_request_body_opt(&self) -> ::std::option::Option<super::BufferSettingsView<'_>> {
    self.has_with_request_body().then(|| self.with_request_body())
  }
  pub fn with_request_body(&self) -> super::BufferSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BufferSettingsView::default())
  }
  pub fn with_request_body_mut(&mut self) -> super::BufferSettingsMut<'_> {
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
  pub fn set_with_request_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::BufferSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // grpc_service: optional message envoy.config.core.v3.GrpcService
  pub fn has_grpc_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_grpc_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn grpc_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_>> {
    self.has_grpc_service().then(|| self.grpc_service())
  }
  pub fn grpc_service(&self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }
  pub fn grpc_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceMut<'_> {
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
  pub fn set_grpc_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // http_service: optional message envoy.extensions.filters.http.ext_authz.v3.HttpService
  pub fn has_http_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_http_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn http_service_opt(&self) -> ::std::option::Option<super::HttpServiceView<'_>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(&self) -> super::HttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpServiceView::default())
  }
  pub fn http_service_mut(&mut self) -> super::HttpServiceMut<'_> {
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
  pub fn set_http_service(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn service_override(&self) -> super::check_settings::ServiceOverrideOneof<'_> {
    match &self.service_override_case() {
      super::check_settings::ServiceOverrideCase::GrpcService =>
          super::check_settings::ServiceOverrideOneof::GrpcService(self.grpc_service()),
      super::check_settings::ServiceOverrideCase::HttpService =>
          super::check_settings::ServiceOverrideOneof::HttpService(self.http_service()),
      _ => super::check_settings::ServiceOverrideOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn service_override_case(&self) -> super::check_settings::ServiceOverrideCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::check_settings::ServiceOverrideCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl CheckSettings

impl ::std::ops::Drop for CheckSettings {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CheckSettings {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CheckSettings {
  type Proxied = Self;
  fn as_view(&self) -> CheckSettingsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CheckSettings {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CheckSettingsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CheckSettings {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0authz__v3__CheckSettings_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G/P333^%|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0authz__v3__CheckSettings_msg_init.0, &[<super::check_settings::ContextExtensionsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::BufferSettings as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0authz__v3__CheckSettings_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CheckSettings {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CheckSettings {
  type Msg = CheckSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckSettings {
  type Msg = CheckSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CheckSettingsMut<'_> {
  type Msg = CheckSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckSettingsMut<'_> {
  type Msg = CheckSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckSettingsView<'_> {
  type Msg = CheckSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckSettings> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CheckSettingsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod check_settings {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0authz__v3__CheckSettings__ContextExtensionsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct ContextExtensionsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ContextExtensionsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::check_settings::envoy__extensions__filters__http__ext_0authz__v3__CheckSettings__ContextExtensionsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::check_settings::envoy__extensions__filters__http__ext_0authz__v3__CheckSettings__ContextExtensionsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::check_settings::envoy__extensions__filters__http__ext_0authz__v3__CheckSettings__ContextExtensionsEntry_msg_init.0)
      }).0
    }
  }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ServiceOverrideOneof<'msg> {
  GrpcService(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>) = 4,
  HttpService(::protobuf::View<'msg, super::super::HttpService>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ServiceOverrideCase {
  GrpcService = 4,
  HttpService = 5,

  not_set = 0
}

impl ServiceOverrideCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ServiceOverrideCase> {
    match v {
      0 => Some(ServiceOverrideCase::not_set),
      4 => Some(ServiceOverrideCase::GrpcService),
      5 => Some(ServiceOverrideCase::HttpService),
      _ => None
    }
  }
}
}  // pub mod check_settings


