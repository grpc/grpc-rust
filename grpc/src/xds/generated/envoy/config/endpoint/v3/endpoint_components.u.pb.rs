const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__Endpoint_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Endpoint {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Endpoint>
}

impl ::protobuf::Message for Endpoint {
  type MessageView<'msg> = EndpointView<'msg>;
  type MessageMut<'msg> = EndpointMut<'msg>;
}

impl ::std::default::Default for Endpoint {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Endpoint {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Endpoint` is `Sync` because it does not implement interior mutability.
//    Neither does `EndpointMut`.
unsafe impl ::std::marker::Sync for Endpoint {}

// SAFETY:
// - `Endpoint` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Endpoint {}

impl ::protobuf::Proxied for Endpoint {
  type View<'msg> = EndpointView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Endpoint {}

impl ::protobuf::MutProxied for Endpoint {
  type Mut<'msg> = EndpointMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EndpointView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Endpoint>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EndpointView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EndpointView<'msg> {
  type Message = Endpoint;
}

impl ::std::fmt::Debug for EndpointView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EndpointView<'_> {
  fn default() -> EndpointView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Endpoint>> for EndpointView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Endpoint>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EndpointView<'msg> {

  pub fn to_owned(&self) -> Endpoint {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

  // health_check_config: optional message envoy.config.endpoint.v3.Endpoint.HealthCheckConfig
  pub fn has_health_check_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn health_check_config_opt(self) -> ::std::option::Option<super::endpoint::HealthCheckConfigView<'msg>> {
    self.has_health_check_config().then(|| self.health_check_config())
  }
  pub fn health_check_config(self) -> super::endpoint::HealthCheckConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::endpoint::HealthCheckConfigView::default())
  }

  // hostname: optional string
  pub fn hostname(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // additional_addresses: repeated message envoy.config.endpoint.v3.Endpoint.AdditionalAddress
  pub fn additional_addresses(self) -> ::protobuf::RepeatedView<'msg, super::endpoint::AdditionalAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::endpoint::AdditionalAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `EndpointView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EndpointView<'_> {}

// SAFETY:
// - `EndpointView` is `Send` because while its alive a `EndpointMut` cannot.
// - `EndpointView` does not use thread-local data.
unsafe impl ::std::marker::Send for EndpointView<'_> {}

impl<'msg> ::protobuf::AsView for EndpointView<'msg> {
  type Proxied = Endpoint;
  fn as_view(&self) -> ::protobuf::View<'msg, Endpoint> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EndpointView<'msg> {
  fn into_view<'shorter>(self) -> EndpointView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Endpoint> for EndpointView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Endpoint {
    let mut dst = Endpoint::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Endpoint> for EndpointMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Endpoint {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Endpoint {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EndpointView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EndpointMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EndpointMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Endpoint>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EndpointMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EndpointMut<'msg> {
  type Message = Endpoint;
}

impl ::std::fmt::Debug for EndpointMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Endpoint>> for EndpointMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Endpoint>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EndpointMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Endpoint> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Endpoint {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // health_check_config: optional message envoy.config.endpoint.v3.Endpoint.HealthCheckConfig
  pub fn has_health_check_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_health_check_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn health_check_config_opt(&self) -> ::std::option::Option<super::endpoint::HealthCheckConfigView<'_>> {
    self.has_health_check_config().then(|| self.health_check_config())
  }
  pub fn health_check_config(&self) -> super::endpoint::HealthCheckConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::endpoint::HealthCheckConfigView::default())
  }
  pub fn health_check_config_mut(&mut self) -> super::endpoint::HealthCheckConfigMut<'_> {
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
  pub fn set_health_check_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::endpoint::HealthCheckConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // hostname: optional string
  pub fn hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // additional_addresses: repeated message envoy.config.endpoint.v3.Endpoint.AdditionalAddress
  pub fn additional_addresses(&self) -> ::protobuf::RepeatedView<'_, super::endpoint::AdditionalAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::endpoint::AdditionalAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn additional_addresses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::endpoint::AdditionalAddress> {
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
  pub fn set_additional_addresses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::endpoint::AdditionalAddress>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `EndpointMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EndpointMut<'_> {}

// SAFETY:
// - `EndpointMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EndpointMut<'_> {}

impl<'msg> ::protobuf::AsView for EndpointMut<'msg> {
  type Proxied = Endpoint;
  fn as_view(&self) -> ::protobuf::View<'_, Endpoint> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EndpointMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Endpoint>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EndpointMut<'msg> {
  type MutProxied = Endpoint;
  fn as_mut(&mut self) -> EndpointMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EndpointMut<'msg> {
  fn into_mut<'shorter>(self) -> EndpointMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Endpoint {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Endpoint> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EndpointView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EndpointMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // health_check_config: optional message envoy.config.endpoint.v3.Endpoint.HealthCheckConfig
  pub fn has_health_check_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_health_check_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn health_check_config_opt(&self) -> ::std::option::Option<super::endpoint::HealthCheckConfigView<'_>> {
    self.has_health_check_config().then(|| self.health_check_config())
  }
  pub fn health_check_config(&self) -> super::endpoint::HealthCheckConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::endpoint::HealthCheckConfigView::default())
  }
  pub fn health_check_config_mut(&mut self) -> super::endpoint::HealthCheckConfigMut<'_> {
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
  pub fn set_health_check_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::endpoint::HealthCheckConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // hostname: optional string
  pub fn hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // additional_addresses: repeated message envoy.config.endpoint.v3.Endpoint.AdditionalAddress
  pub fn additional_addresses(&self) -> ::protobuf::RepeatedView<'_, super::endpoint::AdditionalAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::endpoint::AdditionalAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn additional_addresses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::endpoint::AdditionalAddress> {
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
  pub fn set_additional_addresses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::endpoint::AdditionalAddress>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl Endpoint

impl ::std::ops::Drop for Endpoint {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Endpoint {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Endpoint {
  type Proxied = Self;
  fn as_view(&self) -> EndpointView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Endpoint {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EndpointMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Endpoint {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__endpoint__v3__Endpoint_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$331XG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__endpoint__v3__Endpoint_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::endpoint::HealthCheckConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::endpoint::AdditionalAddress as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__endpoint__v3__Endpoint_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Endpoint {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Endpoint {
  type Msg = Endpoint;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Endpoint> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Endpoint {
  type Msg = Endpoint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Endpoint> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EndpointMut<'_> {
  type Msg = Endpoint;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Endpoint> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EndpointMut<'_> {
  type Msg = Endpoint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Endpoint> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EndpointView<'_> {
  type Msg = Endpoint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Endpoint> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EndpointMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod endpoint {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__Endpoint__HealthCheckConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HealthCheckConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HealthCheckConfig>
}

impl ::protobuf::Message for HealthCheckConfig {
  type MessageView<'msg> = HealthCheckConfigView<'msg>;
  type MessageMut<'msg> = HealthCheckConfigMut<'msg>;
}

impl ::std::default::Default for HealthCheckConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HealthCheckConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HealthCheckConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `HealthCheckConfigMut`.
unsafe impl ::std::marker::Sync for HealthCheckConfig {}

// SAFETY:
// - `HealthCheckConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HealthCheckConfig {}

impl ::protobuf::Proxied for HealthCheckConfig {
  type View<'msg> = HealthCheckConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HealthCheckConfig {}

impl ::protobuf::MutProxied for HealthCheckConfig {
  type Mut<'msg> = HealthCheckConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HealthCheckConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HealthCheckConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HealthCheckConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HealthCheckConfigView<'msg> {
  type Message = HealthCheckConfig;
}

impl ::std::fmt::Debug for HealthCheckConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HealthCheckConfigView<'_> {
  fn default() -> HealthCheckConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HealthCheckConfig>> for HealthCheckConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HealthCheckConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HealthCheckConfigView<'msg> {

  pub fn to_owned(&self) -> HealthCheckConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // port_value: optional uint32
  pub fn port_value(self) -> u32 {
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

  // hostname: optional string
  pub fn hostname(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

  // disable_active_health_check: optional bool
  pub fn disable_active_health_check(self) -> bool {
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
// - `HealthCheckConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HealthCheckConfigView<'_> {}

// SAFETY:
// - `HealthCheckConfigView` is `Send` because while its alive a `HealthCheckConfigMut` cannot.
// - `HealthCheckConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for HealthCheckConfigView<'_> {}

impl<'msg> ::protobuf::AsView for HealthCheckConfigView<'msg> {
  type Proxied = HealthCheckConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, HealthCheckConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HealthCheckConfigView<'msg> {
  fn into_view<'shorter>(self) -> HealthCheckConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HealthCheckConfig> for HealthCheckConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HealthCheckConfig {
    let mut dst = HealthCheckConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HealthCheckConfig> for HealthCheckConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HealthCheckConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HealthCheckConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HealthCheckConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HealthCheckConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HealthCheckConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HealthCheckConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HealthCheckConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HealthCheckConfigMut<'msg> {
  type Message = HealthCheckConfig;
}

impl ::std::fmt::Debug for HealthCheckConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HealthCheckConfig>> for HealthCheckConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HealthCheckConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HealthCheckConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HealthCheckConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HealthCheckConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // port_value: optional uint32
  pub fn port_value(&self) -> u32 {
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
  pub fn set_port_value(&mut self, val: u32) {
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

  // hostname: optional string
  pub fn hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // disable_active_health_check: optional bool
  pub fn disable_active_health_check(&self) -> bool {
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
  pub fn set_disable_active_health_check(&mut self, val: bool) {
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
// - `HealthCheckConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HealthCheckConfigMut<'_> {}

// SAFETY:
// - `HealthCheckConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HealthCheckConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for HealthCheckConfigMut<'msg> {
  type Proxied = HealthCheckConfig;
  fn as_view(&self) -> ::protobuf::View<'_, HealthCheckConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HealthCheckConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HealthCheckConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HealthCheckConfigMut<'msg> {
  type MutProxied = HealthCheckConfig;
  fn as_mut(&mut self) -> HealthCheckConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HealthCheckConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> HealthCheckConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HealthCheckConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HealthCheckConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HealthCheckConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HealthCheckConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // port_value: optional uint32
  pub fn port_value(&self) -> u32 {
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
  pub fn set_port_value(&mut self, val: u32) {
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

  // hostname: optional string
  pub fn hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // disable_active_health_check: optional bool
  pub fn disable_active_health_check(&self) -> bool {
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
  pub fn set_disable_active_health_check(&mut self, val: bool) {
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

}  // impl HealthCheckConfig

impl ::std::ops::Drop for HealthCheckConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HealthCheckConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HealthCheckConfig {
  type Proxied = Self;
  fn as_view(&self) -> HealthCheckConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HealthCheckConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HealthCheckConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HealthCheckConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::endpoint::envoy__config__endpoint__v3__Endpoint__HealthCheckConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P1X3/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::endpoint::envoy__config__endpoint__v3__Endpoint__HealthCheckConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::endpoint::envoy__config__endpoint__v3__Endpoint__HealthCheckConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HealthCheckConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HealthCheckConfig {
  type Msg = HealthCheckConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthCheckConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HealthCheckConfig {
  type Msg = HealthCheckConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthCheckConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HealthCheckConfigMut<'_> {
  type Msg = HealthCheckConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthCheckConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HealthCheckConfigMut<'_> {
  type Msg = HealthCheckConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthCheckConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HealthCheckConfigView<'_> {
  type Msg = HealthCheckConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthCheckConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HealthCheckConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__Endpoint__AdditionalAddress_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AdditionalAddress {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AdditionalAddress>
}

impl ::protobuf::Message for AdditionalAddress {
  type MessageView<'msg> = AdditionalAddressView<'msg>;
  type MessageMut<'msg> = AdditionalAddressMut<'msg>;
}

impl ::std::default::Default for AdditionalAddress {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AdditionalAddress {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AdditionalAddress` is `Sync` because it does not implement interior mutability.
//    Neither does `AdditionalAddressMut`.
unsafe impl ::std::marker::Sync for AdditionalAddress {}

// SAFETY:
// - `AdditionalAddress` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AdditionalAddress {}

impl ::protobuf::Proxied for AdditionalAddress {
  type View<'msg> = AdditionalAddressView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AdditionalAddress {}

impl ::protobuf::MutProxied for AdditionalAddress {
  type Mut<'msg> = AdditionalAddressMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AdditionalAddressView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AdditionalAddress>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdditionalAddressView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AdditionalAddressView<'msg> {
  type Message = AdditionalAddress;
}

impl ::std::fmt::Debug for AdditionalAddressView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AdditionalAddressView<'_> {
  fn default() -> AdditionalAddressView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AdditionalAddress>> for AdditionalAddressView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AdditionalAddress>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdditionalAddressView<'msg> {

  pub fn to_owned(&self) -> AdditionalAddress {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

}

// SAFETY:
// - `AdditionalAddressView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AdditionalAddressView<'_> {}

// SAFETY:
// - `AdditionalAddressView` is `Send` because while its alive a `AdditionalAddressMut` cannot.
// - `AdditionalAddressView` does not use thread-local data.
unsafe impl ::std::marker::Send for AdditionalAddressView<'_> {}

impl<'msg> ::protobuf::AsView for AdditionalAddressView<'msg> {
  type Proxied = AdditionalAddress;
  fn as_view(&self) -> ::protobuf::View<'msg, AdditionalAddress> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdditionalAddressView<'msg> {
  fn into_view<'shorter>(self) -> AdditionalAddressView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AdditionalAddress> for AdditionalAddressView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AdditionalAddress {
    let mut dst = AdditionalAddress::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AdditionalAddress> for AdditionalAddressMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AdditionalAddress {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AdditionalAddress {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AdditionalAddressView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AdditionalAddressMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AdditionalAddressMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AdditionalAddress>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdditionalAddressMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AdditionalAddressMut<'msg> {
  type Message = AdditionalAddress;
}

impl ::std::fmt::Debug for AdditionalAddressMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AdditionalAddress>> for AdditionalAddressMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AdditionalAddress>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdditionalAddressMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AdditionalAddress> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AdditionalAddress {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

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
// - `AdditionalAddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AdditionalAddressMut<'_> {}

// SAFETY:
// - `AdditionalAddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AdditionalAddressMut<'_> {}

impl<'msg> ::protobuf::AsView for AdditionalAddressMut<'msg> {
  type Proxied = AdditionalAddress;
  fn as_view(&self) -> ::protobuf::View<'_, AdditionalAddress> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdditionalAddressMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AdditionalAddress>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AdditionalAddressMut<'msg> {
  type MutProxied = AdditionalAddress;
  fn as_mut(&mut self) -> AdditionalAddressMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AdditionalAddressMut<'msg> {
  fn into_mut<'shorter>(self) -> AdditionalAddressMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AdditionalAddress {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AdditionalAddress> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AdditionalAddressView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AdditionalAddressMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl AdditionalAddress

impl ::std::ops::Drop for AdditionalAddress {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AdditionalAddress {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AdditionalAddress {
  type Proxied = Self;
  fn as_view(&self) -> AdditionalAddressView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AdditionalAddress {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AdditionalAddressMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AdditionalAddress {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::endpoint::envoy__config__endpoint__v3__Endpoint__AdditionalAddress_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::endpoint::envoy__config__endpoint__v3__Endpoint__AdditionalAddress_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::endpoint::envoy__config__endpoint__v3__Endpoint__AdditionalAddress_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AdditionalAddress {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AdditionalAddress {
  type Msg = AdditionalAddress;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdditionalAddress> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdditionalAddress {
  type Msg = AdditionalAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdditionalAddress> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AdditionalAddressMut<'_> {
  type Msg = AdditionalAddress;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdditionalAddress> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdditionalAddressMut<'_> {
  type Msg = AdditionalAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdditionalAddress> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdditionalAddressView<'_> {
  type Msg = AdditionalAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdditionalAddress> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AdditionalAddressMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod endpoint


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__LbEndpoint_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LbEndpoint {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LbEndpoint>
}

impl ::protobuf::Message for LbEndpoint {
  type MessageView<'msg> = LbEndpointView<'msg>;
  type MessageMut<'msg> = LbEndpointMut<'msg>;
}

impl ::std::default::Default for LbEndpoint {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LbEndpoint {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LbEndpoint` is `Sync` because it does not implement interior mutability.
//    Neither does `LbEndpointMut`.
unsafe impl ::std::marker::Sync for LbEndpoint {}

// SAFETY:
// - `LbEndpoint` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LbEndpoint {}

impl ::protobuf::Proxied for LbEndpoint {
  type View<'msg> = LbEndpointView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LbEndpoint {}

impl ::protobuf::MutProxied for LbEndpoint {
  type Mut<'msg> = LbEndpointMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LbEndpointView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LbEndpoint>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LbEndpointView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LbEndpointView<'msg> {
  type Message = LbEndpoint;
}

impl ::std::fmt::Debug for LbEndpointView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LbEndpointView<'_> {
  fn default() -> LbEndpointView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LbEndpoint>> for LbEndpointView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LbEndpoint>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LbEndpointView<'msg> {

  pub fn to_owned(&self) -> LbEndpoint {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // endpoint: optional message envoy.config.endpoint.v3.Endpoint
  pub fn has_endpoint(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn endpoint_opt(self) -> ::std::option::Option<super::EndpointView<'msg>> {
    self.has_endpoint().then(|| self.endpoint())
  }
  pub fn endpoint(self) -> super::EndpointView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EndpointView::default())
  }

  // endpoint_name: optional string
  pub fn has_endpoint_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn endpoint_name_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_endpoint_name().then(|| self.endpoint_name())
  }
  pub fn endpoint_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // health_status: optional enum envoy.config.core.v3.HealthStatus
  pub fn health_status(self) -> crate::xds::generated::envoy::config::core::v3::health_check::HealthStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (crate::xds::generated::envoy::config::core::v3::health_check::HealthStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }

  // load_balancing_weight: optional message google.protobuf.UInt32Value
  pub fn has_load_balancing_weight(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn load_balancing_weight_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_load_balancing_weight().then(|| self.load_balancing_weight())
  }
  pub fn load_balancing_weight(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  pub fn host_identifier(self) -> super::lb_endpoint::HostIdentifierOneof<'msg> {
    match self.host_identifier_case() {
      super::lb_endpoint::HostIdentifierCase::Endpoint =>
          super::lb_endpoint::HostIdentifierOneof::Endpoint(self.endpoint()),
      super::lb_endpoint::HostIdentifierCase::EndpointName =>
          super::lb_endpoint::HostIdentifierOneof::EndpointName(self.endpoint_name()),
      _ => super::lb_endpoint::HostIdentifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn host_identifier_case(self) -> super::lb_endpoint::HostIdentifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::lb_endpoint::HostIdentifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `LbEndpointView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LbEndpointView<'_> {}

// SAFETY:
// - `LbEndpointView` is `Send` because while its alive a `LbEndpointMut` cannot.
// - `LbEndpointView` does not use thread-local data.
unsafe impl ::std::marker::Send for LbEndpointView<'_> {}

impl<'msg> ::protobuf::AsView for LbEndpointView<'msg> {
  type Proxied = LbEndpoint;
  fn as_view(&self) -> ::protobuf::View<'msg, LbEndpoint> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbEndpointView<'msg> {
  fn into_view<'shorter>(self) -> LbEndpointView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LbEndpoint> for LbEndpointView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LbEndpoint {
    let mut dst = LbEndpoint::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LbEndpoint> for LbEndpointMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LbEndpoint {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LbEndpoint {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LbEndpointView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LbEndpointMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LbEndpointMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpoint>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LbEndpointMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LbEndpointMut<'msg> {
  type Message = LbEndpoint;
}

impl ::std::fmt::Debug for LbEndpointMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpoint>> for LbEndpointMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpoint>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LbEndpointMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpoint> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LbEndpoint {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // endpoint: optional message envoy.config.endpoint.v3.Endpoint
  pub fn has_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn endpoint_opt(&self) -> ::std::option::Option<super::EndpointView<'_>> {
    self.has_endpoint().then(|| self.endpoint())
  }
  pub fn endpoint(&self) -> super::EndpointView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EndpointView::default())
  }
  pub fn endpoint_mut(&mut self) -> super::EndpointMut<'_> {
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
  pub fn set_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::Endpoint>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // endpoint_name: optional string
  pub fn has_endpoint_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_endpoint_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn endpoint_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_endpoint_name().then(|| self.endpoint_name())
  }
  pub fn endpoint_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_endpoint_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // health_status: optional enum envoy.config.core.v3.HealthStatus
  pub fn health_status(&self) -> crate::xds::generated::envoy::config::core::v3::health_check::HealthStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (crate::xds::generated::envoy::config::core::v3::health_check::HealthStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_health_status(&mut self, val: crate::xds::generated::envoy::config::core::v3::health_check::HealthStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // load_balancing_weight: optional message google.protobuf.UInt32Value
  pub fn has_load_balancing_weight(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_load_balancing_weight(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn load_balancing_weight_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_load_balancing_weight().then(|| self.load_balancing_weight())
  }
  pub fn load_balancing_weight(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn load_balancing_weight_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_load_balancing_weight(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn host_identifier(&self) -> super::lb_endpoint::HostIdentifierOneof<'_> {
    match &self.host_identifier_case() {
      super::lb_endpoint::HostIdentifierCase::Endpoint =>
          super::lb_endpoint::HostIdentifierOneof::Endpoint(self.endpoint()),
      super::lb_endpoint::HostIdentifierCase::EndpointName =>
          super::lb_endpoint::HostIdentifierOneof::EndpointName(self.endpoint_name()),
      _ => super::lb_endpoint::HostIdentifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn host_identifier_case(&self) -> super::lb_endpoint::HostIdentifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::lb_endpoint::HostIdentifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `LbEndpointMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LbEndpointMut<'_> {}

// SAFETY:
// - `LbEndpointMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LbEndpointMut<'_> {}

impl<'msg> ::protobuf::AsView for LbEndpointMut<'msg> {
  type Proxied = LbEndpoint;
  fn as_view(&self) -> ::protobuf::View<'_, LbEndpoint> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbEndpointMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LbEndpoint>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LbEndpointMut<'msg> {
  type MutProxied = LbEndpoint;
  fn as_mut(&mut self) -> LbEndpointMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LbEndpointMut<'msg> {
  fn into_mut<'shorter>(self) -> LbEndpointMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LbEndpoint {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LbEndpoint> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LbEndpointView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LbEndpointMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // endpoint: optional message envoy.config.endpoint.v3.Endpoint
  pub fn has_endpoint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_endpoint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn endpoint_opt(&self) -> ::std::option::Option<super::EndpointView<'_>> {
    self.has_endpoint().then(|| self.endpoint())
  }
  pub fn endpoint(&self) -> super::EndpointView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EndpointView::default())
  }
  pub fn endpoint_mut(&mut self) -> super::EndpointMut<'_> {
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
  pub fn set_endpoint(&mut self,
    val: impl ::protobuf::IntoProxied<super::Endpoint>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // endpoint_name: optional string
  pub fn has_endpoint_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_endpoint_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn endpoint_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_endpoint_name().then(|| self.endpoint_name())
  }
  pub fn endpoint_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_endpoint_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // health_status: optional enum envoy.config.core.v3.HealthStatus
  pub fn health_status(&self) -> crate::xds::generated::envoy::config::core::v3::health_check::HealthStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (crate::xds::generated::envoy::config::core::v3::health_check::HealthStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_health_status(&mut self, val: crate::xds::generated::envoy::config::core::v3::health_check::HealthStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // load_balancing_weight: optional message google.protobuf.UInt32Value
  pub fn has_load_balancing_weight(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_load_balancing_weight(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn load_balancing_weight_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_load_balancing_weight().then(|| self.load_balancing_weight())
  }
  pub fn load_balancing_weight(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn load_balancing_weight_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_load_balancing_weight(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn host_identifier(&self) -> super::lb_endpoint::HostIdentifierOneof<'_> {
    match &self.host_identifier_case() {
      super::lb_endpoint::HostIdentifierCase::Endpoint =>
          super::lb_endpoint::HostIdentifierOneof::Endpoint(self.endpoint()),
      super::lb_endpoint::HostIdentifierCase::EndpointName =>
          super::lb_endpoint::HostIdentifierOneof::EndpointName(self.endpoint_name()),
      _ => super::lb_endpoint::HostIdentifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn host_identifier_case(&self) -> super::lb_endpoint::HostIdentifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::lb_endpoint::HostIdentifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl LbEndpoint

impl ::std::ops::Drop for LbEndpoint {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LbEndpoint {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LbEndpoint {
  type Proxied = Self;
  fn as_view(&self) -> LbEndpointView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LbEndpoint {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LbEndpointMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LbEndpoint {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__endpoint__v3__LbEndpoint_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3.P331T^!|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__endpoint__v3__LbEndpoint_msg_init.0, &[<super::Endpoint as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__endpoint__v3__LbEndpoint_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LbEndpoint {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LbEndpoint {
  type Msg = LbEndpoint;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpoint> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbEndpoint {
  type Msg = LbEndpoint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpoint> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LbEndpointMut<'_> {
  type Msg = LbEndpoint;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpoint> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbEndpointMut<'_> {
  type Msg = LbEndpoint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpoint> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbEndpointView<'_> {
  type Msg = LbEndpoint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpoint> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LbEndpointMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod lb_endpoint {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum HostIdentifierOneof<'msg> {
  Endpoint(::protobuf::View<'msg, super::super::Endpoint>) = 1,
  EndpointName(&'msg ::protobuf::ProtoStr) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum HostIdentifierCase {
  Endpoint = 1,
  EndpointName = 5,

  not_set = 0
}

impl HostIdentifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<HostIdentifierCase> {
    match v {
      0 => Some(HostIdentifierCase::not_set),
      1 => Some(HostIdentifierCase::Endpoint),
      5 => Some(HostIdentifierCase::EndpointName),
      _ => None
    }
  }
}
}  // pub mod lb_endpoint


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__LbEndpointCollection_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LbEndpointCollection {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LbEndpointCollection>
}

impl ::protobuf::Message for LbEndpointCollection {
  type MessageView<'msg> = LbEndpointCollectionView<'msg>;
  type MessageMut<'msg> = LbEndpointCollectionMut<'msg>;
}

impl ::std::default::Default for LbEndpointCollection {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LbEndpointCollection {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LbEndpointCollection` is `Sync` because it does not implement interior mutability.
//    Neither does `LbEndpointCollectionMut`.
unsafe impl ::std::marker::Sync for LbEndpointCollection {}

// SAFETY:
// - `LbEndpointCollection` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LbEndpointCollection {}

impl ::protobuf::Proxied for LbEndpointCollection {
  type View<'msg> = LbEndpointCollectionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LbEndpointCollection {}

impl ::protobuf::MutProxied for LbEndpointCollection {
  type Mut<'msg> = LbEndpointCollectionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LbEndpointCollectionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LbEndpointCollection>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LbEndpointCollectionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LbEndpointCollectionView<'msg> {
  type Message = LbEndpointCollection;
}

impl ::std::fmt::Debug for LbEndpointCollectionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LbEndpointCollectionView<'_> {
  fn default() -> LbEndpointCollectionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LbEndpointCollection>> for LbEndpointCollectionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LbEndpointCollection>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LbEndpointCollectionView<'msg> {

  pub fn to_owned(&self) -> LbEndpointCollection {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // entries: optional message xds.core.v3.CollectionEntry
  pub fn has_entries(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn entries_opt(self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'msg>> {
    self.has_entries().then(|| self.entries())
  }
  pub fn entries(self) -> crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView::default())
  }

}

// SAFETY:
// - `LbEndpointCollectionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LbEndpointCollectionView<'_> {}

// SAFETY:
// - `LbEndpointCollectionView` is `Send` because while its alive a `LbEndpointCollectionMut` cannot.
// - `LbEndpointCollectionView` does not use thread-local data.
unsafe impl ::std::marker::Send for LbEndpointCollectionView<'_> {}

impl<'msg> ::protobuf::AsView for LbEndpointCollectionView<'msg> {
  type Proxied = LbEndpointCollection;
  fn as_view(&self) -> ::protobuf::View<'msg, LbEndpointCollection> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbEndpointCollectionView<'msg> {
  fn into_view<'shorter>(self) -> LbEndpointCollectionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LbEndpointCollection> for LbEndpointCollectionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LbEndpointCollection {
    let mut dst = LbEndpointCollection::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LbEndpointCollection> for LbEndpointCollectionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LbEndpointCollection {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LbEndpointCollection {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LbEndpointCollectionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LbEndpointCollectionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LbEndpointCollectionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpointCollection>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LbEndpointCollectionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LbEndpointCollectionMut<'msg> {
  type Message = LbEndpointCollection;
}

impl ::std::fmt::Debug for LbEndpointCollectionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpointCollection>> for LbEndpointCollectionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpointCollection>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LbEndpointCollectionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpointCollection> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LbEndpointCollection {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // entries: optional message xds.core.v3.CollectionEntry
  pub fn has_entries(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_entries(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn entries_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'_>> {
    self.has_entries().then(|| self.entries())
  }
  pub fn entries(&self) -> crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView::default())
  }
  pub fn entries_mut(&mut self) -> crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryMut<'_> {
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
  pub fn set_entries(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry>) {

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
// - `LbEndpointCollectionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LbEndpointCollectionMut<'_> {}

// SAFETY:
// - `LbEndpointCollectionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LbEndpointCollectionMut<'_> {}

impl<'msg> ::protobuf::AsView for LbEndpointCollectionMut<'msg> {
  type Proxied = LbEndpointCollection;
  fn as_view(&self) -> ::protobuf::View<'_, LbEndpointCollection> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbEndpointCollectionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LbEndpointCollection>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LbEndpointCollectionMut<'msg> {
  type MutProxied = LbEndpointCollection;
  fn as_mut(&mut self) -> LbEndpointCollectionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LbEndpointCollectionMut<'msg> {
  fn into_mut<'shorter>(self) -> LbEndpointCollectionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LbEndpointCollection {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LbEndpointCollection> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LbEndpointCollectionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LbEndpointCollectionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // entries: optional message xds.core.v3.CollectionEntry
  pub fn has_entries(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_entries(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn entries_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'_>> {
    self.has_entries().then(|| self.entries())
  }
  pub fn entries(&self) -> crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView::default())
  }
  pub fn entries_mut(&mut self) -> crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryMut<'_> {
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
  pub fn set_entries(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl LbEndpointCollection

impl ::std::ops::Drop for LbEndpointCollection {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LbEndpointCollection {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LbEndpointCollection {
  type Proxied = Self;
  fn as_view(&self) -> LbEndpointCollectionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LbEndpointCollection {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LbEndpointCollectionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LbEndpointCollection {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__endpoint__v3__LbEndpointCollection_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__endpoint__v3__LbEndpointCollection_msg_init.0, &[<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__endpoint__v3__LbEndpointCollection_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LbEndpointCollection {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LbEndpointCollection {
  type Msg = LbEndpointCollection;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpointCollection> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbEndpointCollection {
  type Msg = LbEndpointCollection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpointCollection> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LbEndpointCollectionMut<'_> {
  type Msg = LbEndpointCollection;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpointCollection> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbEndpointCollectionMut<'_> {
  type Msg = LbEndpointCollection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpointCollection> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbEndpointCollectionView<'_> {
  type Msg = LbEndpointCollection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpointCollection> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LbEndpointCollectionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__LedsClusterLocalityConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LedsClusterLocalityConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LedsClusterLocalityConfig>
}

impl ::protobuf::Message for LedsClusterLocalityConfig {
  type MessageView<'msg> = LedsClusterLocalityConfigView<'msg>;
  type MessageMut<'msg> = LedsClusterLocalityConfigMut<'msg>;
}

impl ::std::default::Default for LedsClusterLocalityConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LedsClusterLocalityConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LedsClusterLocalityConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `LedsClusterLocalityConfigMut`.
unsafe impl ::std::marker::Sync for LedsClusterLocalityConfig {}

// SAFETY:
// - `LedsClusterLocalityConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LedsClusterLocalityConfig {}

impl ::protobuf::Proxied for LedsClusterLocalityConfig {
  type View<'msg> = LedsClusterLocalityConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LedsClusterLocalityConfig {}

impl ::protobuf::MutProxied for LedsClusterLocalityConfig {
  type Mut<'msg> = LedsClusterLocalityConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LedsClusterLocalityConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LedsClusterLocalityConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LedsClusterLocalityConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LedsClusterLocalityConfigView<'msg> {
  type Message = LedsClusterLocalityConfig;
}

impl ::std::fmt::Debug for LedsClusterLocalityConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LedsClusterLocalityConfigView<'_> {
  fn default() -> LedsClusterLocalityConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LedsClusterLocalityConfig>> for LedsClusterLocalityConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LedsClusterLocalityConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LedsClusterLocalityConfigView<'msg> {

  pub fn to_owned(&self) -> LedsClusterLocalityConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // leds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_leds_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn leds_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_leds_config().then(|| self.leds_config())
  }
  pub fn leds_config(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

  // leds_collection_name: optional string
  pub fn leds_collection_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `LedsClusterLocalityConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LedsClusterLocalityConfigView<'_> {}

// SAFETY:
// - `LedsClusterLocalityConfigView` is `Send` because while its alive a `LedsClusterLocalityConfigMut` cannot.
// - `LedsClusterLocalityConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for LedsClusterLocalityConfigView<'_> {}

impl<'msg> ::protobuf::AsView for LedsClusterLocalityConfigView<'msg> {
  type Proxied = LedsClusterLocalityConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, LedsClusterLocalityConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LedsClusterLocalityConfigView<'msg> {
  fn into_view<'shorter>(self) -> LedsClusterLocalityConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LedsClusterLocalityConfig> for LedsClusterLocalityConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LedsClusterLocalityConfig {
    let mut dst = LedsClusterLocalityConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LedsClusterLocalityConfig> for LedsClusterLocalityConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LedsClusterLocalityConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LedsClusterLocalityConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LedsClusterLocalityConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LedsClusterLocalityConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LedsClusterLocalityConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LedsClusterLocalityConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LedsClusterLocalityConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LedsClusterLocalityConfigMut<'msg> {
  type Message = LedsClusterLocalityConfig;
}

impl ::std::fmt::Debug for LedsClusterLocalityConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LedsClusterLocalityConfig>> for LedsClusterLocalityConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LedsClusterLocalityConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LedsClusterLocalityConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LedsClusterLocalityConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LedsClusterLocalityConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // leds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_leds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_leds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn leds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_leds_config().then(|| self.leds_config())
  }
  pub fn leds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn leds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_leds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // leds_collection_name: optional string
  pub fn leds_collection_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_leds_collection_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `LedsClusterLocalityConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LedsClusterLocalityConfigMut<'_> {}

// SAFETY:
// - `LedsClusterLocalityConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LedsClusterLocalityConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for LedsClusterLocalityConfigMut<'msg> {
  type Proxied = LedsClusterLocalityConfig;
  fn as_view(&self) -> ::protobuf::View<'_, LedsClusterLocalityConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LedsClusterLocalityConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LedsClusterLocalityConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LedsClusterLocalityConfigMut<'msg> {
  type MutProxied = LedsClusterLocalityConfig;
  fn as_mut(&mut self) -> LedsClusterLocalityConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LedsClusterLocalityConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> LedsClusterLocalityConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LedsClusterLocalityConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LedsClusterLocalityConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LedsClusterLocalityConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LedsClusterLocalityConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // leds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_leds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_leds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn leds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_leds_config().then(|| self.leds_config())
  }
  pub fn leds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn leds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_leds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // leds_collection_name: optional string
  pub fn leds_collection_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_leds_collection_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl LedsClusterLocalityConfig

impl ::std::ops::Drop for LedsClusterLocalityConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LedsClusterLocalityConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LedsClusterLocalityConfig {
  type Proxied = Self;
  fn as_view(&self) -> LedsClusterLocalityConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LedsClusterLocalityConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LedsClusterLocalityConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LedsClusterLocalityConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__endpoint__v3__LedsClusterLocalityConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__endpoint__v3__LedsClusterLocalityConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__endpoint__v3__LedsClusterLocalityConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LedsClusterLocalityConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LedsClusterLocalityConfig {
  type Msg = LedsClusterLocalityConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LedsClusterLocalityConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LedsClusterLocalityConfig {
  type Msg = LedsClusterLocalityConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LedsClusterLocalityConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LedsClusterLocalityConfigMut<'_> {
  type Msg = LedsClusterLocalityConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LedsClusterLocalityConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LedsClusterLocalityConfigMut<'_> {
  type Msg = LedsClusterLocalityConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LedsClusterLocalityConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LedsClusterLocalityConfigView<'_> {
  type Msg = LedsClusterLocalityConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LedsClusterLocalityConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LedsClusterLocalityConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__LocalityLbEndpoints_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LocalityLbEndpoints {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LocalityLbEndpoints>
}

impl ::protobuf::Message for LocalityLbEndpoints {
  type MessageView<'msg> = LocalityLbEndpointsView<'msg>;
  type MessageMut<'msg> = LocalityLbEndpointsMut<'msg>;
}

impl ::std::default::Default for LocalityLbEndpoints {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LocalityLbEndpoints {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LocalityLbEndpoints` is `Sync` because it does not implement interior mutability.
//    Neither does `LocalityLbEndpointsMut`.
unsafe impl ::std::marker::Sync for LocalityLbEndpoints {}

// SAFETY:
// - `LocalityLbEndpoints` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LocalityLbEndpoints {}

impl ::protobuf::Proxied for LocalityLbEndpoints {
  type View<'msg> = LocalityLbEndpointsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LocalityLbEndpoints {}

impl ::protobuf::MutProxied for LocalityLbEndpoints {
  type Mut<'msg> = LocalityLbEndpointsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LocalityLbEndpointsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityLbEndpoints>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalityLbEndpointsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LocalityLbEndpointsView<'msg> {
  type Message = LocalityLbEndpoints;
}

impl ::std::fmt::Debug for LocalityLbEndpointsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LocalityLbEndpointsView<'_> {
  fn default() -> LocalityLbEndpointsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityLbEndpoints>> for LocalityLbEndpointsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityLbEndpoints>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalityLbEndpointsView<'msg> {

  pub fn to_owned(&self) -> LocalityLbEndpoints {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // locality: optional message envoy.config.core.v3.Locality
  pub fn has_locality(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn locality_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::LocalityView<'msg>> {
    self.has_locality().then(|| self.locality())
  }
  pub fn locality(self) -> crate::xds::generated::envoy::config::core::v3::base::LocalityView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::LocalityView::default())
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }

  // lb_endpoints: repeated message envoy.config.endpoint.v3.LbEndpoint
  pub fn lb_endpoints(self) -> ::protobuf::RepeatedView<'msg, super::LbEndpoint> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::LbEndpoint>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // load_balancer_endpoints: optional message envoy.config.endpoint.v3.LocalityLbEndpoints.LbEndpointList
  pub fn has_load_balancer_endpoints(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn load_balancer_endpoints_opt(self) -> ::std::option::Option<super::locality_lb_endpoints::LbEndpointListView<'msg>> {
    self.has_load_balancer_endpoints().then(|| self.load_balancer_endpoints())
  }
  pub fn load_balancer_endpoints(self) -> super::locality_lb_endpoints::LbEndpointListView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::locality_lb_endpoints::LbEndpointListView::default())
  }

  // leds_cluster_locality_config: optional message envoy.config.endpoint.v3.LedsClusterLocalityConfig
  pub fn has_leds_cluster_locality_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn leds_cluster_locality_config_opt(self) -> ::std::option::Option<super::LedsClusterLocalityConfigView<'msg>> {
    self.has_leds_cluster_locality_config().then(|| self.leds_cluster_locality_config())
  }
  pub fn leds_cluster_locality_config(self) -> super::LedsClusterLocalityConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LedsClusterLocalityConfigView::default())
  }

  // load_balancing_weight: optional message google.protobuf.UInt32Value
  pub fn has_load_balancing_weight(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn load_balancing_weight_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_load_balancing_weight().then(|| self.load_balancing_weight())
  }
  pub fn load_balancing_weight(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // priority: optional uint32
  pub fn priority(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // proximity: optional message google.protobuf.UInt32Value
  pub fn has_proximity(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn proximity_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_proximity().then(|| self.proximity())
  }
  pub fn proximity(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  pub fn lb_config(self) -> super::locality_lb_endpoints::LbConfigOneof<'msg> {
    match self.lb_config_case() {
      super::locality_lb_endpoints::LbConfigCase::LoadBalancerEndpoints =>
          super::locality_lb_endpoints::LbConfigOneof::LoadBalancerEndpoints(self.load_balancer_endpoints()),
      super::locality_lb_endpoints::LbConfigCase::LedsClusterLocalityConfig =>
          super::locality_lb_endpoints::LbConfigOneof::LedsClusterLocalityConfig(self.leds_cluster_locality_config()),
      _ => super::locality_lb_endpoints::LbConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn lb_config_case(self) -> super::locality_lb_endpoints::LbConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(5);
      super::locality_lb_endpoints::LbConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `LocalityLbEndpointsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LocalityLbEndpointsView<'_> {}

// SAFETY:
// - `LocalityLbEndpointsView` is `Send` because while its alive a `LocalityLbEndpointsMut` cannot.
// - `LocalityLbEndpointsView` does not use thread-local data.
unsafe impl ::std::marker::Send for LocalityLbEndpointsView<'_> {}

impl<'msg> ::protobuf::AsView for LocalityLbEndpointsView<'msg> {
  type Proxied = LocalityLbEndpoints;
  fn as_view(&self) -> ::protobuf::View<'msg, LocalityLbEndpoints> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalityLbEndpointsView<'msg> {
  fn into_view<'shorter>(self) -> LocalityLbEndpointsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalityLbEndpoints> for LocalityLbEndpointsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalityLbEndpoints {
    let mut dst = LocalityLbEndpoints::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalityLbEndpoints> for LocalityLbEndpointsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalityLbEndpoints {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LocalityLbEndpoints {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalityLbEndpointsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalityLbEndpointsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LocalityLbEndpointsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityLbEndpoints>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalityLbEndpointsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LocalityLbEndpointsMut<'msg> {
  type Message = LocalityLbEndpoints;
}

impl ::std::fmt::Debug for LocalityLbEndpointsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityLbEndpoints>> for LocalityLbEndpointsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityLbEndpoints>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalityLbEndpointsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityLbEndpoints> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LocalityLbEndpoints {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // locality: optional message envoy.config.core.v3.Locality
  pub fn has_locality(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_locality(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn locality_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::LocalityView<'_>> {
    self.has_locality().then(|| self.locality())
  }
  pub fn locality(&self) -> crate::xds::generated::envoy::config::core::v3::base::LocalityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::LocalityView::default())
  }
  pub fn locality_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::LocalityMut<'_> {
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
  pub fn set_locality(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Locality>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // lb_endpoints: repeated message envoy.config.endpoint.v3.LbEndpoint
  pub fn lb_endpoints(&self) -> ::protobuf::RepeatedView<'_, super::LbEndpoint> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::LbEndpoint>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn lb_endpoints_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::LbEndpoint> {
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
  pub fn set_lb_endpoints(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::LbEndpoint>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // load_balancer_endpoints: optional message envoy.config.endpoint.v3.LocalityLbEndpoints.LbEndpointList
  pub fn has_load_balancer_endpoints(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_load_balancer_endpoints(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn load_balancer_endpoints_opt(&self) -> ::std::option::Option<super::locality_lb_endpoints::LbEndpointListView<'_>> {
    self.has_load_balancer_endpoints().then(|| self.load_balancer_endpoints())
  }
  pub fn load_balancer_endpoints(&self) -> super::locality_lb_endpoints::LbEndpointListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::locality_lb_endpoints::LbEndpointListView::default())
  }
  pub fn load_balancer_endpoints_mut(&mut self) -> super::locality_lb_endpoints::LbEndpointListMut<'_> {
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
  pub fn set_load_balancer_endpoints(&mut self,
    val: impl ::protobuf::IntoProxied<super::locality_lb_endpoints::LbEndpointList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // leds_cluster_locality_config: optional message envoy.config.endpoint.v3.LedsClusterLocalityConfig
  pub fn has_leds_cluster_locality_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_leds_cluster_locality_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn leds_cluster_locality_config_opt(&self) -> ::std::option::Option<super::LedsClusterLocalityConfigView<'_>> {
    self.has_leds_cluster_locality_config().then(|| self.leds_cluster_locality_config())
  }
  pub fn leds_cluster_locality_config(&self) -> super::LedsClusterLocalityConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LedsClusterLocalityConfigView::default())
  }
  pub fn leds_cluster_locality_config_mut(&mut self) -> super::LedsClusterLocalityConfigMut<'_> {
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
  pub fn set_leds_cluster_locality_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::LedsClusterLocalityConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // load_balancing_weight: optional message google.protobuf.UInt32Value
  pub fn has_load_balancing_weight(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_load_balancing_weight(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn load_balancing_weight_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_load_balancing_weight().then(|| self.load_balancing_weight())
  }
  pub fn load_balancing_weight(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn load_balancing_weight_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_load_balancing_weight(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // priority: optional uint32
  pub fn priority(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_priority(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

  // proximity: optional message google.protobuf.UInt32Value
  pub fn has_proximity(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_proximity(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn proximity_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_proximity().then(|| self.proximity())
  }
  pub fn proximity(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn proximity_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_proximity(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn lb_config(&self) -> super::locality_lb_endpoints::LbConfigOneof<'_> {
    match &self.lb_config_case() {
      super::locality_lb_endpoints::LbConfigCase::LoadBalancerEndpoints =>
          super::locality_lb_endpoints::LbConfigOneof::LoadBalancerEndpoints(self.load_balancer_endpoints()),
      super::locality_lb_endpoints::LbConfigCase::LedsClusterLocalityConfig =>
          super::locality_lb_endpoints::LbConfigOneof::LedsClusterLocalityConfig(self.leds_cluster_locality_config()),
      _ => super::locality_lb_endpoints::LbConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn lb_config_case(&self) -> super::locality_lb_endpoints::LbConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(5);
      super::locality_lb_endpoints::LbConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `LocalityLbEndpointsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LocalityLbEndpointsMut<'_> {}

// SAFETY:
// - `LocalityLbEndpointsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LocalityLbEndpointsMut<'_> {}

impl<'msg> ::protobuf::AsView for LocalityLbEndpointsMut<'msg> {
  type Proxied = LocalityLbEndpoints;
  fn as_view(&self) -> ::protobuf::View<'_, LocalityLbEndpoints> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalityLbEndpointsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LocalityLbEndpoints>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LocalityLbEndpointsMut<'msg> {
  type MutProxied = LocalityLbEndpoints;
  fn as_mut(&mut self) -> LocalityLbEndpointsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LocalityLbEndpointsMut<'msg> {
  fn into_mut<'shorter>(self) -> LocalityLbEndpointsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LocalityLbEndpoints {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LocalityLbEndpoints> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LocalityLbEndpointsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LocalityLbEndpointsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // locality: optional message envoy.config.core.v3.Locality
  pub fn has_locality(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_locality(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn locality_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::LocalityView<'_>> {
    self.has_locality().then(|| self.locality())
  }
  pub fn locality(&self) -> crate::xds::generated::envoy::config::core::v3::base::LocalityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::LocalityView::default())
  }
  pub fn locality_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::LocalityMut<'_> {
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
  pub fn set_locality(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Locality>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // lb_endpoints: repeated message envoy.config.endpoint.v3.LbEndpoint
  pub fn lb_endpoints(&self) -> ::protobuf::RepeatedView<'_, super::LbEndpoint> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::LbEndpoint>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn lb_endpoints_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::LbEndpoint> {
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
  pub fn set_lb_endpoints(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::LbEndpoint>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // load_balancer_endpoints: optional message envoy.config.endpoint.v3.LocalityLbEndpoints.LbEndpointList
  pub fn has_load_balancer_endpoints(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_load_balancer_endpoints(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn load_balancer_endpoints_opt(&self) -> ::std::option::Option<super::locality_lb_endpoints::LbEndpointListView<'_>> {
    self.has_load_balancer_endpoints().then(|| self.load_balancer_endpoints())
  }
  pub fn load_balancer_endpoints(&self) -> super::locality_lb_endpoints::LbEndpointListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::locality_lb_endpoints::LbEndpointListView::default())
  }
  pub fn load_balancer_endpoints_mut(&mut self) -> super::locality_lb_endpoints::LbEndpointListMut<'_> {
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
  pub fn set_load_balancer_endpoints(&mut self,
    val: impl ::protobuf::IntoProxied<super::locality_lb_endpoints::LbEndpointList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // leds_cluster_locality_config: optional message envoy.config.endpoint.v3.LedsClusterLocalityConfig
  pub fn has_leds_cluster_locality_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_leds_cluster_locality_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn leds_cluster_locality_config_opt(&self) -> ::std::option::Option<super::LedsClusterLocalityConfigView<'_>> {
    self.has_leds_cluster_locality_config().then(|| self.leds_cluster_locality_config())
  }
  pub fn leds_cluster_locality_config(&self) -> super::LedsClusterLocalityConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LedsClusterLocalityConfigView::default())
  }
  pub fn leds_cluster_locality_config_mut(&mut self) -> super::LedsClusterLocalityConfigMut<'_> {
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
  pub fn set_leds_cluster_locality_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::LedsClusterLocalityConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // load_balancing_weight: optional message google.protobuf.UInt32Value
  pub fn has_load_balancing_weight(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_load_balancing_weight(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn load_balancing_weight_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_load_balancing_weight().then(|| self.load_balancing_weight())
  }
  pub fn load_balancing_weight(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn load_balancing_weight_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_load_balancing_weight(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // priority: optional uint32
  pub fn priority(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_priority(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

  // proximity: optional message google.protobuf.UInt32Value
  pub fn has_proximity(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_proximity(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn proximity_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_proximity().then(|| self.proximity())
  }
  pub fn proximity(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn proximity_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_proximity(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn lb_config(&self) -> super::locality_lb_endpoints::LbConfigOneof<'_> {
    match &self.lb_config_case() {
      super::locality_lb_endpoints::LbConfigCase::LoadBalancerEndpoints =>
          super::locality_lb_endpoints::LbConfigOneof::LoadBalancerEndpoints(self.load_balancer_endpoints()),
      super::locality_lb_endpoints::LbConfigCase::LedsClusterLocalityConfig =>
          super::locality_lb_endpoints::LbConfigOneof::LedsClusterLocalityConfig(self.leds_cluster_locality_config()),
      _ => super::locality_lb_endpoints::LbConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn lb_config_case(&self) -> super::locality_lb_endpoints::LbConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(5);
      super::locality_lb_endpoints::LbConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl LocalityLbEndpoints

impl ::std::ops::Drop for LocalityLbEndpoints {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LocalityLbEndpoints {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LocalityLbEndpoints {
  type Proxied = Self;
  fn as_view(&self) -> LocalityLbEndpointsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LocalityLbEndpoints {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LocalityLbEndpointsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LocalityLbEndpoints {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__endpoint__v3__LocalityLbEndpoints_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G3a)P3333^)|*");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__endpoint__v3__LocalityLbEndpoints_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::Locality as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::LbEndpoint as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::locality_lb_endpoints::LbEndpointList as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::LedsClusterLocalityConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__endpoint__v3__LocalityLbEndpoints_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalityLbEndpoints {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalityLbEndpoints {
  type Msg = LocalityLbEndpoints;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityLbEndpoints> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityLbEndpoints {
  type Msg = LocalityLbEndpoints;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityLbEndpoints> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalityLbEndpointsMut<'_> {
  type Msg = LocalityLbEndpoints;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityLbEndpoints> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityLbEndpointsMut<'_> {
  type Msg = LocalityLbEndpoints;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityLbEndpoints> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityLbEndpointsView<'_> {
  type Msg = LocalityLbEndpoints;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityLbEndpoints> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalityLbEndpointsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod locality_lb_endpoints {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__LocalityLbEndpoints__LbEndpointList_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LbEndpointList {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LbEndpointList>
}

impl ::protobuf::Message for LbEndpointList {
  type MessageView<'msg> = LbEndpointListView<'msg>;
  type MessageMut<'msg> = LbEndpointListMut<'msg>;
}

impl ::std::default::Default for LbEndpointList {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LbEndpointList {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LbEndpointList` is `Sync` because it does not implement interior mutability.
//    Neither does `LbEndpointListMut`.
unsafe impl ::std::marker::Sync for LbEndpointList {}

// SAFETY:
// - `LbEndpointList` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LbEndpointList {}

impl ::protobuf::Proxied for LbEndpointList {
  type View<'msg> = LbEndpointListView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LbEndpointList {}

impl ::protobuf::MutProxied for LbEndpointList {
  type Mut<'msg> = LbEndpointListMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LbEndpointListView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LbEndpointList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LbEndpointListView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LbEndpointListView<'msg> {
  type Message = LbEndpointList;
}

impl ::std::fmt::Debug for LbEndpointListView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LbEndpointListView<'_> {
  fn default() -> LbEndpointListView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LbEndpointList>> for LbEndpointListView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LbEndpointList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LbEndpointListView<'msg> {

  pub fn to_owned(&self) -> LbEndpointList {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // lb_endpoints: repeated message envoy.config.endpoint.v3.LbEndpoint
  pub fn lb_endpoints(self) -> ::protobuf::RepeatedView<'msg, super::super::LbEndpoint> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::LbEndpoint>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `LbEndpointListView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LbEndpointListView<'_> {}

// SAFETY:
// - `LbEndpointListView` is `Send` because while its alive a `LbEndpointListMut` cannot.
// - `LbEndpointListView` does not use thread-local data.
unsafe impl ::std::marker::Send for LbEndpointListView<'_> {}

impl<'msg> ::protobuf::AsView for LbEndpointListView<'msg> {
  type Proxied = LbEndpointList;
  fn as_view(&self) -> ::protobuf::View<'msg, LbEndpointList> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbEndpointListView<'msg> {
  fn into_view<'shorter>(self) -> LbEndpointListView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LbEndpointList> for LbEndpointListView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LbEndpointList {
    let mut dst = LbEndpointList::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LbEndpointList> for LbEndpointListMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LbEndpointList {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LbEndpointList {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LbEndpointListView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LbEndpointListMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LbEndpointListMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpointList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LbEndpointListMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LbEndpointListMut<'msg> {
  type Message = LbEndpointList;
}

impl ::std::fmt::Debug for LbEndpointListMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpointList>> for LbEndpointListMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpointList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LbEndpointListMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LbEndpointList> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LbEndpointList {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // lb_endpoints: repeated message envoy.config.endpoint.v3.LbEndpoint
  pub fn lb_endpoints(&self) -> ::protobuf::RepeatedView<'_, super::super::LbEndpoint> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::LbEndpoint>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn lb_endpoints_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::LbEndpoint> {
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
  pub fn set_lb_endpoints(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::LbEndpoint>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `LbEndpointListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LbEndpointListMut<'_> {}

// SAFETY:
// - `LbEndpointListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LbEndpointListMut<'_> {}

impl<'msg> ::protobuf::AsView for LbEndpointListMut<'msg> {
  type Proxied = LbEndpointList;
  fn as_view(&self) -> ::protobuf::View<'_, LbEndpointList> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbEndpointListMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LbEndpointList>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LbEndpointListMut<'msg> {
  type MutProxied = LbEndpointList;
  fn as_mut(&mut self) -> LbEndpointListMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LbEndpointListMut<'msg> {
  fn into_mut<'shorter>(self) -> LbEndpointListMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LbEndpointList {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LbEndpointList> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LbEndpointListView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LbEndpointListMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // lb_endpoints: repeated message envoy.config.endpoint.v3.LbEndpoint
  pub fn lb_endpoints(&self) -> ::protobuf::RepeatedView<'_, super::super::LbEndpoint> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::LbEndpoint>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn lb_endpoints_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::LbEndpoint> {
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
  pub fn set_lb_endpoints(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::LbEndpoint>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl LbEndpointList

impl ::std::ops::Drop for LbEndpointList {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LbEndpointList {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LbEndpointList {
  type Proxied = Self;
  fn as_view(&self) -> LbEndpointListView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LbEndpointList {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LbEndpointListMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LbEndpointList {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::locality_lb_endpoints::envoy__config__endpoint__v3__LocalityLbEndpoints__LbEndpointList_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::locality_lb_endpoints::envoy__config__endpoint__v3__LocalityLbEndpoints__LbEndpointList_msg_init.0, &[<super::super::LbEndpoint as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::locality_lb_endpoints::envoy__config__endpoint__v3__LocalityLbEndpoints__LbEndpointList_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LbEndpointList {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LbEndpointList {
  type Msg = LbEndpointList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpointList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbEndpointList {
  type Msg = LbEndpointList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpointList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LbEndpointListMut<'_> {
  type Msg = LbEndpointList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpointList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbEndpointListMut<'_> {
  type Msg = LbEndpointList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpointList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbEndpointListView<'_> {
  type Msg = LbEndpointList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbEndpointList> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LbEndpointListMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum LbConfigOneof<'msg> {
  LoadBalancerEndpoints(::protobuf::View<'msg, super::super::locality_lb_endpoints::LbEndpointList>) = 7,
  LedsClusterLocalityConfig(::protobuf::View<'msg, super::super::LedsClusterLocalityConfig>) = 8,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum LbConfigCase {
  LoadBalancerEndpoints = 7,
  LedsClusterLocalityConfig = 8,

  not_set = 0
}

impl LbConfigCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<LbConfigCase> {
    match v {
      0 => Some(LbConfigCase::not_set),
      7 => Some(LbConfigCase::LoadBalancerEndpoints),
      8 => Some(LbConfigCase::LedsClusterLocalityConfig),
      _ => None
    }
  }
}
}  // pub mod locality_lb_endpoints


