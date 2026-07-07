const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__ApiConfigSource_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ApiConfigSource {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ApiConfigSource>
}

impl ::protobuf::Message for ApiConfigSource {
  type MessageView<'msg> = ApiConfigSourceView<'msg>;
  type MessageMut<'msg> = ApiConfigSourceMut<'msg>;
}

impl ::std::default::Default for ApiConfigSource {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ApiConfigSource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ApiConfigSource` is `Sync` because it does not implement interior mutability.
//    Neither does `ApiConfigSourceMut`.
unsafe impl ::std::marker::Sync for ApiConfigSource {}

// SAFETY:
// - `ApiConfigSource` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ApiConfigSource {}

impl ::protobuf::Proxied for ApiConfigSource {
  type View<'msg> = ApiConfigSourceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ApiConfigSource {}

impl ::protobuf::MutProxied for ApiConfigSource {
  type Mut<'msg> = ApiConfigSourceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ApiConfigSourceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ApiConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ApiConfigSourceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ApiConfigSourceView<'msg> {
  type Message = ApiConfigSource;
}

impl ::std::fmt::Debug for ApiConfigSourceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ApiConfigSourceView<'_> {
  fn default() -> ApiConfigSourceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ApiConfigSource>> for ApiConfigSourceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ApiConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ApiConfigSourceView<'msg> {

  pub fn to_owned(&self) -> ApiConfigSource {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // api_type: optional enum envoy.config.core.v3.ApiConfigSource.ApiType
  pub fn api_type(self) -> super::api_config_source::ApiType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::api_config_source::ApiType::DeprecatedAndUnavailableDoNotUse).into()
      ).try_into().unwrap()
    }
  }

  // transport_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn transport_api_version(self) -> super::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }

  // cluster_names: repeated string
  pub fn cluster_names(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // grpc_services: repeated message envoy.config.core.v3.GrpcService
  pub fn grpc_services(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // refresh_delay: optional message google.protobuf.Duration
  pub fn has_refresh_delay(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn refresh_delay_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_refresh_delay().then(|| self.refresh_delay())
  }
  pub fn refresh_delay(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // request_timeout: optional message google.protobuf.Duration
  pub fn has_request_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn request_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_request_timeout().then(|| self.request_timeout())
  }
  pub fn request_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // rate_limit_settings: optional message envoy.config.core.v3.RateLimitSettings
  pub fn has_rate_limit_settings(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn rate_limit_settings_opt(self) -> ::std::option::Option<super::RateLimitSettingsView<'msg>> {
    self.has_rate_limit_settings().then(|| self.rate_limit_settings())
  }
  pub fn rate_limit_settings(self) -> super::RateLimitSettingsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RateLimitSettingsView::default())
  }

  // set_node_on_first_message_only: optional bool
  pub fn set_node_on_first_message_only(self) -> bool {
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

  // config_validators: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn config_validators(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ApiConfigSourceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ApiConfigSourceView<'_> {}

// SAFETY:
// - `ApiConfigSourceView` is `Send` because while its alive a `ApiConfigSourceMut` cannot.
// - `ApiConfigSourceView` does not use thread-local data.
unsafe impl ::std::marker::Send for ApiConfigSourceView<'_> {}

impl<'msg> ::protobuf::AsView for ApiConfigSourceView<'msg> {
  type Proxied = ApiConfigSource;
  fn as_view(&self) -> ::protobuf::View<'msg, ApiConfigSource> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApiConfigSourceView<'msg> {
  fn into_view<'shorter>(self) -> ApiConfigSourceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ApiConfigSource> for ApiConfigSourceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ApiConfigSource {
    let mut dst = ApiConfigSource::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ApiConfigSource> for ApiConfigSourceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ApiConfigSource {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ApiConfigSource {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ApiConfigSourceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ApiConfigSourceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ApiConfigSourceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ApiConfigSourceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ApiConfigSourceMut<'msg> {
  type Message = ApiConfigSource;
}

impl ::std::fmt::Debug for ApiConfigSourceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ApiConfigSource>> for ApiConfigSourceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ApiConfigSourceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiConfigSource> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ApiConfigSource {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // api_type: optional enum envoy.config.core.v3.ApiConfigSource.ApiType
  pub fn api_type(&self) -> super::api_config_source::ApiType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::api_config_source::ApiType::DeprecatedAndUnavailableDoNotUse).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_api_type(&mut self, val: super::api_config_source::ApiType) {
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

  // transport_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn transport_api_version(&self) -> super::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_transport_api_version(&mut self, val: super::ApiVersion) {
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

  // cluster_names: repeated string
  pub fn cluster_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn cluster_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_cluster_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // grpc_services: repeated message envoy.config.core.v3.GrpcService
  pub fn grpc_services(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn grpc_services_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService> {
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
  pub fn set_grpc_services(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // refresh_delay: optional message google.protobuf.Duration
  pub fn has_refresh_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_refresh_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn refresh_delay_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_refresh_delay().then(|| self.refresh_delay())
  }
  pub fn refresh_delay(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn refresh_delay_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_refresh_delay(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // request_timeout: optional message google.protobuf.Duration
  pub fn has_request_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_request_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn request_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_request_timeout().then(|| self.request_timeout())
  }
  pub fn request_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn request_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_request_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // rate_limit_settings: optional message envoy.config.core.v3.RateLimitSettings
  pub fn has_rate_limit_settings(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_rate_limit_settings(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn rate_limit_settings_opt(&self) -> ::std::option::Option<super::RateLimitSettingsView<'_>> {
    self.has_rate_limit_settings().then(|| self.rate_limit_settings())
  }
  pub fn rate_limit_settings(&self) -> super::RateLimitSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RateLimitSettingsView::default())
  }
  pub fn rate_limit_settings_mut(&mut self) -> super::RateLimitSettingsMut<'_> {
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
  pub fn set_rate_limit_settings(&mut self,
    val: impl ::protobuf::IntoProxied<super::RateLimitSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // set_node_on_first_message_only: optional bool
  pub fn set_node_on_first_message_only(&self) -> bool {
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
  pub fn set_set_node_on_first_message_only(&mut self, val: bool) {
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

  // config_validators: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn config_validators(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn config_validators_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        8,
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
  pub fn set_config_validators(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

}

// SAFETY:
// - `ApiConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ApiConfigSourceMut<'_> {}

// SAFETY:
// - `ApiConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ApiConfigSourceMut<'_> {}

impl<'msg> ::protobuf::AsView for ApiConfigSourceMut<'msg> {
  type Proxied = ApiConfigSource;
  fn as_view(&self) -> ::protobuf::View<'_, ApiConfigSource> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApiConfigSourceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ApiConfigSource>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ApiConfigSourceMut<'msg> {
  type MutProxied = ApiConfigSource;
  fn as_mut(&mut self) -> ApiConfigSourceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ApiConfigSourceMut<'msg> {
  fn into_mut<'shorter>(self) -> ApiConfigSourceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ApiConfigSource {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ApiConfigSource> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ApiConfigSourceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ApiConfigSourceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // api_type: optional enum envoy.config.core.v3.ApiConfigSource.ApiType
  pub fn api_type(&self) -> super::api_config_source::ApiType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::api_config_source::ApiType::DeprecatedAndUnavailableDoNotUse).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_api_type(&mut self, val: super::api_config_source::ApiType) {
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

  // transport_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn transport_api_version(&self) -> super::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_transport_api_version(&mut self, val: super::ApiVersion) {
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

  // cluster_names: repeated string
  pub fn cluster_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn cluster_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_cluster_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // grpc_services: repeated message envoy.config.core.v3.GrpcService
  pub fn grpc_services(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn grpc_services_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService> {
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
  pub fn set_grpc_services(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // refresh_delay: optional message google.protobuf.Duration
  pub fn has_refresh_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_refresh_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn refresh_delay_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_refresh_delay().then(|| self.refresh_delay())
  }
  pub fn refresh_delay(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn refresh_delay_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_refresh_delay(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // request_timeout: optional message google.protobuf.Duration
  pub fn has_request_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_request_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn request_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_request_timeout().then(|| self.request_timeout())
  }
  pub fn request_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn request_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_request_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // rate_limit_settings: optional message envoy.config.core.v3.RateLimitSettings
  pub fn has_rate_limit_settings(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_rate_limit_settings(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn rate_limit_settings_opt(&self) -> ::std::option::Option<super::RateLimitSettingsView<'_>> {
    self.has_rate_limit_settings().then(|| self.rate_limit_settings())
  }
  pub fn rate_limit_settings(&self) -> super::RateLimitSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RateLimitSettingsView::default())
  }
  pub fn rate_limit_settings_mut(&mut self) -> super::RateLimitSettingsMut<'_> {
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
  pub fn set_rate_limit_settings(&mut self,
    val: impl ::protobuf::IntoProxied<super::RateLimitSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // set_node_on_first_message_only: optional bool
  pub fn set_node_on_first_message_only(&self) -> bool {
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
  pub fn set_set_node_on_first_message_only(&mut self, val: bool) {
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

  // config_validators: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn config_validators(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn config_validators_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        8,
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
  pub fn set_config_validators(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

}  // impl ApiConfigSource

impl ::std::ops::Drop for ApiConfigSource {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ApiConfigSource {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ApiConfigSource {
  type Proxied = Self;
  fn as_view(&self) -> ApiConfigSourceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ApiConfigSource {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ApiConfigSourceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ApiConfigSource {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__ApiConfigSource_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.PET3G33/P.PG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__ApiConfigSource_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::RateLimitSettings as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__ApiConfigSource_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ApiConfigSource {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ApiConfigSource {
  type Msg = ApiConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiConfigSource {
  type Msg = ApiConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ApiConfigSourceMut<'_> {
  type Msg = ApiConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiConfigSourceMut<'_> {
  type Msg = ApiConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiConfigSourceView<'_> {
  type Msg = ApiConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiConfigSource> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ApiConfigSourceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod api_config_source {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApiType(i32);

#[allow(non_upper_case_globals)]
impl ApiType {
  pub const DeprecatedAndUnavailableDoNotUse: ApiType = ApiType(0);
  pub const Rest: ApiType = ApiType(1);
  pub const Grpc: ApiType = ApiType(2);
  pub const DeltaGrpc: ApiType = ApiType(3);
  pub const AggregatedGrpc: ApiType = ApiType(5);
  pub const AggregatedDeltaGrpc: ApiType = ApiType(6);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "DeprecatedAndUnavailableDoNotUse",
      1 => "Rest",
      2 => "Grpc",
      3 => "DeltaGrpc",
      5 => "AggregatedGrpc",
      6 => "AggregatedDeltaGrpc",
      _ => return None
    })
  }
}

impl ::std::convert::From<ApiType> for i32 {
  fn from(val: ApiType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ApiType {
  fn from(val: i32) -> ApiType {
    Self(val)
  }
}

impl ::std::default::Default for ApiType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ApiType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ApiType::{}", constant_name)
    } else {
      write!(f, "ApiType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ApiType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ApiType {}

impl ::protobuf::Proxied for ApiType {
  type View<'a> = ApiType;
}

impl ::protobuf::AsView for ApiType {
  type Proxied = ApiType;

  fn as_view(&self) -> ApiType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApiType {
  fn into_view<'shorter>(self) -> ApiType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ApiType {
  const NAME: &'static str = "ApiType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|5|6)
  }
}

impl ::protobuf::__internal::EntityType for ApiType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod api_config_source


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__AggregatedConfigSource_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AggregatedConfigSource {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AggregatedConfigSource>
}

impl ::protobuf::Message for AggregatedConfigSource {
  type MessageView<'msg> = AggregatedConfigSourceView<'msg>;
  type MessageMut<'msg> = AggregatedConfigSourceMut<'msg>;
}

impl ::std::default::Default for AggregatedConfigSource {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AggregatedConfigSource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AggregatedConfigSource` is `Sync` because it does not implement interior mutability.
//    Neither does `AggregatedConfigSourceMut`.
unsafe impl ::std::marker::Sync for AggregatedConfigSource {}

// SAFETY:
// - `AggregatedConfigSource` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AggregatedConfigSource {}

impl ::protobuf::Proxied for AggregatedConfigSource {
  type View<'msg> = AggregatedConfigSourceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AggregatedConfigSource {}

impl ::protobuf::MutProxied for AggregatedConfigSource {
  type Mut<'msg> = AggregatedConfigSourceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AggregatedConfigSourceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AggregatedConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AggregatedConfigSourceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AggregatedConfigSourceView<'msg> {
  type Message = AggregatedConfigSource;
}

impl ::std::fmt::Debug for AggregatedConfigSourceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AggregatedConfigSourceView<'_> {
  fn default() -> AggregatedConfigSourceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AggregatedConfigSource>> for AggregatedConfigSourceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AggregatedConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AggregatedConfigSourceView<'msg> {

  pub fn to_owned(&self) -> AggregatedConfigSource {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `AggregatedConfigSourceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AggregatedConfigSourceView<'_> {}

// SAFETY:
// - `AggregatedConfigSourceView` is `Send` because while its alive a `AggregatedConfigSourceMut` cannot.
// - `AggregatedConfigSourceView` does not use thread-local data.
unsafe impl ::std::marker::Send for AggregatedConfigSourceView<'_> {}

impl<'msg> ::protobuf::AsView for AggregatedConfigSourceView<'msg> {
  type Proxied = AggregatedConfigSource;
  fn as_view(&self) -> ::protobuf::View<'msg, AggregatedConfigSource> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AggregatedConfigSourceView<'msg> {
  fn into_view<'shorter>(self) -> AggregatedConfigSourceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AggregatedConfigSource> for AggregatedConfigSourceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AggregatedConfigSource {
    let mut dst = AggregatedConfigSource::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AggregatedConfigSource> for AggregatedConfigSourceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AggregatedConfigSource {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AggregatedConfigSource {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AggregatedConfigSourceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AggregatedConfigSourceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AggregatedConfigSourceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AggregatedConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AggregatedConfigSourceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AggregatedConfigSourceMut<'msg> {
  type Message = AggregatedConfigSource;
}

impl ::std::fmt::Debug for AggregatedConfigSourceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AggregatedConfigSource>> for AggregatedConfigSourceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AggregatedConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AggregatedConfigSourceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AggregatedConfigSource> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AggregatedConfigSource {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `AggregatedConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AggregatedConfigSourceMut<'_> {}

// SAFETY:
// - `AggregatedConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AggregatedConfigSourceMut<'_> {}

impl<'msg> ::protobuf::AsView for AggregatedConfigSourceMut<'msg> {
  type Proxied = AggregatedConfigSource;
  fn as_view(&self) -> ::protobuf::View<'_, AggregatedConfigSource> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AggregatedConfigSourceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AggregatedConfigSource>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AggregatedConfigSourceMut<'msg> {
  type MutProxied = AggregatedConfigSource;
  fn as_mut(&mut self) -> AggregatedConfigSourceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AggregatedConfigSourceMut<'msg> {
  fn into_mut<'shorter>(self) -> AggregatedConfigSourceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AggregatedConfigSource {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AggregatedConfigSource> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AggregatedConfigSourceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AggregatedConfigSourceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl AggregatedConfigSource

impl ::std::ops::Drop for AggregatedConfigSource {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AggregatedConfigSource {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AggregatedConfigSource {
  type Proxied = Self;
  fn as_view(&self) -> AggregatedConfigSourceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AggregatedConfigSource {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AggregatedConfigSourceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AggregatedConfigSource {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__AggregatedConfigSource_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__AggregatedConfigSource_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__AggregatedConfigSource_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AggregatedConfigSource {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AggregatedConfigSource {
  type Msg = AggregatedConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AggregatedConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AggregatedConfigSource {
  type Msg = AggregatedConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AggregatedConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AggregatedConfigSourceMut<'_> {
  type Msg = AggregatedConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AggregatedConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AggregatedConfigSourceMut<'_> {
  type Msg = AggregatedConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AggregatedConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AggregatedConfigSourceView<'_> {
  type Msg = AggregatedConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AggregatedConfigSource> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AggregatedConfigSourceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__SelfConfigSource_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SelfConfigSource {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SelfConfigSource>
}

impl ::protobuf::Message for SelfConfigSource {
  type MessageView<'msg> = SelfConfigSourceView<'msg>;
  type MessageMut<'msg> = SelfConfigSourceMut<'msg>;
}

impl ::std::default::Default for SelfConfigSource {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SelfConfigSource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SelfConfigSource` is `Sync` because it does not implement interior mutability.
//    Neither does `SelfConfigSourceMut`.
unsafe impl ::std::marker::Sync for SelfConfigSource {}

// SAFETY:
// - `SelfConfigSource` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SelfConfigSource {}

impl ::protobuf::Proxied for SelfConfigSource {
  type View<'msg> = SelfConfigSourceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SelfConfigSource {}

impl ::protobuf::MutProxied for SelfConfigSource {
  type Mut<'msg> = SelfConfigSourceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SelfConfigSourceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SelfConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SelfConfigSourceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SelfConfigSourceView<'msg> {
  type Message = SelfConfigSource;
}

impl ::std::fmt::Debug for SelfConfigSourceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SelfConfigSourceView<'_> {
  fn default() -> SelfConfigSourceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SelfConfigSource>> for SelfConfigSourceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SelfConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SelfConfigSourceView<'msg> {

  pub fn to_owned(&self) -> SelfConfigSource {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // transport_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn transport_api_version(self) -> super::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SelfConfigSourceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SelfConfigSourceView<'_> {}

// SAFETY:
// - `SelfConfigSourceView` is `Send` because while its alive a `SelfConfigSourceMut` cannot.
// - `SelfConfigSourceView` does not use thread-local data.
unsafe impl ::std::marker::Send for SelfConfigSourceView<'_> {}

impl<'msg> ::protobuf::AsView for SelfConfigSourceView<'msg> {
  type Proxied = SelfConfigSource;
  fn as_view(&self) -> ::protobuf::View<'msg, SelfConfigSource> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SelfConfigSourceView<'msg> {
  fn into_view<'shorter>(self) -> SelfConfigSourceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SelfConfigSource> for SelfConfigSourceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SelfConfigSource {
    let mut dst = SelfConfigSource::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SelfConfigSource> for SelfConfigSourceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SelfConfigSource {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SelfConfigSource {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SelfConfigSourceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SelfConfigSourceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SelfConfigSourceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SelfConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SelfConfigSourceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SelfConfigSourceMut<'msg> {
  type Message = SelfConfigSource;
}

impl ::std::fmt::Debug for SelfConfigSourceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SelfConfigSource>> for SelfConfigSourceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SelfConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SelfConfigSourceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SelfConfigSource> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SelfConfigSource {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // transport_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn transport_api_version(&self) -> super::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_transport_api_version(&mut self, val: super::ApiVersion) {
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

}

// SAFETY:
// - `SelfConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SelfConfigSourceMut<'_> {}

// SAFETY:
// - `SelfConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SelfConfigSourceMut<'_> {}

impl<'msg> ::protobuf::AsView for SelfConfigSourceMut<'msg> {
  type Proxied = SelfConfigSource;
  fn as_view(&self) -> ::protobuf::View<'_, SelfConfigSource> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SelfConfigSourceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SelfConfigSource>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SelfConfigSourceMut<'msg> {
  type MutProxied = SelfConfigSource;
  fn as_mut(&mut self) -> SelfConfigSourceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SelfConfigSourceMut<'msg> {
  fn into_mut<'shorter>(self) -> SelfConfigSourceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SelfConfigSource {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SelfConfigSource> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SelfConfigSourceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SelfConfigSourceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // transport_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn transport_api_version(&self) -> super::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_transport_api_version(&mut self, val: super::ApiVersion) {
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

}  // impl SelfConfigSource

impl ::std::ops::Drop for SelfConfigSource {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SelfConfigSource {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SelfConfigSource {
  type Proxied = Self;
  fn as_view(&self) -> SelfConfigSourceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SelfConfigSource {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SelfConfigSourceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SelfConfigSource {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__SelfConfigSource_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__SelfConfigSource_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__SelfConfigSource_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SelfConfigSource {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SelfConfigSource {
  type Msg = SelfConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SelfConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SelfConfigSource {
  type Msg = SelfConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SelfConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SelfConfigSourceMut<'_> {
  type Msg = SelfConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SelfConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SelfConfigSourceMut<'_> {
  type Msg = SelfConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SelfConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SelfConfigSourceView<'_> {
  type Msg = SelfConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SelfConfigSource> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SelfConfigSourceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__RateLimitSettings_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RateLimitSettings {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RateLimitSettings>
}

impl ::protobuf::Message for RateLimitSettings {
  type MessageView<'msg> = RateLimitSettingsView<'msg>;
  type MessageMut<'msg> = RateLimitSettingsMut<'msg>;
}

impl ::std::default::Default for RateLimitSettings {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RateLimitSettings {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RateLimitSettings` is `Sync` because it does not implement interior mutability.
//    Neither does `RateLimitSettingsMut`.
unsafe impl ::std::marker::Sync for RateLimitSettings {}

// SAFETY:
// - `RateLimitSettings` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitSettings {}

impl ::protobuf::Proxied for RateLimitSettings {
  type View<'msg> = RateLimitSettingsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RateLimitSettings {}

impl ::protobuf::MutProxied for RateLimitSettings {
  type Mut<'msg> = RateLimitSettingsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RateLimitSettingsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitSettingsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RateLimitSettingsView<'msg> {
  type Message = RateLimitSettings;
}

impl ::std::fmt::Debug for RateLimitSettingsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RateLimitSettingsView<'_> {
  fn default() -> RateLimitSettingsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitSettings>> for RateLimitSettingsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitSettingsView<'msg> {

  pub fn to_owned(&self) -> RateLimitSettings {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // max_tokens: optional message google.protobuf.UInt32Value
  pub fn has_max_tokens(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn max_tokens_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_tokens().then(|| self.max_tokens())
  }
  pub fn max_tokens(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // fill_rate: optional message google.protobuf.DoubleValue
  pub fn has_fill_rate(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn fill_rate_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DoubleValueView<'msg>> {
    self.has_fill_rate().then(|| self.fill_rate())
  }
  pub fn fill_rate(self) -> ::protobuf_well_known_types::DoubleValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DoubleValueView::default())
  }

}

// SAFETY:
// - `RateLimitSettingsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RateLimitSettingsView<'_> {}

// SAFETY:
// - `RateLimitSettingsView` is `Send` because while its alive a `RateLimitSettingsMut` cannot.
// - `RateLimitSettingsView` does not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitSettingsView<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitSettingsView<'msg> {
  type Proxied = RateLimitSettings;
  fn as_view(&self) -> ::protobuf::View<'msg, RateLimitSettings> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitSettingsView<'msg> {
  fn into_view<'shorter>(self) -> RateLimitSettingsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitSettings> for RateLimitSettingsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitSettings {
    let mut dst = RateLimitSettings::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitSettings> for RateLimitSettingsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitSettings {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RateLimitSettings {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitSettingsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitSettingsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RateLimitSettingsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitSettingsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RateLimitSettingsMut<'msg> {
  type Message = RateLimitSettings;
}

impl ::std::fmt::Debug for RateLimitSettingsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitSettings>> for RateLimitSettingsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitSettingsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitSettings> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RateLimitSettings {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // max_tokens: optional message google.protobuf.UInt32Value
  pub fn has_max_tokens(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_tokens(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_tokens_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_tokens().then(|| self.max_tokens())
  }
  pub fn max_tokens(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_tokens_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_tokens(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // fill_rate: optional message google.protobuf.DoubleValue
  pub fn has_fill_rate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_fill_rate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn fill_rate_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DoubleValueView<'_>> {
    self.has_fill_rate().then(|| self.fill_rate())
  }
  pub fn fill_rate(&self) -> ::protobuf_well_known_types::DoubleValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DoubleValueView::default())
  }
  pub fn fill_rate_mut(&mut self) -> ::protobuf_well_known_types::DoubleValueMut<'_> {
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
  pub fn set_fill_rate(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::DoubleValue>) {

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
// - `RateLimitSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RateLimitSettingsMut<'_> {}

// SAFETY:
// - `RateLimitSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RateLimitSettingsMut<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitSettingsMut<'msg> {
  type Proxied = RateLimitSettings;
  fn as_view(&self) -> ::protobuf::View<'_, RateLimitSettings> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitSettingsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RateLimitSettings>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RateLimitSettingsMut<'msg> {
  type MutProxied = RateLimitSettings;
  fn as_mut(&mut self) -> RateLimitSettingsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RateLimitSettingsMut<'msg> {
  fn into_mut<'shorter>(self) -> RateLimitSettingsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RateLimitSettings {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RateLimitSettings> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RateLimitSettingsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RateLimitSettingsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // max_tokens: optional message google.protobuf.UInt32Value
  pub fn has_max_tokens(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_tokens(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_tokens_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_tokens().then(|| self.max_tokens())
  }
  pub fn max_tokens(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_tokens_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_tokens(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // fill_rate: optional message google.protobuf.DoubleValue
  pub fn has_fill_rate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_fill_rate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn fill_rate_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DoubleValueView<'_>> {
    self.has_fill_rate().then(|| self.fill_rate())
  }
  pub fn fill_rate(&self) -> ::protobuf_well_known_types::DoubleValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DoubleValueView::default())
  }
  pub fn fill_rate_mut(&mut self) -> ::protobuf_well_known_types::DoubleValueMut<'_> {
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
  pub fn set_fill_rate(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::DoubleValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl RateLimitSettings

impl ::std::ops::Drop for RateLimitSettings {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RateLimitSettings {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RateLimitSettings {
  type Proxied = Self;
  fn as_view(&self) -> RateLimitSettingsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RateLimitSettings {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RateLimitSettingsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RateLimitSettings {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__RateLimitSettings_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__RateLimitSettings_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::DoubleValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__RateLimitSettings_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitSettings {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitSettings {
  type Msg = RateLimitSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitSettings {
  type Msg = RateLimitSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitSettingsMut<'_> {
  type Msg = RateLimitSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitSettingsMut<'_> {
  type Msg = RateLimitSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitSettingsView<'_> {
  type Msg = RateLimitSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitSettings> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitSettingsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__PathConfigSource_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PathConfigSource {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PathConfigSource>
}

impl ::protobuf::Message for PathConfigSource {
  type MessageView<'msg> = PathConfigSourceView<'msg>;
  type MessageMut<'msg> = PathConfigSourceMut<'msg>;
}

impl ::std::default::Default for PathConfigSource {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PathConfigSource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PathConfigSource` is `Sync` because it does not implement interior mutability.
//    Neither does `PathConfigSourceMut`.
unsafe impl ::std::marker::Sync for PathConfigSource {}

// SAFETY:
// - `PathConfigSource` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PathConfigSource {}

impl ::protobuf::Proxied for PathConfigSource {
  type View<'msg> = PathConfigSourceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PathConfigSource {}

impl ::protobuf::MutProxied for PathConfigSource {
  type Mut<'msg> = PathConfigSourceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PathConfigSourceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathConfigSourceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PathConfigSourceView<'msg> {
  type Message = PathConfigSource;
}

impl ::std::fmt::Debug for PathConfigSourceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PathConfigSourceView<'_> {
  fn default() -> PathConfigSourceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PathConfigSource>> for PathConfigSourceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathConfigSourceView<'msg> {

  pub fn to_owned(&self) -> PathConfigSource {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // path: optional string
  pub fn path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn watched_directory_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'msg>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView::default())
  }

}

// SAFETY:
// - `PathConfigSourceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PathConfigSourceView<'_> {}

// SAFETY:
// - `PathConfigSourceView` is `Send` because while its alive a `PathConfigSourceMut` cannot.
// - `PathConfigSourceView` does not use thread-local data.
unsafe impl ::std::marker::Send for PathConfigSourceView<'_> {}

impl<'msg> ::protobuf::AsView for PathConfigSourceView<'msg> {
  type Proxied = PathConfigSource;
  fn as_view(&self) -> ::protobuf::View<'msg, PathConfigSource> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathConfigSourceView<'msg> {
  fn into_view<'shorter>(self) -> PathConfigSourceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PathConfigSource> for PathConfigSourceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathConfigSource {
    let mut dst = PathConfigSource::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PathConfigSource> for PathConfigSourceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathConfigSource {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PathConfigSource {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathConfigSourceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathConfigSourceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PathConfigSourceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathConfigSourceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PathConfigSourceMut<'msg> {
  type Message = PathConfigSource;
}

impl ::std::fmt::Debug for PathConfigSourceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PathConfigSource>> for PathConfigSourceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathConfigSourceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PathConfigSource> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PathConfigSource {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_watched_directory(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn watched_directory_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(&self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView::default())
  }
  pub fn watched_directory_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryMut<'_> {
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
  pub fn set_watched_directory(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectory>) {

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
// - `PathConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PathConfigSourceMut<'_> {}

// SAFETY:
// - `PathConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PathConfigSourceMut<'_> {}

impl<'msg> ::protobuf::AsView for PathConfigSourceMut<'msg> {
  type Proxied = PathConfigSource;
  fn as_view(&self) -> ::protobuf::View<'_, PathConfigSource> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathConfigSourceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PathConfigSource>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PathConfigSourceMut<'msg> {
  type MutProxied = PathConfigSource;
  fn as_mut(&mut self) -> PathConfigSourceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PathConfigSourceMut<'msg> {
  fn into_mut<'shorter>(self) -> PathConfigSourceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PathConfigSource {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PathConfigSource> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PathConfigSourceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PathConfigSourceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_watched_directory(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn watched_directory_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(&self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView::default())
  }
  pub fn watched_directory_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryMut<'_> {
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
  pub fn set_watched_directory(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectory>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl PathConfigSource

impl ::std::ops::Drop for PathConfigSource {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PathConfigSource {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PathConfigSource {
  type Proxied = Self;
  fn as_view(&self) -> PathConfigSourceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PathConfigSource {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PathConfigSourceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PathConfigSource {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__PathConfigSource_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__PathConfigSource_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectory as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__PathConfigSource_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathConfigSource {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathConfigSource {
  type Msg = PathConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathConfigSource {
  type Msg = PathConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathConfigSourceMut<'_> {
  type Msg = PathConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathConfigSourceMut<'_> {
  type Msg = PathConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathConfigSourceView<'_> {
  type Msg = PathConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathConfigSource> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathConfigSourceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__ConfigSource_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ConfigSource {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ConfigSource>
}

impl ::protobuf::Message for ConfigSource {
  type MessageView<'msg> = ConfigSourceView<'msg>;
  type MessageMut<'msg> = ConfigSourceMut<'msg>;
}

impl ::std::default::Default for ConfigSource {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ConfigSource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ConfigSource` is `Sync` because it does not implement interior mutability.
//    Neither does `ConfigSourceMut`.
unsafe impl ::std::marker::Sync for ConfigSource {}

// SAFETY:
// - `ConfigSource` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ConfigSource {}

impl ::protobuf::Proxied for ConfigSource {
  type View<'msg> = ConfigSourceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ConfigSource {}

impl ::protobuf::MutProxied for ConfigSource {
  type Mut<'msg> = ConfigSourceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ConfigSourceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConfigSourceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ConfigSourceView<'msg> {
  type Message = ConfigSource;
}

impl ::std::fmt::Debug for ConfigSourceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ConfigSourceView<'_> {
  fn default() -> ConfigSourceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ConfigSource>> for ConfigSourceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConfigSourceView<'msg> {

  pub fn to_owned(&self) -> ConfigSource {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // authorities: repeated message xds.core.v3.Authority
  pub fn authorities(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::xds::core::v3::authority::Authority> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::xds::core::v3::authority::Authority>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // path: optional string
  pub fn has_path(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn path_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_path().then(|| self.path())
  }
  pub fn path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // path_config_source: optional message envoy.config.core.v3.PathConfigSource
  pub fn has_path_config_source(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn path_config_source_opt(self) -> ::std::option::Option<super::PathConfigSourceView<'msg>> {
    self.has_path_config_source().then(|| self.path_config_source())
  }
  pub fn path_config_source(self) -> super::PathConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PathConfigSourceView::default())
  }

  // api_config_source: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_api_config_source(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn api_config_source_opt(self) -> ::std::option::Option<super::ApiConfigSourceView<'msg>> {
    self.has_api_config_source().then(|| self.api_config_source())
  }
  pub fn api_config_source(self) -> super::ApiConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiConfigSourceView::default())
  }

  // ads: optional message envoy.config.core.v3.AggregatedConfigSource
  pub fn has_ads(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn ads_opt(self) -> ::std::option::Option<super::AggregatedConfigSourceView<'msg>> {
    self.has_ads().then(|| self.ads())
  }
  pub fn ads(self) -> super::AggregatedConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AggregatedConfigSourceView::default())
  }

  // self: optional message envoy.config.core.v3.SelfConfigSource
  pub fn has_self(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn self_opt(self) -> ::std::option::Option<super::SelfConfigSourceView<'msg>> {
    self.has_self().then(|| self.self_())
  }
  pub fn self_(self) -> super::SelfConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SelfConfigSourceView::default())
  }

  // initial_fetch_timeout: optional message google.protobuf.Duration
  pub fn has_initial_fetch_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn initial_fetch_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_initial_fetch_timeout().then(|| self.initial_fetch_timeout())
  }
  pub fn initial_fetch_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // resource_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn resource_api_version(self) -> super::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }

  pub fn config_source_specifier(self) -> super::config_source::ConfigSourceSpecifierOneof<'msg> {
    match self.config_source_specifier_case() {
      super::config_source::ConfigSourceSpecifierCase::Path =>
          super::config_source::ConfigSourceSpecifierOneof::Path(self.path()),
      super::config_source::ConfigSourceSpecifierCase::PathConfigSource =>
          super::config_source::ConfigSourceSpecifierOneof::PathConfigSource(self.path_config_source()),
      super::config_source::ConfigSourceSpecifierCase::ApiConfigSource =>
          super::config_source::ConfigSourceSpecifierOneof::ApiConfigSource(self.api_config_source()),
      super::config_source::ConfigSourceSpecifierCase::Ads =>
          super::config_source::ConfigSourceSpecifierOneof::Ads(self.ads()),
      super::config_source::ConfigSourceSpecifierCase::Self_ =>
          super::config_source::ConfigSourceSpecifierOneof::Self_(self.self_()),
      _ => super::config_source::ConfigSourceSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_source_specifier_case(self) -> super::config_source::ConfigSourceSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::config_source::ConfigSourceSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ConfigSourceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ConfigSourceView<'_> {}

// SAFETY:
// - `ConfigSourceView` is `Send` because while its alive a `ConfigSourceMut` cannot.
// - `ConfigSourceView` does not use thread-local data.
unsafe impl ::std::marker::Send for ConfigSourceView<'_> {}

impl<'msg> ::protobuf::AsView for ConfigSourceView<'msg> {
  type Proxied = ConfigSource;
  fn as_view(&self) -> ::protobuf::View<'msg, ConfigSource> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConfigSourceView<'msg> {
  fn into_view<'shorter>(self) -> ConfigSourceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ConfigSource> for ConfigSourceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConfigSource {
    let mut dst = ConfigSource::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ConfigSource> for ConfigSourceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConfigSource {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ConfigSource {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConfigSourceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConfigSourceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ConfigSourceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConfigSourceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ConfigSourceMut<'msg> {
  type Message = ConfigSource;
}

impl ::std::fmt::Debug for ConfigSourceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ConfigSource>> for ConfigSourceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConfigSourceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ConfigSource> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ConfigSource {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // authorities: repeated message xds.core.v3.Authority
  pub fn authorities(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::xds::core::v3::authority::Authority> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::xds::core::v3::authority::Authority>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn authorities_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::xds::core::v3::authority::Authority> {
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
  pub fn set_authorities(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::xds::core::v3::authority::Authority>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // path: optional string
  pub fn has_path(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_path(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn path_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_path().then(|| self.path())
  }
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // path_config_source: optional message envoy.config.core.v3.PathConfigSource
  pub fn has_path_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_path_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn path_config_source_opt(&self) -> ::std::option::Option<super::PathConfigSourceView<'_>> {
    self.has_path_config_source().then(|| self.path_config_source())
  }
  pub fn path_config_source(&self) -> super::PathConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PathConfigSourceView::default())
  }
  pub fn path_config_source_mut(&mut self) -> super::PathConfigSourceMut<'_> {
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
  pub fn set_path_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<super::PathConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // api_config_source: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_api_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_api_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn api_config_source_opt(&self) -> ::std::option::Option<super::ApiConfigSourceView<'_>> {
    self.has_api_config_source().then(|| self.api_config_source())
  }
  pub fn api_config_source(&self) -> super::ApiConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiConfigSourceView::default())
  }
  pub fn api_config_source_mut(&mut self) -> super::ApiConfigSourceMut<'_> {
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
  pub fn set_api_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // ads: optional message envoy.config.core.v3.AggregatedConfigSource
  pub fn has_ads(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_ads(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn ads_opt(&self) -> ::std::option::Option<super::AggregatedConfigSourceView<'_>> {
    self.has_ads().then(|| self.ads())
  }
  pub fn ads(&self) -> super::AggregatedConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AggregatedConfigSourceView::default())
  }
  pub fn ads_mut(&mut self) -> super::AggregatedConfigSourceMut<'_> {
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
  pub fn set_ads(&mut self,
    val: impl ::protobuf::IntoProxied<super::AggregatedConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // self: optional message envoy.config.core.v3.SelfConfigSource
  pub fn has_self(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_self(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn self_opt(&self) -> ::std::option::Option<super::SelfConfigSourceView<'_>> {
    self.has_self().then(|| self.self_())
  }
  pub fn self_(&self) -> super::SelfConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SelfConfigSourceView::default())
  }
  pub fn self_mut(&mut self) -> super::SelfConfigSourceMut<'_> {
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
  pub fn set_self(&mut self,
    val: impl ::protobuf::IntoProxied<super::SelfConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // initial_fetch_timeout: optional message google.protobuf.Duration
  pub fn has_initial_fetch_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_initial_fetch_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn initial_fetch_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_initial_fetch_timeout().then(|| self.initial_fetch_timeout())
  }
  pub fn initial_fetch_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn initial_fetch_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_initial_fetch_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // resource_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn resource_api_version(&self) -> super::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_resource_api_version(&mut self, val: super::ApiVersion) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

  pub fn config_source_specifier(&self) -> super::config_source::ConfigSourceSpecifierOneof<'_> {
    match &self.config_source_specifier_case() {
      super::config_source::ConfigSourceSpecifierCase::Path =>
          super::config_source::ConfigSourceSpecifierOneof::Path(self.path()),
      super::config_source::ConfigSourceSpecifierCase::PathConfigSource =>
          super::config_source::ConfigSourceSpecifierOneof::PathConfigSource(self.path_config_source()),
      super::config_source::ConfigSourceSpecifierCase::ApiConfigSource =>
          super::config_source::ConfigSourceSpecifierOneof::ApiConfigSource(self.api_config_source()),
      super::config_source::ConfigSourceSpecifierCase::Ads =>
          super::config_source::ConfigSourceSpecifierOneof::Ads(self.ads()),
      super::config_source::ConfigSourceSpecifierCase::Self_ =>
          super::config_source::ConfigSourceSpecifierOneof::Self_(self.self_()),
      _ => super::config_source::ConfigSourceSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_source_specifier_case(&self) -> super::config_source::ConfigSourceSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::config_source::ConfigSourceSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ConfigSourceMut<'_> {}

// SAFETY:
// - `ConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ConfigSourceMut<'_> {}

impl<'msg> ::protobuf::AsView for ConfigSourceMut<'msg> {
  type Proxied = ConfigSource;
  fn as_view(&self) -> ::protobuf::View<'_, ConfigSource> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConfigSourceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ConfigSource>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ConfigSourceMut<'msg> {
  type MutProxied = ConfigSource;
  fn as_mut(&mut self) -> ConfigSourceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ConfigSourceMut<'msg> {
  fn into_mut<'shorter>(self) -> ConfigSourceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ConfigSource {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ConfigSource> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ConfigSourceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ConfigSourceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // authorities: repeated message xds.core.v3.Authority
  pub fn authorities(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::xds::core::v3::authority::Authority> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::xds::core::v3::authority::Authority>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn authorities_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::xds::core::v3::authority::Authority> {
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
  pub fn set_authorities(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::xds::core::v3::authority::Authority>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // path: optional string
  pub fn has_path(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_path(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn path_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_path().then(|| self.path())
  }
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // path_config_source: optional message envoy.config.core.v3.PathConfigSource
  pub fn has_path_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_path_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn path_config_source_opt(&self) -> ::std::option::Option<super::PathConfigSourceView<'_>> {
    self.has_path_config_source().then(|| self.path_config_source())
  }
  pub fn path_config_source(&self) -> super::PathConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PathConfigSourceView::default())
  }
  pub fn path_config_source_mut(&mut self) -> super::PathConfigSourceMut<'_> {
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
  pub fn set_path_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<super::PathConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // api_config_source: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_api_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_api_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn api_config_source_opt(&self) -> ::std::option::Option<super::ApiConfigSourceView<'_>> {
    self.has_api_config_source().then(|| self.api_config_source())
  }
  pub fn api_config_source(&self) -> super::ApiConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ApiConfigSourceView::default())
  }
  pub fn api_config_source_mut(&mut self) -> super::ApiConfigSourceMut<'_> {
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
  pub fn set_api_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<super::ApiConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // ads: optional message envoy.config.core.v3.AggregatedConfigSource
  pub fn has_ads(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_ads(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn ads_opt(&self) -> ::std::option::Option<super::AggregatedConfigSourceView<'_>> {
    self.has_ads().then(|| self.ads())
  }
  pub fn ads(&self) -> super::AggregatedConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AggregatedConfigSourceView::default())
  }
  pub fn ads_mut(&mut self) -> super::AggregatedConfigSourceMut<'_> {
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
  pub fn set_ads(&mut self,
    val: impl ::protobuf::IntoProxied<super::AggregatedConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // self: optional message envoy.config.core.v3.SelfConfigSource
  pub fn has_self(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_self(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn self_opt(&self) -> ::std::option::Option<super::SelfConfigSourceView<'_>> {
    self.has_self().then(|| self.self_())
  }
  pub fn self_(&self) -> super::SelfConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SelfConfigSourceView::default())
  }
  pub fn self_mut(&mut self) -> super::SelfConfigSourceMut<'_> {
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
  pub fn set_self(&mut self,
    val: impl ::protobuf::IntoProxied<super::SelfConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // initial_fetch_timeout: optional message google.protobuf.Duration
  pub fn has_initial_fetch_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_initial_fetch_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn initial_fetch_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_initial_fetch_timeout().then(|| self.initial_fetch_timeout())
  }
  pub fn initial_fetch_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn initial_fetch_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_initial_fetch_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // resource_api_version: optional enum envoy.config.core.v3.ApiVersion
  pub fn resource_api_version(&self) -> super::ApiVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::ApiVersion::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_resource_api_version(&mut self, val: super::ApiVersion) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

  pub fn config_source_specifier(&self) -> super::config_source::ConfigSourceSpecifierOneof<'_> {
    match &self.config_source_specifier_case() {
      super::config_source::ConfigSourceSpecifierCase::Path =>
          super::config_source::ConfigSourceSpecifierOneof::Path(self.path()),
      super::config_source::ConfigSourceSpecifierCase::PathConfigSource =>
          super::config_source::ConfigSourceSpecifierOneof::PathConfigSource(self.path_config_source()),
      super::config_source::ConfigSourceSpecifierCase::ApiConfigSource =>
          super::config_source::ConfigSourceSpecifierOneof::ApiConfigSource(self.api_config_source()),
      super::config_source::ConfigSourceSpecifierCase::Ads =>
          super::config_source::ConfigSourceSpecifierOneof::Ads(self.ads()),
      super::config_source::ConfigSourceSpecifierCase::Self_ =>
          super::config_source::ConfigSourceSpecifierOneof::Self_(self.self_()),
      _ => super::config_source::ConfigSourceSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_source_specifier_case(&self) -> super::config_source::ConfigSourceSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::config_source::ConfigSourceSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ConfigSource

impl ::std::ops::Drop for ConfigSource {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ConfigSource {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ConfigSource {
  type Proxied = Self;
  fn as_view(&self) -> ConfigSourceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ConfigSource {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ConfigSourceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ConfigSource {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__ConfigSource_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T3333.PG3^!|*|#|$|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__ConfigSource_msg_init.0, &[<super::ApiConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::AggregatedConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SelfConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::xds::core::v3::authority::Authority as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::PathConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__ConfigSource_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConfigSource {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConfigSource {
  type Msg = ConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConfigSource {
  type Msg = ConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConfigSourceMut<'_> {
  type Msg = ConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConfigSourceMut<'_> {
  type Msg = ConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConfigSourceView<'_> {
  type Msg = ConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConfigSource> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConfigSourceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod config_source {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigSourceSpecifierOneof<'msg> {
  Path(&'msg ::protobuf::ProtoStr) = 1,
  PathConfigSource(::protobuf::View<'msg, super::super::PathConfigSource>) = 8,
  ApiConfigSource(::protobuf::View<'msg, super::super::ApiConfigSource>) = 2,
  Ads(::protobuf::View<'msg, super::super::AggregatedConfigSource>) = 3,
  Self_(::protobuf::View<'msg, super::super::SelfConfigSource>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigSourceSpecifierCase {
  Path = 1,
  PathConfigSource = 8,
  ApiConfigSource = 2,
  Ads = 3,
  Self_ = 5,

  not_set = 0
}

impl ConfigSourceSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigSourceSpecifierCase> {
    match v {
      0 => Some(ConfigSourceSpecifierCase::not_set),
      1 => Some(ConfigSourceSpecifierCase::Path),
      8 => Some(ConfigSourceSpecifierCase::PathConfigSource),
      2 => Some(ConfigSourceSpecifierCase::ApiConfigSource),
      3 => Some(ConfigSourceSpecifierCase::Ads),
      5 => Some(ConfigSourceSpecifierCase::Self_),
      _ => None
    }
  }
}
}  // pub mod config_source


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__ExtensionConfigSource_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExtensionConfigSource {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExtensionConfigSource>
}

impl ::protobuf::Message for ExtensionConfigSource {
  type MessageView<'msg> = ExtensionConfigSourceView<'msg>;
  type MessageMut<'msg> = ExtensionConfigSourceMut<'msg>;
}

impl ::std::default::Default for ExtensionConfigSource {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExtensionConfigSource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExtensionConfigSource` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtensionConfigSourceMut`.
unsafe impl ::std::marker::Sync for ExtensionConfigSource {}

// SAFETY:
// - `ExtensionConfigSource` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExtensionConfigSource {}

impl ::protobuf::Proxied for ExtensionConfigSource {
  type View<'msg> = ExtensionConfigSourceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExtensionConfigSource {}

impl ::protobuf::MutProxied for ExtensionConfigSource {
  type Mut<'msg> = ExtensionConfigSourceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtensionConfigSourceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionConfigSourceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtensionConfigSourceView<'msg> {
  type Message = ExtensionConfigSource;
}

impl ::std::fmt::Debug for ExtensionConfigSourceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtensionConfigSourceView<'_> {
  fn default() -> ExtensionConfigSourceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionConfigSource>> for ExtensionConfigSourceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionConfigSourceView<'msg> {

  pub fn to_owned(&self) -> ExtensionConfigSource {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_config_source(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn config_source_opt(self) -> ::std::option::Option<super::ConfigSourceView<'msg>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(self) -> super::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConfigSourceView::default())
  }

  // default_config: optional message google.protobuf.Any
  pub fn has_default_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn default_config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_default_config().then(|| self.default_config())
  }
  pub fn default_config(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // apply_default_config_without_warming: optional bool
  pub fn apply_default_config_without_warming(self) -> bool {
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

  // type_urls: repeated string
  pub fn type_urls(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

}

// SAFETY:
// - `ExtensionConfigSourceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtensionConfigSourceView<'_> {}

// SAFETY:
// - `ExtensionConfigSourceView` is `Send` because while its alive a `ExtensionConfigSourceMut` cannot.
// - `ExtensionConfigSourceView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtensionConfigSourceView<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionConfigSourceView<'msg> {
  type Proxied = ExtensionConfigSource;
  fn as_view(&self) -> ::protobuf::View<'msg, ExtensionConfigSource> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionConfigSourceView<'msg> {
  fn into_view<'shorter>(self) -> ExtensionConfigSourceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtensionConfigSource> for ExtensionConfigSourceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtensionConfigSource {
    let mut dst = ExtensionConfigSource::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtensionConfigSource> for ExtensionConfigSourceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtensionConfigSource {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExtensionConfigSource {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionConfigSourceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionConfigSourceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtensionConfigSourceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionConfigSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionConfigSourceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtensionConfigSourceMut<'msg> {
  type Message = ExtensionConfigSource;
}

impl ::std::fmt::Debug for ExtensionConfigSourceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionConfigSource>> for ExtensionConfigSourceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionConfigSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionConfigSourceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionConfigSource> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExtensionConfigSource {
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
  pub fn config_source_opt(&self) -> ::std::option::Option<super::ConfigSourceView<'_>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(&self) -> super::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConfigSourceView::default())
  }
  pub fn config_source_mut(&mut self) -> super::ConfigSourceMut<'_> {
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
    val: impl ::protobuf::IntoProxied<super::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // default_config: optional message google.protobuf.Any
  pub fn has_default_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_default_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn default_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_default_config().then(|| self.default_config())
  }
  pub fn default_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn default_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_default_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // apply_default_config_without_warming: optional bool
  pub fn apply_default_config_without_warming(&self) -> bool {
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
  pub fn set_apply_default_config_without_warming(&mut self, val: bool) {
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

  // type_urls: repeated string
  pub fn type_urls(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn type_urls_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_type_urls(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `ExtensionConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtensionConfigSourceMut<'_> {}

// SAFETY:
// - `ExtensionConfigSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtensionConfigSourceMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionConfigSourceMut<'msg> {
  type Proxied = ExtensionConfigSource;
  fn as_view(&self) -> ::protobuf::View<'_, ExtensionConfigSource> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionConfigSourceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExtensionConfigSource>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtensionConfigSourceMut<'msg> {
  type MutProxied = ExtensionConfigSource;
  fn as_mut(&mut self) -> ExtensionConfigSourceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtensionConfigSourceMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtensionConfigSourceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExtensionConfigSource {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExtensionConfigSource> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtensionConfigSourceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtensionConfigSourceMut<'_> {
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
  pub fn config_source_opt(&self) -> ::std::option::Option<super::ConfigSourceView<'_>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(&self) -> super::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConfigSourceView::default())
  }
  pub fn config_source_mut(&mut self) -> super::ConfigSourceMut<'_> {
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
    val: impl ::protobuf::IntoProxied<super::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // default_config: optional message google.protobuf.Any
  pub fn has_default_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_default_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn default_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_default_config().then(|| self.default_config())
  }
  pub fn default_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn default_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_default_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // apply_default_config_without_warming: optional bool
  pub fn apply_default_config_without_warming(&self) -> bool {
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
  pub fn set_apply_default_config_without_warming(&mut self, val: bool) {
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

  // type_urls: repeated string
  pub fn type_urls(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn type_urls_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_type_urls(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl ExtensionConfigSource

impl ::std::ops::Drop for ExtensionConfigSource {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExtensionConfigSource {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExtensionConfigSource {
  type Proxied = Self;
  fn as_view(&self) -> ExtensionConfigSourceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExtensionConfigSource {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtensionConfigSourceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExtensionConfigSource {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__ExtensionConfigSource_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33/PET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__ExtensionConfigSource_msg_init.0, &[<super::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__ExtensionConfigSource_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtensionConfigSource {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtensionConfigSource {
  type Msg = ExtensionConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionConfigSource {
  type Msg = ExtensionConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtensionConfigSourceMut<'_> {
  type Msg = ExtensionConfigSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionConfigSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionConfigSourceMut<'_> {
  type Msg = ExtensionConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionConfigSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionConfigSourceView<'_> {
  type Msg = ExtensionConfigSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionConfigSource> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtensionConfigSourceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApiVersion(i32);

#[allow(non_upper_case_globals)]
impl ApiVersion {
  pub const Auto: ApiVersion = ApiVersion(0);
  pub const V2: ApiVersion = ApiVersion(1);
  pub const V3: ApiVersion = ApiVersion(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Auto",
      1 => "V2",
      2 => "V3",
      _ => return None
    })
  }
}

impl ::std::convert::From<ApiVersion> for i32 {
  fn from(val: ApiVersion) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ApiVersion {
  fn from(val: i32) -> ApiVersion {
    Self(val)
  }
}

impl ::std::default::Default for ApiVersion {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ApiVersion {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ApiVersion::{}", constant_name)
    } else {
      write!(f, "ApiVersion::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ApiVersion {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ApiVersion {}

impl ::protobuf::Proxied for ApiVersion {
  type View<'a> = ApiVersion;
}

impl ::protobuf::AsView for ApiVersion {
  type Proxied = ApiVersion;

  fn as_view(&self) -> ApiVersion {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApiVersion {
  fn into_view<'shorter>(self) -> ApiVersion where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ApiVersion {
  const NAME: &'static str = "ApiVersion";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for ApiVersion {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


