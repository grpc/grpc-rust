const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0proc__v3__ExternalProcessor_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExternalProcessor {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExternalProcessor>
}

impl ::protobuf::Message for ExternalProcessor {
  type MessageView<'msg> = ExternalProcessorView<'msg>;
  type MessageMut<'msg> = ExternalProcessorMut<'msg>;
}

impl ::std::default::Default for ExternalProcessor {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExternalProcessor {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExternalProcessor` is `Sync` because it does not implement interior mutability.
//    Neither does `ExternalProcessorMut`.
unsafe impl ::std::marker::Sync for ExternalProcessor {}

// SAFETY:
// - `ExternalProcessor` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExternalProcessor {}

impl ::protobuf::Proxied for ExternalProcessor {
  type View<'msg> = ExternalProcessorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExternalProcessor {}

impl ::protobuf::MutProxied for ExternalProcessor {
  type Mut<'msg> = ExternalProcessorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExternalProcessorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExternalProcessor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExternalProcessorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExternalProcessorView<'msg> {
  type Message = ExternalProcessor;
}

impl ::std::fmt::Debug for ExternalProcessorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExternalProcessorView<'_> {
  fn default() -> ExternalProcessorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExternalProcessor>> for ExternalProcessorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExternalProcessor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExternalProcessorView<'msg> {

  pub fn to_owned(&self) -> ExternalProcessor {
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

  // http_service: optional message envoy.extensions.filters.http.ext_proc.v3.ExtProcHttpService
  pub fn has_http_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn http_service_opt(self) -> ::std::option::Option<super::ExtProcHttpServiceView<'msg>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(self) -> super::ExtProcHttpServiceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExtProcHttpServiceView::default())
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

  // processing_mode: optional message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn has_processing_mode(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn processing_mode_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'msg>> {
    self.has_processing_mode().then(|| self.processing_mode())
  }
  pub fn processing_mode(self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView::default())
  }

  // request_attributes: repeated string
  pub fn request_attributes(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // response_attributes: repeated string
  pub fn response_attributes(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // message_timeout: optional message google.protobuf.Duration
  pub fn has_message_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn message_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_message_timeout().then(|| self.message_timeout())
  }
  pub fn message_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // stat_prefix: optional string
  pub fn stat_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // mutation_rules: optional message envoy.config.common.mutation_rules.v3.HeaderMutationRules
  pub fn has_mutation_rules(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn mutation_rules_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'msg>> {
    self.has_mutation_rules().then(|| self.mutation_rules())
  }
  pub fn mutation_rules(self) -> crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView::default())
  }

  // max_message_timeout: optional message google.protobuf.Duration
  pub fn has_max_message_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn max_message_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_max_message_timeout().then(|| self.max_message_timeout())
  }
  pub fn max_message_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // forward_rules: optional message envoy.extensions.filters.http.ext_proc.v3.HeaderForwardingRules
  pub fn has_forward_rules(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn forward_rules_opt(self) -> ::std::option::Option<super::HeaderForwardingRulesView<'msg>> {
    self.has_forward_rules().then(|| self.forward_rules())
  }
  pub fn forward_rules(self) -> super::HeaderForwardingRulesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderForwardingRulesView::default())
  }

  // filter_metadata: optional message google.protobuf.Struct
  pub fn has_filter_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn filter_metadata_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_filter_metadata().then(|| self.filter_metadata())
  }
  pub fn filter_metadata(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // allow_mode_override: optional bool
  pub fn allow_mode_override(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        12, (false).into()
      ).try_into().unwrap()
    }
  }

  // disable_immediate_response: optional bool
  pub fn disable_immediate_response(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }

  // metadata_options: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions
  pub fn has_metadata_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn metadata_options_opt(self) -> ::std::option::Option<super::MetadataOptionsView<'msg>> {
    self.has_metadata_options().then(|| self.metadata_options())
  }
  pub fn metadata_options(self) -> super::MetadataOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetadataOptionsView::default())
  }

  // observability_mode: optional bool
  pub fn observability_mode(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }

  // disable_clear_route_cache: optional bool
  pub fn disable_clear_route_cache(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }

  // route_cache_action: optional enum envoy.extensions.filters.http.ext_proc.v3.ExternalProcessor.RouteCacheAction
  pub fn route_cache_action(self) -> super::external_processor::RouteCacheAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        16, (super::external_processor::RouteCacheAction::Default).into()
      ).try_into().unwrap()
    }
  }

  // deferred_close_timeout: optional message google.protobuf.Duration
  pub fn has_deferred_close_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn deferred_close_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_deferred_close_timeout().then(|| self.deferred_close_timeout())
  }
  pub fn deferred_close_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // send_body_without_waiting_for_header_response: optional bool
  pub fn send_body_without_waiting_for_header_response(self) -> bool {
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

  // allowed_override_modes: repeated message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn allowed_override_modes(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // processing_request_modifier: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_processing_request_modifier(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn processing_request_modifier_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_processing_request_modifier().then(|| self.processing_request_modifier())
  }
  pub fn processing_request_modifier(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // on_processing_response: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_on_processing_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn on_processing_response_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_on_processing_response().then(|| self.on_processing_response())
  }
  pub fn on_processing_response(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // status_on_error: optional message envoy.type.v3.HttpStatus
  pub fn has_status_on_error(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn status_on_error_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'msg>> {
    self.has_status_on_error().then(|| self.status_on_error())
  }
  pub fn status_on_error(self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }

}

// SAFETY:
// - `ExternalProcessorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExternalProcessorView<'_> {}

// SAFETY:
// - `ExternalProcessorView` is `Send` because while its alive a `ExternalProcessorMut` cannot.
// - `ExternalProcessorView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExternalProcessorView<'_> {}

impl<'msg> ::protobuf::AsView for ExternalProcessorView<'msg> {
  type Proxied = ExternalProcessor;
  fn as_view(&self) -> ::protobuf::View<'msg, ExternalProcessor> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExternalProcessorView<'msg> {
  fn into_view<'shorter>(self) -> ExternalProcessorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExternalProcessor> for ExternalProcessorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExternalProcessor {
    let mut dst = ExternalProcessor::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExternalProcessor> for ExternalProcessorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExternalProcessor {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExternalProcessor {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExternalProcessorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExternalProcessorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExternalProcessorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExternalProcessor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExternalProcessorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExternalProcessorMut<'msg> {
  type Message = ExternalProcessor;
}

impl ::std::fmt::Debug for ExternalProcessorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExternalProcessor>> for ExternalProcessorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExternalProcessor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExternalProcessorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExternalProcessor> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExternalProcessor {
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

  // http_service: optional message envoy.extensions.filters.http.ext_proc.v3.ExtProcHttpService
  pub fn has_http_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_http_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn http_service_opt(&self) -> ::std::option::Option<super::ExtProcHttpServiceView<'_>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(&self) -> super::ExtProcHttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExtProcHttpServiceView::default())
  }
  pub fn http_service_mut(&mut self) -> super::ExtProcHttpServiceMut<'_> {
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
  pub fn set_http_service(&mut self,
    val: impl ::protobuf::IntoProxied<super::ExtProcHttpService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
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

  // processing_mode: optional message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn has_processing_mode(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_processing_mode(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn processing_mode_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_>> {
    self.has_processing_mode().then(|| self.processing_mode())
  }
  pub fn processing_mode(&self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView::default())
  }
  pub fn processing_mode_mut(&mut self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeMut<'_> {
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
  pub fn set_processing_mode(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // request_attributes: repeated string
  pub fn request_attributes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_attributes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_request_attributes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // response_attributes: repeated string
  pub fn response_attributes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn response_attributes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_response_attributes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // message_timeout: optional message google.protobuf.Duration
  pub fn has_message_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_message_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn message_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_message_timeout().then(|| self.message_timeout())
  }
  pub fn message_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn message_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_message_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // mutation_rules: optional message envoy.config.common.mutation_rules.v3.HeaderMutationRules
  pub fn has_mutation_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_mutation_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn mutation_rules_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'_>> {
    self.has_mutation_rules().then(|| self.mutation_rules())
  }
  pub fn mutation_rules(&self) -> crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView::default())
  }
  pub fn mutation_rules_mut(&mut self) -> crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesMut<'_> {
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
  pub fn set_mutation_rules(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // max_message_timeout: optional message google.protobuf.Duration
  pub fn has_max_message_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_max_message_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn max_message_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_message_timeout().then(|| self.max_message_timeout())
  }
  pub fn max_message_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_message_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_message_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // forward_rules: optional message envoy.extensions.filters.http.ext_proc.v3.HeaderForwardingRules
  pub fn has_forward_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_forward_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn forward_rules_opt(&self) -> ::std::option::Option<super::HeaderForwardingRulesView<'_>> {
    self.has_forward_rules().then(|| self.forward_rules())
  }
  pub fn forward_rules(&self) -> super::HeaderForwardingRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderForwardingRulesView::default())
  }
  pub fn forward_rules_mut(&mut self) -> super::HeaderForwardingRulesMut<'_> {
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
  pub fn set_forward_rules(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderForwardingRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // filter_metadata: optional message google.protobuf.Struct
  pub fn has_filter_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_filter_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn filter_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_filter_metadata().then(|| self.filter_metadata())
  }
  pub fn filter_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn filter_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
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
        11,
        val
      );
    }
  }

  // allow_mode_override: optional bool
  pub fn allow_mode_override(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        12, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_allow_mode_override(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        12, val.into()
      )
    }
  }

  // disable_immediate_response: optional bool
  pub fn disable_immediate_response(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_disable_immediate_response(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  // metadata_options: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions
  pub fn has_metadata_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_metadata_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn metadata_options_opt(&self) -> ::std::option::Option<super::MetadataOptionsView<'_>> {
    self.has_metadata_options().then(|| self.metadata_options())
  }
  pub fn metadata_options(&self) -> super::MetadataOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetadataOptionsView::default())
  }
  pub fn metadata_options_mut(&mut self) -> super::MetadataOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::MetadataOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // observability_mode: optional bool
  pub fn observability_mode(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_observability_mode(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        15, val.into()
      )
    }
  }

  // disable_clear_route_cache: optional bool
  pub fn disable_clear_route_cache(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_disable_clear_route_cache(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        9, val.into()
      )
    }
  }

  // route_cache_action: optional enum envoy.extensions.filters.http.ext_proc.v3.ExternalProcessor.RouteCacheAction
  pub fn route_cache_action(&self) -> super::external_processor::RouteCacheAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        16, (super::external_processor::RouteCacheAction::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_route_cache_action(&mut self, val: super::external_processor::RouteCacheAction) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        16, val.into()
      )
    }
  }

  // deferred_close_timeout: optional message google.protobuf.Duration
  pub fn has_deferred_close_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_deferred_close_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn deferred_close_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_deferred_close_timeout().then(|| self.deferred_close_timeout())
  }
  pub fn deferred_close_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn deferred_close_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         17, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_deferred_close_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

  // send_body_without_waiting_for_header_response: optional bool
  pub fn send_body_without_waiting_for_header_response(&self) -> bool {
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
  pub fn set_send_body_without_waiting_for_header_response(&mut self, val: bool) {
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

  // allowed_override_modes: repeated message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn allowed_override_modes(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn allowed_override_modes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode> {
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
  pub fn set_allowed_override_modes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        src);
    }
  }

  // processing_request_modifier: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_processing_request_modifier(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_processing_request_modifier(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn processing_request_modifier_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_processing_request_modifier().then(|| self.processing_request_modifier())
  }
  pub fn processing_request_modifier(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn processing_request_modifier_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_processing_request_modifier(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val
      );
    }
  }

  // on_processing_response: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_on_processing_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_on_processing_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn on_processing_response_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_on_processing_response().then(|| self.on_processing_response())
  }
  pub fn on_processing_response(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn on_processing_response_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         21, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_on_processing_response(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val
      );
    }
  }

  // status_on_error: optional message envoy.type.v3.HttpStatus
  pub fn has_status_on_error(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_status_on_error(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn status_on_error_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_>> {
    self.has_status_on_error().then(|| self.status_on_error())
  }
  pub fn status_on_error(&self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }
  pub fn status_on_error_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusMut<'_> {
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
  pub fn set_status_on_error(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

}

// SAFETY:
// - `ExternalProcessorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExternalProcessorMut<'_> {}

// SAFETY:
// - `ExternalProcessorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExternalProcessorMut<'_> {}

impl<'msg> ::protobuf::AsView for ExternalProcessorMut<'msg> {
  type Proxied = ExternalProcessor;
  fn as_view(&self) -> ::protobuf::View<'_, ExternalProcessor> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExternalProcessorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExternalProcessor>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExternalProcessorMut<'msg> {
  type MutProxied = ExternalProcessor;
  fn as_mut(&mut self) -> ExternalProcessorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExternalProcessorMut<'msg> {
  fn into_mut<'shorter>(self) -> ExternalProcessorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExternalProcessor {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExternalProcessor> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExternalProcessorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExternalProcessorMut<'_> {
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

  // http_service: optional message envoy.extensions.filters.http.ext_proc.v3.ExtProcHttpService
  pub fn has_http_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_http_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn http_service_opt(&self) -> ::std::option::Option<super::ExtProcHttpServiceView<'_>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(&self) -> super::ExtProcHttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExtProcHttpServiceView::default())
  }
  pub fn http_service_mut(&mut self) -> super::ExtProcHttpServiceMut<'_> {
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
  pub fn set_http_service(&mut self,
    val: impl ::protobuf::IntoProxied<super::ExtProcHttpService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
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

  // processing_mode: optional message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn has_processing_mode(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_processing_mode(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn processing_mode_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_>> {
    self.has_processing_mode().then(|| self.processing_mode())
  }
  pub fn processing_mode(&self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView::default())
  }
  pub fn processing_mode_mut(&mut self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeMut<'_> {
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
  pub fn set_processing_mode(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // request_attributes: repeated string
  pub fn request_attributes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_attributes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_request_attributes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // response_attributes: repeated string
  pub fn response_attributes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn response_attributes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_response_attributes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // message_timeout: optional message google.protobuf.Duration
  pub fn has_message_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_message_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn message_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_message_timeout().then(|| self.message_timeout())
  }
  pub fn message_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn message_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_message_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // mutation_rules: optional message envoy.config.common.mutation_rules.v3.HeaderMutationRules
  pub fn has_mutation_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_mutation_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn mutation_rules_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'_>> {
    self.has_mutation_rules().then(|| self.mutation_rules())
  }
  pub fn mutation_rules(&self) -> crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesView::default())
  }
  pub fn mutation_rules_mut(&mut self) -> crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRulesMut<'_> {
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
  pub fn set_mutation_rules(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // max_message_timeout: optional message google.protobuf.Duration
  pub fn has_max_message_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_max_message_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn max_message_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_message_timeout().then(|| self.max_message_timeout())
  }
  pub fn max_message_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_message_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_message_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // forward_rules: optional message envoy.extensions.filters.http.ext_proc.v3.HeaderForwardingRules
  pub fn has_forward_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_forward_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn forward_rules_opt(&self) -> ::std::option::Option<super::HeaderForwardingRulesView<'_>> {
    self.has_forward_rules().then(|| self.forward_rules())
  }
  pub fn forward_rules(&self) -> super::HeaderForwardingRulesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderForwardingRulesView::default())
  }
  pub fn forward_rules_mut(&mut self) -> super::HeaderForwardingRulesMut<'_> {
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
  pub fn set_forward_rules(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderForwardingRules>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // filter_metadata: optional message google.protobuf.Struct
  pub fn has_filter_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_filter_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn filter_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_filter_metadata().then(|| self.filter_metadata())
  }
  pub fn filter_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn filter_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
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
        11,
        val
      );
    }
  }

  // allow_mode_override: optional bool
  pub fn allow_mode_override(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        12, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_allow_mode_override(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        12, val.into()
      )
    }
  }

  // disable_immediate_response: optional bool
  pub fn disable_immediate_response(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_disable_immediate_response(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  // metadata_options: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions
  pub fn has_metadata_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_metadata_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn metadata_options_opt(&self) -> ::std::option::Option<super::MetadataOptionsView<'_>> {
    self.has_metadata_options().then(|| self.metadata_options())
  }
  pub fn metadata_options(&self) -> super::MetadataOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetadataOptionsView::default())
  }
  pub fn metadata_options_mut(&mut self) -> super::MetadataOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::MetadataOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // observability_mode: optional bool
  pub fn observability_mode(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_observability_mode(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        15, val.into()
      )
    }
  }

  // disable_clear_route_cache: optional bool
  pub fn disable_clear_route_cache(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_disable_clear_route_cache(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        9, val.into()
      )
    }
  }

  // route_cache_action: optional enum envoy.extensions.filters.http.ext_proc.v3.ExternalProcessor.RouteCacheAction
  pub fn route_cache_action(&self) -> super::external_processor::RouteCacheAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        16, (super::external_processor::RouteCacheAction::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_route_cache_action(&mut self, val: super::external_processor::RouteCacheAction) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        16, val.into()
      )
    }
  }

  // deferred_close_timeout: optional message google.protobuf.Duration
  pub fn has_deferred_close_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_deferred_close_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn deferred_close_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_deferred_close_timeout().then(|| self.deferred_close_timeout())
  }
  pub fn deferred_close_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn deferred_close_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         17, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_deferred_close_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

  // send_body_without_waiting_for_header_response: optional bool
  pub fn send_body_without_waiting_for_header_response(&self) -> bool {
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
  pub fn set_send_body_without_waiting_for_header_response(&mut self, val: bool) {
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

  // allowed_override_modes: repeated message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn allowed_override_modes(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn allowed_override_modes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode> {
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
  pub fn set_allowed_override_modes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        src);
    }
  }

  // processing_request_modifier: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_processing_request_modifier(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_processing_request_modifier(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn processing_request_modifier_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_processing_request_modifier().then(|| self.processing_request_modifier())
  }
  pub fn processing_request_modifier(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn processing_request_modifier_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_processing_request_modifier(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val
      );
    }
  }

  // on_processing_response: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_on_processing_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_on_processing_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn on_processing_response_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_on_processing_response().then(|| self.on_processing_response())
  }
  pub fn on_processing_response(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn on_processing_response_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         21, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_on_processing_response(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val
      );
    }
  }

  // status_on_error: optional message envoy.type.v3.HttpStatus
  pub fn has_status_on_error(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_status_on_error(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn status_on_error_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_>> {
    self.has_status_on_error().then(|| self.status_on_error())
  }
  pub fn status_on_error(&self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }
  pub fn status_on_error_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusMut<'_> {
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
  pub fn set_status_on_error(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

}  // impl ExternalProcessor

impl ::std::ops::Drop for ExternalProcessor {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExternalProcessor {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExternalProcessor {
  type Proxied = Self;
  fn as_view(&self) -> ExternalProcessorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExternalProcessor {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExternalProcessorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExternalProcessor {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0proc__v3__ExternalProcessor_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3/P3aETET31X33/P33/P/P3/P.P33/PG333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0proc__v3__ExternalProcessor_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::common::mutation_rules::v3::mutation_rules::HeaderMutationRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HeaderForwardingRules as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::MetadataOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ExtProcHttpService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0proc__v3__ExternalProcessor_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExternalProcessor {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExternalProcessor {
  type Msg = ExternalProcessor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExternalProcessor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExternalProcessor {
  type Msg = ExternalProcessor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExternalProcessor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExternalProcessorMut<'_> {
  type Msg = ExternalProcessor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExternalProcessor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExternalProcessorMut<'_> {
  type Msg = ExternalProcessor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExternalProcessor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExternalProcessorView<'_> {
  type Msg = ExternalProcessor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExternalProcessor> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExternalProcessorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod external_processor {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RouteCacheAction(i32);

#[allow(non_upper_case_globals)]
impl RouteCacheAction {
  pub const Default: RouteCacheAction = RouteCacheAction(0);
  pub const Clear: RouteCacheAction = RouteCacheAction(1);
  pub const Retain: RouteCacheAction = RouteCacheAction(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Default",
      1 => "Clear",
      2 => "Retain",
      _ => return None
    })
  }
}

impl ::std::convert::From<RouteCacheAction> for i32 {
  fn from(val: RouteCacheAction) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for RouteCacheAction {
  fn from(val: i32) -> RouteCacheAction {
    Self(val)
  }
}

impl ::std::default::Default for RouteCacheAction {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for RouteCacheAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "RouteCacheAction::{}", constant_name)
    } else {
      write!(f, "RouteCacheAction::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for RouteCacheAction {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for RouteCacheAction {}

impl ::protobuf::Proxied for RouteCacheAction {
  type View<'a> = RouteCacheAction;
}

impl ::protobuf::AsView for RouteCacheAction {
  type Proxied = RouteCacheAction;

  fn as_view(&self) -> RouteCacheAction {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RouteCacheAction {
  fn into_view<'shorter>(self) -> RouteCacheAction where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for RouteCacheAction {
  const NAME: &'static str = "RouteCacheAction";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for RouteCacheAction {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod external_processor


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0proc__v3__ExtProcHttpService_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExtProcHttpService {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExtProcHttpService>
}

impl ::protobuf::Message for ExtProcHttpService {
  type MessageView<'msg> = ExtProcHttpServiceView<'msg>;
  type MessageMut<'msg> = ExtProcHttpServiceMut<'msg>;
}

impl ::std::default::Default for ExtProcHttpService {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExtProcHttpService {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExtProcHttpService` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtProcHttpServiceMut`.
unsafe impl ::std::marker::Sync for ExtProcHttpService {}

// SAFETY:
// - `ExtProcHttpService` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExtProcHttpService {}

impl ::protobuf::Proxied for ExtProcHttpService {
  type View<'msg> = ExtProcHttpServiceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExtProcHttpService {}

impl ::protobuf::MutProxied for ExtProcHttpService {
  type Mut<'msg> = ExtProcHttpServiceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtProcHttpServiceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtProcHttpService>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtProcHttpServiceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtProcHttpServiceView<'msg> {
  type Message = ExtProcHttpService;
}

impl ::std::fmt::Debug for ExtProcHttpServiceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtProcHttpServiceView<'_> {
  fn default() -> ExtProcHttpServiceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExtProcHttpService>> for ExtProcHttpServiceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtProcHttpService>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtProcHttpServiceView<'msg> {

  pub fn to_owned(&self) -> ExtProcHttpService {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // http_service: optional message envoy.config.core.v3.HttpService
  pub fn has_http_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn http_service_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'msg>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView::default())
  }

}

// SAFETY:
// - `ExtProcHttpServiceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtProcHttpServiceView<'_> {}

// SAFETY:
// - `ExtProcHttpServiceView` is `Send` because while its alive a `ExtProcHttpServiceMut` cannot.
// - `ExtProcHttpServiceView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtProcHttpServiceView<'_> {}

impl<'msg> ::protobuf::AsView for ExtProcHttpServiceView<'msg> {
  type Proxied = ExtProcHttpService;
  fn as_view(&self) -> ::protobuf::View<'msg, ExtProcHttpService> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtProcHttpServiceView<'msg> {
  fn into_view<'shorter>(self) -> ExtProcHttpServiceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtProcHttpService> for ExtProcHttpServiceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtProcHttpService {
    let mut dst = ExtProcHttpService::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtProcHttpService> for ExtProcHttpServiceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtProcHttpService {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExtProcHttpService {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtProcHttpServiceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtProcHttpServiceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtProcHttpServiceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcHttpService>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtProcHttpServiceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtProcHttpServiceMut<'msg> {
  type Message = ExtProcHttpService;
}

impl ::std::fmt::Debug for ExtProcHttpServiceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcHttpService>> for ExtProcHttpServiceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcHttpService>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtProcHttpServiceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcHttpService> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExtProcHttpService {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // http_service: optional message envoy.config.core.v3.HttpService
  pub fn has_http_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(&self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView::default())
  }
  pub fn http_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceMut<'_> {
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
  pub fn set_http_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_service::HttpService>) {

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
// - `ExtProcHttpServiceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtProcHttpServiceMut<'_> {}

// SAFETY:
// - `ExtProcHttpServiceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtProcHttpServiceMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtProcHttpServiceMut<'msg> {
  type Proxied = ExtProcHttpService;
  fn as_view(&self) -> ::protobuf::View<'_, ExtProcHttpService> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtProcHttpServiceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExtProcHttpService>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtProcHttpServiceMut<'msg> {
  type MutProxied = ExtProcHttpService;
  fn as_mut(&mut self) -> ExtProcHttpServiceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtProcHttpServiceMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtProcHttpServiceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExtProcHttpService {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExtProcHttpService> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtProcHttpServiceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtProcHttpServiceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // http_service: optional message envoy.config.core.v3.HttpService
  pub fn has_http_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(&self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView::default())
  }
  pub fn http_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceMut<'_> {
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
  pub fn set_http_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_service::HttpService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl ExtProcHttpService

impl ::std::ops::Drop for ExtProcHttpService {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExtProcHttpService {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExtProcHttpService {
  type Proxied = Self;
  fn as_view(&self) -> ExtProcHttpServiceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExtProcHttpService {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtProcHttpServiceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExtProcHttpService {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0proc__v3__ExtProcHttpService_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0proc__v3__ExtProcHttpService_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::http_service::HttpService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0proc__v3__ExtProcHttpService_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtProcHttpService {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtProcHttpService {
  type Msg = ExtProcHttpService;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcHttpService> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtProcHttpService {
  type Msg = ExtProcHttpService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcHttpService> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtProcHttpServiceMut<'_> {
  type Msg = ExtProcHttpService;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcHttpService> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtProcHttpServiceMut<'_> {
  type Msg = ExtProcHttpService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcHttpService> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtProcHttpServiceView<'_> {
  type Msg = ExtProcHttpService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcHttpService> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtProcHttpServiceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0proc__v3__MetadataOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MetadataOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MetadataOptions>
}

impl ::protobuf::Message for MetadataOptions {
  type MessageView<'msg> = MetadataOptionsView<'msg>;
  type MessageMut<'msg> = MetadataOptionsMut<'msg>;
}

impl ::std::default::Default for MetadataOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MetadataOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MetadataOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `MetadataOptionsMut`.
unsafe impl ::std::marker::Sync for MetadataOptions {}

// SAFETY:
// - `MetadataOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MetadataOptions {}

impl ::protobuf::Proxied for MetadataOptions {
  type View<'msg> = MetadataOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MetadataOptions {}

impl ::protobuf::MutProxied for MetadataOptions {
  type Mut<'msg> = MetadataOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MetadataOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MetadataOptionsView<'msg> {
  type Message = MetadataOptions;
}

impl ::std::fmt::Debug for MetadataOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MetadataOptionsView<'_> {
  fn default() -> MetadataOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataOptions>> for MetadataOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataOptionsView<'msg> {

  pub fn to_owned(&self) -> MetadataOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // forwarding_namespaces: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions.MetadataNamespaces
  pub fn has_forwarding_namespaces(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn forwarding_namespaces_opt(self) -> ::std::option::Option<super::metadata_options::MetadataNamespacesView<'msg>> {
    self.has_forwarding_namespaces().then(|| self.forwarding_namespaces())
  }
  pub fn forwarding_namespaces(self) -> super::metadata_options::MetadataNamespacesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_options::MetadataNamespacesView::default())
  }

  // receiving_namespaces: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions.MetadataNamespaces
  pub fn has_receiving_namespaces(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn receiving_namespaces_opt(self) -> ::std::option::Option<super::metadata_options::MetadataNamespacesView<'msg>> {
    self.has_receiving_namespaces().then(|| self.receiving_namespaces())
  }
  pub fn receiving_namespaces(self) -> super::metadata_options::MetadataNamespacesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_options::MetadataNamespacesView::default())
  }

  // cluster_metadata_forwarding_namespaces: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions.MetadataNamespaces
  pub fn has_cluster_metadata_forwarding_namespaces(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn cluster_metadata_forwarding_namespaces_opt(self) -> ::std::option::Option<super::metadata_options::MetadataNamespacesView<'msg>> {
    self.has_cluster_metadata_forwarding_namespaces().then(|| self.cluster_metadata_forwarding_namespaces())
  }
  pub fn cluster_metadata_forwarding_namespaces(self) -> super::metadata_options::MetadataNamespacesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_options::MetadataNamespacesView::default())
  }

}

// SAFETY:
// - `MetadataOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MetadataOptionsView<'_> {}

// SAFETY:
// - `MetadataOptionsView` is `Send` because while its alive a `MetadataOptionsMut` cannot.
// - `MetadataOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for MetadataOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for MetadataOptionsView<'msg> {
  type Proxied = MetadataOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, MetadataOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataOptionsView<'msg> {
  fn into_view<'shorter>(self) -> MetadataOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataOptions> for MetadataOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataOptions {
    let mut dst = MetadataOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataOptions> for MetadataOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MetadataOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MetadataOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MetadataOptionsMut<'msg> {
  type Message = MetadataOptions;
}

impl ::std::fmt::Debug for MetadataOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataOptions>> for MetadataOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MetadataOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // forwarding_namespaces: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions.MetadataNamespaces
  pub fn has_forwarding_namespaces(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_forwarding_namespaces(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn forwarding_namespaces_opt(&self) -> ::std::option::Option<super::metadata_options::MetadataNamespacesView<'_>> {
    self.has_forwarding_namespaces().then(|| self.forwarding_namespaces())
  }
  pub fn forwarding_namespaces(&self) -> super::metadata_options::MetadataNamespacesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_options::MetadataNamespacesView::default())
  }
  pub fn forwarding_namespaces_mut(&mut self) -> super::metadata_options::MetadataNamespacesMut<'_> {
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
  pub fn set_forwarding_namespaces(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_options::MetadataNamespaces>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // receiving_namespaces: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions.MetadataNamespaces
  pub fn has_receiving_namespaces(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_receiving_namespaces(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn receiving_namespaces_opt(&self) -> ::std::option::Option<super::metadata_options::MetadataNamespacesView<'_>> {
    self.has_receiving_namespaces().then(|| self.receiving_namespaces())
  }
  pub fn receiving_namespaces(&self) -> super::metadata_options::MetadataNamespacesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_options::MetadataNamespacesView::default())
  }
  pub fn receiving_namespaces_mut(&mut self) -> super::metadata_options::MetadataNamespacesMut<'_> {
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
  pub fn set_receiving_namespaces(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_options::MetadataNamespaces>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cluster_metadata_forwarding_namespaces: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions.MetadataNamespaces
  pub fn has_cluster_metadata_forwarding_namespaces(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cluster_metadata_forwarding_namespaces(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cluster_metadata_forwarding_namespaces_opt(&self) -> ::std::option::Option<super::metadata_options::MetadataNamespacesView<'_>> {
    self.has_cluster_metadata_forwarding_namespaces().then(|| self.cluster_metadata_forwarding_namespaces())
  }
  pub fn cluster_metadata_forwarding_namespaces(&self) -> super::metadata_options::MetadataNamespacesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_options::MetadataNamespacesView::default())
  }
  pub fn cluster_metadata_forwarding_namespaces_mut(&mut self) -> super::metadata_options::MetadataNamespacesMut<'_> {
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
  pub fn set_cluster_metadata_forwarding_namespaces(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_options::MetadataNamespaces>) {

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
// - `MetadataOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MetadataOptionsMut<'_> {}

// SAFETY:
// - `MetadataOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MetadataOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for MetadataOptionsMut<'msg> {
  type Proxied = MetadataOptions;
  fn as_view(&self) -> ::protobuf::View<'_, MetadataOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MetadataOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MetadataOptionsMut<'msg> {
  type MutProxied = MetadataOptions;
  fn as_mut(&mut self) -> MetadataOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MetadataOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> MetadataOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MetadataOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MetadataOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MetadataOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MetadataOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // forwarding_namespaces: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions.MetadataNamespaces
  pub fn has_forwarding_namespaces(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_forwarding_namespaces(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn forwarding_namespaces_opt(&self) -> ::std::option::Option<super::metadata_options::MetadataNamespacesView<'_>> {
    self.has_forwarding_namespaces().then(|| self.forwarding_namespaces())
  }
  pub fn forwarding_namespaces(&self) -> super::metadata_options::MetadataNamespacesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_options::MetadataNamespacesView::default())
  }
  pub fn forwarding_namespaces_mut(&mut self) -> super::metadata_options::MetadataNamespacesMut<'_> {
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
  pub fn set_forwarding_namespaces(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_options::MetadataNamespaces>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // receiving_namespaces: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions.MetadataNamespaces
  pub fn has_receiving_namespaces(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_receiving_namespaces(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn receiving_namespaces_opt(&self) -> ::std::option::Option<super::metadata_options::MetadataNamespacesView<'_>> {
    self.has_receiving_namespaces().then(|| self.receiving_namespaces())
  }
  pub fn receiving_namespaces(&self) -> super::metadata_options::MetadataNamespacesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_options::MetadataNamespacesView::default())
  }
  pub fn receiving_namespaces_mut(&mut self) -> super::metadata_options::MetadataNamespacesMut<'_> {
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
  pub fn set_receiving_namespaces(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_options::MetadataNamespaces>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cluster_metadata_forwarding_namespaces: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions.MetadataNamespaces
  pub fn has_cluster_metadata_forwarding_namespaces(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cluster_metadata_forwarding_namespaces(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cluster_metadata_forwarding_namespaces_opt(&self) -> ::std::option::Option<super::metadata_options::MetadataNamespacesView<'_>> {
    self.has_cluster_metadata_forwarding_namespaces().then(|| self.cluster_metadata_forwarding_namespaces())
  }
  pub fn cluster_metadata_forwarding_namespaces(&self) -> super::metadata_options::MetadataNamespacesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_options::MetadataNamespacesView::default())
  }
  pub fn cluster_metadata_forwarding_namespaces_mut(&mut self) -> super::metadata_options::MetadataNamespacesMut<'_> {
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
  pub fn set_cluster_metadata_forwarding_namespaces(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_options::MetadataNamespaces>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl MetadataOptions

impl ::std::ops::Drop for MetadataOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MetadataOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MetadataOptions {
  type Proxied = Self;
  fn as_view(&self) -> MetadataOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MetadataOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MetadataOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MetadataOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0proc__v3__MetadataOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0proc__v3__MetadataOptions_msg_init.0, &[<super::metadata_options::MetadataNamespaces as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::metadata_options::MetadataNamespaces as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::metadata_options::MetadataNamespaces as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0proc__v3__MetadataOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataOptions {
  type Msg = MetadataOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataOptions {
  type Msg = MetadataOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataOptionsMut<'_> {
  type Msg = MetadataOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataOptionsMut<'_> {
  type Msg = MetadataOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataOptionsView<'_> {
  type Msg = MetadataOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod metadata_options {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0proc__v3__MetadataOptions__MetadataNamespaces_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MetadataNamespaces {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MetadataNamespaces>
}

impl ::protobuf::Message for MetadataNamespaces {
  type MessageView<'msg> = MetadataNamespacesView<'msg>;
  type MessageMut<'msg> = MetadataNamespacesMut<'msg>;
}

impl ::std::default::Default for MetadataNamespaces {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MetadataNamespaces {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MetadataNamespaces` is `Sync` because it does not implement interior mutability.
//    Neither does `MetadataNamespacesMut`.
unsafe impl ::std::marker::Sync for MetadataNamespaces {}

// SAFETY:
// - `MetadataNamespaces` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MetadataNamespaces {}

impl ::protobuf::Proxied for MetadataNamespaces {
  type View<'msg> = MetadataNamespacesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MetadataNamespaces {}

impl ::protobuf::MutProxied for MetadataNamespaces {
  type Mut<'msg> = MetadataNamespacesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MetadataNamespacesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataNamespaces>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataNamespacesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MetadataNamespacesView<'msg> {
  type Message = MetadataNamespaces;
}

impl ::std::fmt::Debug for MetadataNamespacesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MetadataNamespacesView<'_> {
  fn default() -> MetadataNamespacesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataNamespaces>> for MetadataNamespacesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataNamespaces>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataNamespacesView<'msg> {

  pub fn to_owned(&self) -> MetadataNamespaces {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // untyped: repeated string
  pub fn untyped(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // typed: repeated string
  pub fn typed(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `MetadataNamespacesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MetadataNamespacesView<'_> {}

// SAFETY:
// - `MetadataNamespacesView` is `Send` because while its alive a `MetadataNamespacesMut` cannot.
// - `MetadataNamespacesView` does not use thread-local data.
unsafe impl ::std::marker::Send for MetadataNamespacesView<'_> {}

impl<'msg> ::protobuf::AsView for MetadataNamespacesView<'msg> {
  type Proxied = MetadataNamespaces;
  fn as_view(&self) -> ::protobuf::View<'msg, MetadataNamespaces> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataNamespacesView<'msg> {
  fn into_view<'shorter>(self) -> MetadataNamespacesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataNamespaces> for MetadataNamespacesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataNamespaces {
    let mut dst = MetadataNamespaces::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataNamespaces> for MetadataNamespacesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataNamespaces {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MetadataNamespaces {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataNamespacesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataNamespacesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MetadataNamespacesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataNamespaces>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataNamespacesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MetadataNamespacesMut<'msg> {
  type Message = MetadataNamespaces;
}

impl ::std::fmt::Debug for MetadataNamespacesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataNamespaces>> for MetadataNamespacesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataNamespaces>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataNamespacesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataNamespaces> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MetadataNamespaces {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // untyped: repeated string
  pub fn untyped(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn untyped_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_untyped(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // typed: repeated string
  pub fn typed(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn typed_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_typed(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `MetadataNamespacesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MetadataNamespacesMut<'_> {}

// SAFETY:
// - `MetadataNamespacesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MetadataNamespacesMut<'_> {}

impl<'msg> ::protobuf::AsView for MetadataNamespacesMut<'msg> {
  type Proxied = MetadataNamespaces;
  fn as_view(&self) -> ::protobuf::View<'_, MetadataNamespaces> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataNamespacesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MetadataNamespaces>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MetadataNamespacesMut<'msg> {
  type MutProxied = MetadataNamespaces;
  fn as_mut(&mut self) -> MetadataNamespacesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MetadataNamespacesMut<'msg> {
  fn into_mut<'shorter>(self) -> MetadataNamespacesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MetadataNamespaces {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MetadataNamespaces> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MetadataNamespacesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MetadataNamespacesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // untyped: repeated string
  pub fn untyped(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn untyped_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_untyped(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // typed: repeated string
  pub fn typed(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn typed_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_typed(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl MetadataNamespaces

impl ::std::ops::Drop for MetadataNamespaces {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MetadataNamespaces {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MetadataNamespaces {
  type Proxied = Self;
  fn as_view(&self) -> MetadataNamespacesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MetadataNamespaces {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MetadataNamespacesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MetadataNamespaces {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::metadata_options::envoy__extensions__filters__http__ext_0proc__v3__MetadataOptions__MetadataNamespaces_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$MEE");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::metadata_options::envoy__extensions__filters__http__ext_0proc__v3__MetadataOptions__MetadataNamespaces_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::metadata_options::envoy__extensions__filters__http__ext_0proc__v3__MetadataOptions__MetadataNamespaces_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataNamespaces {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataNamespaces {
  type Msg = MetadataNamespaces;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataNamespaces> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataNamespaces {
  type Msg = MetadataNamespaces;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataNamespaces> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataNamespacesMut<'_> {
  type Msg = MetadataNamespaces;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataNamespaces> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataNamespacesMut<'_> {
  type Msg = MetadataNamespaces;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataNamespaces> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataNamespacesView<'_> {
  type Msg = MetadataNamespaces;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataNamespaces> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataNamespacesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod metadata_options


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0proc__v3__HeaderForwardingRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderForwardingRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderForwardingRules>
}

impl ::protobuf::Message for HeaderForwardingRules {
  type MessageView<'msg> = HeaderForwardingRulesView<'msg>;
  type MessageMut<'msg> = HeaderForwardingRulesMut<'msg>;
}

impl ::std::default::Default for HeaderForwardingRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderForwardingRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderForwardingRules` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderForwardingRulesMut`.
unsafe impl ::std::marker::Sync for HeaderForwardingRules {}

// SAFETY:
// - `HeaderForwardingRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderForwardingRules {}

impl ::protobuf::Proxied for HeaderForwardingRules {
  type View<'msg> = HeaderForwardingRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderForwardingRules {}

impl ::protobuf::MutProxied for HeaderForwardingRules {
  type Mut<'msg> = HeaderForwardingRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderForwardingRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderForwardingRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderForwardingRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderForwardingRulesView<'msg> {
  type Message = HeaderForwardingRules;
}

impl ::std::fmt::Debug for HeaderForwardingRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderForwardingRulesView<'_> {
  fn default() -> HeaderForwardingRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderForwardingRules>> for HeaderForwardingRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderForwardingRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderForwardingRulesView<'msg> {

  pub fn to_owned(&self) -> HeaderForwardingRules {
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

  // disallowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_disallowed_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn disallowed_headers_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg>> {
    self.has_disallowed_headers().then(|| self.disallowed_headers())
  }
  pub fn disallowed_headers(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }

}

// SAFETY:
// - `HeaderForwardingRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderForwardingRulesView<'_> {}

// SAFETY:
// - `HeaderForwardingRulesView` is `Send` because while its alive a `HeaderForwardingRulesMut` cannot.
// - `HeaderForwardingRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderForwardingRulesView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderForwardingRulesView<'msg> {
  type Proxied = HeaderForwardingRules;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderForwardingRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderForwardingRulesView<'msg> {
  fn into_view<'shorter>(self) -> HeaderForwardingRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderForwardingRules> for HeaderForwardingRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderForwardingRules {
    let mut dst = HeaderForwardingRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderForwardingRules> for HeaderForwardingRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderForwardingRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderForwardingRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderForwardingRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderForwardingRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderForwardingRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderForwardingRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderForwardingRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderForwardingRulesMut<'msg> {
  type Message = HeaderForwardingRules;
}

impl ::std::fmt::Debug for HeaderForwardingRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderForwardingRules>> for HeaderForwardingRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderForwardingRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderForwardingRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderForwardingRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderForwardingRules {
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

  // disallowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_disallowed_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_disallowed_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn disallowed_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_disallowed_headers().then(|| self.disallowed_headers())
  }
  pub fn disallowed_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn disallowed_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_disallowed_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

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
// - `HeaderForwardingRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderForwardingRulesMut<'_> {}

// SAFETY:
// - `HeaderForwardingRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderForwardingRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderForwardingRulesMut<'msg> {
  type Proxied = HeaderForwardingRules;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderForwardingRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderForwardingRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderForwardingRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderForwardingRulesMut<'msg> {
  type MutProxied = HeaderForwardingRules;
  fn as_mut(&mut self) -> HeaderForwardingRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderForwardingRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderForwardingRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderForwardingRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderForwardingRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderForwardingRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderForwardingRulesMut<'_> {
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

  // disallowed_headers: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_disallowed_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_disallowed_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn disallowed_headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_disallowed_headers().then(|| self.disallowed_headers())
  }
  pub fn disallowed_headers(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn disallowed_headers_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_disallowed_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl HeaderForwardingRules

impl ::std::ops::Drop for HeaderForwardingRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderForwardingRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderForwardingRules {
  type Proxied = Self;
  fn as_view(&self) -> HeaderForwardingRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderForwardingRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderForwardingRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderForwardingRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0proc__v3__HeaderForwardingRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0proc__v3__HeaderForwardingRules_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0proc__v3__HeaderForwardingRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderForwardingRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderForwardingRules {
  type Msg = HeaderForwardingRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderForwardingRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderForwardingRules {
  type Msg = HeaderForwardingRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderForwardingRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderForwardingRulesMut<'_> {
  type Msg = HeaderForwardingRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderForwardingRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderForwardingRulesMut<'_> {
  type Msg = HeaderForwardingRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderForwardingRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderForwardingRulesView<'_> {
  type Msg = HeaderForwardingRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderForwardingRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderForwardingRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0proc__v3__ExtProcPerRoute_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExtProcPerRoute {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExtProcPerRoute>
}

impl ::protobuf::Message for ExtProcPerRoute {
  type MessageView<'msg> = ExtProcPerRouteView<'msg>;
  type MessageMut<'msg> = ExtProcPerRouteMut<'msg>;
}

impl ::std::default::Default for ExtProcPerRoute {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExtProcPerRoute {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExtProcPerRoute` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtProcPerRouteMut`.
unsafe impl ::std::marker::Sync for ExtProcPerRoute {}

// SAFETY:
// - `ExtProcPerRoute` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExtProcPerRoute {}

impl ::protobuf::Proxied for ExtProcPerRoute {
  type View<'msg> = ExtProcPerRouteView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExtProcPerRoute {}

impl ::protobuf::MutProxied for ExtProcPerRoute {
  type Mut<'msg> = ExtProcPerRouteMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtProcPerRouteView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtProcPerRoute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtProcPerRouteView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtProcPerRouteView<'msg> {
  type Message = ExtProcPerRoute;
}

impl ::std::fmt::Debug for ExtProcPerRouteView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtProcPerRouteView<'_> {
  fn default() -> ExtProcPerRouteView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExtProcPerRoute>> for ExtProcPerRouteView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtProcPerRoute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtProcPerRouteView<'msg> {

  pub fn to_owned(&self) -> ExtProcPerRoute {
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

  // overrides: optional message envoy.extensions.filters.http.ext_proc.v3.ExtProcOverrides
  pub fn has_overrides(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn overrides_opt(self) -> ::std::option::Option<super::ExtProcOverridesView<'msg>> {
    self.has_overrides().then(|| self.overrides())
  }
  pub fn overrides(self) -> super::ExtProcOverridesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExtProcOverridesView::default())
  }

  pub fn r#override(self) -> super::ext_proc_per_route::OverrideOneof<'msg> {
    match self.r#override_case() {
      super::ext_proc_per_route::OverrideCase::Disabled =>
          super::ext_proc_per_route::OverrideOneof::Disabled(self.disabled()),
      super::ext_proc_per_route::OverrideCase::Overrides =>
          super::ext_proc_per_route::OverrideOneof::Overrides(self.overrides()),
      _ => super::ext_proc_per_route::OverrideOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#override_case(self) -> super::ext_proc_per_route::OverrideCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::ext_proc_per_route::OverrideCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExtProcPerRouteView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtProcPerRouteView<'_> {}

// SAFETY:
// - `ExtProcPerRouteView` is `Send` because while its alive a `ExtProcPerRouteMut` cannot.
// - `ExtProcPerRouteView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtProcPerRouteView<'_> {}

impl<'msg> ::protobuf::AsView for ExtProcPerRouteView<'msg> {
  type Proxied = ExtProcPerRoute;
  fn as_view(&self) -> ::protobuf::View<'msg, ExtProcPerRoute> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtProcPerRouteView<'msg> {
  fn into_view<'shorter>(self) -> ExtProcPerRouteView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtProcPerRoute> for ExtProcPerRouteView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtProcPerRoute {
    let mut dst = ExtProcPerRoute::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtProcPerRoute> for ExtProcPerRouteMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtProcPerRoute {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExtProcPerRoute {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtProcPerRouteView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtProcPerRouteMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtProcPerRouteMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcPerRoute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtProcPerRouteMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtProcPerRouteMut<'msg> {
  type Message = ExtProcPerRoute;
}

impl ::std::fmt::Debug for ExtProcPerRouteMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcPerRoute>> for ExtProcPerRouteMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcPerRoute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtProcPerRouteMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcPerRoute> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExtProcPerRoute {
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

  // overrides: optional message envoy.extensions.filters.http.ext_proc.v3.ExtProcOverrides
  pub fn has_overrides(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_overrides(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn overrides_opt(&self) -> ::std::option::Option<super::ExtProcOverridesView<'_>> {
    self.has_overrides().then(|| self.overrides())
  }
  pub fn overrides(&self) -> super::ExtProcOverridesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExtProcOverridesView::default())
  }
  pub fn overrides_mut(&mut self) -> super::ExtProcOverridesMut<'_> {
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
  pub fn set_overrides(&mut self,
    val: impl ::protobuf::IntoProxied<super::ExtProcOverrides>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn r#override(&self) -> super::ext_proc_per_route::OverrideOneof<'_> {
    match &self.r#override_case() {
      super::ext_proc_per_route::OverrideCase::Disabled =>
          super::ext_proc_per_route::OverrideOneof::Disabled(self.disabled()),
      super::ext_proc_per_route::OverrideCase::Overrides =>
          super::ext_proc_per_route::OverrideOneof::Overrides(self.overrides()),
      _ => super::ext_proc_per_route::OverrideOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#override_case(&self) -> super::ext_proc_per_route::OverrideCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::ext_proc_per_route::OverrideCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExtProcPerRouteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtProcPerRouteMut<'_> {}

// SAFETY:
// - `ExtProcPerRouteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtProcPerRouteMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtProcPerRouteMut<'msg> {
  type Proxied = ExtProcPerRoute;
  fn as_view(&self) -> ::protobuf::View<'_, ExtProcPerRoute> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtProcPerRouteMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExtProcPerRoute>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtProcPerRouteMut<'msg> {
  type MutProxied = ExtProcPerRoute;
  fn as_mut(&mut self) -> ExtProcPerRouteMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtProcPerRouteMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtProcPerRouteMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExtProcPerRoute {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExtProcPerRoute> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtProcPerRouteView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtProcPerRouteMut<'_> {
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

  // overrides: optional message envoy.extensions.filters.http.ext_proc.v3.ExtProcOverrides
  pub fn has_overrides(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_overrides(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn overrides_opt(&self) -> ::std::option::Option<super::ExtProcOverridesView<'_>> {
    self.has_overrides().then(|| self.overrides())
  }
  pub fn overrides(&self) -> super::ExtProcOverridesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExtProcOverridesView::default())
  }
  pub fn overrides_mut(&mut self) -> super::ExtProcOverridesMut<'_> {
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
  pub fn set_overrides(&mut self,
    val: impl ::protobuf::IntoProxied<super::ExtProcOverrides>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn r#override(&self) -> super::ext_proc_per_route::OverrideOneof<'_> {
    match &self.r#override_case() {
      super::ext_proc_per_route::OverrideCase::Disabled =>
          super::ext_proc_per_route::OverrideOneof::Disabled(self.disabled()),
      super::ext_proc_per_route::OverrideCase::Overrides =>
          super::ext_proc_per_route::OverrideOneof::Overrides(self.overrides()),
      _ => super::ext_proc_per_route::OverrideOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#override_case(&self) -> super::ext_proc_per_route::OverrideCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::ext_proc_per_route::OverrideCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ExtProcPerRoute

impl ::std::ops::Drop for ExtProcPerRoute {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExtProcPerRoute {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExtProcPerRoute {
  type Proxied = Self;
  fn as_view(&self) -> ExtProcPerRouteView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExtProcPerRoute {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtProcPerRouteMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExtProcPerRoute {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0proc__v3__ExtProcPerRoute_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/3^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0proc__v3__ExtProcPerRoute_msg_init.0, &[<super::ExtProcOverrides as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0proc__v3__ExtProcPerRoute_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtProcPerRoute {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtProcPerRoute {
  type Msg = ExtProcPerRoute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcPerRoute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtProcPerRoute {
  type Msg = ExtProcPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcPerRoute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtProcPerRouteMut<'_> {
  type Msg = ExtProcPerRoute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcPerRoute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtProcPerRouteMut<'_> {
  type Msg = ExtProcPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcPerRoute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtProcPerRouteView<'_> {
  type Msg = ExtProcPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcPerRoute> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtProcPerRouteMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod ext_proc_per_route {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum OverrideOneof<'msg> {
  Disabled(bool) = 1,
  Overrides(::protobuf::View<'msg, super::super::ExtProcOverrides>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum OverrideCase {
  Disabled = 1,
  Overrides = 2,

  not_set = 0
}

impl OverrideCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<OverrideCase> {
    match v {
      0 => Some(OverrideCase::not_set),
      1 => Some(OverrideCase::Disabled),
      2 => Some(OverrideCase::Overrides),
      _ => None
    }
  }
}
}  // pub mod ext_proc_per_route


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__ext_0proc__v3__ExtProcOverrides_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExtProcOverrides {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExtProcOverrides>
}

impl ::protobuf::Message for ExtProcOverrides {
  type MessageView<'msg> = ExtProcOverridesView<'msg>;
  type MessageMut<'msg> = ExtProcOverridesMut<'msg>;
}

impl ::std::default::Default for ExtProcOverrides {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExtProcOverrides {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExtProcOverrides` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtProcOverridesMut`.
unsafe impl ::std::marker::Sync for ExtProcOverrides {}

// SAFETY:
// - `ExtProcOverrides` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExtProcOverrides {}

impl ::protobuf::Proxied for ExtProcOverrides {
  type View<'msg> = ExtProcOverridesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExtProcOverrides {}

impl ::protobuf::MutProxied for ExtProcOverrides {
  type Mut<'msg> = ExtProcOverridesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtProcOverridesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtProcOverrides>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtProcOverridesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtProcOverridesView<'msg> {
  type Message = ExtProcOverrides;
}

impl ::std::fmt::Debug for ExtProcOverridesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtProcOverridesView<'_> {
  fn default() -> ExtProcOverridesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExtProcOverrides>> for ExtProcOverridesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtProcOverrides>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtProcOverridesView<'msg> {

  pub fn to_owned(&self) -> ExtProcOverrides {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // processing_mode: optional message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn has_processing_mode(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn processing_mode_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'msg>> {
    self.has_processing_mode().then(|| self.processing_mode())
  }
  pub fn processing_mode(self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView::default())
  }

  // async_mode: optional bool
  pub fn async_mode(self) -> bool {
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

  // request_attributes: repeated string
  pub fn request_attributes(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // response_attributes: repeated string
  pub fn response_attributes(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // grpc_service: optional message envoy.config.core.v3.GrpcService
  pub fn has_grpc_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn grpc_service_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'msg>> {
    self.has_grpc_service().then(|| self.grpc_service())
  }
  pub fn grpc_service(self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }

  // metadata_options: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions
  pub fn has_metadata_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn metadata_options_opt(self) -> ::std::option::Option<super::MetadataOptionsView<'msg>> {
    self.has_metadata_options().then(|| self.metadata_options())
  }
  pub fn metadata_options(self) -> super::MetadataOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetadataOptionsView::default())
  }

  // grpc_initial_metadata: repeated message envoy.config.core.v3.HeaderValue
  pub fn grpc_initial_metadata(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // failure_mode_allow: optional message google.protobuf.BoolValue
  pub fn has_failure_mode_allow(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn failure_mode_allow_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_failure_mode_allow().then(|| self.failure_mode_allow())
  }
  pub fn failure_mode_allow(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // processing_request_modifier: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_processing_request_modifier(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn processing_request_modifier_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_processing_request_modifier().then(|| self.processing_request_modifier())
  }
  pub fn processing_request_modifier(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

}

// SAFETY:
// - `ExtProcOverridesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtProcOverridesView<'_> {}

// SAFETY:
// - `ExtProcOverridesView` is `Send` because while its alive a `ExtProcOverridesMut` cannot.
// - `ExtProcOverridesView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtProcOverridesView<'_> {}

impl<'msg> ::protobuf::AsView for ExtProcOverridesView<'msg> {
  type Proxied = ExtProcOverrides;
  fn as_view(&self) -> ::protobuf::View<'msg, ExtProcOverrides> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtProcOverridesView<'msg> {
  fn into_view<'shorter>(self) -> ExtProcOverridesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtProcOverrides> for ExtProcOverridesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtProcOverrides {
    let mut dst = ExtProcOverrides::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtProcOverrides> for ExtProcOverridesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtProcOverrides {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExtProcOverrides {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtProcOverridesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtProcOverridesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtProcOverridesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcOverrides>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtProcOverridesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtProcOverridesMut<'msg> {
  type Message = ExtProcOverrides;
}

impl ::std::fmt::Debug for ExtProcOverridesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcOverrides>> for ExtProcOverridesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcOverrides>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtProcOverridesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtProcOverrides> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExtProcOverrides {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // processing_mode: optional message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn has_processing_mode(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_processing_mode(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn processing_mode_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_>> {
    self.has_processing_mode().then(|| self.processing_mode())
  }
  pub fn processing_mode(&self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView::default())
  }
  pub fn processing_mode_mut(&mut self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeMut<'_> {
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
  pub fn set_processing_mode(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // async_mode: optional bool
  pub fn async_mode(&self) -> bool {
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
  pub fn set_async_mode(&mut self, val: bool) {
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

  // request_attributes: repeated string
  pub fn request_attributes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_attributes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
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
  pub fn set_request_attributes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // response_attributes: repeated string
  pub fn response_attributes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn response_attributes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_response_attributes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // grpc_service: optional message envoy.config.core.v3.GrpcService
  pub fn has_grpc_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_grpc_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn grpc_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_>> {
    self.has_grpc_service().then(|| self.grpc_service())
  }
  pub fn grpc_service(&self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }
  pub fn grpc_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceMut<'_> {
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
  pub fn set_grpc_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // metadata_options: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions
  pub fn has_metadata_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_metadata_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn metadata_options_opt(&self) -> ::std::option::Option<super::MetadataOptionsView<'_>> {
    self.has_metadata_options().then(|| self.metadata_options())
  }
  pub fn metadata_options(&self) -> super::MetadataOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetadataOptionsView::default())
  }
  pub fn metadata_options_mut(&mut self) -> super::MetadataOptionsMut<'_> {
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
  pub fn set_metadata_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::MetadataOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // grpc_initial_metadata: repeated message envoy.config.core.v3.HeaderValue
  pub fn grpc_initial_metadata(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn grpc_initial_metadata_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
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
  pub fn set_grpc_initial_metadata(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // failure_mode_allow: optional message google.protobuf.BoolValue
  pub fn has_failure_mode_allow(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_failure_mode_allow(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn failure_mode_allow_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_failure_mode_allow().then(|| self.failure_mode_allow())
  }
  pub fn failure_mode_allow(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn failure_mode_allow_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_failure_mode_allow(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // processing_request_modifier: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_processing_request_modifier(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_processing_request_modifier(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn processing_request_modifier_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_processing_request_modifier().then(|| self.processing_request_modifier())
  }
  pub fn processing_request_modifier(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn processing_request_modifier_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_processing_request_modifier(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

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
// - `ExtProcOverridesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtProcOverridesMut<'_> {}

// SAFETY:
// - `ExtProcOverridesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtProcOverridesMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtProcOverridesMut<'msg> {
  type Proxied = ExtProcOverrides;
  fn as_view(&self) -> ::protobuf::View<'_, ExtProcOverrides> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtProcOverridesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExtProcOverrides>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtProcOverridesMut<'msg> {
  type MutProxied = ExtProcOverrides;
  fn as_mut(&mut self) -> ExtProcOverridesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtProcOverridesMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtProcOverridesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExtProcOverrides {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExtProcOverrides> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtProcOverridesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtProcOverridesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // processing_mode: optional message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn has_processing_mode(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_processing_mode(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn processing_mode_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_>> {
    self.has_processing_mode().then(|| self.processing_mode())
  }
  pub fn processing_mode(&self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView::default())
  }
  pub fn processing_mode_mut(&mut self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeMut<'_> {
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
  pub fn set_processing_mode(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // async_mode: optional bool
  pub fn async_mode(&self) -> bool {
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
  pub fn set_async_mode(&mut self, val: bool) {
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

  // request_attributes: repeated string
  pub fn request_attributes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_attributes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
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
  pub fn set_request_attributes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // response_attributes: repeated string
  pub fn response_attributes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn response_attributes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_response_attributes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // grpc_service: optional message envoy.config.core.v3.GrpcService
  pub fn has_grpc_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_grpc_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn grpc_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_>> {
    self.has_grpc_service().then(|| self.grpc_service())
  }
  pub fn grpc_service(&self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceView::default())
  }
  pub fn grpc_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcServiceMut<'_> {
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
  pub fn set_grpc_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // metadata_options: optional message envoy.extensions.filters.http.ext_proc.v3.MetadataOptions
  pub fn has_metadata_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_metadata_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn metadata_options_opt(&self) -> ::std::option::Option<super::MetadataOptionsView<'_>> {
    self.has_metadata_options().then(|| self.metadata_options())
  }
  pub fn metadata_options(&self) -> super::MetadataOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetadataOptionsView::default())
  }
  pub fn metadata_options_mut(&mut self) -> super::MetadataOptionsMut<'_> {
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
  pub fn set_metadata_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::MetadataOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // grpc_initial_metadata: repeated message envoy.config.core.v3.HeaderValue
  pub fn grpc_initial_metadata(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn grpc_initial_metadata_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
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
  pub fn set_grpc_initial_metadata(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // failure_mode_allow: optional message google.protobuf.BoolValue
  pub fn has_failure_mode_allow(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_failure_mode_allow(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn failure_mode_allow_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_failure_mode_allow().then(|| self.failure_mode_allow())
  }
  pub fn failure_mode_allow(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn failure_mode_allow_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_failure_mode_allow(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // processing_request_modifier: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_processing_request_modifier(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_processing_request_modifier(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn processing_request_modifier_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_processing_request_modifier().then(|| self.processing_request_modifier())
  }
  pub fn processing_request_modifier(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn processing_request_modifier_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_processing_request_modifier(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

}  // impl ExtProcOverrides

impl ::std::ops::Drop for ExtProcOverrides {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExtProcOverrides {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExtProcOverrides {
  type Proxied = Self;
  fn as_view(&self) -> ExtProcOverridesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExtProcOverrides {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtProcOverridesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExtProcOverrides {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__ext_0proc__v3__ExtProcOverrides_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3/PETET33G33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__ext_0proc__v3__ExtProcOverrides_msg_init.0, &[<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::MetadataOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__ext_0proc__v3__ExtProcOverrides_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtProcOverrides {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtProcOverrides {
  type Msg = ExtProcOverrides;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcOverrides> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtProcOverrides {
  type Msg = ExtProcOverrides;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcOverrides> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtProcOverridesMut<'_> {
  type Msg = ExtProcOverrides;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcOverrides> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtProcOverridesMut<'_> {
  type Msg = ExtProcOverrides;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcOverrides> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtProcOverridesView<'_> {
  type Msg = ExtProcOverrides;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtProcOverrides> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtProcOverridesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



