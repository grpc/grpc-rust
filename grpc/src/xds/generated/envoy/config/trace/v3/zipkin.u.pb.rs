const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__trace__v3__ZipkinConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ZipkinConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ZipkinConfig>
}

impl ::protobuf::Message for ZipkinConfig {
  type MessageView<'msg> = ZipkinConfigView<'msg>;
  type MessageMut<'msg> = ZipkinConfigMut<'msg>;
}

impl ::std::default::Default for ZipkinConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ZipkinConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ZipkinConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ZipkinConfigMut`.
unsafe impl ::std::marker::Sync for ZipkinConfig {}

// SAFETY:
// - `ZipkinConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ZipkinConfig {}

impl ::protobuf::Proxied for ZipkinConfig {
  type View<'msg> = ZipkinConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ZipkinConfig {}

impl ::protobuf::MutProxied for ZipkinConfig {
  type Mut<'msg> = ZipkinConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ZipkinConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ZipkinConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ZipkinConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ZipkinConfigView<'msg> {
  type Message = ZipkinConfig;
}

impl ::std::fmt::Debug for ZipkinConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ZipkinConfigView<'_> {
  fn default() -> ZipkinConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ZipkinConfig>> for ZipkinConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ZipkinConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ZipkinConfigView<'msg> {

  pub fn to_owned(&self) -> ZipkinConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // collector_cluster: optional string
  pub fn collector_cluster(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // collector_endpoint: optional string
  pub fn collector_endpoint(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // trace_id_128bit: optional bool
  pub fn trace_id_128bit(self) -> bool {
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

  // shared_span_context: optional message google.protobuf.BoolValue
  pub fn has_shared_span_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn shared_span_context_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_shared_span_context().then(|| self.shared_span_context())
  }
  pub fn shared_span_context(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // collector_endpoint_version: optional enum envoy.config.trace.v3.ZipkinConfig.CollectorEndpointVersion
  pub fn collector_endpoint_version(self) -> super::zipkin_config::CollectorEndpointVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::zipkin_config::CollectorEndpointVersion::DeprecatedAndUnavailableDoNotUse).into()
      ).try_into().unwrap()
    }
  }

  // collector_hostname: optional string
  pub fn collector_hostname(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // split_spans_for_request: optional bool
  pub fn split_spans_for_request(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }

  // trace_context_option: optional enum envoy.config.trace.v3.ZipkinConfig.TraceContextOption
  pub fn trace_context_option(self) -> super::zipkin_config::TraceContextOption {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::zipkin_config::TraceContextOption::UseB3).into()
      ).try_into().unwrap()
    }
  }

  // collector_service: optional message envoy.config.core.v3.HttpService
  pub fn has_collector_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn collector_service_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'msg>> {
    self.has_collector_service().then(|| self.collector_service())
  }
  pub fn collector_service(self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView::default())
  }

}

// SAFETY:
// - `ZipkinConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ZipkinConfigView<'_> {}

// SAFETY:
// - `ZipkinConfigView` is `Send` because while its alive a `ZipkinConfigMut` cannot.
// - `ZipkinConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ZipkinConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ZipkinConfigView<'msg> {
  type Proxied = ZipkinConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ZipkinConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ZipkinConfigView<'msg> {
  fn into_view<'shorter>(self) -> ZipkinConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ZipkinConfig> for ZipkinConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ZipkinConfig {
    let mut dst = ZipkinConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ZipkinConfig> for ZipkinConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ZipkinConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ZipkinConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ZipkinConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ZipkinConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ZipkinConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ZipkinConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ZipkinConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ZipkinConfigMut<'msg> {
  type Message = ZipkinConfig;
}

impl ::std::fmt::Debug for ZipkinConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ZipkinConfig>> for ZipkinConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ZipkinConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ZipkinConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ZipkinConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ZipkinConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // collector_cluster: optional string
  pub fn collector_cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // collector_endpoint: optional string
  pub fn collector_endpoint(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_endpoint(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // trace_id_128bit: optional bool
  pub fn trace_id_128bit(&self) -> bool {
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
  pub fn set_trace_id_128bit(&mut self, val: bool) {
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

  // shared_span_context: optional message google.protobuf.BoolValue
  pub fn has_shared_span_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_shared_span_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn shared_span_context_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_shared_span_context().then(|| self.shared_span_context())
  }
  pub fn shared_span_context(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn shared_span_context_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_shared_span_context(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // collector_endpoint_version: optional enum envoy.config.trace.v3.ZipkinConfig.CollectorEndpointVersion
  pub fn collector_endpoint_version(&self) -> super::zipkin_config::CollectorEndpointVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::zipkin_config::CollectorEndpointVersion::DeprecatedAndUnavailableDoNotUse).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_collector_endpoint_version(&mut self, val: super::zipkin_config::CollectorEndpointVersion) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // collector_hostname: optional string
  pub fn collector_hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // split_spans_for_request: optional bool
  pub fn split_spans_for_request(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_split_spans_for_request(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // trace_context_option: optional enum envoy.config.trace.v3.ZipkinConfig.TraceContextOption
  pub fn trace_context_option(&self) -> super::zipkin_config::TraceContextOption {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::zipkin_config::TraceContextOption::UseB3).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_trace_context_option(&mut self, val: super::zipkin_config::TraceContextOption) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        7, val.into()
      )
    }
  }

  // collector_service: optional message envoy.config.core.v3.HttpService
  pub fn has_collector_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_collector_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn collector_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_>> {
    self.has_collector_service().then(|| self.collector_service())
  }
  pub fn collector_service(&self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView::default())
  }
  pub fn collector_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceMut<'_> {
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
  pub fn set_collector_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_service::HttpService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

}

// SAFETY:
// - `ZipkinConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ZipkinConfigMut<'_> {}

// SAFETY:
// - `ZipkinConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ZipkinConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ZipkinConfigMut<'msg> {
  type Proxied = ZipkinConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ZipkinConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ZipkinConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ZipkinConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ZipkinConfigMut<'msg> {
  type MutProxied = ZipkinConfig;
  fn as_mut(&mut self) -> ZipkinConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ZipkinConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ZipkinConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ZipkinConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ZipkinConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ZipkinConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ZipkinConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // collector_cluster: optional string
  pub fn collector_cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // collector_endpoint: optional string
  pub fn collector_endpoint(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_endpoint(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // trace_id_128bit: optional bool
  pub fn trace_id_128bit(&self) -> bool {
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
  pub fn set_trace_id_128bit(&mut self, val: bool) {
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

  // shared_span_context: optional message google.protobuf.BoolValue
  pub fn has_shared_span_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_shared_span_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn shared_span_context_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_shared_span_context().then(|| self.shared_span_context())
  }
  pub fn shared_span_context(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn shared_span_context_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_shared_span_context(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // collector_endpoint_version: optional enum envoy.config.trace.v3.ZipkinConfig.CollectorEndpointVersion
  pub fn collector_endpoint_version(&self) -> super::zipkin_config::CollectorEndpointVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::zipkin_config::CollectorEndpointVersion::DeprecatedAndUnavailableDoNotUse).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_collector_endpoint_version(&mut self, val: super::zipkin_config::CollectorEndpointVersion) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // collector_hostname: optional string
  pub fn collector_hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // split_spans_for_request: optional bool
  pub fn split_spans_for_request(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_split_spans_for_request(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // trace_context_option: optional enum envoy.config.trace.v3.ZipkinConfig.TraceContextOption
  pub fn trace_context_option(&self) -> super::zipkin_config::TraceContextOption {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::zipkin_config::TraceContextOption::UseB3).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_trace_context_option(&mut self, val: super::zipkin_config::TraceContextOption) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        7, val.into()
      )
    }
  }

  // collector_service: optional message envoy.config.core.v3.HttpService
  pub fn has_collector_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_collector_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn collector_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_>> {
    self.has_collector_service().then(|| self.collector_service())
  }
  pub fn collector_service(&self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView::default())
  }
  pub fn collector_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceMut<'_> {
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
  pub fn set_collector_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_service::HttpService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

}  // impl ZipkinConfig

impl ::std::ops::Drop for ZipkinConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ZipkinConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ZipkinConfig {
  type Proxied = Self;
  fn as_view(&self) -> ZipkinConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ZipkinConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ZipkinConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ZipkinConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__trace__v3__ZipkinConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X/P3.P1X/P.P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__trace__v3__ZipkinConfig_msg_init.0, &[<::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::http_service::HttpService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__trace__v3__ZipkinConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ZipkinConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ZipkinConfig {
  type Msg = ZipkinConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZipkinConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ZipkinConfig {
  type Msg = ZipkinConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZipkinConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ZipkinConfigMut<'_> {
  type Msg = ZipkinConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZipkinConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ZipkinConfigMut<'_> {
  type Msg = ZipkinConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZipkinConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ZipkinConfigView<'_> {
  type Msg = ZipkinConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZipkinConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ZipkinConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod zipkin_config {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TraceContextOption(i32);

#[allow(non_upper_case_globals)]
impl TraceContextOption {
  pub const UseB3: TraceContextOption = TraceContextOption(0);
  pub const UseB3WithW3CPropagation: TraceContextOption = TraceContextOption(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "UseB3",
      1 => "UseB3WithW3CPropagation",
      _ => return None
    })
  }
}

impl ::std::convert::From<TraceContextOption> for i32 {
  fn from(val: TraceContextOption) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for TraceContextOption {
  fn from(val: i32) -> TraceContextOption {
    Self(val)
  }
}

impl ::std::default::Default for TraceContextOption {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for TraceContextOption {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "TraceContextOption::{}", constant_name)
    } else {
      write!(f, "TraceContextOption::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for TraceContextOption {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for TraceContextOption {}

impl ::protobuf::Proxied for TraceContextOption {
  type View<'a> = TraceContextOption;
}

impl ::protobuf::AsView for TraceContextOption {
  type Proxied = TraceContextOption;

  fn as_view(&self) -> TraceContextOption {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TraceContextOption {
  fn into_view<'shorter>(self) -> TraceContextOption where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for TraceContextOption {
  const NAME: &'static str = "TraceContextOption";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for TraceContextOption {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CollectorEndpointVersion(i32);

#[allow(non_upper_case_globals)]
impl CollectorEndpointVersion {
  pub const DeprecatedAndUnavailableDoNotUse: CollectorEndpointVersion = CollectorEndpointVersion(0);
  pub const HttpJson: CollectorEndpointVersion = CollectorEndpointVersion(1);
  pub const HttpProto: CollectorEndpointVersion = CollectorEndpointVersion(2);
  pub const Grpc: CollectorEndpointVersion = CollectorEndpointVersion(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "DeprecatedAndUnavailableDoNotUse",
      1 => "HttpJson",
      2 => "HttpProto",
      3 => "Grpc",
      _ => return None
    })
  }
}

impl ::std::convert::From<CollectorEndpointVersion> for i32 {
  fn from(val: CollectorEndpointVersion) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for CollectorEndpointVersion {
  fn from(val: i32) -> CollectorEndpointVersion {
    Self(val)
  }
}

impl ::std::default::Default for CollectorEndpointVersion {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for CollectorEndpointVersion {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "CollectorEndpointVersion::{}", constant_name)
    } else {
      write!(f, "CollectorEndpointVersion::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for CollectorEndpointVersion {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for CollectorEndpointVersion {}

impl ::protobuf::Proxied for CollectorEndpointVersion {
  type View<'a> = CollectorEndpointVersion;
}

impl ::protobuf::AsView for CollectorEndpointVersion {
  type Proxied = CollectorEndpointVersion;

  fn as_view(&self) -> CollectorEndpointVersion {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CollectorEndpointVersion {
  fn into_view<'shorter>(self) -> CollectorEndpointVersion where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for CollectorEndpointVersion {
  const NAME: &'static str = "CollectorEndpointVersion";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for CollectorEndpointVersion {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod zipkin_config


