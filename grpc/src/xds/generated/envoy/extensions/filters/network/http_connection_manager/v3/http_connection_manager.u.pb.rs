const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpConnectionManager {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpConnectionManager>
}

impl ::protobuf::Message for HttpConnectionManager {
  type MessageView<'msg> = HttpConnectionManagerView<'msg>;
  type MessageMut<'msg> = HttpConnectionManagerMut<'msg>;
}

impl ::std::default::Default for HttpConnectionManager {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpConnectionManager {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpConnectionManager` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpConnectionManagerMut`.
unsafe impl ::std::marker::Sync for HttpConnectionManager {}

// SAFETY:
// - `HttpConnectionManager` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpConnectionManager {}

impl ::protobuf::Proxied for HttpConnectionManager {
  type View<'msg> = HttpConnectionManagerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpConnectionManager {}

impl ::protobuf::MutProxied for HttpConnectionManager {
  type Mut<'msg> = HttpConnectionManagerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpConnectionManagerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpConnectionManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpConnectionManagerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpConnectionManagerView<'msg> {
  type Message = HttpConnectionManager;
}

impl ::std::fmt::Debug for HttpConnectionManagerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpConnectionManagerView<'_> {
  fn default() -> HttpConnectionManagerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpConnectionManager>> for HttpConnectionManagerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpConnectionManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpConnectionManagerView<'msg> {

  pub fn to_owned(&self) -> HttpConnectionManager {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // codec_type: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.CodecType
  pub fn codec_type(self) -> super::http_connection_manager::CodecType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::http_connection_manager::CodecType::Auto).into()
      ).try_into().unwrap()
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // rds: optional message envoy.extensions.filters.network.http_connection_manager.v3.Rds
  pub fn has_rds(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn rds_opt(self) -> ::std::option::Option<super::RdsView<'msg>> {
    self.has_rds().then(|| self.rds())
  }
  pub fn rds(self) -> super::RdsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RdsView::default())
  }

  // route_config: optional message envoy.config.route.v3.RouteConfiguration
  pub fn has_route_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn route_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'msg>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(self) -> crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView::default())
  }

  // scoped_routes: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes
  pub fn has_scoped_routes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(28)
    }
  }
  pub fn scoped_routes_opt(self) -> ::std::option::Option<super::ScopedRoutesView<'msg>> {
    self.has_scoped_routes().then(|| self.scoped_routes())
  }
  pub fn scoped_routes(self) -> super::ScopedRoutesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(28)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScopedRoutesView::default())
  }

  // http_filters: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter
  pub fn http_filters(self) -> ::protobuf::RepeatedView<'msg, super::HttpFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HttpFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // add_user_agent: optional message google.protobuf.BoolValue
  pub fn has_add_user_agent(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn add_user_agent_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_add_user_agent().then(|| self.add_user_agent())
  }
  pub fn add_user_agent(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // tracing: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing
  pub fn has_tracing(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn tracing_opt(self) -> ::std::option::Option<super::http_connection_manager::TracingView<'msg>> {
    self.has_tracing().then(|| self.tracing())
  }
  pub fn tracing(self) -> super::http_connection_manager::TracingView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::TracingView::default())
  }

  // common_http_protocol_options: optional message envoy.config.core.v3.HttpProtocolOptions
  pub fn has_common_http_protocol_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(32)
    }
  }
  pub fn common_http_protocol_options_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'msg>> {
    self.has_common_http_protocol_options().then(|| self.common_http_protocol_options())
  }
  pub fn common_http_protocol_options(self) -> crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(32)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView::default())
  }

  // http1_safe_max_connection_duration: optional bool
  pub fn http1_safe_max_connection_duration(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        55, (false).into()
      ).try_into().unwrap()
    }
  }

  // http_protocol_options: optional message envoy.config.core.v3.Http1ProtocolOptions
  pub fn has_http_protocol_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn http_protocol_options_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'msg>> {
    self.has_http_protocol_options().then(|| self.http_protocol_options())
  }
  pub fn http_protocol_options(self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView::default())
  }

  // http2_protocol_options: optional message envoy.config.core.v3.Http2ProtocolOptions
  pub fn has_http2_protocol_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn http2_protocol_options_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'msg>> {
    self.has_http2_protocol_options().then(|| self.http2_protocol_options())
  }
  pub fn http2_protocol_options(self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView::default())
  }

  // http3_protocol_options: optional message envoy.config.core.v3.Http3ProtocolOptions
  pub fn has_http3_protocol_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(41)
    }
  }
  pub fn http3_protocol_options_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptionsView<'msg>> {
    self.has_http3_protocol_options().then(|| self.http3_protocol_options())
  }
  pub fn http3_protocol_options(self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(41)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptionsView::default())
  }

  // server_name: optional string
  pub fn server_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // server_header_transformation: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ServerHeaderTransformation
  pub fn server_header_transformation(self) -> super::http_connection_manager::ServerHeaderTransformation {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        31, (super::http_connection_manager::ServerHeaderTransformation::Overwrite).into()
      ).try_into().unwrap()
    }
  }

  // scheme_header_transformation: optional message envoy.config.core.v3.SchemeHeaderTransformation
  pub fn has_scheme_header_transformation(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(45)
    }
  }
  pub fn scheme_header_transformation_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformationView<'msg>> {
    self.has_scheme_header_transformation().then(|| self.scheme_header_transformation())
  }
  pub fn scheme_header_transformation(self) -> crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(45)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformationView::default())
  }

  // max_request_headers_kb: optional message google.protobuf.UInt32Value
  pub fn has_max_request_headers_kb(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn max_request_headers_kb_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_request_headers_kb().then(|| self.max_request_headers_kb())
  }
  pub fn max_request_headers_kb(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(26)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // stream_idle_timeout: optional message google.protobuf.Duration
  pub fn has_stream_idle_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn stream_idle_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_stream_idle_timeout().then(|| self.stream_idle_timeout())
  }
  pub fn stream_idle_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // stream_flush_timeout: optional message google.protobuf.Duration
  pub fn has_stream_flush_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(56)
    }
  }
  pub fn stream_flush_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_stream_flush_timeout().then(|| self.stream_flush_timeout())
  }
  pub fn stream_flush_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(56)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // request_timeout: optional message google.protobuf.Duration
  pub fn has_request_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn request_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_request_timeout().then(|| self.request_timeout())
  }
  pub fn request_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // request_headers_timeout: optional message google.protobuf.Duration
  pub fn has_request_headers_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(38)
    }
  }
  pub fn request_headers_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_request_headers_timeout().then(|| self.request_headers_timeout())
  }
  pub fn request_headers_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(38)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // drain_timeout: optional message google.protobuf.Duration
  pub fn has_drain_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn drain_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_drain_timeout().then(|| self.drain_timeout())
  }
  pub fn drain_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // delayed_close_timeout: optional message google.protobuf.Duration
  pub fn has_delayed_close_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn delayed_close_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_delayed_close_timeout().then(|| self.delayed_close_timeout())
  }
  pub fn delayed_close_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // access_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn access_log(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        11
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // access_log_flush_interval: optional message google.protobuf.Duration
  pub fn has_access_log_flush_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(51)
    }
  }
  pub fn access_log_flush_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_access_log_flush_interval().then(|| self.access_log_flush_interval())
  }
  pub fn access_log_flush_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(51)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // flush_access_log_on_new_request: optional bool
  pub fn flush_access_log_on_new_request(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        52, (false).into()
      ).try_into().unwrap()
    }
  }

  // access_log_options: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.HcmAccessLogOptions
  pub fn has_access_log_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(53)
    }
  }
  pub fn access_log_options_opt(self) -> ::std::option::Option<super::http_connection_manager::HcmAccessLogOptionsView<'msg>> {
    self.has_access_log_options().then(|| self.access_log_options())
  }
  pub fn access_log_options(self) -> super::http_connection_manager::HcmAccessLogOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(53)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::HcmAccessLogOptionsView::default())
  }

  // use_remote_address: optional message google.protobuf.BoolValue
  pub fn has_use_remote_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn use_remote_address_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_use_remote_address().then(|| self.use_remote_address())
  }
  pub fn use_remote_address(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // xff_num_trusted_hops: optional uint32
  pub fn xff_num_trusted_hops(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        17, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // original_ip_detection_extensions: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn original_ip_detection_extensions(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        43
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // early_header_mutation_extensions: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn early_header_mutation_extensions(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        49
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // internal_address_config: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.InternalAddressConfig
  pub fn has_internal_address_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn internal_address_config_opt(self) -> ::std::option::Option<super::http_connection_manager::InternalAddressConfigView<'msg>> {
    self.has_internal_address_config().then(|| self.internal_address_config())
  }
  pub fn internal_address_config(self) -> super::http_connection_manager::InternalAddressConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::InternalAddressConfigView::default())
  }

  // skip_xff_append: optional bool
  pub fn skip_xff_append(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        19, (false).into()
      ).try_into().unwrap()
    }
  }

  // via: optional string
  pub fn via(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        20, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // generate_request_id: optional message google.protobuf.BoolValue
  pub fn has_generate_request_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn generate_request_id_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_generate_request_id().then(|| self.generate_request_id())
  }
  pub fn generate_request_id(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // preserve_external_request_id: optional bool
  pub fn preserve_external_request_id(self) -> bool {
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

  // always_set_request_id_in_response: optional bool
  pub fn always_set_request_id_in_response(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        34, (false).into()
      ).try_into().unwrap()
    }
  }

  // forward_client_cert_details: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ForwardClientCertDetails
  pub fn forward_client_cert_details(self) -> super::http_connection_manager::ForwardClientCertDetails {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (super::http_connection_manager::ForwardClientCertDetails::Sanitize).into()
      ).try_into().unwrap()
    }
  }

  // set_current_client_cert_details: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.SetCurrentClientCertDetails
  pub fn has_set_current_client_cert_details(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn set_current_client_cert_details_opt(self) -> ::std::option::Option<super::http_connection_manager::SetCurrentClientCertDetailsView<'msg>> {
    self.has_set_current_client_cert_details().then(|| self.set_current_client_cert_details())
  }
  pub fn set_current_client_cert_details(self) -> super::http_connection_manager::SetCurrentClientCertDetailsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::SetCurrentClientCertDetailsView::default())
  }

  // forward_client_cert_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_forward_client_cert_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(57)
    }
  }
  pub fn forward_client_cert_matcher_opt(self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg>> {
    self.has_forward_client_cert_matcher().then(|| self.forward_client_cert_matcher())
  }
  pub fn forward_client_cert_matcher(self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(57)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }

  // proxy_100_continue: optional bool
  pub fn proxy_100_continue(self) -> bool {
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

  // represent_ipv4_remote_address_as_ipv4_mapped_ipv6: optional bool
  pub fn represent_ipv4_remote_address_as_ipv4_mapped_ipv6(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        18, (false).into()
      ).try_into().unwrap()
    }
  }

  // upgrade_configs: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.UpgradeConfig
  pub fn upgrade_configs(self) -> ::protobuf::RepeatedView<'msg, super::http_connection_manager::UpgradeConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        21
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http_connection_manager::UpgradeConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // normalize_path: optional message google.protobuf.BoolValue
  pub fn has_normalize_path(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(27)
    }
  }
  pub fn normalize_path_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_normalize_path().then(|| self.normalize_path())
  }
  pub fn normalize_path(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(27)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // merge_slashes: optional bool
  pub fn merge_slashes(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        30, (false).into()
      ).try_into().unwrap()
    }
  }

  // path_with_escaped_slashes_action: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.PathWithEscapedSlashesAction
  pub fn path_with_escaped_slashes_action(self) -> super::http_connection_manager::PathWithEscapedSlashesAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        42, (super::http_connection_manager::PathWithEscapedSlashesAction::ImplementationSpecificDefault).into()
      ).try_into().unwrap()
    }
  }

  // request_id_extension: optional message envoy.extensions.filters.network.http_connection_manager.v3.RequestIDExtension
  pub fn has_request_id_extension(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(33)
    }
  }
  pub fn request_id_extension_opt(self) -> ::std::option::Option<super::RequestIDExtensionView<'msg>> {
    self.has_request_id_extension().then(|| self.request_id_extension())
  }
  pub fn request_id_extension(self) -> super::RequestIDExtensionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(33)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RequestIDExtensionView::default())
  }

  // local_reply_config: optional message envoy.extensions.filters.network.http_connection_manager.v3.LocalReplyConfig
  pub fn has_local_reply_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(35)
    }
  }
  pub fn local_reply_config_opt(self) -> ::std::option::Option<super::LocalReplyConfigView<'msg>> {
    self.has_local_reply_config().then(|| self.local_reply_config())
  }
  pub fn local_reply_config(self) -> super::LocalReplyConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(35)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LocalReplyConfigView::default())
  }

  // strip_matching_host_port: optional bool
  pub fn strip_matching_host_port(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        36, (false).into()
      ).try_into().unwrap()
    }
  }

  // strip_any_host_port: optional bool
  pub fn has_strip_any_host_port(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(39)
    }
  }
  pub fn strip_any_host_port_opt(self) -> ::std::option::Option<bool> {
    self.has_strip_any_host_port().then(|| self.strip_any_host_port())
  }
  pub fn strip_any_host_port(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        39, (false).into()
      ).try_into().unwrap()
    }
  }

  // stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_stream_error_on_invalid_http_message(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(37)
    }
  }
  pub fn stream_error_on_invalid_http_message_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_stream_error_on_invalid_http_message().then(|| self.stream_error_on_invalid_http_message())
  }
  pub fn stream_error_on_invalid_http_message(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(37)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // path_normalization_options: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.PathNormalizationOptions
  pub fn has_path_normalization_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(40)
    }
  }
  pub fn path_normalization_options_opt(self) -> ::std::option::Option<super::http_connection_manager::PathNormalizationOptionsView<'msg>> {
    self.has_path_normalization_options().then(|| self.path_normalization_options())
  }
  pub fn path_normalization_options(self) -> super::http_connection_manager::PathNormalizationOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(40)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::PathNormalizationOptionsView::default())
  }

  // strip_trailing_host_dot: optional bool
  pub fn strip_trailing_host_dot(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        44, (false).into()
      ).try_into().unwrap()
    }
  }

  // proxy_status_config: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ProxyStatusConfig
  pub fn has_proxy_status_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(46)
    }
  }
  pub fn proxy_status_config_opt(self) -> ::std::option::Option<super::http_connection_manager::ProxyStatusConfigView<'msg>> {
    self.has_proxy_status_config().then(|| self.proxy_status_config())
  }
  pub fn proxy_status_config(self) -> super::http_connection_manager::ProxyStatusConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(46)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::ProxyStatusConfigView::default())
  }

  // typed_header_validation_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_header_validation_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(47)
    }
  }
  pub fn typed_header_validation_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_typed_header_validation_config().then(|| self.typed_header_validation_config())
  }
  pub fn typed_header_validation_config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(47)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // append_x_forwarded_port: optional bool
  pub fn append_x_forwarded_port(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        48, (false).into()
      ).try_into().unwrap()
    }
  }

  // append_local_overload: optional bool
  pub fn append_local_overload(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        54, (false).into()
      ).try_into().unwrap()
    }
  }

  // add_proxy_protocol_connection_state: optional message google.protobuf.BoolValue
  pub fn has_add_proxy_protocol_connection_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(50)
    }
  }
  pub fn add_proxy_protocol_connection_state_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_add_proxy_protocol_connection_state().then(|| self.add_proxy_protocol_connection_state())
  }
  pub fn add_proxy_protocol_connection_state(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(50)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  pub fn route_specifier(self) -> super::http_connection_manager::RouteSpecifierOneof<'msg> {
    match self.route_specifier_case() {
      super::http_connection_manager::RouteSpecifierCase::Rds =>
          super::http_connection_manager::RouteSpecifierOneof::Rds(self.rds()),
      super::http_connection_manager::RouteSpecifierCase::RouteConfig =>
          super::http_connection_manager::RouteSpecifierOneof::RouteConfig(self.route_config()),
      super::http_connection_manager::RouteSpecifierCase::ScopedRoutes =>
          super::http_connection_manager::RouteSpecifierOneof::ScopedRoutes(self.scoped_routes()),
      _ => super::http_connection_manager::RouteSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn route_specifier_case(self) -> super::http_connection_manager::RouteSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::http_connection_manager::RouteSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
  pub fn strip_port_mode(self) -> super::http_connection_manager::StripPortModeOneof<'msg> {
    match self.strip_port_mode_case() {
      super::http_connection_manager::StripPortModeCase::StripAnyHostPort =>
          super::http_connection_manager::StripPortModeOneof::StripAnyHostPort(self.strip_any_host_port()),
      _ => super::http_connection_manager::StripPortModeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn strip_port_mode_case(self) -> super::http_connection_manager::StripPortModeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(39);
      super::http_connection_manager::StripPortModeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HttpConnectionManagerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpConnectionManagerView<'_> {}

// SAFETY:
// - `HttpConnectionManagerView` is `Send` because while its alive a `HttpConnectionManagerMut` cannot.
// - `HttpConnectionManagerView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpConnectionManagerView<'_> {}

impl<'msg> ::protobuf::AsView for HttpConnectionManagerView<'msg> {
  type Proxied = HttpConnectionManager;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpConnectionManager> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpConnectionManagerView<'msg> {
  fn into_view<'shorter>(self) -> HttpConnectionManagerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpConnectionManager> for HttpConnectionManagerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpConnectionManager {
    let mut dst = HttpConnectionManager::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpConnectionManager> for HttpConnectionManagerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpConnectionManager {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpConnectionManager {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpConnectionManagerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpConnectionManagerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpConnectionManagerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpConnectionManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpConnectionManagerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpConnectionManagerMut<'msg> {
  type Message = HttpConnectionManager;
}

impl ::std::fmt::Debug for HttpConnectionManagerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpConnectionManager>> for HttpConnectionManagerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpConnectionManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpConnectionManagerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpConnectionManager> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpConnectionManager {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // codec_type: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.CodecType
  pub fn codec_type(&self) -> super::http_connection_manager::CodecType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::http_connection_manager::CodecType::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_codec_type(&mut self, val: super::http_connection_manager::CodecType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // rds: optional message envoy.extensions.filters.network.http_connection_manager.v3.Rds
  pub fn has_rds(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_rds(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn rds_opt(&self) -> ::std::option::Option<super::RdsView<'_>> {
    self.has_rds().then(|| self.rds())
  }
  pub fn rds(&self) -> super::RdsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RdsView::default())
  }
  pub fn rds_mut(&mut self) -> super::RdsMut<'_> {
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
  pub fn set_rds(&mut self,
    val: impl ::protobuf::IntoProxied<super::Rds>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // route_config: optional message envoy.config.route.v3.RouteConfiguration
  pub fn has_route_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_route_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn route_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'_>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(&self) -> crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView::default())
  }
  pub fn route_config_mut(&mut self) -> crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationMut<'_> {
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
  pub fn set_route_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::route::v3::route::RouteConfiguration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // scoped_routes: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes
  pub fn has_scoped_routes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(28)
    }
  }
  pub fn clear_scoped_routes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        28
      );
    }
  }
  pub fn scoped_routes_opt(&self) -> ::std::option::Option<super::ScopedRoutesView<'_>> {
    self.has_scoped_routes().then(|| self.scoped_routes())
  }
  pub fn scoped_routes(&self) -> super::ScopedRoutesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(28)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScopedRoutesView::default())
  }
  pub fn scoped_routes_mut(&mut self) -> super::ScopedRoutesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         28, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_scoped_routes(&mut self,
    val: impl ::protobuf::IntoProxied<super::ScopedRoutes>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        28,
        val
      );
    }
  }

  // http_filters: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter
  pub fn http_filters(&self) -> ::protobuf::RepeatedView<'_, super::HttpFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HttpFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn http_filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HttpFilter> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_http_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HttpFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // add_user_agent: optional message google.protobuf.BoolValue
  pub fn has_add_user_agent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_add_user_agent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn add_user_agent_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_add_user_agent().then(|| self.add_user_agent())
  }
  pub fn add_user_agent(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn add_user_agent_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_add_user_agent(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // tracing: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing
  pub fn has_tracing(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_tracing(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn tracing_opt(&self) -> ::std::option::Option<super::http_connection_manager::TracingView<'_>> {
    self.has_tracing().then(|| self.tracing())
  }
  pub fn tracing(&self) -> super::http_connection_manager::TracingView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::TracingView::default())
  }
  pub fn tracing_mut(&mut self) -> super::http_connection_manager::TracingMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tracing(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::Tracing>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // common_http_protocol_options: optional message envoy.config.core.v3.HttpProtocolOptions
  pub fn has_common_http_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(32)
    }
  }
  pub fn clear_common_http_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        32
      );
    }
  }
  pub fn common_http_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'_>> {
    self.has_common_http_protocol_options().then(|| self.common_http_protocol_options())
  }
  pub fn common_http_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(32)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView::default())
  }
  pub fn common_http_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         32, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_common_http_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        32,
        val
      );
    }
  }

  // http1_safe_max_connection_duration: optional bool
  pub fn http1_safe_max_connection_duration(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        55, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_http1_safe_max_connection_duration(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        55, val.into()
      )
    }
  }

  // http_protocol_options: optional message envoy.config.core.v3.Http1ProtocolOptions
  pub fn has_http_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_http_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn http_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'_>> {
    self.has_http_protocol_options().then(|| self.http_protocol_options())
  }
  pub fn http_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView::default())
  }
  pub fn http_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsMut<'_> {
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
  pub fn set_http_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // http2_protocol_options: optional message envoy.config.core.v3.Http2ProtocolOptions
  pub fn has_http2_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_http2_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn http2_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'_>> {
    self.has_http2_protocol_options().then(|| self.http2_protocol_options())
  }
  pub fn http2_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView::default())
  }
  pub fn http2_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_http2_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // http3_protocol_options: optional message envoy.config.core.v3.Http3ProtocolOptions
  pub fn has_http3_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(41)
    }
  }
  pub fn clear_http3_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        41
      );
    }
  }
  pub fn http3_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptionsView<'_>> {
    self.has_http3_protocol_options().then(|| self.http3_protocol_options())
  }
  pub fn http3_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(41)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptionsView::default())
  }
  pub fn http3_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         41, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_http3_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        41,
        val
      );
    }
  }

  // server_name: optional string
  pub fn server_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_server_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // server_header_transformation: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ServerHeaderTransformation
  pub fn server_header_transformation(&self) -> super::http_connection_manager::ServerHeaderTransformation {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        31, (super::http_connection_manager::ServerHeaderTransformation::Overwrite).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_server_header_transformation(&mut self, val: super::http_connection_manager::ServerHeaderTransformation) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        31, val.into()
      )
    }
  }

  // scheme_header_transformation: optional message envoy.config.core.v3.SchemeHeaderTransformation
  pub fn has_scheme_header_transformation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(45)
    }
  }
  pub fn clear_scheme_header_transformation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        45
      );
    }
  }
  pub fn scheme_header_transformation_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformationView<'_>> {
    self.has_scheme_header_transformation().then(|| self.scheme_header_transformation())
  }
  pub fn scheme_header_transformation(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(45)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformationView::default())
  }
  pub fn scheme_header_transformation_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         45, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_scheme_header_transformation(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        45,
        val
      );
    }
  }

  // max_request_headers_kb: optional message google.protobuf.UInt32Value
  pub fn has_max_request_headers_kb(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn clear_max_request_headers_kb(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        26
      );
    }
  }
  pub fn max_request_headers_kb_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_request_headers_kb().then(|| self.max_request_headers_kb())
  }
  pub fn max_request_headers_kb(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(26)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_request_headers_kb_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_request_headers_kb(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        26,
        val
      );
    }
  }

  // stream_idle_timeout: optional message google.protobuf.Duration
  pub fn has_stream_idle_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_stream_idle_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn stream_idle_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_stream_idle_timeout().then(|| self.stream_idle_timeout())
  }
  pub fn stream_idle_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn stream_idle_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         22, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stream_idle_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // stream_flush_timeout: optional message google.protobuf.Duration
  pub fn has_stream_flush_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(56)
    }
  }
  pub fn clear_stream_flush_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        56
      );
    }
  }
  pub fn stream_flush_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_stream_flush_timeout().then(|| self.stream_flush_timeout())
  }
  pub fn stream_flush_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(56)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn stream_flush_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         56, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stream_flush_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        56,
        val
      );
    }
  }

  // request_timeout: optional message google.protobuf.Duration
  pub fn has_request_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn clear_request_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        25
      );
    }
  }
  pub fn request_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_request_timeout().then(|| self.request_timeout())
  }
  pub fn request_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn request_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_request_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val
      );
    }
  }

  // request_headers_timeout: optional message google.protobuf.Duration
  pub fn has_request_headers_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(38)
    }
  }
  pub fn clear_request_headers_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        38
      );
    }
  }
  pub fn request_headers_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_request_headers_timeout().then(|| self.request_headers_timeout())
  }
  pub fn request_headers_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(38)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn request_headers_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         38, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_request_headers_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        38,
        val
      );
    }
  }

  // drain_timeout: optional message google.protobuf.Duration
  pub fn has_drain_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_drain_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn drain_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_drain_timeout().then(|| self.drain_timeout())
  }
  pub fn drain_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn drain_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_drain_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // delayed_close_timeout: optional message google.protobuf.Duration
  pub fn has_delayed_close_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_delayed_close_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn delayed_close_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_delayed_close_timeout().then(|| self.delayed_close_timeout())
  }
  pub fn delayed_close_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn delayed_close_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_delayed_close_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // access_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn access_log(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        11
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn access_log_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        11,
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
  pub fn set_access_log(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        src);
    }
  }

  // access_log_flush_interval: optional message google.protobuf.Duration
  pub fn has_access_log_flush_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(51)
    }
  }
  pub fn clear_access_log_flush_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        51
      );
    }
  }
  pub fn access_log_flush_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_access_log_flush_interval().then(|| self.access_log_flush_interval())
  }
  pub fn access_log_flush_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(51)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn access_log_flush_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         51, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_access_log_flush_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        51,
        val
      );
    }
  }

  // flush_access_log_on_new_request: optional bool
  pub fn flush_access_log_on_new_request(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        52, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_flush_access_log_on_new_request(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        52, val.into()
      )
    }
  }

  // access_log_options: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.HcmAccessLogOptions
  pub fn has_access_log_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(53)
    }
  }
  pub fn clear_access_log_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        53
      );
    }
  }
  pub fn access_log_options_opt(&self) -> ::std::option::Option<super::http_connection_manager::HcmAccessLogOptionsView<'_>> {
    self.has_access_log_options().then(|| self.access_log_options())
  }
  pub fn access_log_options(&self) -> super::http_connection_manager::HcmAccessLogOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(53)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::HcmAccessLogOptionsView::default())
  }
  pub fn access_log_options_mut(&mut self) -> super::http_connection_manager::HcmAccessLogOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         53, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_access_log_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::HcmAccessLogOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        53,
        val
      );
    }
  }

  // use_remote_address: optional message google.protobuf.BoolValue
  pub fn has_use_remote_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_use_remote_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn use_remote_address_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_remote_address().then(|| self.use_remote_address())
  }
  pub fn use_remote_address(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_remote_address_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_remote_address(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // xff_num_trusted_hops: optional uint32
  pub fn xff_num_trusted_hops(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        17, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_xff_num_trusted_hops(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        17, val.into()
      )
    }
  }

  // original_ip_detection_extensions: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn original_ip_detection_extensions(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        43
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn original_ip_detection_extensions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        43,
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
  pub fn set_original_ip_detection_extensions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        43,
        src);
    }
  }

  // early_header_mutation_extensions: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn early_header_mutation_extensions(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        49
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn early_header_mutation_extensions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        49,
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
  pub fn set_early_header_mutation_extensions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        49,
        src);
    }
  }

  // internal_address_config: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.InternalAddressConfig
  pub fn has_internal_address_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_internal_address_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn internal_address_config_opt(&self) -> ::std::option::Option<super::http_connection_manager::InternalAddressConfigView<'_>> {
    self.has_internal_address_config().then(|| self.internal_address_config())
  }
  pub fn internal_address_config(&self) -> super::http_connection_manager::InternalAddressConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::InternalAddressConfigView::default())
  }
  pub fn internal_address_config_mut(&mut self) -> super::http_connection_manager::InternalAddressConfigMut<'_> {
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
  pub fn set_internal_address_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::InternalAddressConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val
      );
    }
  }

  // skip_xff_append: optional bool
  pub fn skip_xff_append(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        19, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_skip_xff_append(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        19, val.into()
      )
    }
  }

  // via: optional string
  pub fn via(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        20, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_via(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        val);
    }
  }

  // generate_request_id: optional message google.protobuf.BoolValue
  pub fn has_generate_request_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_generate_request_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn generate_request_id_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_generate_request_id().then(|| self.generate_request_id())
  }
  pub fn generate_request_id(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn generate_request_id_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_generate_request_id(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // preserve_external_request_id: optional bool
  pub fn preserve_external_request_id(&self) -> bool {
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
  pub fn set_preserve_external_request_id(&mut self, val: bool) {
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

  // always_set_request_id_in_response: optional bool
  pub fn always_set_request_id_in_response(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        34, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_always_set_request_id_in_response(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        34, val.into()
      )
    }
  }

  // forward_client_cert_details: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ForwardClientCertDetails
  pub fn forward_client_cert_details(&self) -> super::http_connection_manager::ForwardClientCertDetails {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (super::http_connection_manager::ForwardClientCertDetails::Sanitize).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_forward_client_cert_details(&mut self, val: super::http_connection_manager::ForwardClientCertDetails) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        14, val.into()
      )
    }
  }

  // set_current_client_cert_details: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.SetCurrentClientCertDetails
  pub fn has_set_current_client_cert_details(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_set_current_client_cert_details(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn set_current_client_cert_details_opt(&self) -> ::std::option::Option<super::http_connection_manager::SetCurrentClientCertDetailsView<'_>> {
    self.has_set_current_client_cert_details().then(|| self.set_current_client_cert_details())
  }
  pub fn set_current_client_cert_details(&self) -> super::http_connection_manager::SetCurrentClientCertDetailsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::SetCurrentClientCertDetailsView::default())
  }
  pub fn set_current_client_cert_details_mut(&mut self) -> super::http_connection_manager::SetCurrentClientCertDetailsMut<'_> {
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
  pub fn set_set_current_client_cert_details(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::SetCurrentClientCertDetails>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // forward_client_cert_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_forward_client_cert_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(57)
    }
  }
  pub fn clear_forward_client_cert_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        57
      );
    }
  }
  pub fn forward_client_cert_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_forward_client_cert_matcher().then(|| self.forward_client_cert_matcher())
  }
  pub fn forward_client_cert_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(57)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn forward_client_cert_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         57, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_forward_client_cert_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        57,
        val
      );
    }
  }

  // proxy_100_continue: optional bool
  pub fn proxy_100_continue(&self) -> bool {
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
  pub fn set_proxy_100_continue(&mut self, val: bool) {
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

  // represent_ipv4_remote_address_as_ipv4_mapped_ipv6: optional bool
  pub fn represent_ipv4_remote_address_as_ipv4_mapped_ipv6(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        18, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_represent_ipv4_remote_address_as_ipv4_mapped_ipv6(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        18, val.into()
      )
    }
  }

  // upgrade_configs: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.UpgradeConfig
  pub fn upgrade_configs(&self) -> ::protobuf::RepeatedView<'_, super::http_connection_manager::UpgradeConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        21
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http_connection_manager::UpgradeConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn upgrade_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::http_connection_manager::UpgradeConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        21,
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
  pub fn set_upgrade_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::http_connection_manager::UpgradeConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        src);
    }
  }

  // normalize_path: optional message google.protobuf.BoolValue
  pub fn has_normalize_path(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(27)
    }
  }
  pub fn clear_normalize_path(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        27
      );
    }
  }
  pub fn normalize_path_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_normalize_path().then(|| self.normalize_path())
  }
  pub fn normalize_path(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(27)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn normalize_path_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         27, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_normalize_path(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        27,
        val
      );
    }
  }

  // merge_slashes: optional bool
  pub fn merge_slashes(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        30, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_merge_slashes(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        30, val.into()
      )
    }
  }

  // path_with_escaped_slashes_action: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.PathWithEscapedSlashesAction
  pub fn path_with_escaped_slashes_action(&self) -> super::http_connection_manager::PathWithEscapedSlashesAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        42, (super::http_connection_manager::PathWithEscapedSlashesAction::ImplementationSpecificDefault).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_path_with_escaped_slashes_action(&mut self, val: super::http_connection_manager::PathWithEscapedSlashesAction) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        42, val.into()
      )
    }
  }

  // request_id_extension: optional message envoy.extensions.filters.network.http_connection_manager.v3.RequestIDExtension
  pub fn has_request_id_extension(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(33)
    }
  }
  pub fn clear_request_id_extension(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        33
      );
    }
  }
  pub fn request_id_extension_opt(&self) -> ::std::option::Option<super::RequestIDExtensionView<'_>> {
    self.has_request_id_extension().then(|| self.request_id_extension())
  }
  pub fn request_id_extension(&self) -> super::RequestIDExtensionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(33)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RequestIDExtensionView::default())
  }
  pub fn request_id_extension_mut(&mut self) -> super::RequestIDExtensionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         33, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_request_id_extension(&mut self,
    val: impl ::protobuf::IntoProxied<super::RequestIDExtension>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        33,
        val
      );
    }
  }

  // local_reply_config: optional message envoy.extensions.filters.network.http_connection_manager.v3.LocalReplyConfig
  pub fn has_local_reply_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(35)
    }
  }
  pub fn clear_local_reply_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        35
      );
    }
  }
  pub fn local_reply_config_opt(&self) -> ::std::option::Option<super::LocalReplyConfigView<'_>> {
    self.has_local_reply_config().then(|| self.local_reply_config())
  }
  pub fn local_reply_config(&self) -> super::LocalReplyConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(35)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LocalReplyConfigView::default())
  }
  pub fn local_reply_config_mut(&mut self) -> super::LocalReplyConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         35, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_local_reply_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::LocalReplyConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        35,
        val
      );
    }
  }

  // strip_matching_host_port: optional bool
  pub fn strip_matching_host_port(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        36, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_strip_matching_host_port(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        36, val.into()
      )
    }
  }

  // strip_any_host_port: optional bool
  pub fn has_strip_any_host_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(39)
    }
  }
  pub fn clear_strip_any_host_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        39
      );
    }
  }
  pub fn strip_any_host_port_opt(&self) -> ::std::option::Option<bool> {
    self.has_strip_any_host_port().then(|| self.strip_any_host_port())
  }
  pub fn strip_any_host_port(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        39, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_strip_any_host_port(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        39, val.into()
      )
    }
  }

  // stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_stream_error_on_invalid_http_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(37)
    }
  }
  pub fn clear_stream_error_on_invalid_http_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        37
      );
    }
  }
  pub fn stream_error_on_invalid_http_message_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_stream_error_on_invalid_http_message().then(|| self.stream_error_on_invalid_http_message())
  }
  pub fn stream_error_on_invalid_http_message(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(37)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn stream_error_on_invalid_http_message_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         37, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stream_error_on_invalid_http_message(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        37,
        val
      );
    }
  }

  // path_normalization_options: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.PathNormalizationOptions
  pub fn has_path_normalization_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(40)
    }
  }
  pub fn clear_path_normalization_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        40
      );
    }
  }
  pub fn path_normalization_options_opt(&self) -> ::std::option::Option<super::http_connection_manager::PathNormalizationOptionsView<'_>> {
    self.has_path_normalization_options().then(|| self.path_normalization_options())
  }
  pub fn path_normalization_options(&self) -> super::http_connection_manager::PathNormalizationOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(40)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::PathNormalizationOptionsView::default())
  }
  pub fn path_normalization_options_mut(&mut self) -> super::http_connection_manager::PathNormalizationOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         40, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_path_normalization_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::PathNormalizationOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        40,
        val
      );
    }
  }

  // strip_trailing_host_dot: optional bool
  pub fn strip_trailing_host_dot(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        44, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_strip_trailing_host_dot(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        44, val.into()
      )
    }
  }

  // proxy_status_config: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ProxyStatusConfig
  pub fn has_proxy_status_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(46)
    }
  }
  pub fn clear_proxy_status_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        46
      );
    }
  }
  pub fn proxy_status_config_opt(&self) -> ::std::option::Option<super::http_connection_manager::ProxyStatusConfigView<'_>> {
    self.has_proxy_status_config().then(|| self.proxy_status_config())
  }
  pub fn proxy_status_config(&self) -> super::http_connection_manager::ProxyStatusConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(46)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::ProxyStatusConfigView::default())
  }
  pub fn proxy_status_config_mut(&mut self) -> super::http_connection_manager::ProxyStatusConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         46, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_proxy_status_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::ProxyStatusConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        46,
        val
      );
    }
  }

  // typed_header_validation_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_header_validation_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(47)
    }
  }
  pub fn clear_typed_header_validation_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        47
      );
    }
  }
  pub fn typed_header_validation_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_header_validation_config().then(|| self.typed_header_validation_config())
  }
  pub fn typed_header_validation_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(47)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_header_validation_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         47, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_typed_header_validation_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        47,
        val
      );
    }
  }

  // append_x_forwarded_port: optional bool
  pub fn append_x_forwarded_port(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        48, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_append_x_forwarded_port(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        48, val.into()
      )
    }
  }

  // append_local_overload: optional bool
  pub fn append_local_overload(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        54, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_append_local_overload(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        54, val.into()
      )
    }
  }

  // add_proxy_protocol_connection_state: optional message google.protobuf.BoolValue
  pub fn has_add_proxy_protocol_connection_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(50)
    }
  }
  pub fn clear_add_proxy_protocol_connection_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        50
      );
    }
  }
  pub fn add_proxy_protocol_connection_state_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_add_proxy_protocol_connection_state().then(|| self.add_proxy_protocol_connection_state())
  }
  pub fn add_proxy_protocol_connection_state(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(50)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn add_proxy_protocol_connection_state_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         50, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_add_proxy_protocol_connection_state(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        50,
        val
      );
    }
  }

  pub fn route_specifier(&self) -> super::http_connection_manager::RouteSpecifierOneof<'_> {
    match &self.route_specifier_case() {
      super::http_connection_manager::RouteSpecifierCase::Rds =>
          super::http_connection_manager::RouteSpecifierOneof::Rds(self.rds()),
      super::http_connection_manager::RouteSpecifierCase::RouteConfig =>
          super::http_connection_manager::RouteSpecifierOneof::RouteConfig(self.route_config()),
      super::http_connection_manager::RouteSpecifierCase::ScopedRoutes =>
          super::http_connection_manager::RouteSpecifierOneof::ScopedRoutes(self.scoped_routes()),
      _ => super::http_connection_manager::RouteSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn route_specifier_case(&self) -> super::http_connection_manager::RouteSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::http_connection_manager::RouteSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
  pub fn strip_port_mode(&self) -> super::http_connection_manager::StripPortModeOneof<'_> {
    match &self.strip_port_mode_case() {
      super::http_connection_manager::StripPortModeCase::StripAnyHostPort =>
          super::http_connection_manager::StripPortModeOneof::StripAnyHostPort(self.strip_any_host_port()),
      _ => super::http_connection_manager::StripPortModeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn strip_port_mode_case(&self) -> super::http_connection_manager::StripPortModeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(39);
      super::http_connection_manager::StripPortModeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HttpConnectionManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpConnectionManagerMut<'_> {}

// SAFETY:
// - `HttpConnectionManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpConnectionManagerMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpConnectionManagerMut<'msg> {
  type Proxied = HttpConnectionManager;
  fn as_view(&self) -> ::protobuf::View<'_, HttpConnectionManager> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpConnectionManagerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpConnectionManager>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpConnectionManagerMut<'msg> {
  type MutProxied = HttpConnectionManager;
  fn as_mut(&mut self) -> HttpConnectionManagerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpConnectionManagerMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpConnectionManagerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpConnectionManager {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpConnectionManager> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpConnectionManagerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpConnectionManagerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // codec_type: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.CodecType
  pub fn codec_type(&self) -> super::http_connection_manager::CodecType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::http_connection_manager::CodecType::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_codec_type(&mut self, val: super::http_connection_manager::CodecType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // rds: optional message envoy.extensions.filters.network.http_connection_manager.v3.Rds
  pub fn has_rds(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_rds(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn rds_opt(&self) -> ::std::option::Option<super::RdsView<'_>> {
    self.has_rds().then(|| self.rds())
  }
  pub fn rds(&self) -> super::RdsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RdsView::default())
  }
  pub fn rds_mut(&mut self) -> super::RdsMut<'_> {
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
  pub fn set_rds(&mut self,
    val: impl ::protobuf::IntoProxied<super::Rds>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // route_config: optional message envoy.config.route.v3.RouteConfiguration
  pub fn has_route_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_route_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn route_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'_>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(&self) -> crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView::default())
  }
  pub fn route_config_mut(&mut self) -> crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationMut<'_> {
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
  pub fn set_route_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::route::v3::route::RouteConfiguration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // scoped_routes: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes
  pub fn has_scoped_routes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(28)
    }
  }
  pub fn clear_scoped_routes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        28
      );
    }
  }
  pub fn scoped_routes_opt(&self) -> ::std::option::Option<super::ScopedRoutesView<'_>> {
    self.has_scoped_routes().then(|| self.scoped_routes())
  }
  pub fn scoped_routes(&self) -> super::ScopedRoutesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(28)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScopedRoutesView::default())
  }
  pub fn scoped_routes_mut(&mut self) -> super::ScopedRoutesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         28, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_scoped_routes(&mut self,
    val: impl ::protobuf::IntoProxied<super::ScopedRoutes>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        28,
        val
      );
    }
  }

  // http_filters: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter
  pub fn http_filters(&self) -> ::protobuf::RepeatedView<'_, super::HttpFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HttpFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn http_filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HttpFilter> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_http_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HttpFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // add_user_agent: optional message google.protobuf.BoolValue
  pub fn has_add_user_agent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_add_user_agent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn add_user_agent_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_add_user_agent().then(|| self.add_user_agent())
  }
  pub fn add_user_agent(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn add_user_agent_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_add_user_agent(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // tracing: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing
  pub fn has_tracing(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_tracing(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn tracing_opt(&self) -> ::std::option::Option<super::http_connection_manager::TracingView<'_>> {
    self.has_tracing().then(|| self.tracing())
  }
  pub fn tracing(&self) -> super::http_connection_manager::TracingView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::TracingView::default())
  }
  pub fn tracing_mut(&mut self) -> super::http_connection_manager::TracingMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tracing(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::Tracing>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // common_http_protocol_options: optional message envoy.config.core.v3.HttpProtocolOptions
  pub fn has_common_http_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(32)
    }
  }
  pub fn clear_common_http_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        32
      );
    }
  }
  pub fn common_http_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'_>> {
    self.has_common_http_protocol_options().then(|| self.common_http_protocol_options())
  }
  pub fn common_http_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(32)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView::default())
  }
  pub fn common_http_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         32, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_common_http_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        32,
        val
      );
    }
  }

  // http1_safe_max_connection_duration: optional bool
  pub fn http1_safe_max_connection_duration(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        55, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_http1_safe_max_connection_duration(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        55, val.into()
      )
    }
  }

  // http_protocol_options: optional message envoy.config.core.v3.Http1ProtocolOptions
  pub fn has_http_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_http_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn http_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'_>> {
    self.has_http_protocol_options().then(|| self.http_protocol_options())
  }
  pub fn http_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView::default())
  }
  pub fn http_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsMut<'_> {
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
  pub fn set_http_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // http2_protocol_options: optional message envoy.config.core.v3.Http2ProtocolOptions
  pub fn has_http2_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_http2_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn http2_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'_>> {
    self.has_http2_protocol_options().then(|| self.http2_protocol_options())
  }
  pub fn http2_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView::default())
  }
  pub fn http2_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_http2_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // http3_protocol_options: optional message envoy.config.core.v3.Http3ProtocolOptions
  pub fn has_http3_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(41)
    }
  }
  pub fn clear_http3_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        41
      );
    }
  }
  pub fn http3_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptionsView<'_>> {
    self.has_http3_protocol_options().then(|| self.http3_protocol_options())
  }
  pub fn http3_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(41)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptionsView::default())
  }
  pub fn http3_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         41, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_http3_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        41,
        val
      );
    }
  }

  // server_name: optional string
  pub fn server_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_server_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // server_header_transformation: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ServerHeaderTransformation
  pub fn server_header_transformation(&self) -> super::http_connection_manager::ServerHeaderTransformation {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        31, (super::http_connection_manager::ServerHeaderTransformation::Overwrite).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_server_header_transformation(&mut self, val: super::http_connection_manager::ServerHeaderTransformation) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        31, val.into()
      )
    }
  }

  // scheme_header_transformation: optional message envoy.config.core.v3.SchemeHeaderTransformation
  pub fn has_scheme_header_transformation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(45)
    }
  }
  pub fn clear_scheme_header_transformation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        45
      );
    }
  }
  pub fn scheme_header_transformation_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformationView<'_>> {
    self.has_scheme_header_transformation().then(|| self.scheme_header_transformation())
  }
  pub fn scheme_header_transformation(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(45)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformationView::default())
  }
  pub fn scheme_header_transformation_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         45, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_scheme_header_transformation(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        45,
        val
      );
    }
  }

  // max_request_headers_kb: optional message google.protobuf.UInt32Value
  pub fn has_max_request_headers_kb(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn clear_max_request_headers_kb(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        26
      );
    }
  }
  pub fn max_request_headers_kb_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_request_headers_kb().then(|| self.max_request_headers_kb())
  }
  pub fn max_request_headers_kb(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(26)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_request_headers_kb_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_request_headers_kb(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        26,
        val
      );
    }
  }

  // stream_idle_timeout: optional message google.protobuf.Duration
  pub fn has_stream_idle_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_stream_idle_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn stream_idle_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_stream_idle_timeout().then(|| self.stream_idle_timeout())
  }
  pub fn stream_idle_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn stream_idle_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         22, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stream_idle_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // stream_flush_timeout: optional message google.protobuf.Duration
  pub fn has_stream_flush_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(56)
    }
  }
  pub fn clear_stream_flush_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        56
      );
    }
  }
  pub fn stream_flush_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_stream_flush_timeout().then(|| self.stream_flush_timeout())
  }
  pub fn stream_flush_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(56)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn stream_flush_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         56, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stream_flush_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        56,
        val
      );
    }
  }

  // request_timeout: optional message google.protobuf.Duration
  pub fn has_request_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn clear_request_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        25
      );
    }
  }
  pub fn request_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_request_timeout().then(|| self.request_timeout())
  }
  pub fn request_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn request_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_request_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val
      );
    }
  }

  // request_headers_timeout: optional message google.protobuf.Duration
  pub fn has_request_headers_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(38)
    }
  }
  pub fn clear_request_headers_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        38
      );
    }
  }
  pub fn request_headers_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_request_headers_timeout().then(|| self.request_headers_timeout())
  }
  pub fn request_headers_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(38)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn request_headers_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         38, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_request_headers_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        38,
        val
      );
    }
  }

  // drain_timeout: optional message google.protobuf.Duration
  pub fn has_drain_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_drain_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn drain_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_drain_timeout().then(|| self.drain_timeout())
  }
  pub fn drain_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn drain_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_drain_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // delayed_close_timeout: optional message google.protobuf.Duration
  pub fn has_delayed_close_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_delayed_close_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn delayed_close_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_delayed_close_timeout().then(|| self.delayed_close_timeout())
  }
  pub fn delayed_close_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn delayed_close_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_delayed_close_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // access_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn access_log(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        11
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn access_log_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        11,
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
  pub fn set_access_log(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        src);
    }
  }

  // access_log_flush_interval: optional message google.protobuf.Duration
  pub fn has_access_log_flush_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(51)
    }
  }
  pub fn clear_access_log_flush_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        51
      );
    }
  }
  pub fn access_log_flush_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_access_log_flush_interval().then(|| self.access_log_flush_interval())
  }
  pub fn access_log_flush_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(51)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn access_log_flush_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         51, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_access_log_flush_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        51,
        val
      );
    }
  }

  // flush_access_log_on_new_request: optional bool
  pub fn flush_access_log_on_new_request(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        52, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_flush_access_log_on_new_request(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        52, val.into()
      )
    }
  }

  // access_log_options: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.HcmAccessLogOptions
  pub fn has_access_log_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(53)
    }
  }
  pub fn clear_access_log_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        53
      );
    }
  }
  pub fn access_log_options_opt(&self) -> ::std::option::Option<super::http_connection_manager::HcmAccessLogOptionsView<'_>> {
    self.has_access_log_options().then(|| self.access_log_options())
  }
  pub fn access_log_options(&self) -> super::http_connection_manager::HcmAccessLogOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(53)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::HcmAccessLogOptionsView::default())
  }
  pub fn access_log_options_mut(&mut self) -> super::http_connection_manager::HcmAccessLogOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         53, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_access_log_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::HcmAccessLogOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        53,
        val
      );
    }
  }

  // use_remote_address: optional message google.protobuf.BoolValue
  pub fn has_use_remote_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_use_remote_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn use_remote_address_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_remote_address().then(|| self.use_remote_address())
  }
  pub fn use_remote_address(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_remote_address_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_remote_address(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // xff_num_trusted_hops: optional uint32
  pub fn xff_num_trusted_hops(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        17, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_xff_num_trusted_hops(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        17, val.into()
      )
    }
  }

  // original_ip_detection_extensions: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn original_ip_detection_extensions(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        43
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn original_ip_detection_extensions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        43,
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
  pub fn set_original_ip_detection_extensions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        43,
        src);
    }
  }

  // early_header_mutation_extensions: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn early_header_mutation_extensions(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        49
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn early_header_mutation_extensions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        49,
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
  pub fn set_early_header_mutation_extensions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        49,
        src);
    }
  }

  // internal_address_config: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.InternalAddressConfig
  pub fn has_internal_address_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_internal_address_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn internal_address_config_opt(&self) -> ::std::option::Option<super::http_connection_manager::InternalAddressConfigView<'_>> {
    self.has_internal_address_config().then(|| self.internal_address_config())
  }
  pub fn internal_address_config(&self) -> super::http_connection_manager::InternalAddressConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::InternalAddressConfigView::default())
  }
  pub fn internal_address_config_mut(&mut self) -> super::http_connection_manager::InternalAddressConfigMut<'_> {
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
  pub fn set_internal_address_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::InternalAddressConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val
      );
    }
  }

  // skip_xff_append: optional bool
  pub fn skip_xff_append(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        19, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_skip_xff_append(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        19, val.into()
      )
    }
  }

  // via: optional string
  pub fn via(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        20, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_via(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        val);
    }
  }

  // generate_request_id: optional message google.protobuf.BoolValue
  pub fn has_generate_request_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_generate_request_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn generate_request_id_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_generate_request_id().then(|| self.generate_request_id())
  }
  pub fn generate_request_id(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn generate_request_id_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_generate_request_id(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // preserve_external_request_id: optional bool
  pub fn preserve_external_request_id(&self) -> bool {
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
  pub fn set_preserve_external_request_id(&mut self, val: bool) {
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

  // always_set_request_id_in_response: optional bool
  pub fn always_set_request_id_in_response(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        34, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_always_set_request_id_in_response(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        34, val.into()
      )
    }
  }

  // forward_client_cert_details: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ForwardClientCertDetails
  pub fn forward_client_cert_details(&self) -> super::http_connection_manager::ForwardClientCertDetails {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (super::http_connection_manager::ForwardClientCertDetails::Sanitize).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_forward_client_cert_details(&mut self, val: super::http_connection_manager::ForwardClientCertDetails) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        14, val.into()
      )
    }
  }

  // set_current_client_cert_details: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.SetCurrentClientCertDetails
  pub fn has_set_current_client_cert_details(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_set_current_client_cert_details(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn set_current_client_cert_details_opt(&self) -> ::std::option::Option<super::http_connection_manager::SetCurrentClientCertDetailsView<'_>> {
    self.has_set_current_client_cert_details().then(|| self.set_current_client_cert_details())
  }
  pub fn set_current_client_cert_details(&self) -> super::http_connection_manager::SetCurrentClientCertDetailsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::SetCurrentClientCertDetailsView::default())
  }
  pub fn set_current_client_cert_details_mut(&mut self) -> super::http_connection_manager::SetCurrentClientCertDetailsMut<'_> {
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
  pub fn set_set_current_client_cert_details(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::SetCurrentClientCertDetails>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // forward_client_cert_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_forward_client_cert_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(57)
    }
  }
  pub fn clear_forward_client_cert_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        57
      );
    }
  }
  pub fn forward_client_cert_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_forward_client_cert_matcher().then(|| self.forward_client_cert_matcher())
  }
  pub fn forward_client_cert_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(57)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn forward_client_cert_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         57, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_forward_client_cert_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        57,
        val
      );
    }
  }

  // proxy_100_continue: optional bool
  pub fn proxy_100_continue(&self) -> bool {
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
  pub fn set_proxy_100_continue(&mut self, val: bool) {
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

  // represent_ipv4_remote_address_as_ipv4_mapped_ipv6: optional bool
  pub fn represent_ipv4_remote_address_as_ipv4_mapped_ipv6(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        18, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_represent_ipv4_remote_address_as_ipv4_mapped_ipv6(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        18, val.into()
      )
    }
  }

  // upgrade_configs: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.UpgradeConfig
  pub fn upgrade_configs(&self) -> ::protobuf::RepeatedView<'_, super::http_connection_manager::UpgradeConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        21
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http_connection_manager::UpgradeConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn upgrade_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::http_connection_manager::UpgradeConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        21,
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
  pub fn set_upgrade_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::http_connection_manager::UpgradeConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        src);
    }
  }

  // normalize_path: optional message google.protobuf.BoolValue
  pub fn has_normalize_path(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(27)
    }
  }
  pub fn clear_normalize_path(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        27
      );
    }
  }
  pub fn normalize_path_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_normalize_path().then(|| self.normalize_path())
  }
  pub fn normalize_path(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(27)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn normalize_path_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         27, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_normalize_path(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        27,
        val
      );
    }
  }

  // merge_slashes: optional bool
  pub fn merge_slashes(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        30, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_merge_slashes(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        30, val.into()
      )
    }
  }

  // path_with_escaped_slashes_action: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.PathWithEscapedSlashesAction
  pub fn path_with_escaped_slashes_action(&self) -> super::http_connection_manager::PathWithEscapedSlashesAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        42, (super::http_connection_manager::PathWithEscapedSlashesAction::ImplementationSpecificDefault).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_path_with_escaped_slashes_action(&mut self, val: super::http_connection_manager::PathWithEscapedSlashesAction) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        42, val.into()
      )
    }
  }

  // request_id_extension: optional message envoy.extensions.filters.network.http_connection_manager.v3.RequestIDExtension
  pub fn has_request_id_extension(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(33)
    }
  }
  pub fn clear_request_id_extension(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        33
      );
    }
  }
  pub fn request_id_extension_opt(&self) -> ::std::option::Option<super::RequestIDExtensionView<'_>> {
    self.has_request_id_extension().then(|| self.request_id_extension())
  }
  pub fn request_id_extension(&self) -> super::RequestIDExtensionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(33)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RequestIDExtensionView::default())
  }
  pub fn request_id_extension_mut(&mut self) -> super::RequestIDExtensionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         33, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_request_id_extension(&mut self,
    val: impl ::protobuf::IntoProxied<super::RequestIDExtension>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        33,
        val
      );
    }
  }

  // local_reply_config: optional message envoy.extensions.filters.network.http_connection_manager.v3.LocalReplyConfig
  pub fn has_local_reply_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(35)
    }
  }
  pub fn clear_local_reply_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        35
      );
    }
  }
  pub fn local_reply_config_opt(&self) -> ::std::option::Option<super::LocalReplyConfigView<'_>> {
    self.has_local_reply_config().then(|| self.local_reply_config())
  }
  pub fn local_reply_config(&self) -> super::LocalReplyConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(35)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LocalReplyConfigView::default())
  }
  pub fn local_reply_config_mut(&mut self) -> super::LocalReplyConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         35, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_local_reply_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::LocalReplyConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        35,
        val
      );
    }
  }

  // strip_matching_host_port: optional bool
  pub fn strip_matching_host_port(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        36, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_strip_matching_host_port(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        36, val.into()
      )
    }
  }

  // strip_any_host_port: optional bool
  pub fn has_strip_any_host_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(39)
    }
  }
  pub fn clear_strip_any_host_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        39
      );
    }
  }
  pub fn strip_any_host_port_opt(&self) -> ::std::option::Option<bool> {
    self.has_strip_any_host_port().then(|| self.strip_any_host_port())
  }
  pub fn strip_any_host_port(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        39, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_strip_any_host_port(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        39, val.into()
      )
    }
  }

  // stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_stream_error_on_invalid_http_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(37)
    }
  }
  pub fn clear_stream_error_on_invalid_http_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        37
      );
    }
  }
  pub fn stream_error_on_invalid_http_message_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_stream_error_on_invalid_http_message().then(|| self.stream_error_on_invalid_http_message())
  }
  pub fn stream_error_on_invalid_http_message(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(37)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn stream_error_on_invalid_http_message_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         37, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stream_error_on_invalid_http_message(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        37,
        val
      );
    }
  }

  // path_normalization_options: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.PathNormalizationOptions
  pub fn has_path_normalization_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(40)
    }
  }
  pub fn clear_path_normalization_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        40
      );
    }
  }
  pub fn path_normalization_options_opt(&self) -> ::std::option::Option<super::http_connection_manager::PathNormalizationOptionsView<'_>> {
    self.has_path_normalization_options().then(|| self.path_normalization_options())
  }
  pub fn path_normalization_options(&self) -> super::http_connection_manager::PathNormalizationOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(40)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::PathNormalizationOptionsView::default())
  }
  pub fn path_normalization_options_mut(&mut self) -> super::http_connection_manager::PathNormalizationOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         40, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_path_normalization_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::PathNormalizationOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        40,
        val
      );
    }
  }

  // strip_trailing_host_dot: optional bool
  pub fn strip_trailing_host_dot(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        44, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_strip_trailing_host_dot(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        44, val.into()
      )
    }
  }

  // proxy_status_config: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ProxyStatusConfig
  pub fn has_proxy_status_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(46)
    }
  }
  pub fn clear_proxy_status_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        46
      );
    }
  }
  pub fn proxy_status_config_opt(&self) -> ::std::option::Option<super::http_connection_manager::ProxyStatusConfigView<'_>> {
    self.has_proxy_status_config().then(|| self.proxy_status_config())
  }
  pub fn proxy_status_config(&self) -> super::http_connection_manager::ProxyStatusConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(46)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http_connection_manager::ProxyStatusConfigView::default())
  }
  pub fn proxy_status_config_mut(&mut self) -> super::http_connection_manager::ProxyStatusConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         46, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_proxy_status_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::http_connection_manager::ProxyStatusConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        46,
        val
      );
    }
  }

  // typed_header_validation_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_header_validation_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(47)
    }
  }
  pub fn clear_typed_header_validation_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        47
      );
    }
  }
  pub fn typed_header_validation_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_header_validation_config().then(|| self.typed_header_validation_config())
  }
  pub fn typed_header_validation_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(47)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_header_validation_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         47, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_typed_header_validation_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        47,
        val
      );
    }
  }

  // append_x_forwarded_port: optional bool
  pub fn append_x_forwarded_port(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        48, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_append_x_forwarded_port(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        48, val.into()
      )
    }
  }

  // append_local_overload: optional bool
  pub fn append_local_overload(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        54, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_append_local_overload(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        54, val.into()
      )
    }
  }

  // add_proxy_protocol_connection_state: optional message google.protobuf.BoolValue
  pub fn has_add_proxy_protocol_connection_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(50)
    }
  }
  pub fn clear_add_proxy_protocol_connection_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        50
      );
    }
  }
  pub fn add_proxy_protocol_connection_state_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_add_proxy_protocol_connection_state().then(|| self.add_proxy_protocol_connection_state())
  }
  pub fn add_proxy_protocol_connection_state(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(50)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn add_proxy_protocol_connection_state_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         50, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_add_proxy_protocol_connection_state(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        50,
        val
      );
    }
  }

  pub fn route_specifier(&self) -> super::http_connection_manager::RouteSpecifierOneof<'_> {
    match &self.route_specifier_case() {
      super::http_connection_manager::RouteSpecifierCase::Rds =>
          super::http_connection_manager::RouteSpecifierOneof::Rds(self.rds()),
      super::http_connection_manager::RouteSpecifierCase::RouteConfig =>
          super::http_connection_manager::RouteSpecifierOneof::RouteConfig(self.route_config()),
      super::http_connection_manager::RouteSpecifierCase::ScopedRoutes =>
          super::http_connection_manager::RouteSpecifierOneof::ScopedRoutes(self.scoped_routes()),
      _ => super::http_connection_manager::RouteSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn route_specifier_case(&self) -> super::http_connection_manager::RouteSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::http_connection_manager::RouteSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
  pub fn strip_port_mode(&self) -> super::http_connection_manager::StripPortModeOneof<'_> {
    match &self.strip_port_mode_case() {
      super::http_connection_manager::StripPortModeCase::StripAnyHostPort =>
          super::http_connection_manager::StripPortModeOneof::StripAnyHostPort(self.strip_any_host_port()),
      _ => super::http_connection_manager::StripPortModeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn strip_port_mode_case(&self) -> super::http_connection_manager::StripPortModeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(39);
      super::http_connection_manager::StripPortModeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl HttpConnectionManager

impl ::std::ops::Drop for HttpConnectionManager {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpConnectionManager {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpConnectionManager {
  type Proxied = Self;
  fn as_view(&self) -> HttpConnectionManagerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpConnectionManager {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpConnectionManagerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpConnectionManager {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P1X33G33331Xa3G33.P3/P)P/P/P1XG333a3333/P/P.P33/P3/P33/33.PG/P333/PG33/P3/P/P33^$|%|A~L");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager_msg_init.0, &[<super::Rds as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::route::v3::route::RouteConfiguration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::http_connection_manager::Tracing as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::http_connection_manager::SetCurrentClientCertDetails as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::http_connection_manager::UpgradeConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::http_connection_manager::InternalAddressConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ScopedRoutes as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::RequestIDExtension as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::LocalReplyConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::http_connection_manager::PathNormalizationOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::protocol::Http3ProtocolOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::protocol::SchemeHeaderTransformation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::http_connection_manager::ProxyStatusConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::http_connection_manager::HcmAccessLogOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpConnectionManager {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpConnectionManager {
  type Msg = HttpConnectionManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpConnectionManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpConnectionManager {
  type Msg = HttpConnectionManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpConnectionManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpConnectionManagerMut<'_> {
  type Msg = HttpConnectionManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpConnectionManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpConnectionManagerMut<'_> {
  type Msg = HttpConnectionManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpConnectionManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpConnectionManagerView<'_> {
  type Msg = HttpConnectionManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpConnectionManager> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpConnectionManagerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod http_connection_manager {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__Tracing_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Tracing {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Tracing>
}

impl ::protobuf::Message for Tracing {
  type MessageView<'msg> = TracingView<'msg>;
  type MessageMut<'msg> = TracingMut<'msg>;
}

impl ::std::default::Default for Tracing {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Tracing {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Tracing` is `Sync` because it does not implement interior mutability.
//    Neither does `TracingMut`.
unsafe impl ::std::marker::Sync for Tracing {}

// SAFETY:
// - `Tracing` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Tracing {}

impl ::protobuf::Proxied for Tracing {
  type View<'msg> = TracingView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Tracing {}

impl ::protobuf::MutProxied for Tracing {
  type Mut<'msg> = TracingMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TracingView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Tracing>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TracingView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TracingView<'msg> {
  type Message = Tracing;
}

impl ::std::fmt::Debug for TracingView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TracingView<'_> {
  fn default() -> TracingView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Tracing>> for TracingView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Tracing>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TracingView<'msg> {

  pub fn to_owned(&self) -> Tracing {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // client_sampling: optional message envoy.type.v3.Percent
  pub fn has_client_sampling(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn client_sampling_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_client_sampling().then(|| self.client_sampling())
  }
  pub fn client_sampling(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

  // random_sampling: optional message envoy.type.v3.Percent
  pub fn has_random_sampling(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn random_sampling_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_random_sampling().then(|| self.random_sampling())
  }
  pub fn random_sampling(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

  // overall_sampling: optional message envoy.type.v3.Percent
  pub fn has_overall_sampling(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn overall_sampling_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_overall_sampling().then(|| self.overall_sampling())
  }
  pub fn overall_sampling(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

  // verbose: optional bool
  pub fn verbose(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

  // max_path_tag_length: optional message google.protobuf.UInt32Value
  pub fn has_max_path_tag_length(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn max_path_tag_length_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_path_tag_length().then(|| self.max_path_tag_length())
  }
  pub fn max_path_tag_length(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // custom_tags: repeated message envoy.type.tracing.v3.CustomTag
  pub fn custom_tags(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::r#type::tracing::v3::custom_tag::CustomTag> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::tracing::v3::custom_tag::CustomTag>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // provider: optional message envoy.config.trace.v3.Tracing.Http
  pub fn has_provider(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn provider_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::HttpView<'msg>> {
    self.has_provider().then(|| self.provider())
  }
  pub fn provider(self) -> crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::HttpView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::HttpView::default())
  }

  // spawn_upstream_span: optional message google.protobuf.BoolValue
  pub fn has_spawn_upstream_span(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn spawn_upstream_span_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_spawn_upstream_span().then(|| self.spawn_upstream_span())
  }
  pub fn spawn_upstream_span(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // operation: optional string
  pub fn operation(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // upstream_operation: optional string
  pub fn upstream_operation(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `TracingView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TracingView<'_> {}

// SAFETY:
// - `TracingView` is `Send` because while its alive a `TracingMut` cannot.
// - `TracingView` does not use thread-local data.
unsafe impl ::std::marker::Send for TracingView<'_> {}

impl<'msg> ::protobuf::AsView for TracingView<'msg> {
  type Proxied = Tracing;
  fn as_view(&self) -> ::protobuf::View<'msg, Tracing> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TracingView<'msg> {
  fn into_view<'shorter>(self) -> TracingView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Tracing> for TracingView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Tracing {
    let mut dst = Tracing::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Tracing> for TracingMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Tracing {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Tracing {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TracingView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TracingMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TracingMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Tracing>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TracingMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TracingMut<'msg> {
  type Message = Tracing;
}

impl ::std::fmt::Debug for TracingMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Tracing>> for TracingMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Tracing>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TracingMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Tracing> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Tracing {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // client_sampling: optional message envoy.type.v3.Percent
  pub fn has_client_sampling(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_client_sampling(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn client_sampling_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_client_sampling().then(|| self.client_sampling())
  }
  pub fn client_sampling(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn client_sampling_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_client_sampling(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // random_sampling: optional message envoy.type.v3.Percent
  pub fn has_random_sampling(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_random_sampling(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn random_sampling_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_random_sampling().then(|| self.random_sampling())
  }
  pub fn random_sampling(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn random_sampling_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_random_sampling(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // overall_sampling: optional message envoy.type.v3.Percent
  pub fn has_overall_sampling(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_overall_sampling(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn overall_sampling_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_overall_sampling().then(|| self.overall_sampling())
  }
  pub fn overall_sampling(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn overall_sampling_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_overall_sampling(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // verbose: optional bool
  pub fn verbose(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_verbose(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // max_path_tag_length: optional message google.protobuf.UInt32Value
  pub fn has_max_path_tag_length(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_max_path_tag_length(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn max_path_tag_length_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_path_tag_length().then(|| self.max_path_tag_length())
  }
  pub fn max_path_tag_length(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_path_tag_length_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_path_tag_length(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // custom_tags: repeated message envoy.type.tracing.v3.CustomTag
  pub fn custom_tags(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::tracing::v3::custom_tag::CustomTag> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::tracing::v3::custom_tag::CustomTag>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn custom_tags_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::tracing::v3::custom_tag::CustomTag> {
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
  pub fn set_custom_tags(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::tracing::v3::custom_tag::CustomTag>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // provider: optional message envoy.config.trace.v3.Tracing.Http
  pub fn has_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn provider_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::HttpView<'_>> {
    self.has_provider().then(|| self.provider())
  }
  pub fn provider(&self) -> crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::HttpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::HttpView::default())
  }
  pub fn provider_mut(&mut self) -> crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::HttpMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_provider(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::Http>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // spawn_upstream_span: optional message google.protobuf.BoolValue
  pub fn has_spawn_upstream_span(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_spawn_upstream_span(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn spawn_upstream_span_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_spawn_upstream_span().then(|| self.spawn_upstream_span())
  }
  pub fn spawn_upstream_span(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn spawn_upstream_span_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_spawn_upstream_span(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // operation: optional string
  pub fn operation(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_operation(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // upstream_operation: optional string
  pub fn upstream_operation(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_upstream_operation(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

}

// SAFETY:
// - `TracingMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TracingMut<'_> {}

// SAFETY:
// - `TracingMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TracingMut<'_> {}

impl<'msg> ::protobuf::AsView for TracingMut<'msg> {
  type Proxied = Tracing;
  fn as_view(&self) -> ::protobuf::View<'_, Tracing> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TracingMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Tracing>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TracingMut<'msg> {
  type MutProxied = Tracing;
  fn as_mut(&mut self) -> TracingMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TracingMut<'msg> {
  fn into_mut<'shorter>(self) -> TracingMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Tracing {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Tracing> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TracingView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TracingMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // client_sampling: optional message envoy.type.v3.Percent
  pub fn has_client_sampling(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_client_sampling(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn client_sampling_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_client_sampling().then(|| self.client_sampling())
  }
  pub fn client_sampling(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn client_sampling_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_client_sampling(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // random_sampling: optional message envoy.type.v3.Percent
  pub fn has_random_sampling(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_random_sampling(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn random_sampling_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_random_sampling().then(|| self.random_sampling())
  }
  pub fn random_sampling(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn random_sampling_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_random_sampling(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // overall_sampling: optional message envoy.type.v3.Percent
  pub fn has_overall_sampling(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_overall_sampling(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn overall_sampling_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_overall_sampling().then(|| self.overall_sampling())
  }
  pub fn overall_sampling(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn overall_sampling_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_overall_sampling(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // verbose: optional bool
  pub fn verbose(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_verbose(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // max_path_tag_length: optional message google.protobuf.UInt32Value
  pub fn has_max_path_tag_length(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_max_path_tag_length(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn max_path_tag_length_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_path_tag_length().then(|| self.max_path_tag_length())
  }
  pub fn max_path_tag_length(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_path_tag_length_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_path_tag_length(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // custom_tags: repeated message envoy.type.tracing.v3.CustomTag
  pub fn custom_tags(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::tracing::v3::custom_tag::CustomTag> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::tracing::v3::custom_tag::CustomTag>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn custom_tags_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::tracing::v3::custom_tag::CustomTag> {
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
  pub fn set_custom_tags(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::tracing::v3::custom_tag::CustomTag>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // provider: optional message envoy.config.trace.v3.Tracing.Http
  pub fn has_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn provider_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::HttpView<'_>> {
    self.has_provider().then(|| self.provider())
  }
  pub fn provider(&self) -> crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::HttpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::HttpView::default())
  }
  pub fn provider_mut(&mut self) -> crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::HttpMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_provider(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::Http>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // spawn_upstream_span: optional message google.protobuf.BoolValue
  pub fn has_spawn_upstream_span(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_spawn_upstream_span(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn spawn_upstream_span_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_spawn_upstream_span().then(|| self.spawn_upstream_span())
  }
  pub fn spawn_upstream_span(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn spawn_upstream_span_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_spawn_upstream_span(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // operation: optional string
  pub fn operation(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_operation(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // upstream_operation: optional string
  pub fn upstream_operation(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_upstream_operation(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

}  // impl Tracing

impl ::std::ops::Drop for Tracing {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Tracing {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Tracing {
  type Proxied = Self;
  fn as_view(&self) -> TracingView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Tracing {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TracingMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Tracing {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__Tracing_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$b333/P3G331X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__Tracing_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::tracing::v3::custom_tag::CustomTag as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::trace::v3::http_tracer::tracing::Http as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__Tracing_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Tracing {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Tracing {
  type Msg = Tracing;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Tracing> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Tracing {
  type Msg = Tracing;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Tracing> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TracingMut<'_> {
  type Msg = Tracing;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Tracing> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TracingMut<'_> {
  type Msg = Tracing;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Tracing> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TracingView<'_> {
  type Msg = Tracing;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Tracing> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TracingMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod tracing {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationName(i32);

#[allow(non_upper_case_globals)]
impl OperationName {
  pub const Ingress: OperationName = OperationName(0);
  pub const Egress: OperationName = OperationName(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Ingress",
      1 => "Egress",
      _ => return None
    })
  }
}

impl ::std::convert::From<OperationName> for i32 {
  fn from(val: OperationName) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for OperationName {
  fn from(val: i32) -> OperationName {
    Self(val)
  }
}

impl ::std::default::Default for OperationName {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for OperationName {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "OperationName::{}", constant_name)
    } else {
      write!(f, "OperationName::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for OperationName {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for OperationName {}

impl ::protobuf::Proxied for OperationName {
  type View<'a> = OperationName;
}

impl ::protobuf::AsView for OperationName {
  type Proxied = OperationName;

  fn as_view(&self) -> OperationName {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OperationName {
  fn into_view<'shorter>(self) -> OperationName where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for OperationName {
  const NAME: &'static str = "OperationName";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for OperationName {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod tracing

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__InternalAddressConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct InternalAddressConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<InternalAddressConfig>
}

impl ::protobuf::Message for InternalAddressConfig {
  type MessageView<'msg> = InternalAddressConfigView<'msg>;
  type MessageMut<'msg> = InternalAddressConfigMut<'msg>;
}

impl ::std::default::Default for InternalAddressConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for InternalAddressConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `InternalAddressConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `InternalAddressConfigMut`.
unsafe impl ::std::marker::Sync for InternalAddressConfig {}

// SAFETY:
// - `InternalAddressConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for InternalAddressConfig {}

impl ::protobuf::Proxied for InternalAddressConfig {
  type View<'msg> = InternalAddressConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for InternalAddressConfig {}

impl ::protobuf::MutProxied for InternalAddressConfig {
  type Mut<'msg> = InternalAddressConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct InternalAddressConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, InternalAddressConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InternalAddressConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for InternalAddressConfigView<'msg> {
  type Message = InternalAddressConfig;
}

impl ::std::fmt::Debug for InternalAddressConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for InternalAddressConfigView<'_> {
  fn default() -> InternalAddressConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, InternalAddressConfig>> for InternalAddressConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, InternalAddressConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InternalAddressConfigView<'msg> {

  pub fn to_owned(&self) -> InternalAddressConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // unix_sockets: optional bool
  pub fn unix_sockets(self) -> bool {
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

  // cidr_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn cidr_ranges(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `InternalAddressConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for InternalAddressConfigView<'_> {}

// SAFETY:
// - `InternalAddressConfigView` is `Send` because while its alive a `InternalAddressConfigMut` cannot.
// - `InternalAddressConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for InternalAddressConfigView<'_> {}

impl<'msg> ::protobuf::AsView for InternalAddressConfigView<'msg> {
  type Proxied = InternalAddressConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, InternalAddressConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InternalAddressConfigView<'msg> {
  fn into_view<'shorter>(self) -> InternalAddressConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<InternalAddressConfig> for InternalAddressConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> InternalAddressConfig {
    let mut dst = InternalAddressConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<InternalAddressConfig> for InternalAddressConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> InternalAddressConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for InternalAddressConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for InternalAddressConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for InternalAddressConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct InternalAddressConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, InternalAddressConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InternalAddressConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for InternalAddressConfigMut<'msg> {
  type Message = InternalAddressConfig;
}

impl ::std::fmt::Debug for InternalAddressConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, InternalAddressConfig>> for InternalAddressConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, InternalAddressConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InternalAddressConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, InternalAddressConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> InternalAddressConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // unix_sockets: optional bool
  pub fn unix_sockets(&self) -> bool {
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
  pub fn set_unix_sockets(&mut self, val: bool) {
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

  // cidr_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn cidr_ranges(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn cidr_ranges_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
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
  pub fn set_cidr_ranges(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `InternalAddressConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for InternalAddressConfigMut<'_> {}

// SAFETY:
// - `InternalAddressConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for InternalAddressConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for InternalAddressConfigMut<'msg> {
  type Proxied = InternalAddressConfig;
  fn as_view(&self) -> ::protobuf::View<'_, InternalAddressConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InternalAddressConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, InternalAddressConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for InternalAddressConfigMut<'msg> {
  type MutProxied = InternalAddressConfig;
  fn as_mut(&mut self) -> InternalAddressConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for InternalAddressConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> InternalAddressConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl InternalAddressConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, InternalAddressConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> InternalAddressConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> InternalAddressConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // unix_sockets: optional bool
  pub fn unix_sockets(&self) -> bool {
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
  pub fn set_unix_sockets(&mut self, val: bool) {
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

  // cidr_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn cidr_ranges(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn cidr_ranges_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
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
  pub fn set_cidr_ranges(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl InternalAddressConfig

impl ::std::ops::Drop for InternalAddressConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for InternalAddressConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for InternalAddressConfig {
  type Proxied = Self;
  fn as_view(&self) -> InternalAddressConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for InternalAddressConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> InternalAddressConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for InternalAddressConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__InternalAddressConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/PG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__InternalAddressConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::CidrRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__InternalAddressConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for InternalAddressConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for InternalAddressConfig {
  type Msg = InternalAddressConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InternalAddressConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InternalAddressConfig {
  type Msg = InternalAddressConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InternalAddressConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for InternalAddressConfigMut<'_> {
  type Msg = InternalAddressConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InternalAddressConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InternalAddressConfigMut<'_> {
  type Msg = InternalAddressConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InternalAddressConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InternalAddressConfigView<'_> {
  type Msg = InternalAddressConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InternalAddressConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for InternalAddressConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__SetCurrentClientCertDetails_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SetCurrentClientCertDetails {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SetCurrentClientCertDetails>
}

impl ::protobuf::Message for SetCurrentClientCertDetails {
  type MessageView<'msg> = SetCurrentClientCertDetailsView<'msg>;
  type MessageMut<'msg> = SetCurrentClientCertDetailsMut<'msg>;
}

impl ::std::default::Default for SetCurrentClientCertDetails {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SetCurrentClientCertDetails {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SetCurrentClientCertDetails` is `Sync` because it does not implement interior mutability.
//    Neither does `SetCurrentClientCertDetailsMut`.
unsafe impl ::std::marker::Sync for SetCurrentClientCertDetails {}

// SAFETY:
// - `SetCurrentClientCertDetails` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SetCurrentClientCertDetails {}

impl ::protobuf::Proxied for SetCurrentClientCertDetails {
  type View<'msg> = SetCurrentClientCertDetailsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SetCurrentClientCertDetails {}

impl ::protobuf::MutProxied for SetCurrentClientCertDetails {
  type Mut<'msg> = SetCurrentClientCertDetailsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetCurrentClientCertDetailsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SetCurrentClientCertDetails>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetCurrentClientCertDetailsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SetCurrentClientCertDetailsView<'msg> {
  type Message = SetCurrentClientCertDetails;
}

impl ::std::fmt::Debug for SetCurrentClientCertDetailsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SetCurrentClientCertDetailsView<'_> {
  fn default() -> SetCurrentClientCertDetailsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SetCurrentClientCertDetails>> for SetCurrentClientCertDetailsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SetCurrentClientCertDetails>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SetCurrentClientCertDetailsView<'msg> {

  pub fn to_owned(&self) -> SetCurrentClientCertDetails {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subject: optional message google.protobuf.BoolValue
  pub fn has_subject(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn subject_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_subject().then(|| self.subject())
  }
  pub fn subject(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // cert: optional bool
  pub fn cert(self) -> bool {
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

  // chain: optional bool
  pub fn chain(self) -> bool {
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

  // dns: optional bool
  pub fn dns(self) -> bool {
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

  // uri: optional bool
  pub fn uri(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SetCurrentClientCertDetailsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SetCurrentClientCertDetailsView<'_> {}

// SAFETY:
// - `SetCurrentClientCertDetailsView` is `Send` because while its alive a `SetCurrentClientCertDetailsMut` cannot.
// - `SetCurrentClientCertDetailsView` does not use thread-local data.
unsafe impl ::std::marker::Send for SetCurrentClientCertDetailsView<'_> {}

impl<'msg> ::protobuf::AsView for SetCurrentClientCertDetailsView<'msg> {
  type Proxied = SetCurrentClientCertDetails;
  fn as_view(&self) -> ::protobuf::View<'msg, SetCurrentClientCertDetails> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetCurrentClientCertDetailsView<'msg> {
  fn into_view<'shorter>(self) -> SetCurrentClientCertDetailsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SetCurrentClientCertDetails> for SetCurrentClientCertDetailsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SetCurrentClientCertDetails {
    let mut dst = SetCurrentClientCertDetails::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SetCurrentClientCertDetails> for SetCurrentClientCertDetailsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SetCurrentClientCertDetails {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SetCurrentClientCertDetails {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SetCurrentClientCertDetailsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SetCurrentClientCertDetailsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetCurrentClientCertDetailsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SetCurrentClientCertDetails>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SetCurrentClientCertDetailsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SetCurrentClientCertDetailsMut<'msg> {
  type Message = SetCurrentClientCertDetails;
}

impl ::std::fmt::Debug for SetCurrentClientCertDetailsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SetCurrentClientCertDetails>> for SetCurrentClientCertDetailsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SetCurrentClientCertDetails>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SetCurrentClientCertDetailsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SetCurrentClientCertDetails> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SetCurrentClientCertDetails {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subject: optional message google.protobuf.BoolValue
  pub fn has_subject(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_subject(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn subject_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_subject().then(|| self.subject())
  }
  pub fn subject(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn subject_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_subject(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // cert: optional bool
  pub fn cert(&self) -> bool {
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
  pub fn set_cert(&mut self, val: bool) {
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

  // chain: optional bool
  pub fn chain(&self) -> bool {
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
  pub fn set_chain(&mut self, val: bool) {
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

  // dns: optional bool
  pub fn dns(&self) -> bool {
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
  pub fn set_dns(&mut self, val: bool) {
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

  // uri: optional bool
  pub fn uri(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uri(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `SetCurrentClientCertDetailsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SetCurrentClientCertDetailsMut<'_> {}

// SAFETY:
// - `SetCurrentClientCertDetailsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SetCurrentClientCertDetailsMut<'_> {}

impl<'msg> ::protobuf::AsView for SetCurrentClientCertDetailsMut<'msg> {
  type Proxied = SetCurrentClientCertDetails;
  fn as_view(&self) -> ::protobuf::View<'_, SetCurrentClientCertDetails> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SetCurrentClientCertDetailsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SetCurrentClientCertDetails>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SetCurrentClientCertDetailsMut<'msg> {
  type MutProxied = SetCurrentClientCertDetails;
  fn as_mut(&mut self) -> SetCurrentClientCertDetailsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SetCurrentClientCertDetailsMut<'msg> {
  fn into_mut<'shorter>(self) -> SetCurrentClientCertDetailsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SetCurrentClientCertDetails {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SetCurrentClientCertDetails> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SetCurrentClientCertDetailsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SetCurrentClientCertDetailsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subject: optional message google.protobuf.BoolValue
  pub fn has_subject(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_subject(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn subject_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_subject().then(|| self.subject())
  }
  pub fn subject(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn subject_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_subject(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // cert: optional bool
  pub fn cert(&self) -> bool {
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
  pub fn set_cert(&mut self, val: bool) {
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

  // chain: optional bool
  pub fn chain(&self) -> bool {
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
  pub fn set_chain(&mut self, val: bool) {
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

  // dns: optional bool
  pub fn dns(&self) -> bool {
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
  pub fn set_dns(&mut self, val: bool) {
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

  // uri: optional bool
  pub fn uri(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uri(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}  // impl SetCurrentClientCertDetails

impl ::std::ops::Drop for SetCurrentClientCertDetails {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SetCurrentClientCertDetails {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SetCurrentClientCertDetails {
  type Proxied = Self;
  fn as_view(&self) -> SetCurrentClientCertDetailsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SetCurrentClientCertDetails {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SetCurrentClientCertDetailsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SetCurrentClientCertDetails {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__SetCurrentClientCertDetails_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3a/P/P/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__SetCurrentClientCertDetails_msg_init.0, &[<::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__SetCurrentClientCertDetails_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SetCurrentClientCertDetails {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SetCurrentClientCertDetails {
  type Msg = SetCurrentClientCertDetails;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SetCurrentClientCertDetails> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SetCurrentClientCertDetails {
  type Msg = SetCurrentClientCertDetails;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SetCurrentClientCertDetails> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SetCurrentClientCertDetailsMut<'_> {
  type Msg = SetCurrentClientCertDetails;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SetCurrentClientCertDetails> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SetCurrentClientCertDetailsMut<'_> {
  type Msg = SetCurrentClientCertDetails;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SetCurrentClientCertDetails> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SetCurrentClientCertDetailsView<'_> {
  type Msg = SetCurrentClientCertDetails;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SetCurrentClientCertDetails> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SetCurrentClientCertDetailsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__ForwardClientCertConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ForwardClientCertConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ForwardClientCertConfig>
}

impl ::protobuf::Message for ForwardClientCertConfig {
  type MessageView<'msg> = ForwardClientCertConfigView<'msg>;
  type MessageMut<'msg> = ForwardClientCertConfigMut<'msg>;
}

impl ::std::default::Default for ForwardClientCertConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ForwardClientCertConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ForwardClientCertConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ForwardClientCertConfigMut`.
unsafe impl ::std::marker::Sync for ForwardClientCertConfig {}

// SAFETY:
// - `ForwardClientCertConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ForwardClientCertConfig {}

impl ::protobuf::Proxied for ForwardClientCertConfig {
  type View<'msg> = ForwardClientCertConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ForwardClientCertConfig {}

impl ::protobuf::MutProxied for ForwardClientCertConfig {
  type Mut<'msg> = ForwardClientCertConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ForwardClientCertConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ForwardClientCertConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ForwardClientCertConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ForwardClientCertConfigView<'msg> {
  type Message = ForwardClientCertConfig;
}

impl ::std::fmt::Debug for ForwardClientCertConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ForwardClientCertConfigView<'_> {
  fn default() -> ForwardClientCertConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ForwardClientCertConfig>> for ForwardClientCertConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ForwardClientCertConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ForwardClientCertConfigView<'msg> {

  pub fn to_owned(&self) -> ForwardClientCertConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // forward_client_cert_details: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ForwardClientCertDetails
  pub fn forward_client_cert_details(self) -> super::super::http_connection_manager::ForwardClientCertDetails {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::http_connection_manager::ForwardClientCertDetails::Sanitize).into()
      ).try_into().unwrap()
    }
  }

  // set_current_client_cert_details: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.SetCurrentClientCertDetails
  pub fn has_set_current_client_cert_details(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn set_current_client_cert_details_opt(self) -> ::std::option::Option<super::super::http_connection_manager::SetCurrentClientCertDetailsView<'msg>> {
    self.has_set_current_client_cert_details().then(|| self.set_current_client_cert_details())
  }
  pub fn set_current_client_cert_details(self) -> super::super::http_connection_manager::SetCurrentClientCertDetailsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::http_connection_manager::SetCurrentClientCertDetailsView::default())
  }

}

// SAFETY:
// - `ForwardClientCertConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ForwardClientCertConfigView<'_> {}

// SAFETY:
// - `ForwardClientCertConfigView` is `Send` because while its alive a `ForwardClientCertConfigMut` cannot.
// - `ForwardClientCertConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ForwardClientCertConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ForwardClientCertConfigView<'msg> {
  type Proxied = ForwardClientCertConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ForwardClientCertConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ForwardClientCertConfigView<'msg> {
  fn into_view<'shorter>(self) -> ForwardClientCertConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ForwardClientCertConfig> for ForwardClientCertConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ForwardClientCertConfig {
    let mut dst = ForwardClientCertConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ForwardClientCertConfig> for ForwardClientCertConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ForwardClientCertConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ForwardClientCertConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ForwardClientCertConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ForwardClientCertConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ForwardClientCertConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ForwardClientCertConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ForwardClientCertConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ForwardClientCertConfigMut<'msg> {
  type Message = ForwardClientCertConfig;
}

impl ::std::fmt::Debug for ForwardClientCertConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ForwardClientCertConfig>> for ForwardClientCertConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ForwardClientCertConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ForwardClientCertConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ForwardClientCertConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ForwardClientCertConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // forward_client_cert_details: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ForwardClientCertDetails
  pub fn forward_client_cert_details(&self) -> super::super::http_connection_manager::ForwardClientCertDetails {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::http_connection_manager::ForwardClientCertDetails::Sanitize).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_forward_client_cert_details(&mut self, val: super::super::http_connection_manager::ForwardClientCertDetails) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // set_current_client_cert_details: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.SetCurrentClientCertDetails
  pub fn has_set_current_client_cert_details(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_set_current_client_cert_details(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn set_current_client_cert_details_opt(&self) -> ::std::option::Option<super::super::http_connection_manager::SetCurrentClientCertDetailsView<'_>> {
    self.has_set_current_client_cert_details().then(|| self.set_current_client_cert_details())
  }
  pub fn set_current_client_cert_details(&self) -> super::super::http_connection_manager::SetCurrentClientCertDetailsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::http_connection_manager::SetCurrentClientCertDetailsView::default())
  }
  pub fn set_current_client_cert_details_mut(&mut self) -> super::super::http_connection_manager::SetCurrentClientCertDetailsMut<'_> {
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
  pub fn set_set_current_client_cert_details(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::http_connection_manager::SetCurrentClientCertDetails>) {

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
// - `ForwardClientCertConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ForwardClientCertConfigMut<'_> {}

// SAFETY:
// - `ForwardClientCertConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ForwardClientCertConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ForwardClientCertConfigMut<'msg> {
  type Proxied = ForwardClientCertConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ForwardClientCertConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ForwardClientCertConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ForwardClientCertConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ForwardClientCertConfigMut<'msg> {
  type MutProxied = ForwardClientCertConfig;
  fn as_mut(&mut self) -> ForwardClientCertConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ForwardClientCertConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ForwardClientCertConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ForwardClientCertConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ForwardClientCertConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ForwardClientCertConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ForwardClientCertConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // forward_client_cert_details: optional enum envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.ForwardClientCertDetails
  pub fn forward_client_cert_details(&self) -> super::super::http_connection_manager::ForwardClientCertDetails {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::http_connection_manager::ForwardClientCertDetails::Sanitize).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_forward_client_cert_details(&mut self, val: super::super::http_connection_manager::ForwardClientCertDetails) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // set_current_client_cert_details: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.SetCurrentClientCertDetails
  pub fn has_set_current_client_cert_details(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_set_current_client_cert_details(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn set_current_client_cert_details_opt(&self) -> ::std::option::Option<super::super::http_connection_manager::SetCurrentClientCertDetailsView<'_>> {
    self.has_set_current_client_cert_details().then(|| self.set_current_client_cert_details())
  }
  pub fn set_current_client_cert_details(&self) -> super::super::http_connection_manager::SetCurrentClientCertDetailsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::http_connection_manager::SetCurrentClientCertDetailsView::default())
  }
  pub fn set_current_client_cert_details_mut(&mut self) -> super::super::http_connection_manager::SetCurrentClientCertDetailsMut<'_> {
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
  pub fn set_set_current_client_cert_details(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::http_connection_manager::SetCurrentClientCertDetails>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ForwardClientCertConfig

impl ::std::ops::Drop for ForwardClientCertConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ForwardClientCertConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ForwardClientCertConfig {
  type Proxied = Self;
  fn as_view(&self) -> ForwardClientCertConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ForwardClientCertConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ForwardClientCertConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ForwardClientCertConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__ForwardClientCertConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__ForwardClientCertConfig_msg_init.0, &[<super::super::http_connection_manager::SetCurrentClientCertDetails as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__ForwardClientCertConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ForwardClientCertConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ForwardClientCertConfig {
  type Msg = ForwardClientCertConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ForwardClientCertConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ForwardClientCertConfig {
  type Msg = ForwardClientCertConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ForwardClientCertConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ForwardClientCertConfigMut<'_> {
  type Msg = ForwardClientCertConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ForwardClientCertConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ForwardClientCertConfigMut<'_> {
  type Msg = ForwardClientCertConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ForwardClientCertConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ForwardClientCertConfigView<'_> {
  type Msg = ForwardClientCertConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ForwardClientCertConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ForwardClientCertConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__UpgradeConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UpgradeConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UpgradeConfig>
}

impl ::protobuf::Message for UpgradeConfig {
  type MessageView<'msg> = UpgradeConfigView<'msg>;
  type MessageMut<'msg> = UpgradeConfigMut<'msg>;
}

impl ::std::default::Default for UpgradeConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UpgradeConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UpgradeConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `UpgradeConfigMut`.
unsafe impl ::std::marker::Sync for UpgradeConfig {}

// SAFETY:
// - `UpgradeConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UpgradeConfig {}

impl ::protobuf::Proxied for UpgradeConfig {
  type View<'msg> = UpgradeConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UpgradeConfig {}

impl ::protobuf::MutProxied for UpgradeConfig {
  type Mut<'msg> = UpgradeConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpgradeConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpgradeConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpgradeConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpgradeConfigView<'msg> {
  type Message = UpgradeConfig;
}

impl ::std::fmt::Debug for UpgradeConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpgradeConfigView<'_> {
  fn default() -> UpgradeConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UpgradeConfig>> for UpgradeConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpgradeConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpgradeConfigView<'msg> {

  pub fn to_owned(&self) -> UpgradeConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // upgrade_type: optional string
  pub fn upgrade_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // filters: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter
  pub fn filters(self) -> ::protobuf::RepeatedView<'msg, super::super::HttpFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::HttpFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // enabled: optional message google.protobuf.BoolValue
  pub fn has_enabled(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn enabled_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_enabled().then(|| self.enabled())
  }
  pub fn enabled(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

}

// SAFETY:
// - `UpgradeConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UpgradeConfigView<'_> {}

// SAFETY:
// - `UpgradeConfigView` is `Send` because while its alive a `UpgradeConfigMut` cannot.
// - `UpgradeConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for UpgradeConfigView<'_> {}

impl<'msg> ::protobuf::AsView for UpgradeConfigView<'msg> {
  type Proxied = UpgradeConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, UpgradeConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpgradeConfigView<'msg> {
  fn into_view<'shorter>(self) -> UpgradeConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UpgradeConfig> for UpgradeConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpgradeConfig {
    let mut dst = UpgradeConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UpgradeConfig> for UpgradeConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpgradeConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UpgradeConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpgradeConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpgradeConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpgradeConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpgradeConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpgradeConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpgradeConfigMut<'msg> {
  type Message = UpgradeConfig;
}

impl ::std::fmt::Debug for UpgradeConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UpgradeConfig>> for UpgradeConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpgradeConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpgradeConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UpgradeConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UpgradeConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // upgrade_type: optional string
  pub fn upgrade_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_upgrade_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // filters: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter
  pub fn filters(&self) -> ::protobuf::RepeatedView<'_, super::super::HttpFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::HttpFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::HttpFilter> {
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
  pub fn set_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::HttpFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // enabled: optional message google.protobuf.BoolValue
  pub fn has_enabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_enabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn enabled_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enabled().then(|| self.enabled())
  }
  pub fn enabled(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enabled_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_enabled(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}

// SAFETY:
// - `UpgradeConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UpgradeConfigMut<'_> {}

// SAFETY:
// - `UpgradeConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UpgradeConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for UpgradeConfigMut<'msg> {
  type Proxied = UpgradeConfig;
  fn as_view(&self) -> ::protobuf::View<'_, UpgradeConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpgradeConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UpgradeConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UpgradeConfigMut<'msg> {
  type MutProxied = UpgradeConfig;
  fn as_mut(&mut self) -> UpgradeConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpgradeConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> UpgradeConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UpgradeConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UpgradeConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpgradeConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpgradeConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // upgrade_type: optional string
  pub fn upgrade_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_upgrade_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // filters: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter
  pub fn filters(&self) -> ::protobuf::RepeatedView<'_, super::super::HttpFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::HttpFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::HttpFilter> {
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
  pub fn set_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::HttpFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // enabled: optional message google.protobuf.BoolValue
  pub fn has_enabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_enabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn enabled_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enabled().then(|| self.enabled())
  }
  pub fn enabled(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enabled_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_enabled(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl UpgradeConfig

impl ::std::ops::Drop for UpgradeConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UpgradeConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UpgradeConfig {
  type Proxied = Self;
  fn as_view(&self) -> UpgradeConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UpgradeConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpgradeConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UpgradeConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__UpgradeConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__UpgradeConfig_msg_init.0, &[<super::super::HttpFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__UpgradeConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpgradeConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpgradeConfig {
  type Msg = UpgradeConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpgradeConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpgradeConfig {
  type Msg = UpgradeConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpgradeConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpgradeConfigMut<'_> {
  type Msg = UpgradeConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpgradeConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpgradeConfigMut<'_> {
  type Msg = UpgradeConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpgradeConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpgradeConfigView<'_> {
  type Msg = UpgradeConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpgradeConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpgradeConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__PathNormalizationOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PathNormalizationOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PathNormalizationOptions>
}

impl ::protobuf::Message for PathNormalizationOptions {
  type MessageView<'msg> = PathNormalizationOptionsView<'msg>;
  type MessageMut<'msg> = PathNormalizationOptionsMut<'msg>;
}

impl ::std::default::Default for PathNormalizationOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PathNormalizationOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PathNormalizationOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `PathNormalizationOptionsMut`.
unsafe impl ::std::marker::Sync for PathNormalizationOptions {}

// SAFETY:
// - `PathNormalizationOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PathNormalizationOptions {}

impl ::protobuf::Proxied for PathNormalizationOptions {
  type View<'msg> = PathNormalizationOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PathNormalizationOptions {}

impl ::protobuf::MutProxied for PathNormalizationOptions {
  type Mut<'msg> = PathNormalizationOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PathNormalizationOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathNormalizationOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathNormalizationOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PathNormalizationOptionsView<'msg> {
  type Message = PathNormalizationOptions;
}

impl ::std::fmt::Debug for PathNormalizationOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PathNormalizationOptionsView<'_> {
  fn default() -> PathNormalizationOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PathNormalizationOptions>> for PathNormalizationOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathNormalizationOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathNormalizationOptionsView<'msg> {

  pub fn to_owned(&self) -> PathNormalizationOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // forwarding_transformation: optional message envoy.type.http.v3.PathTransformation
  pub fn has_forwarding_transformation(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn forwarding_transformation_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'msg>> {
    self.has_forwarding_transformation().then(|| self.forwarding_transformation())
  }
  pub fn forwarding_transformation(self) -> crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView::default())
  }

  // http_filter_transformation: optional message envoy.type.http.v3.PathTransformation
  pub fn has_http_filter_transformation(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn http_filter_transformation_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'msg>> {
    self.has_http_filter_transformation().then(|| self.http_filter_transformation())
  }
  pub fn http_filter_transformation(self) -> crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView::default())
  }

}

// SAFETY:
// - `PathNormalizationOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PathNormalizationOptionsView<'_> {}

// SAFETY:
// - `PathNormalizationOptionsView` is `Send` because while its alive a `PathNormalizationOptionsMut` cannot.
// - `PathNormalizationOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for PathNormalizationOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for PathNormalizationOptionsView<'msg> {
  type Proxied = PathNormalizationOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, PathNormalizationOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathNormalizationOptionsView<'msg> {
  fn into_view<'shorter>(self) -> PathNormalizationOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PathNormalizationOptions> for PathNormalizationOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathNormalizationOptions {
    let mut dst = PathNormalizationOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PathNormalizationOptions> for PathNormalizationOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathNormalizationOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PathNormalizationOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathNormalizationOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathNormalizationOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PathNormalizationOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathNormalizationOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathNormalizationOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PathNormalizationOptionsMut<'msg> {
  type Message = PathNormalizationOptions;
}

impl ::std::fmt::Debug for PathNormalizationOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PathNormalizationOptions>> for PathNormalizationOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathNormalizationOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathNormalizationOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PathNormalizationOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PathNormalizationOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // forwarding_transformation: optional message envoy.type.http.v3.PathTransformation
  pub fn has_forwarding_transformation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_forwarding_transformation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn forwarding_transformation_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'_>> {
    self.has_forwarding_transformation().then(|| self.forwarding_transformation())
  }
  pub fn forwarding_transformation(&self) -> crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView::default())
  }
  pub fn forwarding_transformation_mut(&mut self) -> crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationMut<'_> {
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
  pub fn set_forwarding_transformation(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // http_filter_transformation: optional message envoy.type.http.v3.PathTransformation
  pub fn has_http_filter_transformation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_http_filter_transformation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn http_filter_transformation_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'_>> {
    self.has_http_filter_transformation().then(|| self.http_filter_transformation())
  }
  pub fn http_filter_transformation(&self) -> crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView::default())
  }
  pub fn http_filter_transformation_mut(&mut self) -> crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationMut<'_> {
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
  pub fn set_http_filter_transformation(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformation>) {

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
// - `PathNormalizationOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PathNormalizationOptionsMut<'_> {}

// SAFETY:
// - `PathNormalizationOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PathNormalizationOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for PathNormalizationOptionsMut<'msg> {
  type Proxied = PathNormalizationOptions;
  fn as_view(&self) -> ::protobuf::View<'_, PathNormalizationOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathNormalizationOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PathNormalizationOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PathNormalizationOptionsMut<'msg> {
  type MutProxied = PathNormalizationOptions;
  fn as_mut(&mut self) -> PathNormalizationOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PathNormalizationOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> PathNormalizationOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PathNormalizationOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PathNormalizationOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PathNormalizationOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PathNormalizationOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // forwarding_transformation: optional message envoy.type.http.v3.PathTransformation
  pub fn has_forwarding_transformation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_forwarding_transformation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn forwarding_transformation_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'_>> {
    self.has_forwarding_transformation().then(|| self.forwarding_transformation())
  }
  pub fn forwarding_transformation(&self) -> crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView::default())
  }
  pub fn forwarding_transformation_mut(&mut self) -> crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationMut<'_> {
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
  pub fn set_forwarding_transformation(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // http_filter_transformation: optional message envoy.type.http.v3.PathTransformation
  pub fn has_http_filter_transformation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_http_filter_transformation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn http_filter_transformation_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'_>> {
    self.has_http_filter_transformation().then(|| self.http_filter_transformation())
  }
  pub fn http_filter_transformation(&self) -> crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationView::default())
  }
  pub fn http_filter_transformation_mut(&mut self) -> crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformationMut<'_> {
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
  pub fn set_http_filter_transformation(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl PathNormalizationOptions

impl ::std::ops::Drop for PathNormalizationOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PathNormalizationOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PathNormalizationOptions {
  type Proxied = Self;
  fn as_view(&self) -> PathNormalizationOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PathNormalizationOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PathNormalizationOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PathNormalizationOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__PathNormalizationOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__PathNormalizationOptions_msg_init.0, &[<crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::http::v3::path_transformation::PathTransformation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__PathNormalizationOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathNormalizationOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathNormalizationOptions {
  type Msg = PathNormalizationOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathNormalizationOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathNormalizationOptions {
  type Msg = PathNormalizationOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathNormalizationOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathNormalizationOptionsMut<'_> {
  type Msg = PathNormalizationOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathNormalizationOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathNormalizationOptionsMut<'_> {
  type Msg = PathNormalizationOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathNormalizationOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathNormalizationOptionsView<'_> {
  type Msg = PathNormalizationOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathNormalizationOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathNormalizationOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__ProxyStatusConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ProxyStatusConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ProxyStatusConfig>
}

impl ::protobuf::Message for ProxyStatusConfig {
  type MessageView<'msg> = ProxyStatusConfigView<'msg>;
  type MessageMut<'msg> = ProxyStatusConfigMut<'msg>;
}

impl ::std::default::Default for ProxyStatusConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ProxyStatusConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ProxyStatusConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ProxyStatusConfigMut`.
unsafe impl ::std::marker::Sync for ProxyStatusConfig {}

// SAFETY:
// - `ProxyStatusConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ProxyStatusConfig {}

impl ::protobuf::Proxied for ProxyStatusConfig {
  type View<'msg> = ProxyStatusConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ProxyStatusConfig {}

impl ::protobuf::MutProxied for ProxyStatusConfig {
  type Mut<'msg> = ProxyStatusConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProxyStatusConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyStatusConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProxyStatusConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ProxyStatusConfigView<'msg> {
  type Message = ProxyStatusConfig;
}

impl ::std::fmt::Debug for ProxyStatusConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ProxyStatusConfigView<'_> {
  fn default() -> ProxyStatusConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyStatusConfig>> for ProxyStatusConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProxyStatusConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProxyStatusConfigView<'msg> {

  pub fn to_owned(&self) -> ProxyStatusConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // remove_details: optional bool
  pub fn remove_details(self) -> bool {
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

  // remove_connection_termination_details: optional bool
  pub fn remove_connection_termination_details(self) -> bool {
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

  // remove_response_flags: optional bool
  pub fn remove_response_flags(self) -> bool {
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

  // set_recommended_response_code: optional bool
  pub fn set_recommended_response_code(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

  // use_node_id: optional bool
  pub fn has_use_node_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn use_node_id_opt(self) -> ::std::option::Option<bool> {
    self.has_use_node_id().then(|| self.use_node_id())
  }
  pub fn use_node_id(self) -> bool {
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

  // literal_proxy_name: optional string
  pub fn has_literal_proxy_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn literal_proxy_name_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_literal_proxy_name().then(|| self.literal_proxy_name())
  }
  pub fn literal_proxy_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn proxy_name(self) -> super::super::http_connection_manager::proxy_status_config::ProxyNameOneof<'msg> {
    match self.proxy_name_case() {
      super::super::http_connection_manager::proxy_status_config::ProxyNameCase::UseNodeId =>
          super::super::http_connection_manager::proxy_status_config::ProxyNameOneof::UseNodeId(self.use_node_id()),
      super::super::http_connection_manager::proxy_status_config::ProxyNameCase::LiteralProxyName =>
          super::super::http_connection_manager::proxy_status_config::ProxyNameOneof::LiteralProxyName(self.literal_proxy_name()),
      _ => super::super::http_connection_manager::proxy_status_config::ProxyNameOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn proxy_name_case(self) -> super::super::http_connection_manager::proxy_status_config::ProxyNameCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(4);
      super::super::http_connection_manager::proxy_status_config::ProxyNameCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ProxyStatusConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ProxyStatusConfigView<'_> {}

// SAFETY:
// - `ProxyStatusConfigView` is `Send` because while its alive a `ProxyStatusConfigMut` cannot.
// - `ProxyStatusConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ProxyStatusConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ProxyStatusConfigView<'msg> {
  type Proxied = ProxyStatusConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ProxyStatusConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProxyStatusConfigView<'msg> {
  fn into_view<'shorter>(self) -> ProxyStatusConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ProxyStatusConfig> for ProxyStatusConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProxyStatusConfig {
    let mut dst = ProxyStatusConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ProxyStatusConfig> for ProxyStatusConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProxyStatusConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ProxyStatusConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProxyStatusConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProxyStatusConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProxyStatusConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyStatusConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProxyStatusConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ProxyStatusConfigMut<'msg> {
  type Message = ProxyStatusConfig;
}

impl ::std::fmt::Debug for ProxyStatusConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyStatusConfig>> for ProxyStatusConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyStatusConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProxyStatusConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ProxyStatusConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ProxyStatusConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // remove_details: optional bool
  pub fn remove_details(&self) -> bool {
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
  pub fn set_remove_details(&mut self, val: bool) {
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

  // remove_connection_termination_details: optional bool
  pub fn remove_connection_termination_details(&self) -> bool {
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
  pub fn set_remove_connection_termination_details(&mut self, val: bool) {
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

  // remove_response_flags: optional bool
  pub fn remove_response_flags(&self) -> bool {
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
  pub fn set_remove_response_flags(&mut self, val: bool) {
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

  // set_recommended_response_code: optional bool
  pub fn set_recommended_response_code(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_set_recommended_response_code(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // use_node_id: optional bool
  pub fn has_use_node_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_use_node_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn use_node_id_opt(&self) -> ::std::option::Option<bool> {
    self.has_use_node_id().then(|| self.use_node_id())
  }
  pub fn use_node_id(&self) -> bool {
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
  pub fn set_use_node_id(&mut self, val: bool) {
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

  // literal_proxy_name: optional string
  pub fn has_literal_proxy_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_literal_proxy_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn literal_proxy_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_literal_proxy_name().then(|| self.literal_proxy_name())
  }
  pub fn literal_proxy_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_literal_proxy_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  pub fn proxy_name(&self) -> super::super::http_connection_manager::proxy_status_config::ProxyNameOneof<'_> {
    match &self.proxy_name_case() {
      super::super::http_connection_manager::proxy_status_config::ProxyNameCase::UseNodeId =>
          super::super::http_connection_manager::proxy_status_config::ProxyNameOneof::UseNodeId(self.use_node_id()),
      super::super::http_connection_manager::proxy_status_config::ProxyNameCase::LiteralProxyName =>
          super::super::http_connection_manager::proxy_status_config::ProxyNameOneof::LiteralProxyName(self.literal_proxy_name()),
      _ => super::super::http_connection_manager::proxy_status_config::ProxyNameOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn proxy_name_case(&self) -> super::super::http_connection_manager::proxy_status_config::ProxyNameCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(4);
      super::super::http_connection_manager::proxy_status_config::ProxyNameCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ProxyStatusConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ProxyStatusConfigMut<'_> {}

// SAFETY:
// - `ProxyStatusConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ProxyStatusConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ProxyStatusConfigMut<'msg> {
  type Proxied = ProxyStatusConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ProxyStatusConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProxyStatusConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ProxyStatusConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ProxyStatusConfigMut<'msg> {
  type MutProxied = ProxyStatusConfig;
  fn as_mut(&mut self) -> ProxyStatusConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ProxyStatusConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ProxyStatusConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ProxyStatusConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ProxyStatusConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ProxyStatusConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ProxyStatusConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // remove_details: optional bool
  pub fn remove_details(&self) -> bool {
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
  pub fn set_remove_details(&mut self, val: bool) {
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

  // remove_connection_termination_details: optional bool
  pub fn remove_connection_termination_details(&self) -> bool {
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
  pub fn set_remove_connection_termination_details(&mut self, val: bool) {
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

  // remove_response_flags: optional bool
  pub fn remove_response_flags(&self) -> bool {
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
  pub fn set_remove_response_flags(&mut self, val: bool) {
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

  // set_recommended_response_code: optional bool
  pub fn set_recommended_response_code(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_set_recommended_response_code(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // use_node_id: optional bool
  pub fn has_use_node_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_use_node_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn use_node_id_opt(&self) -> ::std::option::Option<bool> {
    self.has_use_node_id().then(|| self.use_node_id())
  }
  pub fn use_node_id(&self) -> bool {
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
  pub fn set_use_node_id(&mut self, val: bool) {
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

  // literal_proxy_name: optional string
  pub fn has_literal_proxy_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_literal_proxy_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn literal_proxy_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_literal_proxy_name().then(|| self.literal_proxy_name())
  }
  pub fn literal_proxy_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_literal_proxy_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  pub fn proxy_name(&self) -> super::super::http_connection_manager::proxy_status_config::ProxyNameOneof<'_> {
    match &self.proxy_name_case() {
      super::super::http_connection_manager::proxy_status_config::ProxyNameCase::UseNodeId =>
          super::super::http_connection_manager::proxy_status_config::ProxyNameOneof::UseNodeId(self.use_node_id()),
      super::super::http_connection_manager::proxy_status_config::ProxyNameCase::LiteralProxyName =>
          super::super::http_connection_manager::proxy_status_config::ProxyNameOneof::LiteralProxyName(self.literal_proxy_name()),
      _ => super::super::http_connection_manager::proxy_status_config::ProxyNameOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn proxy_name_case(&self) -> super::super::http_connection_manager::proxy_status_config::ProxyNameCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(4);
      super::super::http_connection_manager::proxy_status_config::ProxyNameCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ProxyStatusConfig

impl ::std::ops::Drop for ProxyStatusConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ProxyStatusConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ProxyStatusConfig {
  type Proxied = Self;
  fn as_view(&self) -> ProxyStatusConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ProxyStatusConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ProxyStatusConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ProxyStatusConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__ProxyStatusConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P/P/P/P/1T^&|(");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__ProxyStatusConfig_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__ProxyStatusConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProxyStatusConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProxyStatusConfig {
  type Msg = ProxyStatusConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyStatusConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyStatusConfig {
  type Msg = ProxyStatusConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyStatusConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProxyStatusConfigMut<'_> {
  type Msg = ProxyStatusConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyStatusConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyStatusConfigMut<'_> {
  type Msg = ProxyStatusConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyStatusConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProxyStatusConfigView<'_> {
  type Msg = ProxyStatusConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProxyStatusConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProxyStatusConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod proxy_status_config {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ProxyNameOneof<'msg> {
  UseNodeId(bool) = 5,
  LiteralProxyName(&'msg ::protobuf::ProtoStr) = 6,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ProxyNameCase {
  UseNodeId = 5,
  LiteralProxyName = 6,

  not_set = 0
}

impl ProxyNameCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ProxyNameCase> {
    match v {
      0 => Some(ProxyNameCase::not_set),
      5 => Some(ProxyNameCase::UseNodeId),
      6 => Some(ProxyNameCase::LiteralProxyName),
      _ => None
    }
  }
}
}  // pub mod proxy_status_config

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__HcmAccessLogOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HcmAccessLogOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HcmAccessLogOptions>
}

impl ::protobuf::Message for HcmAccessLogOptions {
  type MessageView<'msg> = HcmAccessLogOptionsView<'msg>;
  type MessageMut<'msg> = HcmAccessLogOptionsMut<'msg>;
}

impl ::std::default::Default for HcmAccessLogOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HcmAccessLogOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HcmAccessLogOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `HcmAccessLogOptionsMut`.
unsafe impl ::std::marker::Sync for HcmAccessLogOptions {}

// SAFETY:
// - `HcmAccessLogOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HcmAccessLogOptions {}

impl ::protobuf::Proxied for HcmAccessLogOptions {
  type View<'msg> = HcmAccessLogOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HcmAccessLogOptions {}

impl ::protobuf::MutProxied for HcmAccessLogOptions {
  type Mut<'msg> = HcmAccessLogOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HcmAccessLogOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HcmAccessLogOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HcmAccessLogOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HcmAccessLogOptionsView<'msg> {
  type Message = HcmAccessLogOptions;
}

impl ::std::fmt::Debug for HcmAccessLogOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HcmAccessLogOptionsView<'_> {
  fn default() -> HcmAccessLogOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HcmAccessLogOptions>> for HcmAccessLogOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HcmAccessLogOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HcmAccessLogOptionsView<'msg> {

  pub fn to_owned(&self) -> HcmAccessLogOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // access_log_flush_interval: optional message google.protobuf.Duration
  pub fn has_access_log_flush_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn access_log_flush_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_access_log_flush_interval().then(|| self.access_log_flush_interval())
  }
  pub fn access_log_flush_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // flush_access_log_on_new_request: optional bool
  pub fn flush_access_log_on_new_request(self) -> bool {
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

  // flush_log_on_tunnel_successfully_established: optional bool
  pub fn flush_log_on_tunnel_successfully_established(self) -> bool {
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
// - `HcmAccessLogOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HcmAccessLogOptionsView<'_> {}

// SAFETY:
// - `HcmAccessLogOptionsView` is `Send` because while its alive a `HcmAccessLogOptionsMut` cannot.
// - `HcmAccessLogOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for HcmAccessLogOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for HcmAccessLogOptionsView<'msg> {
  type Proxied = HcmAccessLogOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, HcmAccessLogOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HcmAccessLogOptionsView<'msg> {
  fn into_view<'shorter>(self) -> HcmAccessLogOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HcmAccessLogOptions> for HcmAccessLogOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HcmAccessLogOptions {
    let mut dst = HcmAccessLogOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HcmAccessLogOptions> for HcmAccessLogOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HcmAccessLogOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HcmAccessLogOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HcmAccessLogOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HcmAccessLogOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HcmAccessLogOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HcmAccessLogOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HcmAccessLogOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HcmAccessLogOptionsMut<'msg> {
  type Message = HcmAccessLogOptions;
}

impl ::std::fmt::Debug for HcmAccessLogOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HcmAccessLogOptions>> for HcmAccessLogOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HcmAccessLogOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HcmAccessLogOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HcmAccessLogOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HcmAccessLogOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // access_log_flush_interval: optional message google.protobuf.Duration
  pub fn has_access_log_flush_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_access_log_flush_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn access_log_flush_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_access_log_flush_interval().then(|| self.access_log_flush_interval())
  }
  pub fn access_log_flush_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn access_log_flush_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_access_log_flush_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // flush_access_log_on_new_request: optional bool
  pub fn flush_access_log_on_new_request(&self) -> bool {
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
  pub fn set_flush_access_log_on_new_request(&mut self, val: bool) {
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

  // flush_log_on_tunnel_successfully_established: optional bool
  pub fn flush_log_on_tunnel_successfully_established(&self) -> bool {
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
  pub fn set_flush_log_on_tunnel_successfully_established(&mut self, val: bool) {
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
// - `HcmAccessLogOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HcmAccessLogOptionsMut<'_> {}

// SAFETY:
// - `HcmAccessLogOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HcmAccessLogOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for HcmAccessLogOptionsMut<'msg> {
  type Proxied = HcmAccessLogOptions;
  fn as_view(&self) -> ::protobuf::View<'_, HcmAccessLogOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HcmAccessLogOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HcmAccessLogOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HcmAccessLogOptionsMut<'msg> {
  type MutProxied = HcmAccessLogOptions;
  fn as_mut(&mut self) -> HcmAccessLogOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HcmAccessLogOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> HcmAccessLogOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HcmAccessLogOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HcmAccessLogOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HcmAccessLogOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HcmAccessLogOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // access_log_flush_interval: optional message google.protobuf.Duration
  pub fn has_access_log_flush_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_access_log_flush_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn access_log_flush_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_access_log_flush_interval().then(|| self.access_log_flush_interval())
  }
  pub fn access_log_flush_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn access_log_flush_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_access_log_flush_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // flush_access_log_on_new_request: optional bool
  pub fn flush_access_log_on_new_request(&self) -> bool {
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
  pub fn set_flush_access_log_on_new_request(&mut self, val: bool) {
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

  // flush_log_on_tunnel_successfully_established: optional bool
  pub fn flush_log_on_tunnel_successfully_established(&self) -> bool {
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
  pub fn set_flush_log_on_tunnel_successfully_established(&mut self, val: bool) {
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

}  // impl HcmAccessLogOptions

impl ::std::ops::Drop for HcmAccessLogOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HcmAccessLogOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HcmAccessLogOptions {
  type Proxied = Self;
  fn as_view(&self) -> HcmAccessLogOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HcmAccessLogOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HcmAccessLogOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HcmAccessLogOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__HcmAccessLogOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__HcmAccessLogOptions_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http_connection_manager::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpConnectionManager__HcmAccessLogOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HcmAccessLogOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HcmAccessLogOptions {
  type Msg = HcmAccessLogOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HcmAccessLogOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HcmAccessLogOptions {
  type Msg = HcmAccessLogOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HcmAccessLogOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HcmAccessLogOptionsMut<'_> {
  type Msg = HcmAccessLogOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HcmAccessLogOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HcmAccessLogOptionsMut<'_> {
  type Msg = HcmAccessLogOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HcmAccessLogOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HcmAccessLogOptionsView<'_> {
  type Msg = HcmAccessLogOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HcmAccessLogOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HcmAccessLogOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CodecType(i32);

#[allow(non_upper_case_globals)]
impl CodecType {
  pub const Auto: CodecType = CodecType(0);
  pub const Http1: CodecType = CodecType(1);
  pub const Http2: CodecType = CodecType(2);
  pub const Http3: CodecType = CodecType(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Auto",
      1 => "Http1",
      2 => "Http2",
      3 => "Http3",
      _ => return None
    })
  }
}

impl ::std::convert::From<CodecType> for i32 {
  fn from(val: CodecType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for CodecType {
  fn from(val: i32) -> CodecType {
    Self(val)
  }
}

impl ::std::default::Default for CodecType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for CodecType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "CodecType::{}", constant_name)
    } else {
      write!(f, "CodecType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for CodecType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for CodecType {}

impl ::protobuf::Proxied for CodecType {
  type View<'a> = CodecType;
}

impl ::protobuf::AsView for CodecType {
  type Proxied = CodecType;

  fn as_view(&self) -> CodecType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CodecType {
  fn into_view<'shorter>(self) -> CodecType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for CodecType {
  const NAME: &'static str = "CodecType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for CodecType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ServerHeaderTransformation(i32);

#[allow(non_upper_case_globals)]
impl ServerHeaderTransformation {
  pub const Overwrite: ServerHeaderTransformation = ServerHeaderTransformation(0);
  pub const AppendIfAbsent: ServerHeaderTransformation = ServerHeaderTransformation(1);
  pub const PassThrough: ServerHeaderTransformation = ServerHeaderTransformation(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Overwrite",
      1 => "AppendIfAbsent",
      2 => "PassThrough",
      _ => return None
    })
  }
}

impl ::std::convert::From<ServerHeaderTransformation> for i32 {
  fn from(val: ServerHeaderTransformation) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ServerHeaderTransformation {
  fn from(val: i32) -> ServerHeaderTransformation {
    Self(val)
  }
}

impl ::std::default::Default for ServerHeaderTransformation {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ServerHeaderTransformation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ServerHeaderTransformation::{}", constant_name)
    } else {
      write!(f, "ServerHeaderTransformation::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ServerHeaderTransformation {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ServerHeaderTransformation {}

impl ::protobuf::Proxied for ServerHeaderTransformation {
  type View<'a> = ServerHeaderTransformation;
}

impl ::protobuf::AsView for ServerHeaderTransformation {
  type Proxied = ServerHeaderTransformation;

  fn as_view(&self) -> ServerHeaderTransformation {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ServerHeaderTransformation {
  fn into_view<'shorter>(self) -> ServerHeaderTransformation where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ServerHeaderTransformation {
  const NAME: &'static str = "ServerHeaderTransformation";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for ServerHeaderTransformation {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ForwardClientCertDetails(i32);

#[allow(non_upper_case_globals)]
impl ForwardClientCertDetails {
  pub const Sanitize: ForwardClientCertDetails = ForwardClientCertDetails(0);
  pub const ForwardOnly: ForwardClientCertDetails = ForwardClientCertDetails(1);
  pub const AppendForward: ForwardClientCertDetails = ForwardClientCertDetails(2);
  pub const SanitizeSet: ForwardClientCertDetails = ForwardClientCertDetails(3);
  pub const AlwaysForwardOnly: ForwardClientCertDetails = ForwardClientCertDetails(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Sanitize",
      1 => "ForwardOnly",
      2 => "AppendForward",
      3 => "SanitizeSet",
      4 => "AlwaysForwardOnly",
      _ => return None
    })
  }
}

impl ::std::convert::From<ForwardClientCertDetails> for i32 {
  fn from(val: ForwardClientCertDetails) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ForwardClientCertDetails {
  fn from(val: i32) -> ForwardClientCertDetails {
    Self(val)
  }
}

impl ::std::default::Default for ForwardClientCertDetails {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ForwardClientCertDetails {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ForwardClientCertDetails::{}", constant_name)
    } else {
      write!(f, "ForwardClientCertDetails::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ForwardClientCertDetails {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ForwardClientCertDetails {}

impl ::protobuf::Proxied for ForwardClientCertDetails {
  type View<'a> = ForwardClientCertDetails;
}

impl ::protobuf::AsView for ForwardClientCertDetails {
  type Proxied = ForwardClientCertDetails;

  fn as_view(&self) -> ForwardClientCertDetails {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ForwardClientCertDetails {
  fn into_view<'shorter>(self) -> ForwardClientCertDetails where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ForwardClientCertDetails {
  const NAME: &'static str = "ForwardClientCertDetails";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for ForwardClientCertDetails {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PathWithEscapedSlashesAction(i32);

#[allow(non_upper_case_globals)]
impl PathWithEscapedSlashesAction {
  pub const ImplementationSpecificDefault: PathWithEscapedSlashesAction = PathWithEscapedSlashesAction(0);
  pub const KeepUnchanged: PathWithEscapedSlashesAction = PathWithEscapedSlashesAction(1);
  pub const RejectRequest: PathWithEscapedSlashesAction = PathWithEscapedSlashesAction(2);
  pub const UnescapeAndRedirect: PathWithEscapedSlashesAction = PathWithEscapedSlashesAction(3);
  pub const UnescapeAndForward: PathWithEscapedSlashesAction = PathWithEscapedSlashesAction(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "ImplementationSpecificDefault",
      1 => "KeepUnchanged",
      2 => "RejectRequest",
      3 => "UnescapeAndRedirect",
      4 => "UnescapeAndForward",
      _ => return None
    })
  }
}

impl ::std::convert::From<PathWithEscapedSlashesAction> for i32 {
  fn from(val: PathWithEscapedSlashesAction) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for PathWithEscapedSlashesAction {
  fn from(val: i32) -> PathWithEscapedSlashesAction {
    Self(val)
  }
}

impl ::std::default::Default for PathWithEscapedSlashesAction {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for PathWithEscapedSlashesAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "PathWithEscapedSlashesAction::{}", constant_name)
    } else {
      write!(f, "PathWithEscapedSlashesAction::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for PathWithEscapedSlashesAction {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for PathWithEscapedSlashesAction {}

impl ::protobuf::Proxied for PathWithEscapedSlashesAction {
  type View<'a> = PathWithEscapedSlashesAction;
}

impl ::protobuf::AsView for PathWithEscapedSlashesAction {
  type Proxied = PathWithEscapedSlashesAction;

  fn as_view(&self) -> PathWithEscapedSlashesAction {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathWithEscapedSlashesAction {
  fn into_view<'shorter>(self) -> PathWithEscapedSlashesAction where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for PathWithEscapedSlashesAction {
  const NAME: &'static str = "PathWithEscapedSlashesAction";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for PathWithEscapedSlashesAction {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum RouteSpecifierOneof<'msg> {
  Rds(::protobuf::View<'msg, super::super::Rds>) = 3,
  RouteConfig(::protobuf::View<'msg, crate::xds::generated::envoy::config::route::v3::route::RouteConfiguration>) = 4,
  ScopedRoutes(::protobuf::View<'msg, super::super::ScopedRoutes>) = 31,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum RouteSpecifierCase {
  Rds = 3,
  RouteConfig = 4,
  ScopedRoutes = 31,

  not_set = 0
}

impl RouteSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<RouteSpecifierCase> {
    match v {
      0 => Some(RouteSpecifierCase::not_set),
      3 => Some(RouteSpecifierCase::Rds),
      4 => Some(RouteSpecifierCase::RouteConfig),
      31 => Some(RouteSpecifierCase::ScopedRoutes),
      _ => None
    }
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum StripPortModeOneof<'msg> {
  StripAnyHostPort(bool) = 42,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum StripPortModeCase {
  StripAnyHostPort = 42,

  not_set = 0
}

impl StripPortModeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<StripPortModeCase> {
    match v {
      0 => Some(StripPortModeCase::not_set),
      42 => Some(StripPortModeCase::StripAnyHostPort),
      _ => None
    }
  }
}
}  // pub mod http_connection_manager


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__LocalReplyConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LocalReplyConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LocalReplyConfig>
}

impl ::protobuf::Message for LocalReplyConfig {
  type MessageView<'msg> = LocalReplyConfigView<'msg>;
  type MessageMut<'msg> = LocalReplyConfigMut<'msg>;
}

impl ::std::default::Default for LocalReplyConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LocalReplyConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LocalReplyConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `LocalReplyConfigMut`.
unsafe impl ::std::marker::Sync for LocalReplyConfig {}

// SAFETY:
// - `LocalReplyConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LocalReplyConfig {}

impl ::protobuf::Proxied for LocalReplyConfig {
  type View<'msg> = LocalReplyConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LocalReplyConfig {}

impl ::protobuf::MutProxied for LocalReplyConfig {
  type Mut<'msg> = LocalReplyConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LocalReplyConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalReplyConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalReplyConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LocalReplyConfigView<'msg> {
  type Message = LocalReplyConfig;
}

impl ::std::fmt::Debug for LocalReplyConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LocalReplyConfigView<'_> {
  fn default() -> LocalReplyConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LocalReplyConfig>> for LocalReplyConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalReplyConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalReplyConfigView<'msg> {

  pub fn to_owned(&self) -> LocalReplyConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // mappers: repeated message envoy.extensions.filters.network.http_connection_manager.v3.ResponseMapper
  pub fn mappers(self) -> ::protobuf::RepeatedView<'msg, super::ResponseMapper> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResponseMapper>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // body_format: optional message envoy.config.core.v3.SubstitutionFormatString
  pub fn has_body_format(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn body_format_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'msg>> {
    self.has_body_format().then(|| self.body_format())
  }
  pub fn body_format(self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView::default())
  }

}

// SAFETY:
// - `LocalReplyConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LocalReplyConfigView<'_> {}

// SAFETY:
// - `LocalReplyConfigView` is `Send` because while its alive a `LocalReplyConfigMut` cannot.
// - `LocalReplyConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for LocalReplyConfigView<'_> {}

impl<'msg> ::protobuf::AsView for LocalReplyConfigView<'msg> {
  type Proxied = LocalReplyConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, LocalReplyConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalReplyConfigView<'msg> {
  fn into_view<'shorter>(self) -> LocalReplyConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalReplyConfig> for LocalReplyConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalReplyConfig {
    let mut dst = LocalReplyConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalReplyConfig> for LocalReplyConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalReplyConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LocalReplyConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalReplyConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalReplyConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LocalReplyConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalReplyConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalReplyConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LocalReplyConfigMut<'msg> {
  type Message = LocalReplyConfig;
}

impl ::std::fmt::Debug for LocalReplyConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LocalReplyConfig>> for LocalReplyConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalReplyConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalReplyConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalReplyConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LocalReplyConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // mappers: repeated message envoy.extensions.filters.network.http_connection_manager.v3.ResponseMapper
  pub fn mappers(&self) -> ::protobuf::RepeatedView<'_, super::ResponseMapper> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResponseMapper>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn mappers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResponseMapper> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_mappers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResponseMapper>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // body_format: optional message envoy.config.core.v3.SubstitutionFormatString
  pub fn has_body_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_body_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn body_format_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_>> {
    self.has_body_format().then(|| self.body_format())
  }
  pub fn body_format(&self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView::default())
  }
  pub fn body_format_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringMut<'_> {
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
  pub fn set_body_format(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatString>) {

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
// - `LocalReplyConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LocalReplyConfigMut<'_> {}

// SAFETY:
// - `LocalReplyConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LocalReplyConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for LocalReplyConfigMut<'msg> {
  type Proxied = LocalReplyConfig;
  fn as_view(&self) -> ::protobuf::View<'_, LocalReplyConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalReplyConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LocalReplyConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LocalReplyConfigMut<'msg> {
  type MutProxied = LocalReplyConfig;
  fn as_mut(&mut self) -> LocalReplyConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LocalReplyConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> LocalReplyConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LocalReplyConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LocalReplyConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LocalReplyConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LocalReplyConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // mappers: repeated message envoy.extensions.filters.network.http_connection_manager.v3.ResponseMapper
  pub fn mappers(&self) -> ::protobuf::RepeatedView<'_, super::ResponseMapper> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResponseMapper>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn mappers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResponseMapper> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_mappers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResponseMapper>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // body_format: optional message envoy.config.core.v3.SubstitutionFormatString
  pub fn has_body_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_body_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn body_format_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_>> {
    self.has_body_format().then(|| self.body_format())
  }
  pub fn body_format(&self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView::default())
  }
  pub fn body_format_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringMut<'_> {
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
  pub fn set_body_format(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl LocalReplyConfig

impl ::std::ops::Drop for LocalReplyConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LocalReplyConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LocalReplyConfig {
  type Proxied = Self;
  fn as_view(&self) -> LocalReplyConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LocalReplyConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LocalReplyConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LocalReplyConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__network__http_0connection_0manager__v3__LocalReplyConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__network__http_0connection_0manager__v3__LocalReplyConfig_msg_init.0, &[<super::ResponseMapper as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__network__http_0connection_0manager__v3__LocalReplyConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalReplyConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalReplyConfig {
  type Msg = LocalReplyConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalReplyConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalReplyConfig {
  type Msg = LocalReplyConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalReplyConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalReplyConfigMut<'_> {
  type Msg = LocalReplyConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalReplyConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalReplyConfigMut<'_> {
  type Msg = LocalReplyConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalReplyConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalReplyConfigView<'_> {
  type Msg = LocalReplyConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalReplyConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalReplyConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__ResponseMapper_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResponseMapper {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResponseMapper>
}

impl ::protobuf::Message for ResponseMapper {
  type MessageView<'msg> = ResponseMapperView<'msg>;
  type MessageMut<'msg> = ResponseMapperMut<'msg>;
}

impl ::std::default::Default for ResponseMapper {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResponseMapper {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResponseMapper` is `Sync` because it does not implement interior mutability.
//    Neither does `ResponseMapperMut`.
unsafe impl ::std::marker::Sync for ResponseMapper {}

// SAFETY:
// - `ResponseMapper` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ResponseMapper {}

impl ::protobuf::Proxied for ResponseMapper {
  type View<'msg> = ResponseMapperView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResponseMapper {}

impl ::protobuf::MutProxied for ResponseMapper {
  type Mut<'msg> = ResponseMapperMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResponseMapperView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResponseMapper>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResponseMapperView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResponseMapperView<'msg> {
  type Message = ResponseMapper;
}

impl ::std::fmt::Debug for ResponseMapperView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResponseMapperView<'_> {
  fn default() -> ResponseMapperView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResponseMapper>> for ResponseMapperView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResponseMapper>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResponseMapperView<'msg> {

  pub fn to_owned(&self) -> ResponseMapper {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // filter: optional message envoy.config.accesslog.v3.AccessLogFilter
  pub fn has_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn filter_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilterView<'msg>> {
    self.has_filter().then(|| self.filter())
  }
  pub fn filter(self) -> crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilterView::default())
  }

  // status_code: optional message google.protobuf.UInt32Value
  pub fn has_status_code(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn status_code_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_status_code().then(|| self.status_code())
  }
  pub fn status_code(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // body: optional message envoy.config.core.v3.DataSource
  pub fn has_body(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn body_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_body().then(|| self.body())
  }
  pub fn body(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // body_format_override: optional message envoy.config.core.v3.SubstitutionFormatString
  pub fn has_body_format_override(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn body_format_override_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'msg>> {
    self.has_body_format_override().then(|| self.body_format_override())
  }
  pub fn body_format_override(self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView::default())
  }

  // headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn headers_to_add(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
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
// - `ResponseMapperView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ResponseMapperView<'_> {}

// SAFETY:
// - `ResponseMapperView` is `Send` because while its alive a `ResponseMapperMut` cannot.
// - `ResponseMapperView` does not use thread-local data.
unsafe impl ::std::marker::Send for ResponseMapperView<'_> {}

impl<'msg> ::protobuf::AsView for ResponseMapperView<'msg> {
  type Proxied = ResponseMapper;
  fn as_view(&self) -> ::protobuf::View<'msg, ResponseMapper> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResponseMapperView<'msg> {
  fn into_view<'shorter>(self) -> ResponseMapperView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResponseMapper> for ResponseMapperView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResponseMapper {
    let mut dst = ResponseMapper::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResponseMapper> for ResponseMapperMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResponseMapper {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ResponseMapper {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResponseMapperView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResponseMapperMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResponseMapperMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseMapper>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResponseMapperMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResponseMapperMut<'msg> {
  type Message = ResponseMapper;
}

impl ::std::fmt::Debug for ResponseMapperMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseMapper>> for ResponseMapperMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseMapper>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResponseMapperMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseMapper> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ResponseMapper {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // filter: optional message envoy.config.accesslog.v3.AccessLogFilter
  pub fn has_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn filter_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilterView<'_>> {
    self.has_filter().then(|| self.filter())
  }
  pub fn filter(&self) -> crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilterView::default())
  }
  pub fn filter_mut(&mut self) -> crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilterMut<'_> {
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
  pub fn set_filter(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // status_code: optional message google.protobuf.UInt32Value
  pub fn has_status_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_status_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn status_code_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_status_code().then(|| self.status_code())
  }
  pub fn status_code(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn status_code_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_status_code(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // body: optional message envoy.config.core.v3.DataSource
  pub fn has_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn body_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_body().then(|| self.body())
  }
  pub fn body(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn body_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_body(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // body_format_override: optional message envoy.config.core.v3.SubstitutionFormatString
  pub fn has_body_format_override(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_body_format_override(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn body_format_override_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_>> {
    self.has_body_format_override().then(|| self.body_format_override())
  }
  pub fn body_format_override(&self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView::default())
  }
  pub fn body_format_override_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringMut<'_> {
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
  pub fn set_body_format_override(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

}

// SAFETY:
// - `ResponseMapperMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ResponseMapperMut<'_> {}

// SAFETY:
// - `ResponseMapperMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ResponseMapperMut<'_> {}

impl<'msg> ::protobuf::AsView for ResponseMapperMut<'msg> {
  type Proxied = ResponseMapper;
  fn as_view(&self) -> ::protobuf::View<'_, ResponseMapper> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResponseMapperMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResponseMapper>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ResponseMapperMut<'msg> {
  type MutProxied = ResponseMapper;
  fn as_mut(&mut self) -> ResponseMapperMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResponseMapperMut<'msg> {
  fn into_mut<'shorter>(self) -> ResponseMapperMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResponseMapper {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResponseMapper> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResponseMapperView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResponseMapperMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // filter: optional message envoy.config.accesslog.v3.AccessLogFilter
  pub fn has_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn filter_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilterView<'_>> {
    self.has_filter().then(|| self.filter())
  }
  pub fn filter(&self) -> crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilterView::default())
  }
  pub fn filter_mut(&mut self) -> crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilterMut<'_> {
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
  pub fn set_filter(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // status_code: optional message google.protobuf.UInt32Value
  pub fn has_status_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_status_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn status_code_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_status_code().then(|| self.status_code())
  }
  pub fn status_code(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn status_code_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_status_code(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // body: optional message envoy.config.core.v3.DataSource
  pub fn has_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn body_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_body().then(|| self.body())
  }
  pub fn body(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn body_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_body(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // body_format_override: optional message envoy.config.core.v3.SubstitutionFormatString
  pub fn has_body_format_override(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_body_format_override(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn body_format_override_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_>> {
    self.has_body_format_override().then(|| self.body_format_override())
  }
  pub fn body_format_override(&self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringView::default())
  }
  pub fn body_format_override_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatStringMut<'_> {
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
  pub fn set_body_format_override(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

}  // impl ResponseMapper

impl ::std::ops::Drop for ResponseMapper {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResponseMapper {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResponseMapper {
  type Proxied = Self;
  fn as_view(&self) -> ResponseMapperView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResponseMapper {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResponseMapperMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResponseMapper {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__network__http_0connection_0manager__v3__ResponseMapper_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__network__http_0connection_0manager__v3__ResponseMapper_msg_init.0, &[<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLogFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::substitution_format_string::SubstitutionFormatString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__network__http_0connection_0manager__v3__ResponseMapper_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResponseMapper {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResponseMapper {
  type Msg = ResponseMapper;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseMapper> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResponseMapper {
  type Msg = ResponseMapper;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseMapper> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResponseMapperMut<'_> {
  type Msg = ResponseMapper;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseMapper> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResponseMapperMut<'_> {
  type Msg = ResponseMapper;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseMapper> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResponseMapperView<'_> {
  type Msg = ResponseMapper;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseMapper> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResponseMapperMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__Rds_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Rds {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Rds>
}

impl ::protobuf::Message for Rds {
  type MessageView<'msg> = RdsView<'msg>;
  type MessageMut<'msg> = RdsMut<'msg>;
}

impl ::std::default::Default for Rds {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Rds {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Rds` is `Sync` because it does not implement interior mutability.
//    Neither does `RdsMut`.
unsafe impl ::std::marker::Sync for Rds {}

// SAFETY:
// - `Rds` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Rds {}

impl ::protobuf::Proxied for Rds {
  type View<'msg> = RdsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Rds {}

impl ::protobuf::MutProxied for Rds {
  type Mut<'msg> = RdsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RdsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Rds>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RdsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RdsView<'msg> {
  type Message = Rds;
}

impl ::std::fmt::Debug for RdsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RdsView<'_> {
  fn default() -> RdsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Rds>> for RdsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Rds>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RdsView<'msg> {

  pub fn to_owned(&self) -> Rds {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_config_source(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn config_source_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

  // route_config_name: optional string
  pub fn route_config_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `RdsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RdsView<'_> {}

// SAFETY:
// - `RdsView` is `Send` because while its alive a `RdsMut` cannot.
// - `RdsView` does not use thread-local data.
unsafe impl ::std::marker::Send for RdsView<'_> {}

impl<'msg> ::protobuf::AsView for RdsView<'msg> {
  type Proxied = Rds;
  fn as_view(&self) -> ::protobuf::View<'msg, Rds> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RdsView<'msg> {
  fn into_view<'shorter>(self) -> RdsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Rds> for RdsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Rds {
    let mut dst = Rds::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Rds> for RdsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Rds {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Rds {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RdsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RdsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RdsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Rds>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RdsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RdsMut<'msg> {
  type Message = Rds;
}

impl ::std::fmt::Debug for RdsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Rds>> for RdsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Rds>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RdsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Rds> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Rds {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // route_config_name: optional string
  pub fn route_config_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_config_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `RdsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RdsMut<'_> {}

// SAFETY:
// - `RdsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RdsMut<'_> {}

impl<'msg> ::protobuf::AsView for RdsMut<'msg> {
  type Proxied = Rds;
  fn as_view(&self) -> ::protobuf::View<'_, Rds> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RdsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Rds>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RdsMut<'msg> {
  type MutProxied = Rds;
  fn as_mut(&mut self) -> RdsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RdsMut<'msg> {
  fn into_mut<'shorter>(self) -> RdsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Rds {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Rds> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RdsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RdsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // route_config_name: optional string
  pub fn route_config_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_config_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl Rds

impl ::std::ops::Drop for Rds {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Rds {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Rds {
  type Proxied = Self;
  fn as_view(&self) -> RdsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Rds {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RdsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Rds {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__network__http_0connection_0manager__v3__Rds_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__network__http_0connection_0manager__v3__Rds_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__network__http_0connection_0manager__v3__Rds_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Rds {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Rds {
  type Msg = Rds;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Rds> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Rds {
  type Msg = Rds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Rds> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RdsMut<'_> {
  type Msg = Rds;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Rds> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RdsMut<'_> {
  type Msg = Rds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Rds> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RdsView<'_> {
  type Msg = Rds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Rds> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RdsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRouteConfigurationsList_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ScopedRouteConfigurationsList {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScopedRouteConfigurationsList>
}

impl ::protobuf::Message for ScopedRouteConfigurationsList {
  type MessageView<'msg> = ScopedRouteConfigurationsListView<'msg>;
  type MessageMut<'msg> = ScopedRouteConfigurationsListMut<'msg>;
}

impl ::std::default::Default for ScopedRouteConfigurationsList {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ScopedRouteConfigurationsList {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ScopedRouteConfigurationsList` is `Sync` because it does not implement interior mutability.
//    Neither does `ScopedRouteConfigurationsListMut`.
unsafe impl ::std::marker::Sync for ScopedRouteConfigurationsList {}

// SAFETY:
// - `ScopedRouteConfigurationsList` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ScopedRouteConfigurationsList {}

impl ::protobuf::Proxied for ScopedRouteConfigurationsList {
  type View<'msg> = ScopedRouteConfigurationsListView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScopedRouteConfigurationsList {}

impl ::protobuf::MutProxied for ScopedRouteConfigurationsList {
  type Mut<'msg> = ScopedRouteConfigurationsListMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScopedRouteConfigurationsListView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRouteConfigurationsList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopedRouteConfigurationsListView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScopedRouteConfigurationsListView<'msg> {
  type Message = ScopedRouteConfigurationsList;
}

impl ::std::fmt::Debug for ScopedRouteConfigurationsListView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ScopedRouteConfigurationsListView<'_> {
  fn default() -> ScopedRouteConfigurationsListView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRouteConfigurationsList>> for ScopedRouteConfigurationsListView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRouteConfigurationsList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopedRouteConfigurationsListView<'msg> {

  pub fn to_owned(&self) -> ScopedRouteConfigurationsList {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // scoped_route_configurations: repeated message envoy.config.route.v3.ScopedRouteConfiguration
  pub fn scoped_route_configurations(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::route::v3::scoped_route::ScopedRouteConfiguration> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::scoped_route::ScopedRouteConfiguration>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ScopedRouteConfigurationsListView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ScopedRouteConfigurationsListView<'_> {}

// SAFETY:
// - `ScopedRouteConfigurationsListView` is `Send` because while its alive a `ScopedRouteConfigurationsListMut` cannot.
// - `ScopedRouteConfigurationsListView` does not use thread-local data.
unsafe impl ::std::marker::Send for ScopedRouteConfigurationsListView<'_> {}

impl<'msg> ::protobuf::AsView for ScopedRouteConfigurationsListView<'msg> {
  type Proxied = ScopedRouteConfigurationsList;
  fn as_view(&self) -> ::protobuf::View<'msg, ScopedRouteConfigurationsList> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopedRouteConfigurationsListView<'msg> {
  fn into_view<'shorter>(self) -> ScopedRouteConfigurationsListView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopedRouteConfigurationsList> for ScopedRouteConfigurationsListView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopedRouteConfigurationsList {
    let mut dst = ScopedRouteConfigurationsList::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopedRouteConfigurationsList> for ScopedRouteConfigurationsListMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopedRouteConfigurationsList {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ScopedRouteConfigurationsList {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopedRouteConfigurationsListView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopedRouteConfigurationsListMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScopedRouteConfigurationsListMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRouteConfigurationsList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopedRouteConfigurationsListMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScopedRouteConfigurationsListMut<'msg> {
  type Message = ScopedRouteConfigurationsList;
}

impl ::std::fmt::Debug for ScopedRouteConfigurationsListMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRouteConfigurationsList>> for ScopedRouteConfigurationsListMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRouteConfigurationsList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopedRouteConfigurationsListMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRouteConfigurationsList> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ScopedRouteConfigurationsList {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // scoped_route_configurations: repeated message envoy.config.route.v3.ScopedRouteConfiguration
  pub fn scoped_route_configurations(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::scoped_route::ScopedRouteConfiguration> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::scoped_route::ScopedRouteConfiguration>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn scoped_route_configurations_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::scoped_route::ScopedRouteConfiguration> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_scoped_route_configurations(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::scoped_route::ScopedRouteConfiguration>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ScopedRouteConfigurationsListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ScopedRouteConfigurationsListMut<'_> {}

// SAFETY:
// - `ScopedRouteConfigurationsListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ScopedRouteConfigurationsListMut<'_> {}

impl<'msg> ::protobuf::AsView for ScopedRouteConfigurationsListMut<'msg> {
  type Proxied = ScopedRouteConfigurationsList;
  fn as_view(&self) -> ::protobuf::View<'_, ScopedRouteConfigurationsList> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopedRouteConfigurationsListMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScopedRouteConfigurationsList>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ScopedRouteConfigurationsListMut<'msg> {
  type MutProxied = ScopedRouteConfigurationsList;
  fn as_mut(&mut self) -> ScopedRouteConfigurationsListMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScopedRouteConfigurationsListMut<'msg> {
  fn into_mut<'shorter>(self) -> ScopedRouteConfigurationsListMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScopedRouteConfigurationsList {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScopedRouteConfigurationsList> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ScopedRouteConfigurationsListView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ScopedRouteConfigurationsListMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // scoped_route_configurations: repeated message envoy.config.route.v3.ScopedRouteConfiguration
  pub fn scoped_route_configurations(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::scoped_route::ScopedRouteConfiguration> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::scoped_route::ScopedRouteConfiguration>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn scoped_route_configurations_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::scoped_route::ScopedRouteConfiguration> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_scoped_route_configurations(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::scoped_route::ScopedRouteConfiguration>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ScopedRouteConfigurationsList

impl ::std::ops::Drop for ScopedRouteConfigurationsList {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ScopedRouteConfigurationsList {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScopedRouteConfigurationsList {
  type Proxied = Self;
  fn as_view(&self) -> ScopedRouteConfigurationsListView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScopedRouteConfigurationsList {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScopedRouteConfigurationsListMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ScopedRouteConfigurationsList {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRouteConfigurationsList_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRouteConfigurationsList_msg_init.0, &[<crate::xds::generated::envoy::config::route::v3::scoped_route::ScopedRouteConfiguration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRouteConfigurationsList_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopedRouteConfigurationsList {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopedRouteConfigurationsList {
  type Msg = ScopedRouteConfigurationsList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRouteConfigurationsList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRouteConfigurationsList {
  type Msg = ScopedRouteConfigurationsList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRouteConfigurationsList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopedRouteConfigurationsListMut<'_> {
  type Msg = ScopedRouteConfigurationsList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRouteConfigurationsList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRouteConfigurationsListMut<'_> {
  type Msg = ScopedRouteConfigurationsList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRouteConfigurationsList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRouteConfigurationsListView<'_> {
  type Msg = ScopedRouteConfigurationsList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRouteConfigurationsList> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopedRouteConfigurationsListMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ScopedRoutes {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScopedRoutes>
}

impl ::protobuf::Message for ScopedRoutes {
  type MessageView<'msg> = ScopedRoutesView<'msg>;
  type MessageMut<'msg> = ScopedRoutesMut<'msg>;
}

impl ::std::default::Default for ScopedRoutes {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ScopedRoutes {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ScopedRoutes` is `Sync` because it does not implement interior mutability.
//    Neither does `ScopedRoutesMut`.
unsafe impl ::std::marker::Sync for ScopedRoutes {}

// SAFETY:
// - `ScopedRoutes` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ScopedRoutes {}

impl ::protobuf::Proxied for ScopedRoutes {
  type View<'msg> = ScopedRoutesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScopedRoutes {}

impl ::protobuf::MutProxied for ScopedRoutes {
  type Mut<'msg> = ScopedRoutesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScopedRoutesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRoutes>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopedRoutesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScopedRoutesView<'msg> {
  type Message = ScopedRoutes;
}

impl ::std::fmt::Debug for ScopedRoutesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ScopedRoutesView<'_> {
  fn default() -> ScopedRoutesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRoutes>> for ScopedRoutesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRoutes>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopedRoutesView<'msg> {

  pub fn to_owned(&self) -> ScopedRoutes {
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

  // scope_key_builder: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder
  pub fn has_scope_key_builder(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn scope_key_builder_opt(self) -> ::std::option::Option<super::scoped_routes::ScopeKeyBuilderView<'msg>> {
    self.has_scope_key_builder().then(|| self.scope_key_builder())
  }
  pub fn scope_key_builder(self) -> super::scoped_routes::ScopeKeyBuilderView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::scoped_routes::ScopeKeyBuilderView::default())
  }

  // rds_config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_rds_config_source(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn rds_config_source_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_rds_config_source().then(|| self.rds_config_source())
  }
  pub fn rds_config_source(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

  // scoped_route_configurations_list: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRouteConfigurationsList
  pub fn has_scoped_route_configurations_list(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn scoped_route_configurations_list_opt(self) -> ::std::option::Option<super::ScopedRouteConfigurationsListView<'msg>> {
    self.has_scoped_route_configurations_list().then(|| self.scoped_route_configurations_list())
  }
  pub fn scoped_route_configurations_list(self) -> super::ScopedRouteConfigurationsListView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScopedRouteConfigurationsListView::default())
  }

  // scoped_rds: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRds
  pub fn has_scoped_rds(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn scoped_rds_opt(self) -> ::std::option::Option<super::ScopedRdsView<'msg>> {
    self.has_scoped_rds().then(|| self.scoped_rds())
  }
  pub fn scoped_rds(self) -> super::ScopedRdsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScopedRdsView::default())
  }

  pub fn config_specifier(self) -> super::scoped_routes::ConfigSpecifierOneof<'msg> {
    match self.config_specifier_case() {
      super::scoped_routes::ConfigSpecifierCase::ScopedRouteConfigurationsList =>
          super::scoped_routes::ConfigSpecifierOneof::ScopedRouteConfigurationsList(self.scoped_route_configurations_list()),
      super::scoped_routes::ConfigSpecifierCase::ScopedRds =>
          super::scoped_routes::ConfigSpecifierOneof::ScopedRds(self.scoped_rds()),
      _ => super::scoped_routes::ConfigSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_specifier_case(self) -> super::scoped_routes::ConfigSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::scoped_routes::ConfigSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ScopedRoutesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ScopedRoutesView<'_> {}

// SAFETY:
// - `ScopedRoutesView` is `Send` because while its alive a `ScopedRoutesMut` cannot.
// - `ScopedRoutesView` does not use thread-local data.
unsafe impl ::std::marker::Send for ScopedRoutesView<'_> {}

impl<'msg> ::protobuf::AsView for ScopedRoutesView<'msg> {
  type Proxied = ScopedRoutes;
  fn as_view(&self) -> ::protobuf::View<'msg, ScopedRoutes> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopedRoutesView<'msg> {
  fn into_view<'shorter>(self) -> ScopedRoutesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopedRoutes> for ScopedRoutesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopedRoutes {
    let mut dst = ScopedRoutes::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopedRoutes> for ScopedRoutesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopedRoutes {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ScopedRoutes {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopedRoutesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopedRoutesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScopedRoutesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRoutes>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopedRoutesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScopedRoutesMut<'msg> {
  type Message = ScopedRoutes;
}

impl ::std::fmt::Debug for ScopedRoutesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRoutes>> for ScopedRoutesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRoutes>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopedRoutesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRoutes> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ScopedRoutes {
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

  // scope_key_builder: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder
  pub fn has_scope_key_builder(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_scope_key_builder(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn scope_key_builder_opt(&self) -> ::std::option::Option<super::scoped_routes::ScopeKeyBuilderView<'_>> {
    self.has_scope_key_builder().then(|| self.scope_key_builder())
  }
  pub fn scope_key_builder(&self) -> super::scoped_routes::ScopeKeyBuilderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::scoped_routes::ScopeKeyBuilderView::default())
  }
  pub fn scope_key_builder_mut(&mut self) -> super::scoped_routes::ScopeKeyBuilderMut<'_> {
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
  pub fn set_scope_key_builder(&mut self,
    val: impl ::protobuf::IntoProxied<super::scoped_routes::ScopeKeyBuilder>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // rds_config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_rds_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_rds_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn rds_config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_rds_config_source().then(|| self.rds_config_source())
  }
  pub fn rds_config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn rds_config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_rds_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // scoped_route_configurations_list: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRouteConfigurationsList
  pub fn has_scoped_route_configurations_list(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_scoped_route_configurations_list(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn scoped_route_configurations_list_opt(&self) -> ::std::option::Option<super::ScopedRouteConfigurationsListView<'_>> {
    self.has_scoped_route_configurations_list().then(|| self.scoped_route_configurations_list())
  }
  pub fn scoped_route_configurations_list(&self) -> super::ScopedRouteConfigurationsListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScopedRouteConfigurationsListView::default())
  }
  pub fn scoped_route_configurations_list_mut(&mut self) -> super::ScopedRouteConfigurationsListMut<'_> {
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
  pub fn set_scoped_route_configurations_list(&mut self,
    val: impl ::protobuf::IntoProxied<super::ScopedRouteConfigurationsList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // scoped_rds: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRds
  pub fn has_scoped_rds(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_scoped_rds(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn scoped_rds_opt(&self) -> ::std::option::Option<super::ScopedRdsView<'_>> {
    self.has_scoped_rds().then(|| self.scoped_rds())
  }
  pub fn scoped_rds(&self) -> super::ScopedRdsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScopedRdsView::default())
  }
  pub fn scoped_rds_mut(&mut self) -> super::ScopedRdsMut<'_> {
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
  pub fn set_scoped_rds(&mut self,
    val: impl ::protobuf::IntoProxied<super::ScopedRds>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn config_specifier(&self) -> super::scoped_routes::ConfigSpecifierOneof<'_> {
    match &self.config_specifier_case() {
      super::scoped_routes::ConfigSpecifierCase::ScopedRouteConfigurationsList =>
          super::scoped_routes::ConfigSpecifierOneof::ScopedRouteConfigurationsList(self.scoped_route_configurations_list()),
      super::scoped_routes::ConfigSpecifierCase::ScopedRds =>
          super::scoped_routes::ConfigSpecifierOneof::ScopedRds(self.scoped_rds()),
      _ => super::scoped_routes::ConfigSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_specifier_case(&self) -> super::scoped_routes::ConfigSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::scoped_routes::ConfigSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ScopedRoutesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ScopedRoutesMut<'_> {}

// SAFETY:
// - `ScopedRoutesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ScopedRoutesMut<'_> {}

impl<'msg> ::protobuf::AsView for ScopedRoutesMut<'msg> {
  type Proxied = ScopedRoutes;
  fn as_view(&self) -> ::protobuf::View<'_, ScopedRoutes> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopedRoutesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScopedRoutes>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ScopedRoutesMut<'msg> {
  type MutProxied = ScopedRoutes;
  fn as_mut(&mut self) -> ScopedRoutesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScopedRoutesMut<'msg> {
  fn into_mut<'shorter>(self) -> ScopedRoutesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScopedRoutes {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScopedRoutes> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ScopedRoutesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ScopedRoutesMut<'_> {
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

  // scope_key_builder: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder
  pub fn has_scope_key_builder(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_scope_key_builder(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn scope_key_builder_opt(&self) -> ::std::option::Option<super::scoped_routes::ScopeKeyBuilderView<'_>> {
    self.has_scope_key_builder().then(|| self.scope_key_builder())
  }
  pub fn scope_key_builder(&self) -> super::scoped_routes::ScopeKeyBuilderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::scoped_routes::ScopeKeyBuilderView::default())
  }
  pub fn scope_key_builder_mut(&mut self) -> super::scoped_routes::ScopeKeyBuilderMut<'_> {
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
  pub fn set_scope_key_builder(&mut self,
    val: impl ::protobuf::IntoProxied<super::scoped_routes::ScopeKeyBuilder>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // rds_config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_rds_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_rds_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn rds_config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_rds_config_source().then(|| self.rds_config_source())
  }
  pub fn rds_config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn rds_config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_rds_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // scoped_route_configurations_list: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRouteConfigurationsList
  pub fn has_scoped_route_configurations_list(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_scoped_route_configurations_list(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn scoped_route_configurations_list_opt(&self) -> ::std::option::Option<super::ScopedRouteConfigurationsListView<'_>> {
    self.has_scoped_route_configurations_list().then(|| self.scoped_route_configurations_list())
  }
  pub fn scoped_route_configurations_list(&self) -> super::ScopedRouteConfigurationsListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScopedRouteConfigurationsListView::default())
  }
  pub fn scoped_route_configurations_list_mut(&mut self) -> super::ScopedRouteConfigurationsListMut<'_> {
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
  pub fn set_scoped_route_configurations_list(&mut self,
    val: impl ::protobuf::IntoProxied<super::ScopedRouteConfigurationsList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // scoped_rds: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRds
  pub fn has_scoped_rds(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_scoped_rds(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn scoped_rds_opt(&self) -> ::std::option::Option<super::ScopedRdsView<'_>> {
    self.has_scoped_rds().then(|| self.scoped_rds())
  }
  pub fn scoped_rds(&self) -> super::ScopedRdsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScopedRdsView::default())
  }
  pub fn scoped_rds_mut(&mut self) -> super::ScopedRdsMut<'_> {
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
  pub fn set_scoped_rds(&mut self,
    val: impl ::protobuf::IntoProxied<super::ScopedRds>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn config_specifier(&self) -> super::scoped_routes::ConfigSpecifierOneof<'_> {
    match &self.config_specifier_case() {
      super::scoped_routes::ConfigSpecifierCase::ScopedRouteConfigurationsList =>
          super::scoped_routes::ConfigSpecifierOneof::ScopedRouteConfigurationsList(self.scoped_route_configurations_list()),
      super::scoped_routes::ConfigSpecifierCase::ScopedRds =>
          super::scoped_routes::ConfigSpecifierOneof::ScopedRds(self.scoped_rds()),
      _ => super::scoped_routes::ConfigSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_specifier_case(&self) -> super::scoped_routes::ConfigSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::scoped_routes::ConfigSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ScopedRoutes

impl ::std::ops::Drop for ScopedRoutes {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ScopedRoutes {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScopedRoutes {
  type Proxied = Self;
  fn as_view(&self) -> ScopedRoutesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScopedRoutes {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScopedRoutesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ScopedRoutes {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3333^%|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes_msg_init.0, &[<super::scoped_routes::ScopeKeyBuilder as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ScopedRouteConfigurationsList as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ScopedRds as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopedRoutes {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopedRoutes {
  type Msg = ScopedRoutes;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRoutes> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRoutes {
  type Msg = ScopedRoutes;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRoutes> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopedRoutesMut<'_> {
  type Msg = ScopedRoutes;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRoutes> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRoutesMut<'_> {
  type Msg = ScopedRoutes;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRoutes> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRoutesView<'_> {
  type Msg = ScopedRoutes;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRoutes> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopedRoutesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod scoped_routes {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ScopeKeyBuilder {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScopeKeyBuilder>
}

impl ::protobuf::Message for ScopeKeyBuilder {
  type MessageView<'msg> = ScopeKeyBuilderView<'msg>;
  type MessageMut<'msg> = ScopeKeyBuilderMut<'msg>;
}

impl ::std::default::Default for ScopeKeyBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ScopeKeyBuilder {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ScopeKeyBuilder` is `Sync` because it does not implement interior mutability.
//    Neither does `ScopeKeyBuilderMut`.
unsafe impl ::std::marker::Sync for ScopeKeyBuilder {}

// SAFETY:
// - `ScopeKeyBuilder` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ScopeKeyBuilder {}

impl ::protobuf::Proxied for ScopeKeyBuilder {
  type View<'msg> = ScopeKeyBuilderView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScopeKeyBuilder {}

impl ::protobuf::MutProxied for ScopeKeyBuilder {
  type Mut<'msg> = ScopeKeyBuilderMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScopeKeyBuilderView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopeKeyBuilder>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopeKeyBuilderView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScopeKeyBuilderView<'msg> {
  type Message = ScopeKeyBuilder;
}

impl ::std::fmt::Debug for ScopeKeyBuilderView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ScopeKeyBuilderView<'_> {
  fn default() -> ScopeKeyBuilderView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ScopeKeyBuilder>> for ScopeKeyBuilderView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopeKeyBuilder>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopeKeyBuilderView<'msg> {

  pub fn to_owned(&self) -> ScopeKeyBuilder {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // fragments: repeated message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder
  pub fn fragments(self) -> ::protobuf::RepeatedView<'msg, super::super::scoped_routes::scope_key_builder::FragmentBuilder> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::scoped_routes::scope_key_builder::FragmentBuilder>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ScopeKeyBuilderView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ScopeKeyBuilderView<'_> {}

// SAFETY:
// - `ScopeKeyBuilderView` is `Send` because while its alive a `ScopeKeyBuilderMut` cannot.
// - `ScopeKeyBuilderView` does not use thread-local data.
unsafe impl ::std::marker::Send for ScopeKeyBuilderView<'_> {}

impl<'msg> ::protobuf::AsView for ScopeKeyBuilderView<'msg> {
  type Proxied = ScopeKeyBuilder;
  fn as_view(&self) -> ::protobuf::View<'msg, ScopeKeyBuilder> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopeKeyBuilderView<'msg> {
  fn into_view<'shorter>(self) -> ScopeKeyBuilderView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopeKeyBuilder> for ScopeKeyBuilderView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopeKeyBuilder {
    let mut dst = ScopeKeyBuilder::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopeKeyBuilder> for ScopeKeyBuilderMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopeKeyBuilder {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ScopeKeyBuilder {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopeKeyBuilderView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopeKeyBuilderMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScopeKeyBuilderMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopeKeyBuilder>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopeKeyBuilderMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScopeKeyBuilderMut<'msg> {
  type Message = ScopeKeyBuilder;
}

impl ::std::fmt::Debug for ScopeKeyBuilderMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ScopeKeyBuilder>> for ScopeKeyBuilderMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopeKeyBuilder>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopeKeyBuilderMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopeKeyBuilder> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ScopeKeyBuilder {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // fragments: repeated message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder
  pub fn fragments(&self) -> ::protobuf::RepeatedView<'_, super::super::scoped_routes::scope_key_builder::FragmentBuilder> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::scoped_routes::scope_key_builder::FragmentBuilder>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn fragments_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::scoped_routes::scope_key_builder::FragmentBuilder> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_fragments(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::scoped_routes::scope_key_builder::FragmentBuilder>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ScopeKeyBuilderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ScopeKeyBuilderMut<'_> {}

// SAFETY:
// - `ScopeKeyBuilderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ScopeKeyBuilderMut<'_> {}

impl<'msg> ::protobuf::AsView for ScopeKeyBuilderMut<'msg> {
  type Proxied = ScopeKeyBuilder;
  fn as_view(&self) -> ::protobuf::View<'_, ScopeKeyBuilder> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopeKeyBuilderMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScopeKeyBuilder>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ScopeKeyBuilderMut<'msg> {
  type MutProxied = ScopeKeyBuilder;
  fn as_mut(&mut self) -> ScopeKeyBuilderMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScopeKeyBuilderMut<'msg> {
  fn into_mut<'shorter>(self) -> ScopeKeyBuilderMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScopeKeyBuilder {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScopeKeyBuilder> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ScopeKeyBuilderView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ScopeKeyBuilderMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // fragments: repeated message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder
  pub fn fragments(&self) -> ::protobuf::RepeatedView<'_, super::super::scoped_routes::scope_key_builder::FragmentBuilder> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::scoped_routes::scope_key_builder::FragmentBuilder>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn fragments_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::scoped_routes::scope_key_builder::FragmentBuilder> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_fragments(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::scoped_routes::scope_key_builder::FragmentBuilder>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ScopeKeyBuilder

impl ::std::ops::Drop for ScopeKeyBuilder {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ScopeKeyBuilder {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScopeKeyBuilder {
  type Proxied = Self;
  fn as_view(&self) -> ScopeKeyBuilderView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScopeKeyBuilder {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScopeKeyBuilderMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ScopeKeyBuilder {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::scoped_routes::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::scoped_routes::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder_msg_init.0, &[<super::super::scoped_routes::scope_key_builder::FragmentBuilder as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::scoped_routes::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopeKeyBuilder {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopeKeyBuilder {
  type Msg = ScopeKeyBuilder;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopeKeyBuilder> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopeKeyBuilder {
  type Msg = ScopeKeyBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopeKeyBuilder> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopeKeyBuilderMut<'_> {
  type Msg = ScopeKeyBuilder;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopeKeyBuilder> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopeKeyBuilderMut<'_> {
  type Msg = ScopeKeyBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopeKeyBuilder> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopeKeyBuilderView<'_> {
  type Msg = ScopeKeyBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopeKeyBuilder> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopeKeyBuilderMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod scope_key_builder {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FragmentBuilder {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FragmentBuilder>
}

impl ::protobuf::Message for FragmentBuilder {
  type MessageView<'msg> = FragmentBuilderView<'msg>;
  type MessageMut<'msg> = FragmentBuilderMut<'msg>;
}

impl ::std::default::Default for FragmentBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FragmentBuilder {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FragmentBuilder` is `Sync` because it does not implement interior mutability.
//    Neither does `FragmentBuilderMut`.
unsafe impl ::std::marker::Sync for FragmentBuilder {}

// SAFETY:
// - `FragmentBuilder` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FragmentBuilder {}

impl ::protobuf::Proxied for FragmentBuilder {
  type View<'msg> = FragmentBuilderView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FragmentBuilder {}

impl ::protobuf::MutProxied for FragmentBuilder {
  type Mut<'msg> = FragmentBuilderMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FragmentBuilderView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FragmentBuilder>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FragmentBuilderView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FragmentBuilderView<'msg> {
  type Message = FragmentBuilder;
}

impl ::std::fmt::Debug for FragmentBuilderView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FragmentBuilderView<'_> {
  fn default() -> FragmentBuilderView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FragmentBuilder>> for FragmentBuilderView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FragmentBuilder>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FragmentBuilderView<'msg> {

  pub fn to_owned(&self) -> FragmentBuilder {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // header_value_extractor: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor
  pub fn has_header_value_extractor(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn header_value_extractor_opt(self) -> ::std::option::Option<super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractorView<'msg>> {
    self.has_header_value_extractor().then(|| self.header_value_extractor())
  }
  pub fn header_value_extractor(self) -> super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractorView::default())
  }

  pub fn r#type(self) -> super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeOneof<'msg> {
    match self.r#type_case() {
      super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeCase::HeaderValueExtractor =>
          super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeOneof::HeaderValueExtractor(self.header_value_extractor()),
      _ => super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(self) -> super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FragmentBuilderView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FragmentBuilderView<'_> {}

// SAFETY:
// - `FragmentBuilderView` is `Send` because while its alive a `FragmentBuilderMut` cannot.
// - `FragmentBuilderView` does not use thread-local data.
unsafe impl ::std::marker::Send for FragmentBuilderView<'_> {}

impl<'msg> ::protobuf::AsView for FragmentBuilderView<'msg> {
  type Proxied = FragmentBuilder;
  fn as_view(&self) -> ::protobuf::View<'msg, FragmentBuilder> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FragmentBuilderView<'msg> {
  fn into_view<'shorter>(self) -> FragmentBuilderView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FragmentBuilder> for FragmentBuilderView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FragmentBuilder {
    let mut dst = FragmentBuilder::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FragmentBuilder> for FragmentBuilderMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FragmentBuilder {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FragmentBuilder {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FragmentBuilderView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FragmentBuilderMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FragmentBuilderMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FragmentBuilder>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FragmentBuilderMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FragmentBuilderMut<'msg> {
  type Message = FragmentBuilder;
}

impl ::std::fmt::Debug for FragmentBuilderMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FragmentBuilder>> for FragmentBuilderMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FragmentBuilder>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FragmentBuilderMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FragmentBuilder> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FragmentBuilder {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // header_value_extractor: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor
  pub fn has_header_value_extractor(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_header_value_extractor(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn header_value_extractor_opt(&self) -> ::std::option::Option<super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractorView<'_>> {
    self.has_header_value_extractor().then(|| self.header_value_extractor())
  }
  pub fn header_value_extractor(&self) -> super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractorView::default())
  }
  pub fn header_value_extractor_mut(&mut self) -> super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractorMut<'_> {
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
  pub fn set_header_value_extractor(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  pub fn r#type(&self) -> super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeOneof<'_> {
    match &self.r#type_case() {
      super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeCase::HeaderValueExtractor =>
          super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeOneof::HeaderValueExtractor(self.header_value_extractor()),
      _ => super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FragmentBuilderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FragmentBuilderMut<'_> {}

// SAFETY:
// - `FragmentBuilderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FragmentBuilderMut<'_> {}

impl<'msg> ::protobuf::AsView for FragmentBuilderMut<'msg> {
  type Proxied = FragmentBuilder;
  fn as_view(&self) -> ::protobuf::View<'_, FragmentBuilder> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FragmentBuilderMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FragmentBuilder>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FragmentBuilderMut<'msg> {
  type MutProxied = FragmentBuilder;
  fn as_mut(&mut self) -> FragmentBuilderMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FragmentBuilderMut<'msg> {
  fn into_mut<'shorter>(self) -> FragmentBuilderMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FragmentBuilder {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FragmentBuilder> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FragmentBuilderView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FragmentBuilderMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // header_value_extractor: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor
  pub fn has_header_value_extractor(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_header_value_extractor(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn header_value_extractor_opt(&self) -> ::std::option::Option<super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractorView<'_>> {
    self.has_header_value_extractor().then(|| self.header_value_extractor())
  }
  pub fn header_value_extractor(&self) -> super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractorView::default())
  }
  pub fn header_value_extractor_mut(&mut self) -> super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractorMut<'_> {
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
  pub fn set_header_value_extractor(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  pub fn r#type(&self) -> super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeOneof<'_> {
    match &self.r#type_case() {
      super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeCase::HeaderValueExtractor =>
          super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeOneof::HeaderValueExtractor(self.header_value_extractor()),
      _ => super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::scoped_routes::scope_key_builder::fragment_builder::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl FragmentBuilder

impl ::std::ops::Drop for FragmentBuilder {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FragmentBuilder {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FragmentBuilder {
  type Proxied = Self;
  fn as_view(&self) -> FragmentBuilderView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FragmentBuilder {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FragmentBuilderMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FragmentBuilder {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::scoped_routes::scope_key_builder::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3^!");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::scoped_routes::scope_key_builder::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder_msg_init.0, &[<super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::scoped_routes::scope_key_builder::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FragmentBuilder {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FragmentBuilder {
  type Msg = FragmentBuilder;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FragmentBuilder> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FragmentBuilder {
  type Msg = FragmentBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FragmentBuilder> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FragmentBuilderMut<'_> {
  type Msg = FragmentBuilder;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FragmentBuilder> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FragmentBuilderMut<'_> {
  type Msg = FragmentBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FragmentBuilder> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FragmentBuilderView<'_> {
  type Msg = FragmentBuilder;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FragmentBuilder> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FragmentBuilderMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod fragment_builder {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder__HeaderValueExtractor_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderValueExtractor {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderValueExtractor>
}

impl ::protobuf::Message for HeaderValueExtractor {
  type MessageView<'msg> = HeaderValueExtractorView<'msg>;
  type MessageMut<'msg> = HeaderValueExtractorMut<'msg>;
}

impl ::std::default::Default for HeaderValueExtractor {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderValueExtractor {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderValueExtractor` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderValueExtractorMut`.
unsafe impl ::std::marker::Sync for HeaderValueExtractor {}

// SAFETY:
// - `HeaderValueExtractor` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderValueExtractor {}

impl ::protobuf::Proxied for HeaderValueExtractor {
  type View<'msg> = HeaderValueExtractorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderValueExtractor {}

impl ::protobuf::MutProxied for HeaderValueExtractor {
  type Mut<'msg> = HeaderValueExtractorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderValueExtractorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderValueExtractor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderValueExtractorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderValueExtractorView<'msg> {
  type Message = HeaderValueExtractor;
}

impl ::std::fmt::Debug for HeaderValueExtractorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderValueExtractorView<'_> {
  fn default() -> HeaderValueExtractorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderValueExtractor>> for HeaderValueExtractorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderValueExtractor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderValueExtractorView<'msg> {

  pub fn to_owned(&self) -> HeaderValueExtractor {
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

  // element_separator: optional string
  pub fn element_separator(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // index: optional uint32
  pub fn has_index(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn index_opt(self) -> ::std::option::Option<u32> {
    self.has_index().then(|| self.index())
  }
  pub fn index(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // element: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor.KvElement
  pub fn has_element(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn element_opt(self) -> ::std::option::Option<super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElementView<'msg>> {
    self.has_element().then(|| self.element())
  }
  pub fn element(self) -> super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElementView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElementView::default())
  }

  pub fn extract_type(self) -> super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof<'msg> {
    match self.extract_type_case() {
      super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase::Index =>
          super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof::Index(self.index()),
      super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase::Element =>
          super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof::Element(self.element()),
      _ => super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn extract_type_case(self) -> super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HeaderValueExtractorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderValueExtractorView<'_> {}

// SAFETY:
// - `HeaderValueExtractorView` is `Send` because while its alive a `HeaderValueExtractorMut` cannot.
// - `HeaderValueExtractorView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderValueExtractorView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderValueExtractorView<'msg> {
  type Proxied = HeaderValueExtractor;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderValueExtractor> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderValueExtractorView<'msg> {
  fn into_view<'shorter>(self) -> HeaderValueExtractorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderValueExtractor> for HeaderValueExtractorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderValueExtractor {
    let mut dst = HeaderValueExtractor::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderValueExtractor> for HeaderValueExtractorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderValueExtractor {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderValueExtractor {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderValueExtractorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderValueExtractorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderValueExtractorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValueExtractor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderValueExtractorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderValueExtractorMut<'msg> {
  type Message = HeaderValueExtractor;
}

impl ::std::fmt::Debug for HeaderValueExtractorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValueExtractor>> for HeaderValueExtractorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValueExtractor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderValueExtractorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValueExtractor> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderValueExtractor {
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

  // element_separator: optional string
  pub fn element_separator(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_element_separator(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // index: optional uint32
  pub fn has_index(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_index(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn index_opt(&self) -> ::std::option::Option<u32> {
    self.has_index().then(|| self.index())
  }
  pub fn index(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_index(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // element: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor.KvElement
  pub fn has_element(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_element(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn element_opt(&self) -> ::std::option::Option<super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElementView<'_>> {
    self.has_element().then(|| self.element())
  }
  pub fn element(&self) -> super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElementView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElementView::default())
  }
  pub fn element_mut(&mut self) -> super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElementMut<'_> {
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
  pub fn set_element(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn extract_type(&self) -> super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof<'_> {
    match &self.extract_type_case() {
      super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase::Index =>
          super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof::Index(self.index()),
      super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase::Element =>
          super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof::Element(self.element()),
      _ => super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn extract_type_case(&self) -> super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HeaderValueExtractorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderValueExtractorMut<'_> {}

// SAFETY:
// - `HeaderValueExtractorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderValueExtractorMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderValueExtractorMut<'msg> {
  type Proxied = HeaderValueExtractor;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderValueExtractor> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderValueExtractorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderValueExtractor>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderValueExtractorMut<'msg> {
  type MutProxied = HeaderValueExtractor;
  fn as_mut(&mut self) -> HeaderValueExtractorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderValueExtractorMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderValueExtractorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderValueExtractor {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderValueExtractor> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderValueExtractorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderValueExtractorMut<'_> {
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

  // element_separator: optional string
  pub fn element_separator(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_element_separator(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // index: optional uint32
  pub fn has_index(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_index(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn index_opt(&self) -> ::std::option::Option<u32> {
    self.has_index().then(|| self.index())
  }
  pub fn index(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_index(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // element: optional message envoy.extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor.KvElement
  pub fn has_element(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_element(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn element_opt(&self) -> ::std::option::Option<super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElementView<'_>> {
    self.has_element().then(|| self.element())
  }
  pub fn element(&self) -> super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElementView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElementView::default())
  }
  pub fn element_mut(&mut self) -> super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElementMut<'_> {
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
  pub fn set_element(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn extract_type(&self) -> super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof<'_> {
    match &self.extract_type_case() {
      super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase::Index =>
          super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof::Index(self.index()),
      super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase::Element =>
          super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof::Element(self.element()),
      _ => super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn extract_type_case(&self) -> super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl HeaderValueExtractor

impl ::std::ops::Drop for HeaderValueExtractor {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderValueExtractor {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderValueExtractor {
  type Proxied = Self;
  fn as_view(&self) -> HeaderValueExtractorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderValueExtractor {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderValueExtractorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderValueExtractor {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder__HeaderValueExtractor_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X)3^$|%");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder__HeaderValueExtractor_msg_init.0, &[<super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder__HeaderValueExtractor_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderValueExtractor {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderValueExtractor {
  type Msg = HeaderValueExtractor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValueExtractor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderValueExtractor {
  type Msg = HeaderValueExtractor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValueExtractor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderValueExtractorMut<'_> {
  type Msg = HeaderValueExtractor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValueExtractor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderValueExtractorMut<'_> {
  type Msg = HeaderValueExtractor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValueExtractor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderValueExtractorView<'_> {
  type Msg = HeaderValueExtractor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValueExtractor> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderValueExtractorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod header_value_extractor {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder__HeaderValueExtractor__KvElement_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct KvElement {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<KvElement>
}

impl ::protobuf::Message for KvElement {
  type MessageView<'msg> = KvElementView<'msg>;
  type MessageMut<'msg> = KvElementMut<'msg>;
}

impl ::std::default::Default for KvElement {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for KvElement {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `KvElement` is `Sync` because it does not implement interior mutability.
//    Neither does `KvElementMut`.
unsafe impl ::std::marker::Sync for KvElement {}

// SAFETY:
// - `KvElement` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for KvElement {}

impl ::protobuf::Proxied for KvElement {
  type View<'msg> = KvElementView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for KvElement {}

impl ::protobuf::MutProxied for KvElement {
  type Mut<'msg> = KvElementMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct KvElementView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KvElement>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KvElementView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for KvElementView<'msg> {
  type Message = KvElement;
}

impl ::std::fmt::Debug for KvElementView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for KvElementView<'_> {
  fn default() -> KvElementView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, KvElement>> for KvElementView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KvElement>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KvElementView<'msg> {

  pub fn to_owned(&self) -> KvElement {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // separator: optional string
  pub fn separator(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // key: optional string
  pub fn key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `KvElementView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for KvElementView<'_> {}

// SAFETY:
// - `KvElementView` is `Send` because while its alive a `KvElementMut` cannot.
// - `KvElementView` does not use thread-local data.
unsafe impl ::std::marker::Send for KvElementView<'_> {}

impl<'msg> ::protobuf::AsView for KvElementView<'msg> {
  type Proxied = KvElement;
  fn as_view(&self) -> ::protobuf::View<'msg, KvElement> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KvElementView<'msg> {
  fn into_view<'shorter>(self) -> KvElementView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<KvElement> for KvElementView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KvElement {
    let mut dst = KvElement::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<KvElement> for KvElementMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KvElement {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for KvElement {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KvElementView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KvElementMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct KvElementMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KvElement>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KvElementMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for KvElementMut<'msg> {
  type Message = KvElement;
}

impl ::std::fmt::Debug for KvElementMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, KvElement>> for KvElementMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KvElement>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KvElementMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, KvElement> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> KvElement {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // separator: optional string
  pub fn separator(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_separator(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `KvElementMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for KvElementMut<'_> {}

// SAFETY:
// - `KvElementMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for KvElementMut<'_> {}

impl<'msg> ::protobuf::AsView for KvElementMut<'msg> {
  type Proxied = KvElement;
  fn as_view(&self) -> ::protobuf::View<'_, KvElement> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KvElementMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, KvElement>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for KvElementMut<'msg> {
  type MutProxied = KvElement;
  fn as_mut(&mut self) -> KvElementMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for KvElementMut<'msg> {
  fn into_mut<'shorter>(self) -> KvElementMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl KvElement {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, KvElement> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> KvElementView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> KvElementMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // separator: optional string
  pub fn separator(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_separator(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl KvElement

impl ::std::ops::Drop for KvElement {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for KvElement {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for KvElement {
  type Proxied = Self;
  fn as_view(&self) -> KvElementView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for KvElement {
  type MutProxied = Self;
  fn as_mut(&mut self) -> KvElementMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for KvElement {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder__HeaderValueExtractor__KvElement_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder__HeaderValueExtractor__KvElement_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRoutes__ScopeKeyBuilder__FragmentBuilder__HeaderValueExtractor__KvElement_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KvElement {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KvElement {
  type Msg = KvElement;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KvElement> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KvElement {
  type Msg = KvElement;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KvElement> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KvElementMut<'_> {
  type Msg = KvElement;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KvElement> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KvElementMut<'_> {
  type Msg = KvElement;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KvElement> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KvElementView<'_> {
  type Msg = KvElement;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KvElement> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KvElementMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ExtractTypeOneof<'msg> {
  Index(u32) = 3,
  Element(::protobuf::View<'msg, super::super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement>) = 4,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ExtractTypeCase {
  Index = 3,
  Element = 4,

  not_set = 0
}

impl ExtractTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ExtractTypeCase> {
    match v {
      0 => Some(ExtractTypeCase::not_set),
      3 => Some(ExtractTypeCase::Index),
      4 => Some(ExtractTypeCase::Element),
      _ => None
    }
  }
}
}  // pub mod header_value_extractor


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TypeOneof<'msg> {
  HeaderValueExtractor(::protobuf::View<'msg, super::super::super::super::scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor>) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TypeCase {
  HeaderValueExtractor = 1,

  not_set = 0
}

impl TypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TypeCase> {
    match v {
      0 => Some(TypeCase::not_set),
      1 => Some(TypeCase::HeaderValueExtractor),
      _ => None
    }
  }
}
}  // pub mod fragment_builder


}  // pub mod scope_key_builder


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigSpecifierOneof<'msg> {
  ScopedRouteConfigurationsList(::protobuf::View<'msg, super::super::ScopedRouteConfigurationsList>) = 4,
  ScopedRds(::protobuf::View<'msg, super::super::ScopedRds>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigSpecifierCase {
  ScopedRouteConfigurationsList = 4,
  ScopedRds = 5,

  not_set = 0
}

impl ConfigSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigSpecifierCase> {
    match v {
      0 => Some(ConfigSpecifierCase::not_set),
      4 => Some(ConfigSpecifierCase::ScopedRouteConfigurationsList),
      5 => Some(ConfigSpecifierCase::ScopedRds),
      _ => None
    }
  }
}
}  // pub mod scoped_routes


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRds_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ScopedRds {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScopedRds>
}

impl ::protobuf::Message for ScopedRds {
  type MessageView<'msg> = ScopedRdsView<'msg>;
  type MessageMut<'msg> = ScopedRdsMut<'msg>;
}

impl ::std::default::Default for ScopedRds {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ScopedRds {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ScopedRds` is `Sync` because it does not implement interior mutability.
//    Neither does `ScopedRdsMut`.
unsafe impl ::std::marker::Sync for ScopedRds {}

// SAFETY:
// - `ScopedRds` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ScopedRds {}

impl ::protobuf::Proxied for ScopedRds {
  type View<'msg> = ScopedRdsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScopedRds {}

impl ::protobuf::MutProxied for ScopedRds {
  type Mut<'msg> = ScopedRdsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScopedRdsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRds>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopedRdsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScopedRdsView<'msg> {
  type Message = ScopedRds;
}

impl ::std::fmt::Debug for ScopedRdsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ScopedRdsView<'_> {
  fn default() -> ScopedRdsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRds>> for ScopedRdsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRds>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopedRdsView<'msg> {

  pub fn to_owned(&self) -> ScopedRds {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // scoped_rds_config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_scoped_rds_config_source(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn scoped_rds_config_source_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_scoped_rds_config_source().then(|| self.scoped_rds_config_source())
  }
  pub fn scoped_rds_config_source(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

  // srds_resources_locator: optional string
  pub fn srds_resources_locator(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `ScopedRdsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ScopedRdsView<'_> {}

// SAFETY:
// - `ScopedRdsView` is `Send` because while its alive a `ScopedRdsMut` cannot.
// - `ScopedRdsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ScopedRdsView<'_> {}

impl<'msg> ::protobuf::AsView for ScopedRdsView<'msg> {
  type Proxied = ScopedRds;
  fn as_view(&self) -> ::protobuf::View<'msg, ScopedRds> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopedRdsView<'msg> {
  fn into_view<'shorter>(self) -> ScopedRdsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopedRds> for ScopedRdsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopedRds {
    let mut dst = ScopedRds::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopedRds> for ScopedRdsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopedRds {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ScopedRds {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopedRdsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopedRdsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScopedRdsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRds>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopedRdsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScopedRdsMut<'msg> {
  type Message = ScopedRds;
}

impl ::std::fmt::Debug for ScopedRdsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRds>> for ScopedRdsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRds>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopedRdsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRds> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ScopedRds {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // scoped_rds_config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_scoped_rds_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_scoped_rds_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn scoped_rds_config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_scoped_rds_config_source().then(|| self.scoped_rds_config_source())
  }
  pub fn scoped_rds_config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn scoped_rds_config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_scoped_rds_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // srds_resources_locator: optional string
  pub fn srds_resources_locator(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_srds_resources_locator(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `ScopedRdsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ScopedRdsMut<'_> {}

// SAFETY:
// - `ScopedRdsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ScopedRdsMut<'_> {}

impl<'msg> ::protobuf::AsView for ScopedRdsMut<'msg> {
  type Proxied = ScopedRds;
  fn as_view(&self) -> ::protobuf::View<'_, ScopedRds> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopedRdsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScopedRds>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ScopedRdsMut<'msg> {
  type MutProxied = ScopedRds;
  fn as_mut(&mut self) -> ScopedRdsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScopedRdsMut<'msg> {
  fn into_mut<'shorter>(self) -> ScopedRdsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScopedRds {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScopedRds> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ScopedRdsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ScopedRdsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // scoped_rds_config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_scoped_rds_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_scoped_rds_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn scoped_rds_config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_scoped_rds_config_source().then(|| self.scoped_rds_config_source())
  }
  pub fn scoped_rds_config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn scoped_rds_config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_scoped_rds_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // srds_resources_locator: optional string
  pub fn srds_resources_locator(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_srds_resources_locator(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl ScopedRds

impl ::std::ops::Drop for ScopedRds {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ScopedRds {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScopedRds {
  type Proxied = Self;
  fn as_view(&self) -> ScopedRdsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScopedRds {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScopedRdsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ScopedRds {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRds_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRds_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__network__http_0connection_0manager__v3__ScopedRds_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopedRds {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopedRds {
  type Msg = ScopedRds;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRds> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRds {
  type Msg = ScopedRds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRds> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopedRdsMut<'_> {
  type Msg = ScopedRds;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRds> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRdsMut<'_> {
  type Msg = ScopedRds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRds> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRdsView<'_> {
  type Msg = ScopedRds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRds> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopedRdsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__HttpFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpFilter>
}

impl ::protobuf::Message for HttpFilter {
  type MessageView<'msg> = HttpFilterView<'msg>;
  type MessageMut<'msg> = HttpFilterMut<'msg>;
}

impl ::std::default::Default for HttpFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpFilterMut`.
unsafe impl ::std::marker::Sync for HttpFilter {}

// SAFETY:
// - `HttpFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpFilter {}

impl ::protobuf::Proxied for HttpFilter {
  type View<'msg> = HttpFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpFilter {}

impl ::protobuf::MutProxied for HttpFilter {
  type Mut<'msg> = HttpFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpFilterView<'msg> {
  type Message = HttpFilter;
}

impl ::std::fmt::Debug for HttpFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpFilterView<'_> {
  fn default() -> HttpFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpFilter>> for HttpFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpFilterView<'msg> {

  pub fn to_owned(&self) -> HttpFilter {
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

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn typed_config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn config_discovery_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'msg>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }

  // is_optional: optional bool
  pub fn is_optional(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

  // disabled: optional bool
  pub fn disabled(self) -> bool {
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

  pub fn config_type(self) -> super::http_filter::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::http_filter::ConfigTypeCase::TypedConfig =>
          super::http_filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      super::http_filter::ConfigTypeCase::ConfigDiscovery =>
          super::http_filter::ConfigTypeOneof::ConfigDiscovery(self.config_discovery()),
      _ => super::http_filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::http_filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::http_filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HttpFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpFilterView<'_> {}

// SAFETY:
// - `HttpFilterView` is `Send` because while its alive a `HttpFilterMut` cannot.
// - `HttpFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpFilterView<'_> {}

impl<'msg> ::protobuf::AsView for HttpFilterView<'msg> {
  type Proxied = HttpFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpFilterView<'msg> {
  fn into_view<'shorter>(self) -> HttpFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpFilter> for HttpFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpFilter {
    let mut dst = HttpFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpFilter> for HttpFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpFilterMut<'msg> {
  type Message = HttpFilter;
}

impl ::std::fmt::Debug for HttpFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpFilter>> for HttpFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpFilter {
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

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_typed_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_config_discovery(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn config_discovery_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }
  pub fn config_discovery_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceMut<'_> {
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
  pub fn set_config_discovery(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // is_optional: optional bool
  pub fn is_optional(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_optional(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // disabled: optional bool
  pub fn disabled(&self) -> bool {
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
  pub fn set_disabled(&mut self, val: bool) {
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

  pub fn config_type(&self) -> super::http_filter::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::http_filter::ConfigTypeCase::TypedConfig =>
          super::http_filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      super::http_filter::ConfigTypeCase::ConfigDiscovery =>
          super::http_filter::ConfigTypeOneof::ConfigDiscovery(self.config_discovery()),
      _ => super::http_filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::http_filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::http_filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HttpFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpFilterMut<'_> {}

// SAFETY:
// - `HttpFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpFilterMut<'msg> {
  type Proxied = HttpFilter;
  fn as_view(&self) -> ::protobuf::View<'_, HttpFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpFilterMut<'msg> {
  type MutProxied = HttpFilter;
  fn as_mut(&mut self) -> HttpFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpFilterMut<'_> {
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

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_typed_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_config_discovery(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn config_discovery_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }
  pub fn config_discovery_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceMut<'_> {
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
  pub fn set_config_discovery(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // is_optional: optional bool
  pub fn is_optional(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_optional(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // disabled: optional bool
  pub fn disabled(&self) -> bool {
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
  pub fn set_disabled(&mut self, val: bool) {
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

  pub fn config_type(&self) -> super::http_filter::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::http_filter::ConfigTypeCase::TypedConfig =>
          super::http_filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      super::http_filter::ConfigTypeCase::ConfigDiscovery =>
          super::http_filter::ConfigTypeOneof::ConfigDiscovery(self.config_discovery()),
      _ => super::http_filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::http_filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::http_filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl HttpFilter

impl ::std::ops::Drop for HttpFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpFilter {
  type Proxied = Self;
  fn as_view(&self) -> HttpFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xb33/P/P^%|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpFilter_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__network__http_0connection_0manager__v3__HttpFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpFilter {
  type Msg = HttpFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpFilter {
  type Msg = HttpFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpFilterMut<'_> {
  type Msg = HttpFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpFilterMut<'_> {
  type Msg = HttpFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpFilterView<'_> {
  type Msg = HttpFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod http_filter {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigTypeOneof<'msg> {
  TypedConfig(::protobuf::View<'msg, ::protobuf_well_known_types::Any>) = 4,
  ConfigDiscovery(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigTypeCase {
  TypedConfig = 4,
  ConfigDiscovery = 5,

  not_set = 0
}

impl ConfigTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigTypeCase> {
    match v {
      0 => Some(ConfigTypeCase::not_set),
      4 => Some(ConfigTypeCase::TypedConfig),
      5 => Some(ConfigTypeCase::ConfigDiscovery),
      _ => None
    }
  }
}
}  // pub mod http_filter


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__RequestIDExtension_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RequestIDExtension {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RequestIDExtension>
}

impl ::protobuf::Message for RequestIDExtension {
  type MessageView<'msg> = RequestIDExtensionView<'msg>;
  type MessageMut<'msg> = RequestIDExtensionMut<'msg>;
}

impl ::std::default::Default for RequestIDExtension {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RequestIDExtension {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RequestIDExtension` is `Sync` because it does not implement interior mutability.
//    Neither does `RequestIDExtensionMut`.
unsafe impl ::std::marker::Sync for RequestIDExtension {}

// SAFETY:
// - `RequestIDExtension` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RequestIDExtension {}

impl ::protobuf::Proxied for RequestIDExtension {
  type View<'msg> = RequestIDExtensionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RequestIDExtension {}

impl ::protobuf::MutProxied for RequestIDExtension {
  type Mut<'msg> = RequestIDExtensionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RequestIDExtensionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RequestIDExtension>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RequestIDExtensionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RequestIDExtensionView<'msg> {
  type Message = RequestIDExtension;
}

impl ::std::fmt::Debug for RequestIDExtensionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RequestIDExtensionView<'_> {
  fn default() -> RequestIDExtensionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RequestIDExtension>> for RequestIDExtensionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RequestIDExtension>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RequestIDExtensionView<'msg> {

  pub fn to_owned(&self) -> RequestIDExtension {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn typed_config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

}

// SAFETY:
// - `RequestIDExtensionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RequestIDExtensionView<'_> {}

// SAFETY:
// - `RequestIDExtensionView` is `Send` because while its alive a `RequestIDExtensionMut` cannot.
// - `RequestIDExtensionView` does not use thread-local data.
unsafe impl ::std::marker::Send for RequestIDExtensionView<'_> {}

impl<'msg> ::protobuf::AsView for RequestIDExtensionView<'msg> {
  type Proxied = RequestIDExtension;
  fn as_view(&self) -> ::protobuf::View<'msg, RequestIDExtension> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RequestIDExtensionView<'msg> {
  fn into_view<'shorter>(self) -> RequestIDExtensionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RequestIDExtension> for RequestIDExtensionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RequestIDExtension {
    let mut dst = RequestIDExtension::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RequestIDExtension> for RequestIDExtensionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RequestIDExtension {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RequestIDExtension {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RequestIDExtensionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RequestIDExtensionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RequestIDExtensionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RequestIDExtension>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RequestIDExtensionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RequestIDExtensionMut<'msg> {
  type Message = RequestIDExtension;
}

impl ::std::fmt::Debug for RequestIDExtensionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RequestIDExtension>> for RequestIDExtensionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RequestIDExtension>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RequestIDExtensionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RequestIDExtension> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RequestIDExtension {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_typed_config(&mut self,
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
// - `RequestIDExtensionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RequestIDExtensionMut<'_> {}

// SAFETY:
// - `RequestIDExtensionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RequestIDExtensionMut<'_> {}

impl<'msg> ::protobuf::AsView for RequestIDExtensionMut<'msg> {
  type Proxied = RequestIDExtension;
  fn as_view(&self) -> ::protobuf::View<'_, RequestIDExtension> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RequestIDExtensionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RequestIDExtension>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RequestIDExtensionMut<'msg> {
  type MutProxied = RequestIDExtension;
  fn as_mut(&mut self) -> RequestIDExtensionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RequestIDExtensionMut<'msg> {
  fn into_mut<'shorter>(self) -> RequestIDExtensionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RequestIDExtension {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RequestIDExtension> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RequestIDExtensionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RequestIDExtensionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_typed_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl RequestIDExtension

impl ::std::ops::Drop for RequestIDExtension {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RequestIDExtension {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RequestIDExtension {
  type Proxied = Self;
  fn as_view(&self) -> RequestIDExtensionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RequestIDExtension {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RequestIDExtensionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RequestIDExtension {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__network__http_0connection_0manager__v3__RequestIDExtension_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__network__http_0connection_0manager__v3__RequestIDExtension_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__network__http_0connection_0manager__v3__RequestIDExtension_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RequestIDExtension {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RequestIDExtension {
  type Msg = RequestIDExtension;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RequestIDExtension> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RequestIDExtension {
  type Msg = RequestIDExtension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RequestIDExtension> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RequestIDExtensionMut<'_> {
  type Msg = RequestIDExtension;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RequestIDExtension> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RequestIDExtensionMut<'_> {
  type Msg = RequestIDExtension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RequestIDExtension> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RequestIDExtensionView<'_> {
  type Msg = RequestIDExtension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RequestIDExtension> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RequestIDExtensionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__network__http_0connection_0manager__v3__EnvoyMobileHttpConnectionManager_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EnvoyMobileHttpConnectionManager {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EnvoyMobileHttpConnectionManager>
}

impl ::protobuf::Message for EnvoyMobileHttpConnectionManager {
  type MessageView<'msg> = EnvoyMobileHttpConnectionManagerView<'msg>;
  type MessageMut<'msg> = EnvoyMobileHttpConnectionManagerMut<'msg>;
}

impl ::std::default::Default for EnvoyMobileHttpConnectionManager {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EnvoyMobileHttpConnectionManager {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EnvoyMobileHttpConnectionManager` is `Sync` because it does not implement interior mutability.
//    Neither does `EnvoyMobileHttpConnectionManagerMut`.
unsafe impl ::std::marker::Sync for EnvoyMobileHttpConnectionManager {}

// SAFETY:
// - `EnvoyMobileHttpConnectionManager` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for EnvoyMobileHttpConnectionManager {}

impl ::protobuf::Proxied for EnvoyMobileHttpConnectionManager {
  type View<'msg> = EnvoyMobileHttpConnectionManagerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EnvoyMobileHttpConnectionManager {}

impl ::protobuf::MutProxied for EnvoyMobileHttpConnectionManager {
  type Mut<'msg> = EnvoyMobileHttpConnectionManagerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EnvoyMobileHttpConnectionManagerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnvoyMobileHttpConnectionManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnvoyMobileHttpConnectionManagerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EnvoyMobileHttpConnectionManagerView<'msg> {
  type Message = EnvoyMobileHttpConnectionManager;
}

impl ::std::fmt::Debug for EnvoyMobileHttpConnectionManagerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EnvoyMobileHttpConnectionManagerView<'_> {
  fn default() -> EnvoyMobileHttpConnectionManagerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EnvoyMobileHttpConnectionManager>> for EnvoyMobileHttpConnectionManagerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnvoyMobileHttpConnectionManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnvoyMobileHttpConnectionManagerView<'msg> {

  pub fn to_owned(&self) -> EnvoyMobileHttpConnectionManager {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager
  pub fn has_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn config_opt(self) -> ::std::option::Option<super::HttpConnectionManagerView<'msg>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(self) -> super::HttpConnectionManagerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpConnectionManagerView::default())
  }

}

// SAFETY:
// - `EnvoyMobileHttpConnectionManagerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EnvoyMobileHttpConnectionManagerView<'_> {}

// SAFETY:
// - `EnvoyMobileHttpConnectionManagerView` is `Send` because while its alive a `EnvoyMobileHttpConnectionManagerMut` cannot.
// - `EnvoyMobileHttpConnectionManagerView` does not use thread-local data.
unsafe impl ::std::marker::Send for EnvoyMobileHttpConnectionManagerView<'_> {}

impl<'msg> ::protobuf::AsView for EnvoyMobileHttpConnectionManagerView<'msg> {
  type Proxied = EnvoyMobileHttpConnectionManager;
  fn as_view(&self) -> ::protobuf::View<'msg, EnvoyMobileHttpConnectionManager> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnvoyMobileHttpConnectionManagerView<'msg> {
  fn into_view<'shorter>(self) -> EnvoyMobileHttpConnectionManagerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EnvoyMobileHttpConnectionManager> for EnvoyMobileHttpConnectionManagerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnvoyMobileHttpConnectionManager {
    let mut dst = EnvoyMobileHttpConnectionManager::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EnvoyMobileHttpConnectionManager> for EnvoyMobileHttpConnectionManagerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnvoyMobileHttpConnectionManager {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for EnvoyMobileHttpConnectionManager {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EnvoyMobileHttpConnectionManagerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EnvoyMobileHttpConnectionManagerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EnvoyMobileHttpConnectionManagerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyMobileHttpConnectionManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnvoyMobileHttpConnectionManagerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EnvoyMobileHttpConnectionManagerMut<'msg> {
  type Message = EnvoyMobileHttpConnectionManager;
}

impl ::std::fmt::Debug for EnvoyMobileHttpConnectionManagerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyMobileHttpConnectionManager>> for EnvoyMobileHttpConnectionManagerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyMobileHttpConnectionManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnvoyMobileHttpConnectionManagerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyMobileHttpConnectionManager> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> EnvoyMobileHttpConnectionManager {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // config: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager
  pub fn has_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_opt(&self) -> ::std::option::Option<super::HttpConnectionManagerView<'_>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(&self) -> super::HttpConnectionManagerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpConnectionManagerView::default())
  }
  pub fn config_mut(&mut self) -> super::HttpConnectionManagerMut<'_> {
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
  pub fn set_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpConnectionManager>) {

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
// - `EnvoyMobileHttpConnectionManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EnvoyMobileHttpConnectionManagerMut<'_> {}

// SAFETY:
// - `EnvoyMobileHttpConnectionManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EnvoyMobileHttpConnectionManagerMut<'_> {}

impl<'msg> ::protobuf::AsView for EnvoyMobileHttpConnectionManagerMut<'msg> {
  type Proxied = EnvoyMobileHttpConnectionManager;
  fn as_view(&self) -> ::protobuf::View<'_, EnvoyMobileHttpConnectionManager> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnvoyMobileHttpConnectionManagerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EnvoyMobileHttpConnectionManager>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EnvoyMobileHttpConnectionManagerMut<'msg> {
  type MutProxied = EnvoyMobileHttpConnectionManager;
  fn as_mut(&mut self) -> EnvoyMobileHttpConnectionManagerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EnvoyMobileHttpConnectionManagerMut<'msg> {
  fn into_mut<'shorter>(self) -> EnvoyMobileHttpConnectionManagerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EnvoyMobileHttpConnectionManager {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EnvoyMobileHttpConnectionManager> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EnvoyMobileHttpConnectionManagerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EnvoyMobileHttpConnectionManagerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // config: optional message envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager
  pub fn has_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_opt(&self) -> ::std::option::Option<super::HttpConnectionManagerView<'_>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(&self) -> super::HttpConnectionManagerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpConnectionManagerView::default())
  }
  pub fn config_mut(&mut self) -> super::HttpConnectionManagerMut<'_> {
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
  pub fn set_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpConnectionManager>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl EnvoyMobileHttpConnectionManager

impl ::std::ops::Drop for EnvoyMobileHttpConnectionManager {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EnvoyMobileHttpConnectionManager {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EnvoyMobileHttpConnectionManager {
  type Proxied = Self;
  fn as_view(&self) -> EnvoyMobileHttpConnectionManagerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EnvoyMobileHttpConnectionManager {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EnvoyMobileHttpConnectionManagerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EnvoyMobileHttpConnectionManager {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__network__http_0connection_0manager__v3__EnvoyMobileHttpConnectionManager_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__network__http_0connection_0manager__v3__EnvoyMobileHttpConnectionManager_msg_init.0, &[<super::HttpConnectionManager as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__network__http_0connection_0manager__v3__EnvoyMobileHttpConnectionManager_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnvoyMobileHttpConnectionManager {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnvoyMobileHttpConnectionManager {
  type Msg = EnvoyMobileHttpConnectionManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyMobileHttpConnectionManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnvoyMobileHttpConnectionManager {
  type Msg = EnvoyMobileHttpConnectionManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyMobileHttpConnectionManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnvoyMobileHttpConnectionManagerMut<'_> {
  type Msg = EnvoyMobileHttpConnectionManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyMobileHttpConnectionManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnvoyMobileHttpConnectionManagerMut<'_> {
  type Msg = EnvoyMobileHttpConnectionManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyMobileHttpConnectionManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnvoyMobileHttpConnectionManagerView<'_> {
  type Msg = EnvoyMobileHttpConnectionManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyMobileHttpConnectionManager> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnvoyMobileHttpConnectionManagerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



