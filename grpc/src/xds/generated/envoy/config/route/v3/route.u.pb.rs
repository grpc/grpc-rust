const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__route__v3__RouteConfiguration_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RouteConfiguration {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RouteConfiguration>
}

impl ::protobuf::Message for RouteConfiguration {
  type MessageView<'msg> = RouteConfigurationView<'msg>;
  type MessageMut<'msg> = RouteConfigurationMut<'msg>;
}

impl ::std::default::Default for RouteConfiguration {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RouteConfiguration {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RouteConfiguration` is `Sync` because it does not implement interior mutability.
//    Neither does `RouteConfigurationMut`.
unsafe impl ::std::marker::Sync for RouteConfiguration {}

// SAFETY:
// - `RouteConfiguration` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RouteConfiguration {}

impl ::protobuf::Proxied for RouteConfiguration {
  type View<'msg> = RouteConfigurationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RouteConfiguration {}

impl ::protobuf::MutProxied for RouteConfiguration {
  type Mut<'msg> = RouteConfigurationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RouteConfigurationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RouteConfiguration>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RouteConfigurationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RouteConfigurationView<'msg> {
  type Message = RouteConfiguration;
}

impl ::std::fmt::Debug for RouteConfigurationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RouteConfigurationView<'_> {
  fn default() -> RouteConfigurationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RouteConfiguration>> for RouteConfigurationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RouteConfiguration>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RouteConfigurationView<'msg> {

  pub fn to_owned(&self) -> RouteConfiguration {
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

  // virtual_hosts: repeated message envoy.config.route.v3.VirtualHost
  pub fn virtual_hosts(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::route::v3::route_components::VirtualHost> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::VirtualHost>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // vhds: optional message envoy.config.route.v3.Vhds
  pub fn has_vhds(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn vhds_opt(self) -> ::std::option::Option<super::VhdsView<'msg>> {
    self.has_vhds().then(|| self.vhds())
  }
  pub fn vhds(self) -> super::VhdsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VhdsView::default())
  }

  // internal_only_headers: repeated string
  pub fn internal_only_headers(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // response_headers_to_remove: repeated string
  pub fn response_headers_to_remove(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // request_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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

  // request_headers_to_remove: repeated string
  pub fn request_headers_to_remove(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // most_specific_header_mutations_wins: optional bool
  pub fn most_specific_header_mutations_wins(self) -> bool {
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

  // validate_clusters: optional message google.protobuf.BoolValue
  pub fn has_validate_clusters(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn validate_clusters_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_validate_clusters().then(|| self.validate_clusters())
  }
  pub fn validate_clusters(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // max_direct_response_body_size_bytes: optional message google.protobuf.UInt32Value
  pub fn has_max_direct_response_body_size_bytes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn max_direct_response_body_size_bytes_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_direct_response_body_size_bytes().then(|| self.max_direct_response_body_size_bytes())
  }
  pub fn max_direct_response_body_size_bytes(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // cluster_specifier_plugins: repeated message envoy.config.route.v3.ClusterSpecifierPlugin
  pub fn cluster_specifier_plugins(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::route::v3::route_components::ClusterSpecifierPlugin> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        11
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::ClusterSpecifierPlugin>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // request_mirror_policies: repeated message envoy.config.route.v3.RouteAction.RequestMirrorPolicy
  pub fn request_mirror_policies(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::route::v3::route_components::route_action::RequestMirrorPolicy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::route_action::RequestMirrorPolicy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_port_in_host_matching: optional bool
  pub fn ignore_port_in_host_matching(self) -> bool {
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

  // vhost_header: optional string
  pub fn vhost_header(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        17, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // ignore_path_parameters_in_path_matching: optional bool
  pub fn ignore_path_parameters_in_path_matching(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }

  // typed_per_filter_config: repeated message envoy.config.route.v3.RouteConfiguration.TypedPerFilterConfigEntry
  pub fn typed_per_filter_config(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(15)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }

}

// SAFETY:
// - `RouteConfigurationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RouteConfigurationView<'_> {}

// SAFETY:
// - `RouteConfigurationView` is `Send` because while its alive a `RouteConfigurationMut` cannot.
// - `RouteConfigurationView` does not use thread-local data.
unsafe impl ::std::marker::Send for RouteConfigurationView<'_> {}

impl<'msg> ::protobuf::AsView for RouteConfigurationView<'msg> {
  type Proxied = RouteConfiguration;
  fn as_view(&self) -> ::protobuf::View<'msg, RouteConfiguration> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RouteConfigurationView<'msg> {
  fn into_view<'shorter>(self) -> RouteConfigurationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RouteConfiguration> for RouteConfigurationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RouteConfiguration {
    let mut dst = RouteConfiguration::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RouteConfiguration> for RouteConfigurationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RouteConfiguration {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RouteConfiguration {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RouteConfigurationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RouteConfigurationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RouteConfigurationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RouteConfiguration>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RouteConfigurationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RouteConfigurationMut<'msg> {
  type Message = RouteConfiguration;
}

impl ::std::fmt::Debug for RouteConfigurationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RouteConfiguration>> for RouteConfigurationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RouteConfiguration>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RouteConfigurationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RouteConfiguration> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RouteConfiguration {
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

  // virtual_hosts: repeated message envoy.config.route.v3.VirtualHost
  pub fn virtual_hosts(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::VirtualHost> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::VirtualHost>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn virtual_hosts_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::VirtualHost> {
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
  pub fn set_virtual_hosts(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::VirtualHost>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // vhds: optional message envoy.config.route.v3.Vhds
  pub fn has_vhds(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_vhds(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn vhds_opt(&self) -> ::std::option::Option<super::VhdsView<'_>> {
    self.has_vhds().then(|| self.vhds())
  }
  pub fn vhds(&self) -> super::VhdsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VhdsView::default())
  }
  pub fn vhds_mut(&mut self) -> super::VhdsMut<'_> {
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
  pub fn set_vhds(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vhds>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // internal_only_headers: repeated string
  pub fn internal_only_headers(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn internal_only_headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_internal_only_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
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

  // response_headers_to_remove: repeated string
  pub fn response_headers_to_remove(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn response_headers_to_remove_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_response_headers_to_remove(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // request_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn request_headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_request_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // request_headers_to_remove: repeated string
  pub fn request_headers_to_remove(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_headers_to_remove_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_request_headers_to_remove(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // most_specific_header_mutations_wins: optional bool
  pub fn most_specific_header_mutations_wins(&self) -> bool {
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
  pub fn set_most_specific_header_mutations_wins(&mut self, val: bool) {
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

  // validate_clusters: optional message google.protobuf.BoolValue
  pub fn has_validate_clusters(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_validate_clusters(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn validate_clusters_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_validate_clusters().then(|| self.validate_clusters())
  }
  pub fn validate_clusters(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn validate_clusters_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_validate_clusters(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // max_direct_response_body_size_bytes: optional message google.protobuf.UInt32Value
  pub fn has_max_direct_response_body_size_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_max_direct_response_body_size_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn max_direct_response_body_size_bytes_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_direct_response_body_size_bytes().then(|| self.max_direct_response_body_size_bytes())
  }
  pub fn max_direct_response_body_size_bytes(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_direct_response_body_size_bytes_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_direct_response_body_size_bytes(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // cluster_specifier_plugins: repeated message envoy.config.route.v3.ClusterSpecifierPlugin
  pub fn cluster_specifier_plugins(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::ClusterSpecifierPlugin> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        11
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::ClusterSpecifierPlugin>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn cluster_specifier_plugins_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::ClusterSpecifierPlugin> {
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
  pub fn set_cluster_specifier_plugins(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::ClusterSpecifierPlugin>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        src);
    }
  }

  // request_mirror_policies: repeated message envoy.config.route.v3.RouteAction.RequestMirrorPolicy
  pub fn request_mirror_policies(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::route_action::RequestMirrorPolicy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::route_action::RequestMirrorPolicy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_mirror_policies_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::route_action::RequestMirrorPolicy> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        12,
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
  pub fn set_request_mirror_policies(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::route_action::RequestMirrorPolicy>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        src);
    }
  }

  // ignore_port_in_host_matching: optional bool
  pub fn ignore_port_in_host_matching(&self) -> bool {
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
  pub fn set_ignore_port_in_host_matching(&mut self, val: bool) {
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

  // vhost_header: optional string
  pub fn vhost_header(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        17, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_vhost_header(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val);
    }
  }

  // ignore_path_parameters_in_path_matching: optional bool
  pub fn ignore_path_parameters_in_path_matching(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_path_parameters_in_path_matching(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        14, val.into()
      )
    }
  }

  // typed_per_filter_config: repeated message envoy.config.route.v3.RouteConfiguration.TypedPerFilterConfigEntry
  pub fn typed_per_filter_config(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(15)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn typed_per_filter_config_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          15, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_typed_per_filter_config(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        src);
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

}

// SAFETY:
// - `RouteConfigurationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RouteConfigurationMut<'_> {}

// SAFETY:
// - `RouteConfigurationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RouteConfigurationMut<'_> {}

impl<'msg> ::protobuf::AsView for RouteConfigurationMut<'msg> {
  type Proxied = RouteConfiguration;
  fn as_view(&self) -> ::protobuf::View<'_, RouteConfiguration> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RouteConfigurationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RouteConfiguration>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RouteConfigurationMut<'msg> {
  type MutProxied = RouteConfiguration;
  fn as_mut(&mut self) -> RouteConfigurationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RouteConfigurationMut<'msg> {
  fn into_mut<'shorter>(self) -> RouteConfigurationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RouteConfiguration {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RouteConfiguration> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RouteConfigurationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RouteConfigurationMut<'_> {
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

  // virtual_hosts: repeated message envoy.config.route.v3.VirtualHost
  pub fn virtual_hosts(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::VirtualHost> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::VirtualHost>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn virtual_hosts_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::VirtualHost> {
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
  pub fn set_virtual_hosts(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::VirtualHost>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // vhds: optional message envoy.config.route.v3.Vhds
  pub fn has_vhds(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_vhds(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn vhds_opt(&self) -> ::std::option::Option<super::VhdsView<'_>> {
    self.has_vhds().then(|| self.vhds())
  }
  pub fn vhds(&self) -> super::VhdsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VhdsView::default())
  }
  pub fn vhds_mut(&mut self) -> super::VhdsMut<'_> {
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
  pub fn set_vhds(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vhds>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // internal_only_headers: repeated string
  pub fn internal_only_headers(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn internal_only_headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_internal_only_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
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

  // response_headers_to_remove: repeated string
  pub fn response_headers_to_remove(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn response_headers_to_remove_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_response_headers_to_remove(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // request_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn request_headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_request_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // request_headers_to_remove: repeated string
  pub fn request_headers_to_remove(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_headers_to_remove_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_request_headers_to_remove(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // most_specific_header_mutations_wins: optional bool
  pub fn most_specific_header_mutations_wins(&self) -> bool {
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
  pub fn set_most_specific_header_mutations_wins(&mut self, val: bool) {
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

  // validate_clusters: optional message google.protobuf.BoolValue
  pub fn has_validate_clusters(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_validate_clusters(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn validate_clusters_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_validate_clusters().then(|| self.validate_clusters())
  }
  pub fn validate_clusters(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn validate_clusters_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_validate_clusters(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // max_direct_response_body_size_bytes: optional message google.protobuf.UInt32Value
  pub fn has_max_direct_response_body_size_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_max_direct_response_body_size_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn max_direct_response_body_size_bytes_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_direct_response_body_size_bytes().then(|| self.max_direct_response_body_size_bytes())
  }
  pub fn max_direct_response_body_size_bytes(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_direct_response_body_size_bytes_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_direct_response_body_size_bytes(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // cluster_specifier_plugins: repeated message envoy.config.route.v3.ClusterSpecifierPlugin
  pub fn cluster_specifier_plugins(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::ClusterSpecifierPlugin> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        11
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::ClusterSpecifierPlugin>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn cluster_specifier_plugins_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::ClusterSpecifierPlugin> {
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
  pub fn set_cluster_specifier_plugins(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::ClusterSpecifierPlugin>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        src);
    }
  }

  // request_mirror_policies: repeated message envoy.config.route.v3.RouteAction.RequestMirrorPolicy
  pub fn request_mirror_policies(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::route_action::RequestMirrorPolicy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::route_action::RequestMirrorPolicy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_mirror_policies_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::route_action::RequestMirrorPolicy> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        12,
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
  pub fn set_request_mirror_policies(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::route_action::RequestMirrorPolicy>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        src);
    }
  }

  // ignore_port_in_host_matching: optional bool
  pub fn ignore_port_in_host_matching(&self) -> bool {
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
  pub fn set_ignore_port_in_host_matching(&mut self, val: bool) {
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

  // vhost_header: optional string
  pub fn vhost_header(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        17, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_vhost_header(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val);
    }
  }

  // ignore_path_parameters_in_path_matching: optional bool
  pub fn ignore_path_parameters_in_path_matching(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_path_parameters_in_path_matching(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        14, val.into()
      )
    }
  }

  // typed_per_filter_config: repeated message envoy.config.route.v3.RouteConfiguration.TypedPerFilterConfigEntry
  pub fn typed_per_filter_config(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(15)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn typed_per_filter_config_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          15, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_typed_per_filter_config(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        src);
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

}  // impl RouteConfiguration

impl ::std::ops::Drop for RouteConfiguration {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RouteConfiguration {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RouteConfiguration {
  type Proxied = Self;
  fn as_view(&self) -> RouteConfigurationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RouteConfiguration {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RouteConfigurationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RouteConfiguration {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__route__v3__RouteConfiguration_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XGETGETG3ET3/P3GG/P/PG31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__route__v3__RouteConfiguration_msg_init.0, &[<crate::xds::generated::envoy::config::route::v3::route_components::VirtualHost as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Vhds as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::route::v3::route_components::ClusterSpecifierPlugin as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::route::v3::route_components::route_action::RequestMirrorPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::route_configuration::TypedPerFilterConfigEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__route__v3__RouteConfiguration_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RouteConfiguration {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RouteConfiguration {
  type Msg = RouteConfiguration;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RouteConfiguration> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouteConfiguration {
  type Msg = RouteConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RouteConfiguration> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RouteConfigurationMut<'_> {
  type Msg = RouteConfiguration;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RouteConfiguration> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouteConfigurationMut<'_> {
  type Msg = RouteConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RouteConfiguration> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouteConfigurationView<'_> {
  type Msg = RouteConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RouteConfiguration> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RouteConfigurationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod route_configuration {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__route__v3__RouteConfiguration__TypedPerFilterConfigEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct TypedPerFilterConfigEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TypedPerFilterConfigEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::route_configuration::envoy__config__route__v3__RouteConfiguration__TypedPerFilterConfigEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::route_configuration::envoy__config__route__v3__RouteConfiguration__TypedPerFilterConfigEntry_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::route_configuration::envoy__config__route__v3__RouteConfiguration__TypedPerFilterConfigEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod route_configuration


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__route__v3__Vhds_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Vhds {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Vhds>
}

impl ::protobuf::Message for Vhds {
  type MessageView<'msg> = VhdsView<'msg>;
  type MessageMut<'msg> = VhdsMut<'msg>;
}

impl ::std::default::Default for Vhds {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Vhds {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Vhds` is `Sync` because it does not implement interior mutability.
//    Neither does `VhdsMut`.
unsafe impl ::std::marker::Sync for Vhds {}

// SAFETY:
// - `Vhds` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Vhds {}

impl ::protobuf::Proxied for Vhds {
  type View<'msg> = VhdsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Vhds {}

impl ::protobuf::MutProxied for Vhds {
  type Mut<'msg> = VhdsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct VhdsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vhds>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VhdsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for VhdsView<'msg> {
  type Message = Vhds;
}

impl ::std::fmt::Debug for VhdsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for VhdsView<'_> {
  fn default() -> VhdsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Vhds>> for VhdsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vhds>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VhdsView<'msg> {

  pub fn to_owned(&self) -> Vhds {
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

}

// SAFETY:
// - `VhdsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for VhdsView<'_> {}

// SAFETY:
// - `VhdsView` is `Send` because while its alive a `VhdsMut` cannot.
// - `VhdsView` does not use thread-local data.
unsafe impl ::std::marker::Send for VhdsView<'_> {}

impl<'msg> ::protobuf::AsView for VhdsView<'msg> {
  type Proxied = Vhds;
  fn as_view(&self) -> ::protobuf::View<'msg, Vhds> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VhdsView<'msg> {
  fn into_view<'shorter>(self) -> VhdsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Vhds> for VhdsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vhds {
    let mut dst = Vhds::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Vhds> for VhdsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vhds {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Vhds {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VhdsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VhdsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct VhdsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vhds>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VhdsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for VhdsMut<'msg> {
  type Message = Vhds;
}

impl ::std::fmt::Debug for VhdsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Vhds>> for VhdsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vhds>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VhdsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Vhds> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Vhds {
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

}

// SAFETY:
// - `VhdsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for VhdsMut<'_> {}

// SAFETY:
// - `VhdsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for VhdsMut<'_> {}

impl<'msg> ::protobuf::AsView for VhdsMut<'msg> {
  type Proxied = Vhds;
  fn as_view(&self) -> ::protobuf::View<'_, Vhds> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VhdsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Vhds>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for VhdsMut<'msg> {
  type MutProxied = Vhds;
  fn as_mut(&mut self) -> VhdsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for VhdsMut<'msg> {
  fn into_mut<'shorter>(self) -> VhdsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Vhds {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Vhds> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> VhdsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> VhdsMut<'_> {
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

}  // impl Vhds

impl ::std::ops::Drop for Vhds {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Vhds {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Vhds {
  type Proxied = Self;
  fn as_view(&self) -> VhdsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Vhds {
  type MutProxied = Self;
  fn as_mut(&mut self) -> VhdsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Vhds {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__route__v3__Vhds_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__route__v3__Vhds_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__route__v3__Vhds_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Vhds {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Vhds {
  type Msg = Vhds;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vhds> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vhds {
  type Msg = Vhds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vhds> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for VhdsMut<'_> {
  type Msg = Vhds;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vhds> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VhdsMut<'_> {
  type Msg = Vhds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vhds> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VhdsView<'_> {
  type Msg = Vhds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vhds> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for VhdsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



