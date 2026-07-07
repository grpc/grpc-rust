const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__trace__v3__OpenTelemetryConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OpenTelemetryConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OpenTelemetryConfig>
}

impl ::protobuf::Message for OpenTelemetryConfig {
  type MessageView<'msg> = OpenTelemetryConfigView<'msg>;
  type MessageMut<'msg> = OpenTelemetryConfigMut<'msg>;
}

impl ::std::default::Default for OpenTelemetryConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OpenTelemetryConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OpenTelemetryConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `OpenTelemetryConfigMut`.
unsafe impl ::std::marker::Sync for OpenTelemetryConfig {}

// SAFETY:
// - `OpenTelemetryConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for OpenTelemetryConfig {}

impl ::protobuf::Proxied for OpenTelemetryConfig {
  type View<'msg> = OpenTelemetryConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OpenTelemetryConfig {}

impl ::protobuf::MutProxied for OpenTelemetryConfig {
  type Mut<'msg> = OpenTelemetryConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OpenTelemetryConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OpenTelemetryConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OpenTelemetryConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OpenTelemetryConfigView<'msg> {
  type Message = OpenTelemetryConfig;
}

impl ::std::fmt::Debug for OpenTelemetryConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OpenTelemetryConfigView<'_> {
  fn default() -> OpenTelemetryConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OpenTelemetryConfig>> for OpenTelemetryConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OpenTelemetryConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OpenTelemetryConfigView<'msg> {

  pub fn to_owned(&self) -> OpenTelemetryConfig {
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

  // http_service: optional message envoy.config.core.v3.HttpService
  pub fn has_http_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn http_service_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'msg>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView::default())
  }

  // service_name: optional string
  pub fn service_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // resource_detectors: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn resource_detectors(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // sampler: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_sampler(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn sampler_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_sampler().then(|| self.sampler())
  }
  pub fn sampler(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // max_cache_size: optional message google.protobuf.UInt32Value
  pub fn has_max_cache_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn max_cache_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_cache_size().then(|| self.max_cache_size())
  }
  pub fn max_cache_size(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `OpenTelemetryConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OpenTelemetryConfigView<'_> {}

// SAFETY:
// - `OpenTelemetryConfigView` is `Send` because while its alive a `OpenTelemetryConfigMut` cannot.
// - `OpenTelemetryConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for OpenTelemetryConfigView<'_> {}

impl<'msg> ::protobuf::AsView for OpenTelemetryConfigView<'msg> {
  type Proxied = OpenTelemetryConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, OpenTelemetryConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OpenTelemetryConfigView<'msg> {
  fn into_view<'shorter>(self) -> OpenTelemetryConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OpenTelemetryConfig> for OpenTelemetryConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OpenTelemetryConfig {
    let mut dst = OpenTelemetryConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OpenTelemetryConfig> for OpenTelemetryConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OpenTelemetryConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for OpenTelemetryConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OpenTelemetryConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OpenTelemetryConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OpenTelemetryConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OpenTelemetryConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OpenTelemetryConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OpenTelemetryConfigMut<'msg> {
  type Message = OpenTelemetryConfig;
}

impl ::std::fmt::Debug for OpenTelemetryConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OpenTelemetryConfig>> for OpenTelemetryConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OpenTelemetryConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OpenTelemetryConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OpenTelemetryConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> OpenTelemetryConfig {
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

  // http_service: optional message envoy.config.core.v3.HttpService
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
  pub fn http_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(&self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView::default())
  }
  pub fn http_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_service::HttpService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // service_name: optional string
  pub fn service_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // resource_detectors: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn resource_detectors(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_detectors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
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
  pub fn set_resource_detectors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // sampler: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_sampler(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_sampler(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn sampler_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_sampler().then(|| self.sampler())
  }
  pub fn sampler(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn sampler_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_sampler(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // max_cache_size: optional message google.protobuf.UInt32Value
  pub fn has_max_cache_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_max_cache_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn max_cache_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_cache_size().then(|| self.max_cache_size())
  }
  pub fn max_cache_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_cache_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_cache_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

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
// - `OpenTelemetryConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OpenTelemetryConfigMut<'_> {}

// SAFETY:
// - `OpenTelemetryConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OpenTelemetryConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for OpenTelemetryConfigMut<'msg> {
  type Proxied = OpenTelemetryConfig;
  fn as_view(&self) -> ::protobuf::View<'_, OpenTelemetryConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OpenTelemetryConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OpenTelemetryConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OpenTelemetryConfigMut<'msg> {
  type MutProxied = OpenTelemetryConfig;
  fn as_mut(&mut self) -> OpenTelemetryConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OpenTelemetryConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> OpenTelemetryConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OpenTelemetryConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OpenTelemetryConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OpenTelemetryConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OpenTelemetryConfigMut<'_> {
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

  // http_service: optional message envoy.config.core.v3.HttpService
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
  pub fn http_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_>> {
    self.has_http_service().then(|| self.http_service())
  }
  pub fn http_service(&self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceView::default())
  }
  pub fn http_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_service::HttpServiceMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_service::HttpService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // service_name: optional string
  pub fn service_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // resource_detectors: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn resource_detectors(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_detectors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
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
  pub fn set_resource_detectors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // sampler: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_sampler(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_sampler(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn sampler_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_sampler().then(|| self.sampler())
  }
  pub fn sampler(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn sampler_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_sampler(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // max_cache_size: optional message google.protobuf.UInt32Value
  pub fn has_max_cache_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_max_cache_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn max_cache_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_cache_size().then(|| self.max_cache_size())
  }
  pub fn max_cache_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_cache_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_cache_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}  // impl OpenTelemetryConfig

impl ::std::ops::Drop for OpenTelemetryConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OpenTelemetryConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OpenTelemetryConfig {
  type Proxied = Self;
  fn as_view(&self) -> OpenTelemetryConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OpenTelemetryConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OpenTelemetryConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OpenTelemetryConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__trace__v3__OpenTelemetryConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X3G33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__trace__v3__OpenTelemetryConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::http_service::HttpService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__trace__v3__OpenTelemetryConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OpenTelemetryConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OpenTelemetryConfig {
  type Msg = OpenTelemetryConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OpenTelemetryConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OpenTelemetryConfig {
  type Msg = OpenTelemetryConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OpenTelemetryConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OpenTelemetryConfigMut<'_> {
  type Msg = OpenTelemetryConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OpenTelemetryConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OpenTelemetryConfigMut<'_> {
  type Msg = OpenTelemetryConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OpenTelemetryConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OpenTelemetryConfigView<'_> {
  type Msg = OpenTelemetryConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OpenTelemetryConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OpenTelemetryConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



