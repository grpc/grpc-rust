const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__UdpListenerConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UdpListenerConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UdpListenerConfig>
}

impl ::protobuf::Message for UdpListenerConfig {
  type MessageView<'msg> = UdpListenerConfigView<'msg>;
  type MessageMut<'msg> = UdpListenerConfigMut<'msg>;
}

impl ::std::default::Default for UdpListenerConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UdpListenerConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UdpListenerConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `UdpListenerConfigMut`.
unsafe impl ::std::marker::Sync for UdpListenerConfig {}

// SAFETY:
// - `UdpListenerConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UdpListenerConfig {}

impl ::protobuf::Proxied for UdpListenerConfig {
  type View<'msg> = UdpListenerConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UdpListenerConfig {}

impl ::protobuf::MutProxied for UdpListenerConfig {
  type Mut<'msg> = UdpListenerConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UdpListenerConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UdpListenerConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UdpListenerConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UdpListenerConfigView<'msg> {
  type Message = UdpListenerConfig;
}

impl ::std::fmt::Debug for UdpListenerConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UdpListenerConfigView<'_> {
  fn default() -> UdpListenerConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UdpListenerConfig>> for UdpListenerConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UdpListenerConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UdpListenerConfigView<'msg> {

  pub fn to_owned(&self) -> UdpListenerConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // downstream_socket_config: optional message envoy.config.core.v3.UdpSocketConfig
  pub fn has_downstream_socket_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn downstream_socket_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfigView<'msg>> {
    self.has_downstream_socket_config().then(|| self.downstream_socket_config())
  }
  pub fn downstream_socket_config(self) -> crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfigView::default())
  }

  // quic_options: optional message envoy.config.listener.v3.QuicProtocolOptions
  pub fn has_quic_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn quic_options_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptionsView<'msg>> {
    self.has_quic_options().then(|| self.quic_options())
  }
  pub fn quic_options(self) -> crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptionsView::default())
  }

  // udp_packet_packet_writer_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_udp_packet_packet_writer_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn udp_packet_packet_writer_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_udp_packet_packet_writer_config().then(|| self.udp_packet_packet_writer_config())
  }
  pub fn udp_packet_packet_writer_config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

}

// SAFETY:
// - `UdpListenerConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UdpListenerConfigView<'_> {}

// SAFETY:
// - `UdpListenerConfigView` is `Send` because while its alive a `UdpListenerConfigMut` cannot.
// - `UdpListenerConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for UdpListenerConfigView<'_> {}

impl<'msg> ::protobuf::AsView for UdpListenerConfigView<'msg> {
  type Proxied = UdpListenerConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, UdpListenerConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UdpListenerConfigView<'msg> {
  fn into_view<'shorter>(self) -> UdpListenerConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UdpListenerConfig> for UdpListenerConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UdpListenerConfig {
    let mut dst = UdpListenerConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UdpListenerConfig> for UdpListenerConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UdpListenerConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UdpListenerConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UdpListenerConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UdpListenerConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UdpListenerConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UdpListenerConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UdpListenerConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UdpListenerConfigMut<'msg> {
  type Message = UdpListenerConfig;
}

impl ::std::fmt::Debug for UdpListenerConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UdpListenerConfig>> for UdpListenerConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UdpListenerConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UdpListenerConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UdpListenerConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UdpListenerConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // downstream_socket_config: optional message envoy.config.core.v3.UdpSocketConfig
  pub fn has_downstream_socket_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_downstream_socket_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn downstream_socket_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfigView<'_>> {
    self.has_downstream_socket_config().then(|| self.downstream_socket_config())
  }
  pub fn downstream_socket_config(&self) -> crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfigView::default())
  }
  pub fn downstream_socket_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfigMut<'_> {
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
  pub fn set_downstream_socket_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // quic_options: optional message envoy.config.listener.v3.QuicProtocolOptions
  pub fn has_quic_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_quic_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn quic_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptionsView<'_>> {
    self.has_quic_options().then(|| self.quic_options())
  }
  pub fn quic_options(&self) -> crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptionsView::default())
  }
  pub fn quic_options_mut(&mut self) -> crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptionsMut<'_> {
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
  pub fn set_quic_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // udp_packet_packet_writer_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_udp_packet_packet_writer_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_udp_packet_packet_writer_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn udp_packet_packet_writer_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_udp_packet_packet_writer_config().then(|| self.udp_packet_packet_writer_config())
  }
  pub fn udp_packet_packet_writer_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn udp_packet_packet_writer_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_udp_packet_packet_writer_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

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
// - `UdpListenerConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UdpListenerConfigMut<'_> {}

// SAFETY:
// - `UdpListenerConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UdpListenerConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for UdpListenerConfigMut<'msg> {
  type Proxied = UdpListenerConfig;
  fn as_view(&self) -> ::protobuf::View<'_, UdpListenerConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UdpListenerConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UdpListenerConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UdpListenerConfigMut<'msg> {
  type MutProxied = UdpListenerConfig;
  fn as_mut(&mut self) -> UdpListenerConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UdpListenerConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> UdpListenerConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UdpListenerConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UdpListenerConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UdpListenerConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UdpListenerConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // downstream_socket_config: optional message envoy.config.core.v3.UdpSocketConfig
  pub fn has_downstream_socket_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_downstream_socket_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn downstream_socket_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfigView<'_>> {
    self.has_downstream_socket_config().then(|| self.downstream_socket_config())
  }
  pub fn downstream_socket_config(&self) -> crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfigView::default())
  }
  pub fn downstream_socket_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfigMut<'_> {
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
  pub fn set_downstream_socket_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // quic_options: optional message envoy.config.listener.v3.QuicProtocolOptions
  pub fn has_quic_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_quic_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn quic_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptionsView<'_>> {
    self.has_quic_options().then(|| self.quic_options())
  }
  pub fn quic_options(&self) -> crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptionsView::default())
  }
  pub fn quic_options_mut(&mut self) -> crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptionsMut<'_> {
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
  pub fn set_quic_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // udp_packet_packet_writer_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_udp_packet_packet_writer_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_udp_packet_packet_writer_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn udp_packet_packet_writer_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_udp_packet_packet_writer_config().then(|| self.udp_packet_packet_writer_config())
  }
  pub fn udp_packet_packet_writer_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn udp_packet_packet_writer_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_udp_packet_packet_writer_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl UdpListenerConfig

impl ::std::ops::Drop for UdpListenerConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UdpListenerConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UdpListenerConfig {
  type Proxied = Self;
  fn as_view(&self) -> UdpListenerConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UdpListenerConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UdpListenerConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UdpListenerConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__UdpListenerConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$d3a33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__UdpListenerConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::udp_socket_config::UdpSocketConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::listener::v3::quic_config::QuicProtocolOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__UdpListenerConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UdpListenerConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UdpListenerConfig {
  type Msg = UdpListenerConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UdpListenerConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UdpListenerConfig {
  type Msg = UdpListenerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UdpListenerConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UdpListenerConfigMut<'_> {
  type Msg = UdpListenerConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UdpListenerConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UdpListenerConfigMut<'_> {
  type Msg = UdpListenerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UdpListenerConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UdpListenerConfigView<'_> {
  type Msg = UdpListenerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UdpListenerConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UdpListenerConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__ActiveRawUdpListenerConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ActiveRawUdpListenerConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ActiveRawUdpListenerConfig>
}

impl ::protobuf::Message for ActiveRawUdpListenerConfig {
  type MessageView<'msg> = ActiveRawUdpListenerConfigView<'msg>;
  type MessageMut<'msg> = ActiveRawUdpListenerConfigMut<'msg>;
}

impl ::std::default::Default for ActiveRawUdpListenerConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ActiveRawUdpListenerConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ActiveRawUdpListenerConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ActiveRawUdpListenerConfigMut`.
unsafe impl ::std::marker::Sync for ActiveRawUdpListenerConfig {}

// SAFETY:
// - `ActiveRawUdpListenerConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ActiveRawUdpListenerConfig {}

impl ::protobuf::Proxied for ActiveRawUdpListenerConfig {
  type View<'msg> = ActiveRawUdpListenerConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ActiveRawUdpListenerConfig {}

impl ::protobuf::MutProxied for ActiveRawUdpListenerConfig {
  type Mut<'msg> = ActiveRawUdpListenerConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ActiveRawUdpListenerConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ActiveRawUdpListenerConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ActiveRawUdpListenerConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ActiveRawUdpListenerConfigView<'msg> {
  type Message = ActiveRawUdpListenerConfig;
}

impl ::std::fmt::Debug for ActiveRawUdpListenerConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ActiveRawUdpListenerConfigView<'_> {
  fn default() -> ActiveRawUdpListenerConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ActiveRawUdpListenerConfig>> for ActiveRawUdpListenerConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ActiveRawUdpListenerConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ActiveRawUdpListenerConfigView<'msg> {

  pub fn to_owned(&self) -> ActiveRawUdpListenerConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ActiveRawUdpListenerConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ActiveRawUdpListenerConfigView<'_> {}

// SAFETY:
// - `ActiveRawUdpListenerConfigView` is `Send` because while its alive a `ActiveRawUdpListenerConfigMut` cannot.
// - `ActiveRawUdpListenerConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ActiveRawUdpListenerConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ActiveRawUdpListenerConfigView<'msg> {
  type Proxied = ActiveRawUdpListenerConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ActiveRawUdpListenerConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ActiveRawUdpListenerConfigView<'msg> {
  fn into_view<'shorter>(self) -> ActiveRawUdpListenerConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ActiveRawUdpListenerConfig> for ActiveRawUdpListenerConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ActiveRawUdpListenerConfig {
    let mut dst = ActiveRawUdpListenerConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ActiveRawUdpListenerConfig> for ActiveRawUdpListenerConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ActiveRawUdpListenerConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ActiveRawUdpListenerConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ActiveRawUdpListenerConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ActiveRawUdpListenerConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ActiveRawUdpListenerConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ActiveRawUdpListenerConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ActiveRawUdpListenerConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ActiveRawUdpListenerConfigMut<'msg> {
  type Message = ActiveRawUdpListenerConfig;
}

impl ::std::fmt::Debug for ActiveRawUdpListenerConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ActiveRawUdpListenerConfig>> for ActiveRawUdpListenerConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ActiveRawUdpListenerConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ActiveRawUdpListenerConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ActiveRawUdpListenerConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ActiveRawUdpListenerConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ActiveRawUdpListenerConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ActiveRawUdpListenerConfigMut<'_> {}

// SAFETY:
// - `ActiveRawUdpListenerConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ActiveRawUdpListenerConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ActiveRawUdpListenerConfigMut<'msg> {
  type Proxied = ActiveRawUdpListenerConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ActiveRawUdpListenerConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ActiveRawUdpListenerConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ActiveRawUdpListenerConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ActiveRawUdpListenerConfigMut<'msg> {
  type MutProxied = ActiveRawUdpListenerConfig;
  fn as_mut(&mut self) -> ActiveRawUdpListenerConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ActiveRawUdpListenerConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ActiveRawUdpListenerConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ActiveRawUdpListenerConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ActiveRawUdpListenerConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ActiveRawUdpListenerConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ActiveRawUdpListenerConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl ActiveRawUdpListenerConfig

impl ::std::ops::Drop for ActiveRawUdpListenerConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ActiveRawUdpListenerConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ActiveRawUdpListenerConfig {
  type Proxied = Self;
  fn as_view(&self) -> ActiveRawUdpListenerConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ActiveRawUdpListenerConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ActiveRawUdpListenerConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ActiveRawUdpListenerConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__ActiveRawUdpListenerConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__ActiveRawUdpListenerConfig_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__ActiveRawUdpListenerConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ActiveRawUdpListenerConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ActiveRawUdpListenerConfig {
  type Msg = ActiveRawUdpListenerConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ActiveRawUdpListenerConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ActiveRawUdpListenerConfig {
  type Msg = ActiveRawUdpListenerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ActiveRawUdpListenerConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ActiveRawUdpListenerConfigMut<'_> {
  type Msg = ActiveRawUdpListenerConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ActiveRawUdpListenerConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ActiveRawUdpListenerConfigMut<'_> {
  type Msg = ActiveRawUdpListenerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ActiveRawUdpListenerConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ActiveRawUdpListenerConfigView<'_> {
  type Msg = ActiveRawUdpListenerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ActiveRawUdpListenerConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ActiveRawUdpListenerConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



