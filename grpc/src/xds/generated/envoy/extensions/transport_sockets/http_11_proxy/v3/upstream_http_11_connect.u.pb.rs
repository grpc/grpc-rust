const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__http_011_0proxy__v3__Http11ProxyUpstreamTransport_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Http11ProxyUpstreamTransport {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Http11ProxyUpstreamTransport>
}

impl ::protobuf::Message for Http11ProxyUpstreamTransport {
  type MessageView<'msg> = Http11ProxyUpstreamTransportView<'msg>;
  type MessageMut<'msg> = Http11ProxyUpstreamTransportMut<'msg>;
}

impl ::std::default::Default for Http11ProxyUpstreamTransport {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Http11ProxyUpstreamTransport {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Http11ProxyUpstreamTransport` is `Sync` because it does not implement interior mutability.
//    Neither does `Http11ProxyUpstreamTransportMut`.
unsafe impl ::std::marker::Sync for Http11ProxyUpstreamTransport {}

// SAFETY:
// - `Http11ProxyUpstreamTransport` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Http11ProxyUpstreamTransport {}

impl ::protobuf::Proxied for Http11ProxyUpstreamTransport {
  type View<'msg> = Http11ProxyUpstreamTransportView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Http11ProxyUpstreamTransport {}

impl ::protobuf::MutProxied for Http11ProxyUpstreamTransport {
  type Mut<'msg> = Http11ProxyUpstreamTransportMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Http11ProxyUpstreamTransportView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http11ProxyUpstreamTransport>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Http11ProxyUpstreamTransportView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Http11ProxyUpstreamTransportView<'msg> {
  type Message = Http11ProxyUpstreamTransport;
}

impl ::std::fmt::Debug for Http11ProxyUpstreamTransportView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Http11ProxyUpstreamTransportView<'_> {
  fn default() -> Http11ProxyUpstreamTransportView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Http11ProxyUpstreamTransport>> for Http11ProxyUpstreamTransportView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http11ProxyUpstreamTransport>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Http11ProxyUpstreamTransportView<'msg> {

  pub fn to_owned(&self) -> Http11ProxyUpstreamTransport {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn transport_socket_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'msg>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }

}

// SAFETY:
// - `Http11ProxyUpstreamTransportView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Http11ProxyUpstreamTransportView<'_> {}

// SAFETY:
// - `Http11ProxyUpstreamTransportView` is `Send` because while its alive a `Http11ProxyUpstreamTransportMut` cannot.
// - `Http11ProxyUpstreamTransportView` does not use thread-local data.
unsafe impl ::std::marker::Send for Http11ProxyUpstreamTransportView<'_> {}

impl<'msg> ::protobuf::AsView for Http11ProxyUpstreamTransportView<'msg> {
  type Proxied = Http11ProxyUpstreamTransport;
  fn as_view(&self) -> ::protobuf::View<'msg, Http11ProxyUpstreamTransport> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Http11ProxyUpstreamTransportView<'msg> {
  fn into_view<'shorter>(self) -> Http11ProxyUpstreamTransportView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Http11ProxyUpstreamTransport> for Http11ProxyUpstreamTransportView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http11ProxyUpstreamTransport {
    let mut dst = Http11ProxyUpstreamTransport::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Http11ProxyUpstreamTransport> for Http11ProxyUpstreamTransportMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http11ProxyUpstreamTransport {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Http11ProxyUpstreamTransport {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Http11ProxyUpstreamTransportView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Http11ProxyUpstreamTransportMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Http11ProxyUpstreamTransportMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http11ProxyUpstreamTransport>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Http11ProxyUpstreamTransportMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Http11ProxyUpstreamTransportMut<'msg> {
  type Message = Http11ProxyUpstreamTransport;
}

impl ::std::fmt::Debug for Http11ProxyUpstreamTransportMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Http11ProxyUpstreamTransport>> for Http11ProxyUpstreamTransportMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http11ProxyUpstreamTransport>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Http11ProxyUpstreamTransportMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Http11ProxyUpstreamTransport> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Http11ProxyUpstreamTransport {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_transport_socket(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn transport_socket_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(&self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }
  pub fn transport_socket_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketMut<'_> {
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
  pub fn set_transport_socket(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::TransportSocket>) {

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
// - `Http11ProxyUpstreamTransportMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Http11ProxyUpstreamTransportMut<'_> {}

// SAFETY:
// - `Http11ProxyUpstreamTransportMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Http11ProxyUpstreamTransportMut<'_> {}

impl<'msg> ::protobuf::AsView for Http11ProxyUpstreamTransportMut<'msg> {
  type Proxied = Http11ProxyUpstreamTransport;
  fn as_view(&self) -> ::protobuf::View<'_, Http11ProxyUpstreamTransport> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Http11ProxyUpstreamTransportMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Http11ProxyUpstreamTransport>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Http11ProxyUpstreamTransportMut<'msg> {
  type MutProxied = Http11ProxyUpstreamTransport;
  fn as_mut(&mut self) -> Http11ProxyUpstreamTransportMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Http11ProxyUpstreamTransportMut<'msg> {
  fn into_mut<'shorter>(self) -> Http11ProxyUpstreamTransportMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Http11ProxyUpstreamTransport {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Http11ProxyUpstreamTransport> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Http11ProxyUpstreamTransportView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Http11ProxyUpstreamTransportMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_transport_socket(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn transport_socket_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(&self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }
  pub fn transport_socket_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketMut<'_> {
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
  pub fn set_transport_socket(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::TransportSocket>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl Http11ProxyUpstreamTransport

impl ::std::ops::Drop for Http11ProxyUpstreamTransport {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Http11ProxyUpstreamTransport {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Http11ProxyUpstreamTransport {
  type Proxied = Self;
  fn as_view(&self) -> Http11ProxyUpstreamTransportView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Http11ProxyUpstreamTransport {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Http11ProxyUpstreamTransportMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Http11ProxyUpstreamTransport {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__http_011_0proxy__v3__Http11ProxyUpstreamTransport_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__http_011_0proxy__v3__Http11ProxyUpstreamTransport_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::TransportSocket as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__http_011_0proxy__v3__Http11ProxyUpstreamTransport_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Http11ProxyUpstreamTransport {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Http11ProxyUpstreamTransport {
  type Msg = Http11ProxyUpstreamTransport;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http11ProxyUpstreamTransport> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http11ProxyUpstreamTransport {
  type Msg = Http11ProxyUpstreamTransport;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http11ProxyUpstreamTransport> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Http11ProxyUpstreamTransportMut<'_> {
  type Msg = Http11ProxyUpstreamTransport;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http11ProxyUpstreamTransport> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http11ProxyUpstreamTransportMut<'_> {
  type Msg = Http11ProxyUpstreamTransport;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http11ProxyUpstreamTransport> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http11ProxyUpstreamTransportView<'_> {
  type Msg = Http11ProxyUpstreamTransport;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http11ProxyUpstreamTransport> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Http11ProxyUpstreamTransportMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



