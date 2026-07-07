const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__AdditionalAddress_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // socket_options: optional message envoy.config.core.v3.SocketOptionsOverride
  pub fn has_socket_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn socket_options_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'msg>> {
    self.has_socket_options().then(|| self.socket_options())
  }
  pub fn socket_options(self) -> crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView::default())
  }

  // tcp_keepalive: optional message envoy.config.core.v3.TcpKeepalive
  pub fn has_tcp_keepalive(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn tcp_keepalive_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'msg>> {
    self.has_tcp_keepalive().then(|| self.tcp_keepalive())
  }
  pub fn tcp_keepalive(self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView::default())
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

  // socket_options: optional message envoy.config.core.v3.SocketOptionsOverride
  pub fn has_socket_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_socket_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn socket_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'_>> {
    self.has_socket_options().then(|| self.socket_options())
  }
  pub fn socket_options(&self) -> crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView::default())
  }
  pub fn socket_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideMut<'_> {
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
  pub fn set_socket_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverride>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // tcp_keepalive: optional message envoy.config.core.v3.TcpKeepalive
  pub fn has_tcp_keepalive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_tcp_keepalive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn tcp_keepalive_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_>> {
    self.has_tcp_keepalive().then(|| self.tcp_keepalive())
  }
  pub fn tcp_keepalive(&self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView::default())
  }
  pub fn tcp_keepalive_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveMut<'_> {
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
  pub fn set_tcp_keepalive(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::TcpKeepalive>) {

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

  // socket_options: optional message envoy.config.core.v3.SocketOptionsOverride
  pub fn has_socket_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_socket_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn socket_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'_>> {
    self.has_socket_options().then(|| self.socket_options())
  }
  pub fn socket_options(&self) -> crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView::default())
  }
  pub fn socket_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideMut<'_> {
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
  pub fn set_socket_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverride>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // tcp_keepalive: optional message envoy.config.core.v3.TcpKeepalive
  pub fn has_tcp_keepalive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_tcp_keepalive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn tcp_keepalive_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_>> {
    self.has_tcp_keepalive().then(|| self.tcp_keepalive())
  }
  pub fn tcp_keepalive(&self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView::default())
  }
  pub fn tcp_keepalive_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveMut<'_> {
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
  pub fn set_tcp_keepalive(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::TcpKeepalive>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
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
        super::envoy__config__listener__v3__AdditionalAddress_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__AdditionalAddress_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverride as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::TcpKeepalive as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__AdditionalAddress_msg_init.0)
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



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__ListenerCollection_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListenerCollection {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListenerCollection>
}

impl ::protobuf::Message for ListenerCollection {
  type MessageView<'msg> = ListenerCollectionView<'msg>;
  type MessageMut<'msg> = ListenerCollectionMut<'msg>;
}

impl ::std::default::Default for ListenerCollection {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListenerCollection {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListenerCollection` is `Sync` because it does not implement interior mutability.
//    Neither does `ListenerCollectionMut`.
unsafe impl ::std::marker::Sync for ListenerCollection {}

// SAFETY:
// - `ListenerCollection` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ListenerCollection {}

impl ::protobuf::Proxied for ListenerCollection {
  type View<'msg> = ListenerCollectionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListenerCollection {}

impl ::protobuf::MutProxied for ListenerCollection {
  type Mut<'msg> = ListenerCollectionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListenerCollectionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerCollection>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenerCollectionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListenerCollectionView<'msg> {
  type Message = ListenerCollection;
}

impl ::std::fmt::Debug for ListenerCollectionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListenerCollectionView<'_> {
  fn default() -> ListenerCollectionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerCollection>> for ListenerCollectionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerCollection>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenerCollectionView<'msg> {

  pub fn to_owned(&self) -> ListenerCollection {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // entries: repeated message xds.core.v3.CollectionEntry
  pub fn entries(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ListenerCollectionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ListenerCollectionView<'_> {}

// SAFETY:
// - `ListenerCollectionView` is `Send` because while its alive a `ListenerCollectionMut` cannot.
// - `ListenerCollectionView` does not use thread-local data.
unsafe impl ::std::marker::Send for ListenerCollectionView<'_> {}

impl<'msg> ::protobuf::AsView for ListenerCollectionView<'msg> {
  type Proxied = ListenerCollection;
  fn as_view(&self) -> ::protobuf::View<'msg, ListenerCollection> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenerCollectionView<'msg> {
  fn into_view<'shorter>(self) -> ListenerCollectionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListenerCollection> for ListenerCollectionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListenerCollection {
    let mut dst = ListenerCollection::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListenerCollection> for ListenerCollectionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListenerCollection {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ListenerCollection {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenerCollectionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenerCollectionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListenerCollectionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerCollection>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenerCollectionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListenerCollectionMut<'msg> {
  type Message = ListenerCollection;
}

impl ::std::fmt::Debug for ListenerCollectionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerCollection>> for ListenerCollectionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerCollection>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenerCollectionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerCollection> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ListenerCollection {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // entries: repeated message xds.core.v3.CollectionEntry
  pub fn entries(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn entries_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry> {
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
  pub fn set_entries(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ListenerCollectionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ListenerCollectionMut<'_> {}

// SAFETY:
// - `ListenerCollectionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ListenerCollectionMut<'_> {}

impl<'msg> ::protobuf::AsView for ListenerCollectionMut<'msg> {
  type Proxied = ListenerCollection;
  fn as_view(&self) -> ::protobuf::View<'_, ListenerCollection> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenerCollectionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListenerCollection>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ListenerCollectionMut<'msg> {
  type MutProxied = ListenerCollection;
  fn as_mut(&mut self) -> ListenerCollectionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListenerCollectionMut<'msg> {
  fn into_mut<'shorter>(self) -> ListenerCollectionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListenerCollection {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListenerCollection> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListenerCollectionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListenerCollectionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // entries: repeated message xds.core.v3.CollectionEntry
  pub fn entries(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn entries_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry> {
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
  pub fn set_entries(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ListenerCollection

impl ::std::ops::Drop for ListenerCollection {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListenerCollection {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListenerCollection {
  type Proxied = Self;
  fn as_view(&self) -> ListenerCollectionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListenerCollection {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListenerCollectionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListenerCollection {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__ListenerCollection_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__ListenerCollection_msg_init.0, &[<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__ListenerCollection_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListenerCollection {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListenerCollection {
  type Msg = ListenerCollection;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerCollection> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerCollection {
  type Msg = ListenerCollection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerCollection> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListenerCollectionMut<'_> {
  type Msg = ListenerCollection;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerCollection> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerCollectionMut<'_> {
  type Msg = ListenerCollection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerCollection> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerCollectionView<'_> {
  type Msg = ListenerCollection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerCollection> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListenerCollectionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__Listener_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Listener {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Listener>
}

impl ::protobuf::Message for Listener {
  type MessageView<'msg> = ListenerView<'msg>;
  type MessageMut<'msg> = ListenerMut<'msg>;
}

impl ::std::default::Default for Listener {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Listener {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Listener` is `Sync` because it does not implement interior mutability.
//    Neither does `ListenerMut`.
unsafe impl ::std::marker::Sync for Listener {}

// SAFETY:
// - `Listener` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Listener {}

impl ::protobuf::Proxied for Listener {
  type View<'msg> = ListenerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Listener {}

impl ::protobuf::MutProxied for Listener {
  type Mut<'msg> = ListenerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListenerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Listener>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListenerView<'msg> {
  type Message = Listener;
}

impl ::std::fmt::Debug for ListenerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListenerView<'_> {
  fn default() -> ListenerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Listener>> for ListenerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Listener>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenerView<'msg> {

  pub fn to_owned(&self) -> Listener {
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

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

  // additional_addresses: repeated message envoy.config.listener.v3.AdditionalAddress
  pub fn additional_addresses(self) -> ::protobuf::RepeatedView<'msg, super::AdditionalAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        30
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::AdditionalAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // stat_prefix: optional string
  pub fn stat_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        25, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // filter_chains: repeated message envoy.config.listener.v3.FilterChain
  pub fn filter_chains(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // fcds_config: optional message envoy.config.listener.v3.Listener.FcdsConfig
  pub fn has_fcds_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(33)
    }
  }
  pub fn fcds_config_opt(self) -> ::std::option::Option<super::listener::FcdsConfigView<'msg>> {
    self.has_fcds_config().then(|| self.fcds_config())
  }
  pub fn fcds_config(self) -> super::listener::FcdsConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(33)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::FcdsConfigView::default())
  }

  // filter_chain_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_filter_chain_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(29)
    }
  }
  pub fn filter_chain_matcher_opt(self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg>> {
    self.has_filter_chain_matcher().then(|| self.filter_chain_matcher())
  }
  pub fn filter_chain_matcher(self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(29)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }

  // use_original_dst: optional message google.protobuf.BoolValue
  pub fn has_use_original_dst(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn use_original_dst_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_use_original_dst().then(|| self.use_original_dst())
  }
  pub fn use_original_dst(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // default_filter_chain: optional message envoy.config.listener.v3.FilterChain
  pub fn has_default_filter_chain(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn default_filter_chain_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChainView<'msg>> {
    self.has_default_filter_chain().then(|| self.default_filter_chain())
  }
  pub fn default_filter_chain(self) -> crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChainView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChainView::default())
  }

  // per_connection_buffer_limit_bytes: optional message google.protobuf.UInt32Value
  pub fn has_per_connection_buffer_limit_bytes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn per_connection_buffer_limit_bytes_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_per_connection_buffer_limit_bytes().then(|| self.per_connection_buffer_limit_bytes())
  }
  pub fn per_connection_buffer_limit_bytes(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }

  // deprecated_v1: optional message envoy.config.listener.v3.Listener.DeprecatedV1
  pub fn has_deprecated_v1(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn deprecated_v1_opt(self) -> ::std::option::Option<super::listener::DeprecatedV1View<'msg>> {
    self.has_deprecated_v1().then(|| self.deprecated_v1())
  }
  pub fn deprecated_v1(self) -> super::listener::DeprecatedV1View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::DeprecatedV1View::default())
  }

  // drain_type: optional enum envoy.config.listener.v3.Listener.DrainType
  pub fn drain_type(self) -> super::listener::DrainType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::listener::DrainType::Default).into()
      ).try_into().unwrap()
    }
  }

  // listener_filters: repeated message envoy.config.listener.v3.ListenerFilter
  pub fn listener_filters(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::listener::v3::listener_components::ListenerFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::listener::v3::listener_components::ListenerFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // listener_filters_timeout: optional message google.protobuf.Duration
  pub fn has_listener_filters_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn listener_filters_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_listener_filters_timeout().then(|| self.listener_filters_timeout())
  }
  pub fn listener_filters_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // continue_on_listener_filters_timeout: optional bool
  pub fn continue_on_listener_filters_timeout(self) -> bool {
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

  // transparent: optional message google.protobuf.BoolValue
  pub fn has_transparent(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn transparent_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_transparent().then(|| self.transparent())
  }
  pub fn transparent(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // freebind: optional message google.protobuf.BoolValue
  pub fn has_freebind(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn freebind_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_freebind().then(|| self.freebind())
  }
  pub fn freebind(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // tcp_fast_open_queue_length: optional message google.protobuf.UInt32Value
  pub fn has_tcp_fast_open_queue_length(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn tcp_fast_open_queue_length_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_tcp_fast_open_queue_length().then(|| self.tcp_fast_open_queue_length())
  }
  pub fn tcp_fast_open_queue_length(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // traffic_direction: optional enum envoy.config.core.v3.TrafficDirection
  pub fn traffic_direction(self) -> crate::xds::generated::envoy::config::core::v3::base::TrafficDirection {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (crate::xds::generated::envoy::config::core::v3::base::TrafficDirection::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // udp_listener_config: optional message envoy.config.listener.v3.UdpListenerConfig
  pub fn has_udp_listener_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn udp_listener_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfigView<'msg>> {
    self.has_udp_listener_config().then(|| self.udp_listener_config())
  }
  pub fn udp_listener_config(self) -> crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfigView::default())
  }

  // api_listener: optional message envoy.config.listener.v3.ApiListener
  pub fn has_api_listener(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn api_listener_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListenerView<'msg>> {
    self.has_api_listener().then(|| self.api_listener())
  }
  pub fn api_listener(self) -> crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListenerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListenerView::default())
  }

  // connection_balance_config: optional message envoy.config.listener.v3.Listener.ConnectionBalanceConfig
  pub fn has_connection_balance_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn connection_balance_config_opt(self) -> ::std::option::Option<super::listener::ConnectionBalanceConfigView<'msg>> {
    self.has_connection_balance_config().then(|| self.connection_balance_config())
  }
  pub fn connection_balance_config(self) -> super::listener::ConnectionBalanceConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::ConnectionBalanceConfigView::default())
  }

  // reuse_port: optional bool
  pub fn reuse_port(self) -> bool {
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

  // enable_reuse_port: optional message google.protobuf.BoolValue
  pub fn has_enable_reuse_port(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn enable_reuse_port_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_enable_reuse_port().then(|| self.enable_reuse_port())
  }
  pub fn enable_reuse_port(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(26)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // access_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn access_log(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // tcp_backlog_size: optional message google.protobuf.UInt32Value
  pub fn has_tcp_backlog_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn tcp_backlog_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_tcp_backlog_size().then(|| self.tcp_backlog_size())
  }
  pub fn tcp_backlog_size(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // max_connections_to_accept_per_socket_event: optional message google.protobuf.UInt32Value
  pub fn has_max_connections_to_accept_per_socket_event(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(31)
    }
  }
  pub fn max_connections_to_accept_per_socket_event_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_connections_to_accept_per_socket_event().then(|| self.max_connections_to_accept_per_socket_event())
  }
  pub fn max_connections_to_accept_per_socket_event(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(31)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // bind_to_port: optional message google.protobuf.BoolValue
  pub fn has_bind_to_port(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn bind_to_port_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_bind_to_port().then(|| self.bind_to_port())
  }
  pub fn bind_to_port(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // internal_listener: optional message envoy.config.listener.v3.Listener.InternalListenerConfig
  pub fn has_internal_listener(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn internal_listener_opt(self) -> ::std::option::Option<super::listener::InternalListenerConfigView<'msg>> {
    self.has_internal_listener().then(|| self.internal_listener())
  }
  pub fn internal_listener(self) -> super::listener::InternalListenerConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::InternalListenerConfigView::default())
  }

  // enable_mptcp: optional bool
  pub fn enable_mptcp(self) -> bool {
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

  // ignore_global_conn_limit: optional bool
  pub fn ignore_global_conn_limit(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        28, (false).into()
      ).try_into().unwrap()
    }
  }

  // bypass_overload_manager: optional bool
  pub fn bypass_overload_manager(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        32, (false).into()
      ).try_into().unwrap()
    }
  }

  // tcp_keepalive: optional message envoy.config.core.v3.TcpKeepalive
  pub fn has_tcp_keepalive(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(34)
    }
  }
  pub fn tcp_keepalive_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'msg>> {
    self.has_tcp_keepalive().then(|| self.tcp_keepalive())
  }
  pub fn tcp_keepalive(self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(34)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView::default())
  }

  pub fn listener_specifier(self) -> super::listener::ListenerSpecifierOneof<'msg> {
    match self.listener_specifier_case() {
      super::listener::ListenerSpecifierCase::InternalListener =>
          super::listener::ListenerSpecifierOneof::InternalListener(self.internal_listener()),
      _ => super::listener::ListenerSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn listener_specifier_case(self) -> super::listener::ListenerSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(24);
      super::listener::ListenerSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ListenerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ListenerView<'_> {}

// SAFETY:
// - `ListenerView` is `Send` because while its alive a `ListenerMut` cannot.
// - `ListenerView` does not use thread-local data.
unsafe impl ::std::marker::Send for ListenerView<'_> {}

impl<'msg> ::protobuf::AsView for ListenerView<'msg> {
  type Proxied = Listener;
  fn as_view(&self) -> ::protobuf::View<'msg, Listener> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenerView<'msg> {
  fn into_view<'shorter>(self) -> ListenerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Listener> for ListenerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Listener {
    let mut dst = Listener::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Listener> for ListenerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Listener {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Listener {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListenerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Listener>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListenerMut<'msg> {
  type Message = Listener;
}

impl ::std::fmt::Debug for ListenerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Listener>> for ListenerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Listener>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Listener> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Listener {
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

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // additional_addresses: repeated message envoy.config.listener.v3.AdditionalAddress
  pub fn additional_addresses(&self) -> ::protobuf::RepeatedView<'_, super::AdditionalAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        30
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::AdditionalAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn additional_addresses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::AdditionalAddress> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        30,
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
  pub fn set_additional_addresses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::AdditionalAddress>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        30,
        src);
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        25, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val);
    }
  }

  // filter_chains: repeated message envoy.config.listener.v3.FilterChain
  pub fn filter_chains(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filter_chains_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain> {
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
  pub fn set_filter_chains(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // fcds_config: optional message envoy.config.listener.v3.Listener.FcdsConfig
  pub fn has_fcds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(33)
    }
  }
  pub fn clear_fcds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        33
      );
    }
  }
  pub fn fcds_config_opt(&self) -> ::std::option::Option<super::listener::FcdsConfigView<'_>> {
    self.has_fcds_config().then(|| self.fcds_config())
  }
  pub fn fcds_config(&self) -> super::listener::FcdsConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(33)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::FcdsConfigView::default())
  }
  pub fn fcds_config_mut(&mut self) -> super::listener::FcdsConfigMut<'_> {
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
  pub fn set_fcds_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener::FcdsConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        33,
        val
      );
    }
  }

  // filter_chain_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_filter_chain_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(29)
    }
  }
  pub fn clear_filter_chain_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        29
      );
    }
  }
  pub fn filter_chain_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_filter_chain_matcher().then(|| self.filter_chain_matcher())
  }
  pub fn filter_chain_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(29)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn filter_chain_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         29, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_filter_chain_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        29,
        val
      );
    }
  }

  // use_original_dst: optional message google.protobuf.BoolValue
  pub fn has_use_original_dst(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_use_original_dst(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn use_original_dst_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_original_dst().then(|| self.use_original_dst())
  }
  pub fn use_original_dst(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_original_dst_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_original_dst(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // default_filter_chain: optional message envoy.config.listener.v3.FilterChain
  pub fn has_default_filter_chain(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_default_filter_chain(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn default_filter_chain_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChainView<'_>> {
    self.has_default_filter_chain().then(|| self.default_filter_chain())
  }
  pub fn default_filter_chain(&self) -> crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChainView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChainView::default())
  }
  pub fn default_filter_chain_mut(&mut self) -> crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChainMut<'_> {
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
  pub fn set_default_filter_chain(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // per_connection_buffer_limit_bytes: optional message google.protobuf.UInt32Value
  pub fn has_per_connection_buffer_limit_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_per_connection_buffer_limit_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn per_connection_buffer_limit_bytes_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_per_connection_buffer_limit_bytes().then(|| self.per_connection_buffer_limit_bytes())
  }
  pub fn per_connection_buffer_limit_bytes(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn per_connection_buffer_limit_bytes_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_per_connection_buffer_limit_bytes(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // deprecated_v1: optional message envoy.config.listener.v3.Listener.DeprecatedV1
  pub fn has_deprecated_v1(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_deprecated_v1(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn deprecated_v1_opt(&self) -> ::std::option::Option<super::listener::DeprecatedV1View<'_>> {
    self.has_deprecated_v1().then(|| self.deprecated_v1())
  }
  pub fn deprecated_v1(&self) -> super::listener::DeprecatedV1View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::DeprecatedV1View::default())
  }
  pub fn deprecated_v1_mut(&mut self) -> super::listener::DeprecatedV1Mut<'_> {
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
  pub fn set_deprecated_v1(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener::DeprecatedV1>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // drain_type: optional enum envoy.config.listener.v3.Listener.DrainType
  pub fn drain_type(&self) -> super::listener::DrainType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::listener::DrainType::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_drain_type(&mut self, val: super::listener::DrainType) {
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

  // listener_filters: repeated message envoy.config.listener.v3.ListenerFilter
  pub fn listener_filters(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::listener::v3::listener_components::ListenerFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::listener::v3::listener_components::ListenerFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn listener_filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::listener::v3::listener_components::ListenerFilter> {
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
  pub fn set_listener_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::listener::v3::listener_components::ListenerFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // listener_filters_timeout: optional message google.protobuf.Duration
  pub fn has_listener_filters_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_listener_filters_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn listener_filters_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_listener_filters_timeout().then(|| self.listener_filters_timeout())
  }
  pub fn listener_filters_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn listener_filters_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_listener_filters_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // continue_on_listener_filters_timeout: optional bool
  pub fn continue_on_listener_filters_timeout(&self) -> bool {
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
  pub fn set_continue_on_listener_filters_timeout(&mut self, val: bool) {
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

  // transparent: optional message google.protobuf.BoolValue
  pub fn has_transparent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_transparent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn transparent_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_transparent().then(|| self.transparent())
  }
  pub fn transparent(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn transparent_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_transparent(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // freebind: optional message google.protobuf.BoolValue
  pub fn has_freebind(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_freebind(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn freebind_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_freebind().then(|| self.freebind())
  }
  pub fn freebind(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn freebind_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_freebind(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn socket_options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
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
  pub fn set_socket_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        src);
    }
  }

  // tcp_fast_open_queue_length: optional message google.protobuf.UInt32Value
  pub fn has_tcp_fast_open_queue_length(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_tcp_fast_open_queue_length(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn tcp_fast_open_queue_length_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_tcp_fast_open_queue_length().then(|| self.tcp_fast_open_queue_length())
  }
  pub fn tcp_fast_open_queue_length(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn tcp_fast_open_queue_length_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_tcp_fast_open_queue_length(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // traffic_direction: optional enum envoy.config.core.v3.TrafficDirection
  pub fn traffic_direction(&self) -> crate::xds::generated::envoy::config::core::v3::base::TrafficDirection {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (crate::xds::generated::envoy::config::core::v3::base::TrafficDirection::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_traffic_direction(&mut self, val: crate::xds::generated::envoy::config::core::v3::base::TrafficDirection) {
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

  // udp_listener_config: optional message envoy.config.listener.v3.UdpListenerConfig
  pub fn has_udp_listener_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_udp_listener_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn udp_listener_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfigView<'_>> {
    self.has_udp_listener_config().then(|| self.udp_listener_config())
  }
  pub fn udp_listener_config(&self) -> crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfigView::default())
  }
  pub fn udp_listener_config_mut(&mut self) -> crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfigMut<'_> {
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
  pub fn set_udp_listener_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // api_listener: optional message envoy.config.listener.v3.ApiListener
  pub fn has_api_listener(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_api_listener(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn api_listener_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListenerView<'_>> {
    self.has_api_listener().then(|| self.api_listener())
  }
  pub fn api_listener(&self) -> crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListenerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListenerView::default())
  }
  pub fn api_listener_mut(&mut self) -> crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListenerMut<'_> {
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
  pub fn set_api_listener(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListener>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

  // connection_balance_config: optional message envoy.config.listener.v3.Listener.ConnectionBalanceConfig
  pub fn has_connection_balance_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_connection_balance_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn connection_balance_config_opt(&self) -> ::std::option::Option<super::listener::ConnectionBalanceConfigView<'_>> {
    self.has_connection_balance_config().then(|| self.connection_balance_config())
  }
  pub fn connection_balance_config(&self) -> super::listener::ConnectionBalanceConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::ConnectionBalanceConfigView::default())
  }
  pub fn connection_balance_config_mut(&mut self) -> super::listener::ConnectionBalanceConfigMut<'_> {
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
  pub fn set_connection_balance_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener::ConnectionBalanceConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
    }
  }

  // reuse_port: optional bool
  pub fn reuse_port(&self) -> bool {
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
  pub fn set_reuse_port(&mut self, val: bool) {
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

  // enable_reuse_port: optional message google.protobuf.BoolValue
  pub fn has_enable_reuse_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn clear_enable_reuse_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        26
      );
    }
  }
  pub fn enable_reuse_port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enable_reuse_port().then(|| self.enable_reuse_port())
  }
  pub fn enable_reuse_port(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(26)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enable_reuse_port_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_enable_reuse_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        26,
        val
      );
    }
  }

  // access_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn access_log(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
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
  pub fn set_access_log(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        src);
    }
  }

  // tcp_backlog_size: optional message google.protobuf.UInt32Value
  pub fn has_tcp_backlog_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_tcp_backlog_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn tcp_backlog_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_tcp_backlog_size().then(|| self.tcp_backlog_size())
  }
  pub fn tcp_backlog_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn tcp_backlog_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_tcp_backlog_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val
      );
    }
  }

  // max_connections_to_accept_per_socket_event: optional message google.protobuf.UInt32Value
  pub fn has_max_connections_to_accept_per_socket_event(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(31)
    }
  }
  pub fn clear_max_connections_to_accept_per_socket_event(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        31
      );
    }
  }
  pub fn max_connections_to_accept_per_socket_event_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_connections_to_accept_per_socket_event().then(|| self.max_connections_to_accept_per_socket_event())
  }
  pub fn max_connections_to_accept_per_socket_event(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(31)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_connections_to_accept_per_socket_event_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         31, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_max_connections_to_accept_per_socket_event(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        31,
        val
      );
    }
  }

  // bind_to_port: optional message google.protobuf.BoolValue
  pub fn has_bind_to_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_bind_to_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn bind_to_port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_bind_to_port().then(|| self.bind_to_port())
  }
  pub fn bind_to_port(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn bind_to_port_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_bind_to_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val
      );
    }
  }

  // internal_listener: optional message envoy.config.listener.v3.Listener.InternalListenerConfig
  pub fn has_internal_listener(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_internal_listener(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn internal_listener_opt(&self) -> ::std::option::Option<super::listener::InternalListenerConfigView<'_>> {
    self.has_internal_listener().then(|| self.internal_listener())
  }
  pub fn internal_listener(&self) -> super::listener::InternalListenerConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::InternalListenerConfigView::default())
  }
  pub fn internal_listener_mut(&mut self) -> super::listener::InternalListenerConfigMut<'_> {
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
  pub fn set_internal_listener(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener::InternalListenerConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // enable_mptcp: optional bool
  pub fn enable_mptcp(&self) -> bool {
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
  pub fn set_enable_mptcp(&mut self, val: bool) {
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

  // ignore_global_conn_limit: optional bool
  pub fn ignore_global_conn_limit(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        28, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_global_conn_limit(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        28, val.into()
      )
    }
  }

  // bypass_overload_manager: optional bool
  pub fn bypass_overload_manager(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        32, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bypass_overload_manager(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        32, val.into()
      )
    }
  }

  // tcp_keepalive: optional message envoy.config.core.v3.TcpKeepalive
  pub fn has_tcp_keepalive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(34)
    }
  }
  pub fn clear_tcp_keepalive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        34
      );
    }
  }
  pub fn tcp_keepalive_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_>> {
    self.has_tcp_keepalive().then(|| self.tcp_keepalive())
  }
  pub fn tcp_keepalive(&self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(34)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView::default())
  }
  pub fn tcp_keepalive_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         34, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tcp_keepalive(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::TcpKeepalive>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        34,
        val
      );
    }
  }

  pub fn listener_specifier(&self) -> super::listener::ListenerSpecifierOneof<'_> {
    match &self.listener_specifier_case() {
      super::listener::ListenerSpecifierCase::InternalListener =>
          super::listener::ListenerSpecifierOneof::InternalListener(self.internal_listener()),
      _ => super::listener::ListenerSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn listener_specifier_case(&self) -> super::listener::ListenerSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(24);
      super::listener::ListenerSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ListenerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ListenerMut<'_> {}

// SAFETY:
// - `ListenerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ListenerMut<'_> {}

impl<'msg> ::protobuf::AsView for ListenerMut<'msg> {
  type Proxied = Listener;
  fn as_view(&self) -> ::protobuf::View<'_, Listener> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Listener>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ListenerMut<'msg> {
  type MutProxied = Listener;
  fn as_mut(&mut self) -> ListenerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListenerMut<'msg> {
  fn into_mut<'shorter>(self) -> ListenerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Listener {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Listener> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListenerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListenerMut<'_> {
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

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // additional_addresses: repeated message envoy.config.listener.v3.AdditionalAddress
  pub fn additional_addresses(&self) -> ::protobuf::RepeatedView<'_, super::AdditionalAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        30
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::AdditionalAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn additional_addresses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::AdditionalAddress> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        30,
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
  pub fn set_additional_addresses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::AdditionalAddress>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        30,
        src);
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        25, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val);
    }
  }

  // filter_chains: repeated message envoy.config.listener.v3.FilterChain
  pub fn filter_chains(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filter_chains_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain> {
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
  pub fn set_filter_chains(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // fcds_config: optional message envoy.config.listener.v3.Listener.FcdsConfig
  pub fn has_fcds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(33)
    }
  }
  pub fn clear_fcds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        33
      );
    }
  }
  pub fn fcds_config_opt(&self) -> ::std::option::Option<super::listener::FcdsConfigView<'_>> {
    self.has_fcds_config().then(|| self.fcds_config())
  }
  pub fn fcds_config(&self) -> super::listener::FcdsConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(33)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::FcdsConfigView::default())
  }
  pub fn fcds_config_mut(&mut self) -> super::listener::FcdsConfigMut<'_> {
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
  pub fn set_fcds_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener::FcdsConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        33,
        val
      );
    }
  }

  // filter_chain_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_filter_chain_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(29)
    }
  }
  pub fn clear_filter_chain_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        29
      );
    }
  }
  pub fn filter_chain_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_filter_chain_matcher().then(|| self.filter_chain_matcher())
  }
  pub fn filter_chain_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(29)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn filter_chain_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         29, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_filter_chain_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        29,
        val
      );
    }
  }

  // use_original_dst: optional message google.protobuf.BoolValue
  pub fn has_use_original_dst(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_use_original_dst(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn use_original_dst_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_original_dst().then(|| self.use_original_dst())
  }
  pub fn use_original_dst(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_original_dst_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_original_dst(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // default_filter_chain: optional message envoy.config.listener.v3.FilterChain
  pub fn has_default_filter_chain(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_default_filter_chain(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn default_filter_chain_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChainView<'_>> {
    self.has_default_filter_chain().then(|| self.default_filter_chain())
  }
  pub fn default_filter_chain(&self) -> crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChainView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChainView::default())
  }
  pub fn default_filter_chain_mut(&mut self) -> crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChainMut<'_> {
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
  pub fn set_default_filter_chain(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // per_connection_buffer_limit_bytes: optional message google.protobuf.UInt32Value
  pub fn has_per_connection_buffer_limit_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_per_connection_buffer_limit_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn per_connection_buffer_limit_bytes_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_per_connection_buffer_limit_bytes().then(|| self.per_connection_buffer_limit_bytes())
  }
  pub fn per_connection_buffer_limit_bytes(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn per_connection_buffer_limit_bytes_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_per_connection_buffer_limit_bytes(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // deprecated_v1: optional message envoy.config.listener.v3.Listener.DeprecatedV1
  pub fn has_deprecated_v1(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_deprecated_v1(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn deprecated_v1_opt(&self) -> ::std::option::Option<super::listener::DeprecatedV1View<'_>> {
    self.has_deprecated_v1().then(|| self.deprecated_v1())
  }
  pub fn deprecated_v1(&self) -> super::listener::DeprecatedV1View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::DeprecatedV1View::default())
  }
  pub fn deprecated_v1_mut(&mut self) -> super::listener::DeprecatedV1Mut<'_> {
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
  pub fn set_deprecated_v1(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener::DeprecatedV1>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // drain_type: optional enum envoy.config.listener.v3.Listener.DrainType
  pub fn drain_type(&self) -> super::listener::DrainType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::listener::DrainType::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_drain_type(&mut self, val: super::listener::DrainType) {
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

  // listener_filters: repeated message envoy.config.listener.v3.ListenerFilter
  pub fn listener_filters(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::listener::v3::listener_components::ListenerFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::listener::v3::listener_components::ListenerFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn listener_filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::listener::v3::listener_components::ListenerFilter> {
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
  pub fn set_listener_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::listener::v3::listener_components::ListenerFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // listener_filters_timeout: optional message google.protobuf.Duration
  pub fn has_listener_filters_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_listener_filters_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn listener_filters_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_listener_filters_timeout().then(|| self.listener_filters_timeout())
  }
  pub fn listener_filters_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn listener_filters_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_listener_filters_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // continue_on_listener_filters_timeout: optional bool
  pub fn continue_on_listener_filters_timeout(&self) -> bool {
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
  pub fn set_continue_on_listener_filters_timeout(&mut self, val: bool) {
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

  // transparent: optional message google.protobuf.BoolValue
  pub fn has_transparent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_transparent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn transparent_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_transparent().then(|| self.transparent())
  }
  pub fn transparent(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn transparent_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_transparent(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // freebind: optional message google.protobuf.BoolValue
  pub fn has_freebind(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_freebind(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn freebind_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_freebind().then(|| self.freebind())
  }
  pub fn freebind(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn freebind_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_freebind(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn socket_options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
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
  pub fn set_socket_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        src);
    }
  }

  // tcp_fast_open_queue_length: optional message google.protobuf.UInt32Value
  pub fn has_tcp_fast_open_queue_length(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_tcp_fast_open_queue_length(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn tcp_fast_open_queue_length_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_tcp_fast_open_queue_length().then(|| self.tcp_fast_open_queue_length())
  }
  pub fn tcp_fast_open_queue_length(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn tcp_fast_open_queue_length_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_tcp_fast_open_queue_length(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // traffic_direction: optional enum envoy.config.core.v3.TrafficDirection
  pub fn traffic_direction(&self) -> crate::xds::generated::envoy::config::core::v3::base::TrafficDirection {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (crate::xds::generated::envoy::config::core::v3::base::TrafficDirection::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_traffic_direction(&mut self, val: crate::xds::generated::envoy::config::core::v3::base::TrafficDirection) {
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

  // udp_listener_config: optional message envoy.config.listener.v3.UdpListenerConfig
  pub fn has_udp_listener_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_udp_listener_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn udp_listener_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfigView<'_>> {
    self.has_udp_listener_config().then(|| self.udp_listener_config())
  }
  pub fn udp_listener_config(&self) -> crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfigView::default())
  }
  pub fn udp_listener_config_mut(&mut self) -> crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfigMut<'_> {
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
  pub fn set_udp_listener_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // api_listener: optional message envoy.config.listener.v3.ApiListener
  pub fn has_api_listener(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_api_listener(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn api_listener_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListenerView<'_>> {
    self.has_api_listener().then(|| self.api_listener())
  }
  pub fn api_listener(&self) -> crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListenerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListenerView::default())
  }
  pub fn api_listener_mut(&mut self) -> crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListenerMut<'_> {
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
  pub fn set_api_listener(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListener>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

  // connection_balance_config: optional message envoy.config.listener.v3.Listener.ConnectionBalanceConfig
  pub fn has_connection_balance_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_connection_balance_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn connection_balance_config_opt(&self) -> ::std::option::Option<super::listener::ConnectionBalanceConfigView<'_>> {
    self.has_connection_balance_config().then(|| self.connection_balance_config())
  }
  pub fn connection_balance_config(&self) -> super::listener::ConnectionBalanceConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::ConnectionBalanceConfigView::default())
  }
  pub fn connection_balance_config_mut(&mut self) -> super::listener::ConnectionBalanceConfigMut<'_> {
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
  pub fn set_connection_balance_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener::ConnectionBalanceConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
    }
  }

  // reuse_port: optional bool
  pub fn reuse_port(&self) -> bool {
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
  pub fn set_reuse_port(&mut self, val: bool) {
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

  // enable_reuse_port: optional message google.protobuf.BoolValue
  pub fn has_enable_reuse_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn clear_enable_reuse_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        26
      );
    }
  }
  pub fn enable_reuse_port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enable_reuse_port().then(|| self.enable_reuse_port())
  }
  pub fn enable_reuse_port(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(26)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enable_reuse_port_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_enable_reuse_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        26,
        val
      );
    }
  }

  // access_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn access_log(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
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
  pub fn set_access_log(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        src);
    }
  }

  // tcp_backlog_size: optional message google.protobuf.UInt32Value
  pub fn has_tcp_backlog_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_tcp_backlog_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn tcp_backlog_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_tcp_backlog_size().then(|| self.tcp_backlog_size())
  }
  pub fn tcp_backlog_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn tcp_backlog_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_tcp_backlog_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val
      );
    }
  }

  // max_connections_to_accept_per_socket_event: optional message google.protobuf.UInt32Value
  pub fn has_max_connections_to_accept_per_socket_event(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(31)
    }
  }
  pub fn clear_max_connections_to_accept_per_socket_event(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        31
      );
    }
  }
  pub fn max_connections_to_accept_per_socket_event_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_connections_to_accept_per_socket_event().then(|| self.max_connections_to_accept_per_socket_event())
  }
  pub fn max_connections_to_accept_per_socket_event(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(31)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_connections_to_accept_per_socket_event_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         31, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_max_connections_to_accept_per_socket_event(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        31,
        val
      );
    }
  }

  // bind_to_port: optional message google.protobuf.BoolValue
  pub fn has_bind_to_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_bind_to_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn bind_to_port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_bind_to_port().then(|| self.bind_to_port())
  }
  pub fn bind_to_port(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn bind_to_port_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_bind_to_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val
      );
    }
  }

  // internal_listener: optional message envoy.config.listener.v3.Listener.InternalListenerConfig
  pub fn has_internal_listener(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_internal_listener(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn internal_listener_opt(&self) -> ::std::option::Option<super::listener::InternalListenerConfigView<'_>> {
    self.has_internal_listener().then(|| self.internal_listener())
  }
  pub fn internal_listener(&self) -> super::listener::InternalListenerConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener::InternalListenerConfigView::default())
  }
  pub fn internal_listener_mut(&mut self) -> super::listener::InternalListenerConfigMut<'_> {
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
  pub fn set_internal_listener(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener::InternalListenerConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // enable_mptcp: optional bool
  pub fn enable_mptcp(&self) -> bool {
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
  pub fn set_enable_mptcp(&mut self, val: bool) {
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

  // ignore_global_conn_limit: optional bool
  pub fn ignore_global_conn_limit(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        28, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_global_conn_limit(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        28, val.into()
      )
    }
  }

  // bypass_overload_manager: optional bool
  pub fn bypass_overload_manager(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        32, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bypass_overload_manager(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        32, val.into()
      )
    }
  }

  // tcp_keepalive: optional message envoy.config.core.v3.TcpKeepalive
  pub fn has_tcp_keepalive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(34)
    }
  }
  pub fn clear_tcp_keepalive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        34
      );
    }
  }
  pub fn tcp_keepalive_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_>> {
    self.has_tcp_keepalive().then(|| self.tcp_keepalive())
  }
  pub fn tcp_keepalive(&self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(34)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView::default())
  }
  pub fn tcp_keepalive_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         34, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tcp_keepalive(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::TcpKeepalive>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        34,
        val
      );
    }
  }

  pub fn listener_specifier(&self) -> super::listener::ListenerSpecifierOneof<'_> {
    match &self.listener_specifier_case() {
      super::listener::ListenerSpecifierCase::InternalListener =>
          super::listener::ListenerSpecifierOneof::InternalListener(self.internal_listener()),
      _ => super::listener::ListenerSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn listener_specifier_case(&self) -> super::listener::ListenerSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(24);
      super::listener::ListenerSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Listener

impl ::std::ops::Drop for Listener {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Listener {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Listener {
  type Proxied = Self;
  fn as_view(&self) -> ListenerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Listener {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListenerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Listener {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__Listener_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3G3333.PG333Ga3.P/P333/PGa33331X3/P/P3G3/P33^=");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__Listener_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::listener::DeprecatedV1 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::listener::v3::listener_components::ListenerFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::listener::v3::udp_listener_config::UdpListenerConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::listener::v3::api_listener::ApiListener as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::listener::ConnectionBalanceConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::listener::v3::listener_components::FilterChain as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::listener::InternalListenerConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::AdditionalAddress as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::listener::FcdsConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::TcpKeepalive as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__Listener_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Listener {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Listener {
  type Msg = Listener;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Listener> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Listener {
  type Msg = Listener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Listener> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListenerMut<'_> {
  type Msg = Listener;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Listener> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerMut<'_> {
  type Msg = Listener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Listener> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerView<'_> {
  type Msg = Listener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Listener> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListenerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod listener {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__Listener__DeprecatedV1_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DeprecatedV1 {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeprecatedV1>
}

impl ::protobuf::Message for DeprecatedV1 {
  type MessageView<'msg> = DeprecatedV1View<'msg>;
  type MessageMut<'msg> = DeprecatedV1Mut<'msg>;
}

impl ::std::default::Default for DeprecatedV1 {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DeprecatedV1 {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DeprecatedV1` is `Sync` because it does not implement interior mutability.
//    Neither does `DeprecatedV1Mut`.
unsafe impl ::std::marker::Sync for DeprecatedV1 {}

// SAFETY:
// - `DeprecatedV1` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DeprecatedV1 {}

impl ::protobuf::Proxied for DeprecatedV1 {
  type View<'msg> = DeprecatedV1View<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeprecatedV1 {}

impl ::protobuf::MutProxied for DeprecatedV1 {
  type Mut<'msg> = DeprecatedV1Mut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeprecatedV1View<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeprecatedV1>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeprecatedV1View<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeprecatedV1View<'msg> {
  type Message = DeprecatedV1;
}

impl ::std::fmt::Debug for DeprecatedV1View<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DeprecatedV1View<'_> {
  fn default() -> DeprecatedV1View<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DeprecatedV1>> for DeprecatedV1View<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeprecatedV1>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeprecatedV1View<'msg> {

  pub fn to_owned(&self) -> DeprecatedV1 {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bind_to_port: optional message google.protobuf.BoolValue
  pub fn has_bind_to_port(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn bind_to_port_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_bind_to_port().then(|| self.bind_to_port())
  }
  pub fn bind_to_port(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

}

// SAFETY:
// - `DeprecatedV1View` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DeprecatedV1View<'_> {}

// SAFETY:
// - `DeprecatedV1View` is `Send` because while its alive a `DeprecatedV1Mut` cannot.
// - `DeprecatedV1View` does not use thread-local data.
unsafe impl ::std::marker::Send for DeprecatedV1View<'_> {}

impl<'msg> ::protobuf::AsView for DeprecatedV1View<'msg> {
  type Proxied = DeprecatedV1;
  fn as_view(&self) -> ::protobuf::View<'msg, DeprecatedV1> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeprecatedV1View<'msg> {
  fn into_view<'shorter>(self) -> DeprecatedV1View<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeprecatedV1> for DeprecatedV1View<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeprecatedV1 {
    let mut dst = DeprecatedV1::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeprecatedV1> for DeprecatedV1Mut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeprecatedV1 {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DeprecatedV1 {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeprecatedV1View<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeprecatedV1Mut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeprecatedV1Mut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeprecatedV1>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeprecatedV1Mut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeprecatedV1Mut<'msg> {
  type Message = DeprecatedV1;
}

impl ::std::fmt::Debug for DeprecatedV1Mut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DeprecatedV1>> for DeprecatedV1Mut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeprecatedV1>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeprecatedV1Mut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeprecatedV1> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DeprecatedV1 {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bind_to_port: optional message google.protobuf.BoolValue
  pub fn has_bind_to_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_bind_to_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn bind_to_port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_bind_to_port().then(|| self.bind_to_port())
  }
  pub fn bind_to_port(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn bind_to_port_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_bind_to_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

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
// - `DeprecatedV1Mut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DeprecatedV1Mut<'_> {}

// SAFETY:
// - `DeprecatedV1Mut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DeprecatedV1Mut<'_> {}

impl<'msg> ::protobuf::AsView for DeprecatedV1Mut<'msg> {
  type Proxied = DeprecatedV1;
  fn as_view(&self) -> ::protobuf::View<'_, DeprecatedV1> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeprecatedV1Mut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeprecatedV1>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DeprecatedV1Mut<'msg> {
  type MutProxied = DeprecatedV1;
  fn as_mut(&mut self) -> DeprecatedV1Mut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeprecatedV1Mut<'msg> {
  fn into_mut<'shorter>(self) -> DeprecatedV1Mut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeprecatedV1 {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeprecatedV1> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DeprecatedV1View<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DeprecatedV1Mut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bind_to_port: optional message google.protobuf.BoolValue
  pub fn has_bind_to_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_bind_to_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn bind_to_port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_bind_to_port().then(|| self.bind_to_port())
  }
  pub fn bind_to_port(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn bind_to_port_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_bind_to_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl DeprecatedV1

impl ::std::ops::Drop for DeprecatedV1 {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DeprecatedV1 {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeprecatedV1 {
  type Proxied = Self;
  fn as_view(&self) -> DeprecatedV1View<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeprecatedV1 {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeprecatedV1Mut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DeprecatedV1 {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::listener::envoy__config__listener__v3__Listener__DeprecatedV1_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::listener::envoy__config__listener__v3__Listener__DeprecatedV1_msg_init.0, &[<::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::listener::envoy__config__listener__v3__Listener__DeprecatedV1_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeprecatedV1 {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeprecatedV1 {
  type Msg = DeprecatedV1;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeprecatedV1> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeprecatedV1 {
  type Msg = DeprecatedV1;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeprecatedV1> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeprecatedV1Mut<'_> {
  type Msg = DeprecatedV1;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeprecatedV1> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeprecatedV1Mut<'_> {
  type Msg = DeprecatedV1;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeprecatedV1> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeprecatedV1View<'_> {
  type Msg = DeprecatedV1;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeprecatedV1> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeprecatedV1Mut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__Listener__ConnectionBalanceConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ConnectionBalanceConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ConnectionBalanceConfig>
}

impl ::protobuf::Message for ConnectionBalanceConfig {
  type MessageView<'msg> = ConnectionBalanceConfigView<'msg>;
  type MessageMut<'msg> = ConnectionBalanceConfigMut<'msg>;
}

impl ::std::default::Default for ConnectionBalanceConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ConnectionBalanceConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ConnectionBalanceConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ConnectionBalanceConfigMut`.
unsafe impl ::std::marker::Sync for ConnectionBalanceConfig {}

// SAFETY:
// - `ConnectionBalanceConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ConnectionBalanceConfig {}

impl ::protobuf::Proxied for ConnectionBalanceConfig {
  type View<'msg> = ConnectionBalanceConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ConnectionBalanceConfig {}

impl ::protobuf::MutProxied for ConnectionBalanceConfig {
  type Mut<'msg> = ConnectionBalanceConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ConnectionBalanceConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionBalanceConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConnectionBalanceConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ConnectionBalanceConfigView<'msg> {
  type Message = ConnectionBalanceConfig;
}

impl ::std::fmt::Debug for ConnectionBalanceConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ConnectionBalanceConfigView<'_> {
  fn default() -> ConnectionBalanceConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionBalanceConfig>> for ConnectionBalanceConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionBalanceConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConnectionBalanceConfigView<'msg> {

  pub fn to_owned(&self) -> ConnectionBalanceConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // exact_balance: optional message envoy.config.listener.v3.Listener.ConnectionBalanceConfig.ExactBalance
  pub fn has_exact_balance(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn exact_balance_opt(self) -> ::std::option::Option<super::super::listener::connection_balance_config::ExactBalanceView<'msg>> {
    self.has_exact_balance().then(|| self.exact_balance())
  }
  pub fn exact_balance(self) -> super::super::listener::connection_balance_config::ExactBalanceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listener::connection_balance_config::ExactBalanceView::default())
  }

  // extend_balance: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_extend_balance(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn extend_balance_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_extend_balance().then(|| self.extend_balance())
  }
  pub fn extend_balance(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  pub fn balance_type(self) -> super::super::listener::connection_balance_config::BalanceTypeOneof<'msg> {
    match self.balance_type_case() {
      super::super::listener::connection_balance_config::BalanceTypeCase::ExactBalance =>
          super::super::listener::connection_balance_config::BalanceTypeOneof::ExactBalance(self.exact_balance()),
      super::super::listener::connection_balance_config::BalanceTypeCase::ExtendBalance =>
          super::super::listener::connection_balance_config::BalanceTypeOneof::ExtendBalance(self.extend_balance()),
      _ => super::super::listener::connection_balance_config::BalanceTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn balance_type_case(self) -> super::super::listener::connection_balance_config::BalanceTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::listener::connection_balance_config::BalanceTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ConnectionBalanceConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ConnectionBalanceConfigView<'_> {}

// SAFETY:
// - `ConnectionBalanceConfigView` is `Send` because while its alive a `ConnectionBalanceConfigMut` cannot.
// - `ConnectionBalanceConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ConnectionBalanceConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ConnectionBalanceConfigView<'msg> {
  type Proxied = ConnectionBalanceConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ConnectionBalanceConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConnectionBalanceConfigView<'msg> {
  fn into_view<'shorter>(self) -> ConnectionBalanceConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ConnectionBalanceConfig> for ConnectionBalanceConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConnectionBalanceConfig {
    let mut dst = ConnectionBalanceConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ConnectionBalanceConfig> for ConnectionBalanceConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConnectionBalanceConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ConnectionBalanceConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConnectionBalanceConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConnectionBalanceConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ConnectionBalanceConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionBalanceConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConnectionBalanceConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ConnectionBalanceConfigMut<'msg> {
  type Message = ConnectionBalanceConfig;
}

impl ::std::fmt::Debug for ConnectionBalanceConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionBalanceConfig>> for ConnectionBalanceConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionBalanceConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConnectionBalanceConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionBalanceConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ConnectionBalanceConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // exact_balance: optional message envoy.config.listener.v3.Listener.ConnectionBalanceConfig.ExactBalance
  pub fn has_exact_balance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_exact_balance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn exact_balance_opt(&self) -> ::std::option::Option<super::super::listener::connection_balance_config::ExactBalanceView<'_>> {
    self.has_exact_balance().then(|| self.exact_balance())
  }
  pub fn exact_balance(&self) -> super::super::listener::connection_balance_config::ExactBalanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listener::connection_balance_config::ExactBalanceView::default())
  }
  pub fn exact_balance_mut(&mut self) -> super::super::listener::connection_balance_config::ExactBalanceMut<'_> {
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
  pub fn set_exact_balance(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::listener::connection_balance_config::ExactBalance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // extend_balance: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_extend_balance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_extend_balance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn extend_balance_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_extend_balance().then(|| self.extend_balance())
  }
  pub fn extend_balance(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn extend_balance_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_extend_balance(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn balance_type(&self) -> super::super::listener::connection_balance_config::BalanceTypeOneof<'_> {
    match &self.balance_type_case() {
      super::super::listener::connection_balance_config::BalanceTypeCase::ExactBalance =>
          super::super::listener::connection_balance_config::BalanceTypeOneof::ExactBalance(self.exact_balance()),
      super::super::listener::connection_balance_config::BalanceTypeCase::ExtendBalance =>
          super::super::listener::connection_balance_config::BalanceTypeOneof::ExtendBalance(self.extend_balance()),
      _ => super::super::listener::connection_balance_config::BalanceTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn balance_type_case(&self) -> super::super::listener::connection_balance_config::BalanceTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::listener::connection_balance_config::BalanceTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ConnectionBalanceConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ConnectionBalanceConfigMut<'_> {}

// SAFETY:
// - `ConnectionBalanceConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ConnectionBalanceConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ConnectionBalanceConfigMut<'msg> {
  type Proxied = ConnectionBalanceConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ConnectionBalanceConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConnectionBalanceConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ConnectionBalanceConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ConnectionBalanceConfigMut<'msg> {
  type MutProxied = ConnectionBalanceConfig;
  fn as_mut(&mut self) -> ConnectionBalanceConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ConnectionBalanceConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ConnectionBalanceConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ConnectionBalanceConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ConnectionBalanceConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ConnectionBalanceConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ConnectionBalanceConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // exact_balance: optional message envoy.config.listener.v3.Listener.ConnectionBalanceConfig.ExactBalance
  pub fn has_exact_balance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_exact_balance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn exact_balance_opt(&self) -> ::std::option::Option<super::super::listener::connection_balance_config::ExactBalanceView<'_>> {
    self.has_exact_balance().then(|| self.exact_balance())
  }
  pub fn exact_balance(&self) -> super::super::listener::connection_balance_config::ExactBalanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::listener::connection_balance_config::ExactBalanceView::default())
  }
  pub fn exact_balance_mut(&mut self) -> super::super::listener::connection_balance_config::ExactBalanceMut<'_> {
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
  pub fn set_exact_balance(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::listener::connection_balance_config::ExactBalance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // extend_balance: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_extend_balance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_extend_balance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn extend_balance_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_extend_balance().then(|| self.extend_balance())
  }
  pub fn extend_balance(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn extend_balance_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_extend_balance(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn balance_type(&self) -> super::super::listener::connection_balance_config::BalanceTypeOneof<'_> {
    match &self.balance_type_case() {
      super::super::listener::connection_balance_config::BalanceTypeCase::ExactBalance =>
          super::super::listener::connection_balance_config::BalanceTypeOneof::ExactBalance(self.exact_balance()),
      super::super::listener::connection_balance_config::BalanceTypeCase::ExtendBalance =>
          super::super::listener::connection_balance_config::BalanceTypeOneof::ExtendBalance(self.extend_balance()),
      _ => super::super::listener::connection_balance_config::BalanceTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn balance_type_case(&self) -> super::super::listener::connection_balance_config::BalanceTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::listener::connection_balance_config::BalanceTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ConnectionBalanceConfig

impl ::std::ops::Drop for ConnectionBalanceConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ConnectionBalanceConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ConnectionBalanceConfig {
  type Proxied = Self;
  fn as_view(&self) -> ConnectionBalanceConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ConnectionBalanceConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ConnectionBalanceConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ConnectionBalanceConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::listener::envoy__config__listener__v3__Listener__ConnectionBalanceConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::listener::envoy__config__listener__v3__Listener__ConnectionBalanceConfig_msg_init.0, &[<super::super::listener::connection_balance_config::ExactBalance as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::listener::envoy__config__listener__v3__Listener__ConnectionBalanceConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConnectionBalanceConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConnectionBalanceConfig {
  type Msg = ConnectionBalanceConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionBalanceConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionBalanceConfig {
  type Msg = ConnectionBalanceConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionBalanceConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConnectionBalanceConfigMut<'_> {
  type Msg = ConnectionBalanceConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionBalanceConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionBalanceConfigMut<'_> {
  type Msg = ConnectionBalanceConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionBalanceConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionBalanceConfigView<'_> {
  type Msg = ConnectionBalanceConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionBalanceConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConnectionBalanceConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod connection_balance_config {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__Listener__ConnectionBalanceConfig__ExactBalance_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExactBalance {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExactBalance>
}

impl ::protobuf::Message for ExactBalance {
  type MessageView<'msg> = ExactBalanceView<'msg>;
  type MessageMut<'msg> = ExactBalanceMut<'msg>;
}

impl ::std::default::Default for ExactBalance {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExactBalance {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExactBalance` is `Sync` because it does not implement interior mutability.
//    Neither does `ExactBalanceMut`.
unsafe impl ::std::marker::Sync for ExactBalance {}

// SAFETY:
// - `ExactBalance` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExactBalance {}

impl ::protobuf::Proxied for ExactBalance {
  type View<'msg> = ExactBalanceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExactBalance {}

impl ::protobuf::MutProxied for ExactBalance {
  type Mut<'msg> = ExactBalanceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExactBalanceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExactBalance>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExactBalanceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExactBalanceView<'msg> {
  type Message = ExactBalance;
}

impl ::std::fmt::Debug for ExactBalanceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExactBalanceView<'_> {
  fn default() -> ExactBalanceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExactBalance>> for ExactBalanceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExactBalance>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExactBalanceView<'msg> {

  pub fn to_owned(&self) -> ExactBalance {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ExactBalanceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExactBalanceView<'_> {}

// SAFETY:
// - `ExactBalanceView` is `Send` because while its alive a `ExactBalanceMut` cannot.
// - `ExactBalanceView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExactBalanceView<'_> {}

impl<'msg> ::protobuf::AsView for ExactBalanceView<'msg> {
  type Proxied = ExactBalance;
  fn as_view(&self) -> ::protobuf::View<'msg, ExactBalance> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExactBalanceView<'msg> {
  fn into_view<'shorter>(self) -> ExactBalanceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExactBalance> for ExactBalanceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExactBalance {
    let mut dst = ExactBalance::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExactBalance> for ExactBalanceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExactBalance {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExactBalance {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExactBalanceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExactBalanceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExactBalanceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExactBalance>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExactBalanceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExactBalanceMut<'msg> {
  type Message = ExactBalance;
}

impl ::std::fmt::Debug for ExactBalanceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExactBalance>> for ExactBalanceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExactBalance>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExactBalanceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExactBalance> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExactBalance {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ExactBalanceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExactBalanceMut<'_> {}

// SAFETY:
// - `ExactBalanceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExactBalanceMut<'_> {}

impl<'msg> ::protobuf::AsView for ExactBalanceMut<'msg> {
  type Proxied = ExactBalance;
  fn as_view(&self) -> ::protobuf::View<'_, ExactBalance> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExactBalanceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExactBalance>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExactBalanceMut<'msg> {
  type MutProxied = ExactBalance;
  fn as_mut(&mut self) -> ExactBalanceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExactBalanceMut<'msg> {
  fn into_mut<'shorter>(self) -> ExactBalanceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExactBalance {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExactBalance> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExactBalanceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExactBalanceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl ExactBalance

impl ::std::ops::Drop for ExactBalance {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExactBalance {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExactBalance {
  type Proxied = Self;
  fn as_view(&self) -> ExactBalanceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExactBalance {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExactBalanceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExactBalance {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::listener::connection_balance_config::envoy__config__listener__v3__Listener__ConnectionBalanceConfig__ExactBalance_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::listener::connection_balance_config::envoy__config__listener__v3__Listener__ConnectionBalanceConfig__ExactBalance_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::listener::connection_balance_config::envoy__config__listener__v3__Listener__ConnectionBalanceConfig__ExactBalance_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExactBalance {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExactBalance {
  type Msg = ExactBalance;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExactBalance> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExactBalance {
  type Msg = ExactBalance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExactBalance> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExactBalanceMut<'_> {
  type Msg = ExactBalance;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExactBalance> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExactBalanceMut<'_> {
  type Msg = ExactBalance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExactBalance> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExactBalanceView<'_> {
  type Msg = ExactBalance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExactBalance> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExactBalanceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum BalanceTypeOneof<'msg> {
  ExactBalance(::protobuf::View<'msg, super::super::super::listener::connection_balance_config::ExactBalance>) = 1,
  ExtendBalance(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum BalanceTypeCase {
  ExactBalance = 1,
  ExtendBalance = 2,

  not_set = 0
}

impl BalanceTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<BalanceTypeCase> {
    match v {
      0 => Some(BalanceTypeCase::not_set),
      1 => Some(BalanceTypeCase::ExactBalance),
      2 => Some(BalanceTypeCase::ExtendBalance),
      _ => None
    }
  }
}
}  // pub mod connection_balance_config

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__Listener__InternalListenerConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct InternalListenerConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<InternalListenerConfig>
}

impl ::protobuf::Message for InternalListenerConfig {
  type MessageView<'msg> = InternalListenerConfigView<'msg>;
  type MessageMut<'msg> = InternalListenerConfigMut<'msg>;
}

impl ::std::default::Default for InternalListenerConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for InternalListenerConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `InternalListenerConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `InternalListenerConfigMut`.
unsafe impl ::std::marker::Sync for InternalListenerConfig {}

// SAFETY:
// - `InternalListenerConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for InternalListenerConfig {}

impl ::protobuf::Proxied for InternalListenerConfig {
  type View<'msg> = InternalListenerConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for InternalListenerConfig {}

impl ::protobuf::MutProxied for InternalListenerConfig {
  type Mut<'msg> = InternalListenerConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct InternalListenerConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, InternalListenerConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InternalListenerConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for InternalListenerConfigView<'msg> {
  type Message = InternalListenerConfig;
}

impl ::std::fmt::Debug for InternalListenerConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for InternalListenerConfigView<'_> {
  fn default() -> InternalListenerConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, InternalListenerConfig>> for InternalListenerConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, InternalListenerConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InternalListenerConfigView<'msg> {

  pub fn to_owned(&self) -> InternalListenerConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `InternalListenerConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for InternalListenerConfigView<'_> {}

// SAFETY:
// - `InternalListenerConfigView` is `Send` because while its alive a `InternalListenerConfigMut` cannot.
// - `InternalListenerConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for InternalListenerConfigView<'_> {}

impl<'msg> ::protobuf::AsView for InternalListenerConfigView<'msg> {
  type Proxied = InternalListenerConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, InternalListenerConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InternalListenerConfigView<'msg> {
  fn into_view<'shorter>(self) -> InternalListenerConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<InternalListenerConfig> for InternalListenerConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> InternalListenerConfig {
    let mut dst = InternalListenerConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<InternalListenerConfig> for InternalListenerConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> InternalListenerConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for InternalListenerConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for InternalListenerConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for InternalListenerConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct InternalListenerConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, InternalListenerConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for InternalListenerConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for InternalListenerConfigMut<'msg> {
  type Message = InternalListenerConfig;
}

impl ::std::fmt::Debug for InternalListenerConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, InternalListenerConfig>> for InternalListenerConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, InternalListenerConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> InternalListenerConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, InternalListenerConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> InternalListenerConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `InternalListenerConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for InternalListenerConfigMut<'_> {}

// SAFETY:
// - `InternalListenerConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for InternalListenerConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for InternalListenerConfigMut<'msg> {
  type Proxied = InternalListenerConfig;
  fn as_view(&self) -> ::protobuf::View<'_, InternalListenerConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InternalListenerConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, InternalListenerConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for InternalListenerConfigMut<'msg> {
  type MutProxied = InternalListenerConfig;
  fn as_mut(&mut self) -> InternalListenerConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for InternalListenerConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> InternalListenerConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl InternalListenerConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, InternalListenerConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> InternalListenerConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> InternalListenerConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl InternalListenerConfig

impl ::std::ops::Drop for InternalListenerConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for InternalListenerConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for InternalListenerConfig {
  type Proxied = Self;
  fn as_view(&self) -> InternalListenerConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for InternalListenerConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> InternalListenerConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for InternalListenerConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::listener::envoy__config__listener__v3__Listener__InternalListenerConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::listener::envoy__config__listener__v3__Listener__InternalListenerConfig_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::listener::envoy__config__listener__v3__Listener__InternalListenerConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for InternalListenerConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for InternalListenerConfig {
  type Msg = InternalListenerConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InternalListenerConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InternalListenerConfig {
  type Msg = InternalListenerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InternalListenerConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for InternalListenerConfigMut<'_> {
  type Msg = InternalListenerConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InternalListenerConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InternalListenerConfigMut<'_> {
  type Msg = InternalListenerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InternalListenerConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for InternalListenerConfigView<'_> {
  type Msg = InternalListenerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<InternalListenerConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for InternalListenerConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__Listener__FcdsConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FcdsConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FcdsConfig>
}

impl ::protobuf::Message for FcdsConfig {
  type MessageView<'msg> = FcdsConfigView<'msg>;
  type MessageMut<'msg> = FcdsConfigMut<'msg>;
}

impl ::std::default::Default for FcdsConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FcdsConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FcdsConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `FcdsConfigMut`.
unsafe impl ::std::marker::Sync for FcdsConfig {}

// SAFETY:
// - `FcdsConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FcdsConfig {}

impl ::protobuf::Proxied for FcdsConfig {
  type View<'msg> = FcdsConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FcdsConfig {}

impl ::protobuf::MutProxied for FcdsConfig {
  type Mut<'msg> = FcdsConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FcdsConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FcdsConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FcdsConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FcdsConfigView<'msg> {
  type Message = FcdsConfig;
}

impl ::std::fmt::Debug for FcdsConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FcdsConfigView<'_> {
  fn default() -> FcdsConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FcdsConfig>> for FcdsConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FcdsConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FcdsConfigView<'msg> {

  pub fn to_owned(&self) -> FcdsConfig {
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

  // config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_config_source(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn config_source_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

}

// SAFETY:
// - `FcdsConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FcdsConfigView<'_> {}

// SAFETY:
// - `FcdsConfigView` is `Send` because while its alive a `FcdsConfigMut` cannot.
// - `FcdsConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for FcdsConfigView<'_> {}

impl<'msg> ::protobuf::AsView for FcdsConfigView<'msg> {
  type Proxied = FcdsConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, FcdsConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FcdsConfigView<'msg> {
  fn into_view<'shorter>(self) -> FcdsConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FcdsConfig> for FcdsConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FcdsConfig {
    let mut dst = FcdsConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FcdsConfig> for FcdsConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FcdsConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FcdsConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FcdsConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FcdsConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FcdsConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FcdsConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FcdsConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FcdsConfigMut<'msg> {
  type Message = FcdsConfig;
}

impl ::std::fmt::Debug for FcdsConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FcdsConfig>> for FcdsConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FcdsConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FcdsConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FcdsConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FcdsConfig {
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

  // config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

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
// - `FcdsConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FcdsConfigMut<'_> {}

// SAFETY:
// - `FcdsConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FcdsConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for FcdsConfigMut<'msg> {
  type Proxied = FcdsConfig;
  fn as_view(&self) -> ::protobuf::View<'_, FcdsConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FcdsConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FcdsConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FcdsConfigMut<'msg> {
  type MutProxied = FcdsConfig;
  fn as_mut(&mut self) -> FcdsConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FcdsConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> FcdsConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FcdsConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FcdsConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FcdsConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FcdsConfigMut<'_> {
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

  // config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl FcdsConfig

impl ::std::ops::Drop for FcdsConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FcdsConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FcdsConfig {
  type Proxied = Self;
  fn as_view(&self) -> FcdsConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FcdsConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FcdsConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FcdsConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::listener::envoy__config__listener__v3__Listener__FcdsConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::listener::envoy__config__listener__v3__Listener__FcdsConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::listener::envoy__config__listener__v3__Listener__FcdsConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FcdsConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FcdsConfig {
  type Msg = FcdsConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FcdsConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FcdsConfig {
  type Msg = FcdsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FcdsConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FcdsConfigMut<'_> {
  type Msg = FcdsConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FcdsConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FcdsConfigMut<'_> {
  type Msg = FcdsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FcdsConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FcdsConfigView<'_> {
  type Msg = FcdsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FcdsConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FcdsConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DrainType(i32);

#[allow(non_upper_case_globals)]
impl DrainType {
  pub const Default: DrainType = DrainType(0);
  pub const ModifyOnly: DrainType = DrainType(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Default",
      1 => "ModifyOnly",
      _ => return None
    })
  }
}

impl ::std::convert::From<DrainType> for i32 {
  fn from(val: DrainType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for DrainType {
  fn from(val: i32) -> DrainType {
    Self(val)
  }
}

impl ::std::default::Default for DrainType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for DrainType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "DrainType::{}", constant_name)
    } else {
      write!(f, "DrainType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for DrainType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for DrainType {}

impl ::protobuf::Proxied for DrainType {
  type View<'a> = DrainType;
}

impl ::protobuf::AsView for DrainType {
  type Proxied = DrainType;

  fn as_view(&self) -> DrainType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DrainType {
  fn into_view<'shorter>(self) -> DrainType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for DrainType {
  const NAME: &'static str = "DrainType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for DrainType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ListenerSpecifierOneof<'msg> {
  InternalListener(::protobuf::View<'msg, super::super::listener::InternalListenerConfig>) = 27,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ListenerSpecifierCase {
  InternalListener = 27,

  not_set = 0
}

impl ListenerSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ListenerSpecifierCase> {
    match v {
      0 => Some(ListenerSpecifierCase::not_set),
      27 => Some(ListenerSpecifierCase::InternalListener),
      _ => None
    }
  }
}
}  // pub mod listener


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__ListenerManager_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListenerManager {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListenerManager>
}

impl ::protobuf::Message for ListenerManager {
  type MessageView<'msg> = ListenerManagerView<'msg>;
  type MessageMut<'msg> = ListenerManagerMut<'msg>;
}

impl ::std::default::Default for ListenerManager {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListenerManager {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListenerManager` is `Sync` because it does not implement interior mutability.
//    Neither does `ListenerManagerMut`.
unsafe impl ::std::marker::Sync for ListenerManager {}

// SAFETY:
// - `ListenerManager` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ListenerManager {}

impl ::protobuf::Proxied for ListenerManager {
  type View<'msg> = ListenerManagerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListenerManager {}

impl ::protobuf::MutProxied for ListenerManager {
  type Mut<'msg> = ListenerManagerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListenerManagerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenerManagerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListenerManagerView<'msg> {
  type Message = ListenerManager;
}

impl ::std::fmt::Debug for ListenerManagerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListenerManagerView<'_> {
  fn default() -> ListenerManagerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerManager>> for ListenerManagerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenerManagerView<'msg> {

  pub fn to_owned(&self) -> ListenerManager {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ListenerManagerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ListenerManagerView<'_> {}

// SAFETY:
// - `ListenerManagerView` is `Send` because while its alive a `ListenerManagerMut` cannot.
// - `ListenerManagerView` does not use thread-local data.
unsafe impl ::std::marker::Send for ListenerManagerView<'_> {}

impl<'msg> ::protobuf::AsView for ListenerManagerView<'msg> {
  type Proxied = ListenerManager;
  fn as_view(&self) -> ::protobuf::View<'msg, ListenerManager> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenerManagerView<'msg> {
  fn into_view<'shorter>(self) -> ListenerManagerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListenerManager> for ListenerManagerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListenerManager {
    let mut dst = ListenerManager::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListenerManager> for ListenerManagerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListenerManager {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ListenerManager {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenerManagerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenerManagerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListenerManagerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenerManagerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListenerManagerMut<'msg> {
  type Message = ListenerManager;
}

impl ::std::fmt::Debug for ListenerManagerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerManager>> for ListenerManagerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenerManagerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerManager> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ListenerManager {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ListenerManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ListenerManagerMut<'_> {}

// SAFETY:
// - `ListenerManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ListenerManagerMut<'_> {}

impl<'msg> ::protobuf::AsView for ListenerManagerMut<'msg> {
  type Proxied = ListenerManager;
  fn as_view(&self) -> ::protobuf::View<'_, ListenerManager> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenerManagerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListenerManager>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ListenerManagerMut<'msg> {
  type MutProxied = ListenerManager;
  fn as_mut(&mut self) -> ListenerManagerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListenerManagerMut<'msg> {
  fn into_mut<'shorter>(self) -> ListenerManagerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListenerManager {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListenerManager> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListenerManagerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListenerManagerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl ListenerManager

impl ::std::ops::Drop for ListenerManager {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListenerManager {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListenerManager {
  type Proxied = Self;
  fn as_view(&self) -> ListenerManagerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListenerManager {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListenerManagerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListenerManager {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__ListenerManager_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__ListenerManager_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__ListenerManager_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListenerManager {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListenerManager {
  type Msg = ListenerManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerManager {
  type Msg = ListenerManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListenerManagerMut<'_> {
  type Msg = ListenerManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerManagerMut<'_> {
  type Msg = ListenerManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerManagerView<'_> {
  type Msg = ListenerManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerManager> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListenerManagerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__ValidationListenerManager_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ValidationListenerManager {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ValidationListenerManager>
}

impl ::protobuf::Message for ValidationListenerManager {
  type MessageView<'msg> = ValidationListenerManagerView<'msg>;
  type MessageMut<'msg> = ValidationListenerManagerMut<'msg>;
}

impl ::std::default::Default for ValidationListenerManager {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ValidationListenerManager {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ValidationListenerManager` is `Sync` because it does not implement interior mutability.
//    Neither does `ValidationListenerManagerMut`.
unsafe impl ::std::marker::Sync for ValidationListenerManager {}

// SAFETY:
// - `ValidationListenerManager` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ValidationListenerManager {}

impl ::protobuf::Proxied for ValidationListenerManager {
  type View<'msg> = ValidationListenerManagerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ValidationListenerManager {}

impl ::protobuf::MutProxied for ValidationListenerManager {
  type Mut<'msg> = ValidationListenerManagerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ValidationListenerManagerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValidationListenerManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValidationListenerManagerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ValidationListenerManagerView<'msg> {
  type Message = ValidationListenerManager;
}

impl ::std::fmt::Debug for ValidationListenerManagerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ValidationListenerManagerView<'_> {
  fn default() -> ValidationListenerManagerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ValidationListenerManager>> for ValidationListenerManagerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValidationListenerManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValidationListenerManagerView<'msg> {

  pub fn to_owned(&self) -> ValidationListenerManager {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ValidationListenerManagerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ValidationListenerManagerView<'_> {}

// SAFETY:
// - `ValidationListenerManagerView` is `Send` because while its alive a `ValidationListenerManagerMut` cannot.
// - `ValidationListenerManagerView` does not use thread-local data.
unsafe impl ::std::marker::Send for ValidationListenerManagerView<'_> {}

impl<'msg> ::protobuf::AsView for ValidationListenerManagerView<'msg> {
  type Proxied = ValidationListenerManager;
  fn as_view(&self) -> ::protobuf::View<'msg, ValidationListenerManager> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValidationListenerManagerView<'msg> {
  fn into_view<'shorter>(self) -> ValidationListenerManagerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ValidationListenerManager> for ValidationListenerManagerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValidationListenerManager {
    let mut dst = ValidationListenerManager::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ValidationListenerManager> for ValidationListenerManagerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValidationListenerManager {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ValidationListenerManager {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ValidationListenerManagerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ValidationListenerManagerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ValidationListenerManagerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidationListenerManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValidationListenerManagerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ValidationListenerManagerMut<'msg> {
  type Message = ValidationListenerManager;
}

impl ::std::fmt::Debug for ValidationListenerManagerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ValidationListenerManager>> for ValidationListenerManagerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidationListenerManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValidationListenerManagerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ValidationListenerManager> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ValidationListenerManager {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ValidationListenerManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ValidationListenerManagerMut<'_> {}

// SAFETY:
// - `ValidationListenerManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ValidationListenerManagerMut<'_> {}

impl<'msg> ::protobuf::AsView for ValidationListenerManagerMut<'msg> {
  type Proxied = ValidationListenerManager;
  fn as_view(&self) -> ::protobuf::View<'_, ValidationListenerManager> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValidationListenerManagerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ValidationListenerManager>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ValidationListenerManagerMut<'msg> {
  type MutProxied = ValidationListenerManager;
  fn as_mut(&mut self) -> ValidationListenerManagerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ValidationListenerManagerMut<'msg> {
  fn into_mut<'shorter>(self) -> ValidationListenerManagerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ValidationListenerManager {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ValidationListenerManager> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ValidationListenerManagerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ValidationListenerManagerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl ValidationListenerManager

impl ::std::ops::Drop for ValidationListenerManager {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ValidationListenerManager {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ValidationListenerManager {
  type Proxied = Self;
  fn as_view(&self) -> ValidationListenerManagerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ValidationListenerManager {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ValidationListenerManagerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ValidationListenerManager {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__ValidationListenerManager_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__ValidationListenerManager_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__ValidationListenerManager_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValidationListenerManager {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValidationListenerManager {
  type Msg = ValidationListenerManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidationListenerManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidationListenerManager {
  type Msg = ValidationListenerManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidationListenerManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValidationListenerManagerMut<'_> {
  type Msg = ValidationListenerManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidationListenerManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidationListenerManagerMut<'_> {
  type Msg = ValidationListenerManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidationListenerManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValidationListenerManagerView<'_> {
  type Msg = ValidationListenerManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValidationListenerManager> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValidationListenerManagerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__ApiListenerManager_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ApiListenerManager {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ApiListenerManager>
}

impl ::protobuf::Message for ApiListenerManager {
  type MessageView<'msg> = ApiListenerManagerView<'msg>;
  type MessageMut<'msg> = ApiListenerManagerMut<'msg>;
}

impl ::std::default::Default for ApiListenerManager {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ApiListenerManager {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ApiListenerManager` is `Sync` because it does not implement interior mutability.
//    Neither does `ApiListenerManagerMut`.
unsafe impl ::std::marker::Sync for ApiListenerManager {}

// SAFETY:
// - `ApiListenerManager` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ApiListenerManager {}

impl ::protobuf::Proxied for ApiListenerManager {
  type View<'msg> = ApiListenerManagerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ApiListenerManager {}

impl ::protobuf::MutProxied for ApiListenerManager {
  type Mut<'msg> = ApiListenerManagerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ApiListenerManagerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ApiListenerManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ApiListenerManagerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ApiListenerManagerView<'msg> {
  type Message = ApiListenerManager;
}

impl ::std::fmt::Debug for ApiListenerManagerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ApiListenerManagerView<'_> {
  fn default() -> ApiListenerManagerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ApiListenerManager>> for ApiListenerManagerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ApiListenerManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ApiListenerManagerView<'msg> {

  pub fn to_owned(&self) -> ApiListenerManager {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ApiListenerManagerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ApiListenerManagerView<'_> {}

// SAFETY:
// - `ApiListenerManagerView` is `Send` because while its alive a `ApiListenerManagerMut` cannot.
// - `ApiListenerManagerView` does not use thread-local data.
unsafe impl ::std::marker::Send for ApiListenerManagerView<'_> {}

impl<'msg> ::protobuf::AsView for ApiListenerManagerView<'msg> {
  type Proxied = ApiListenerManager;
  fn as_view(&self) -> ::protobuf::View<'msg, ApiListenerManager> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApiListenerManagerView<'msg> {
  fn into_view<'shorter>(self) -> ApiListenerManagerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ApiListenerManager> for ApiListenerManagerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ApiListenerManager {
    let mut dst = ApiListenerManager::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ApiListenerManager> for ApiListenerManagerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ApiListenerManager {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ApiListenerManager {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ApiListenerManagerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ApiListenerManagerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ApiListenerManagerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiListenerManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ApiListenerManagerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ApiListenerManagerMut<'msg> {
  type Message = ApiListenerManager;
}

impl ::std::fmt::Debug for ApiListenerManagerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ApiListenerManager>> for ApiListenerManagerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiListenerManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ApiListenerManagerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiListenerManager> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ApiListenerManager {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ApiListenerManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ApiListenerManagerMut<'_> {}

// SAFETY:
// - `ApiListenerManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ApiListenerManagerMut<'_> {}

impl<'msg> ::protobuf::AsView for ApiListenerManagerMut<'msg> {
  type Proxied = ApiListenerManager;
  fn as_view(&self) -> ::protobuf::View<'_, ApiListenerManager> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApiListenerManagerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ApiListenerManager>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ApiListenerManagerMut<'msg> {
  type MutProxied = ApiListenerManager;
  fn as_mut(&mut self) -> ApiListenerManagerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ApiListenerManagerMut<'msg> {
  fn into_mut<'shorter>(self) -> ApiListenerManagerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ApiListenerManager {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ApiListenerManager> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ApiListenerManagerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ApiListenerManagerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl ApiListenerManager

impl ::std::ops::Drop for ApiListenerManager {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ApiListenerManager {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ApiListenerManager {
  type Proxied = Self;
  fn as_view(&self) -> ApiListenerManagerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ApiListenerManager {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ApiListenerManagerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ApiListenerManager {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__ApiListenerManager_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__ApiListenerManager_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__ApiListenerManager_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ApiListenerManager {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ApiListenerManager {
  type Msg = ApiListenerManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiListenerManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiListenerManager {
  type Msg = ApiListenerManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiListenerManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ApiListenerManagerMut<'_> {
  type Msg = ApiListenerManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiListenerManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiListenerManagerMut<'_> {
  type Msg = ApiListenerManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiListenerManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiListenerManagerView<'_> {
  type Msg = ApiListenerManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiListenerManager> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ApiListenerManagerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



