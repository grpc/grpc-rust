const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__AttributeContext_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AttributeContext {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AttributeContext>
}

impl ::protobuf::Message for AttributeContext {
  type MessageView<'msg> = AttributeContextView<'msg>;
  type MessageMut<'msg> = AttributeContextMut<'msg>;
}

impl ::std::default::Default for AttributeContext {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AttributeContext {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AttributeContext` is `Sync` because it does not implement interior mutability.
//    Neither does `AttributeContextMut`.
unsafe impl ::std::marker::Sync for AttributeContext {}

// SAFETY:
// - `AttributeContext` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AttributeContext {}

impl ::protobuf::Proxied for AttributeContext {
  type View<'msg> = AttributeContextView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AttributeContext {}

impl ::protobuf::MutProxied for AttributeContext {
  type Mut<'msg> = AttributeContextMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AttributeContextView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AttributeContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AttributeContextView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AttributeContextView<'msg> {
  type Message = AttributeContext;
}

impl ::std::fmt::Debug for AttributeContextView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AttributeContextView<'_> {
  fn default() -> AttributeContextView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AttributeContext>> for AttributeContextView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AttributeContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AttributeContextView<'msg> {

  pub fn to_owned(&self) -> AttributeContext {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // source: optional message envoy.service.auth.v3.AttributeContext.Peer
  pub fn has_source(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn source_opt(self) -> ::std::option::Option<super::attribute_context::PeerView<'msg>> {
    self.has_source().then(|| self.source())
  }
  pub fn source(self) -> super::attribute_context::PeerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::PeerView::default())
  }

  // destination: optional message envoy.service.auth.v3.AttributeContext.Peer
  pub fn has_destination(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn destination_opt(self) -> ::std::option::Option<super::attribute_context::PeerView<'msg>> {
    self.has_destination().then(|| self.destination())
  }
  pub fn destination(self) -> super::attribute_context::PeerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::PeerView::default())
  }

  // request: optional message envoy.service.auth.v3.AttributeContext.Request
  pub fn has_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn request_opt(self) -> ::std::option::Option<super::attribute_context::RequestView<'msg>> {
    self.has_request().then(|| self.request())
  }
  pub fn request(self) -> super::attribute_context::RequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::RequestView::default())
  }

  // context_extensions: repeated message envoy.service.auth.v3.AttributeContext.ContextExtensionsEntry
  pub fn context_extensions(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // metadata_context: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn metadata_context_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg>> {
    self.has_metadata_context().then(|| self.metadata_context())
  }
  pub fn metadata_context(self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }

  // route_metadata_context: optional message envoy.config.core.v3.Metadata
  pub fn has_route_metadata_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn route_metadata_context_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg>> {
    self.has_route_metadata_context().then(|| self.route_metadata_context())
  }
  pub fn route_metadata_context(self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }

  // tls_session: optional message envoy.service.auth.v3.AttributeContext.TLSSession
  pub fn has_tls_session(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn tls_session_opt(self) -> ::std::option::Option<super::attribute_context::TLSSessionView<'msg>> {
    self.has_tls_session().then(|| self.tls_session())
  }
  pub fn tls_session(self) -> super::attribute_context::TLSSessionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::TLSSessionView::default())
  }

}

// SAFETY:
// - `AttributeContextView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AttributeContextView<'_> {}

// SAFETY:
// - `AttributeContextView` is `Send` because while its alive a `AttributeContextMut` cannot.
// - `AttributeContextView` does not use thread-local data.
unsafe impl ::std::marker::Send for AttributeContextView<'_> {}

impl<'msg> ::protobuf::AsView for AttributeContextView<'msg> {
  type Proxied = AttributeContext;
  fn as_view(&self) -> ::protobuf::View<'msg, AttributeContext> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AttributeContextView<'msg> {
  fn into_view<'shorter>(self) -> AttributeContextView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AttributeContext> for AttributeContextView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AttributeContext {
    let mut dst = AttributeContext::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AttributeContext> for AttributeContextMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AttributeContext {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AttributeContext {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AttributeContextView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AttributeContextMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AttributeContextMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AttributeContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AttributeContextMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AttributeContextMut<'msg> {
  type Message = AttributeContext;
}

impl ::std::fmt::Debug for AttributeContextMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AttributeContext>> for AttributeContextMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AttributeContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AttributeContextMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AttributeContext> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AttributeContext {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // source: optional message envoy.service.auth.v3.AttributeContext.Peer
  pub fn has_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn source_opt(&self) -> ::std::option::Option<super::attribute_context::PeerView<'_>> {
    self.has_source().then(|| self.source())
  }
  pub fn source(&self) -> super::attribute_context::PeerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::PeerView::default())
  }
  pub fn source_mut(&mut self) -> super::attribute_context::PeerMut<'_> {
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
  pub fn set_source(&mut self,
    val: impl ::protobuf::IntoProxied<super::attribute_context::Peer>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // destination: optional message envoy.service.auth.v3.AttributeContext.Peer
  pub fn has_destination(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_destination(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn destination_opt(&self) -> ::std::option::Option<super::attribute_context::PeerView<'_>> {
    self.has_destination().then(|| self.destination())
  }
  pub fn destination(&self) -> super::attribute_context::PeerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::PeerView::default())
  }
  pub fn destination_mut(&mut self) -> super::attribute_context::PeerMut<'_> {
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
  pub fn set_destination(&mut self,
    val: impl ::protobuf::IntoProxied<super::attribute_context::Peer>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // request: optional message envoy.service.auth.v3.AttributeContext.Request
  pub fn has_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn request_opt(&self) -> ::std::option::Option<super::attribute_context::RequestView<'_>> {
    self.has_request().then(|| self.request())
  }
  pub fn request(&self) -> super::attribute_context::RequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::RequestView::default())
  }
  pub fn request_mut(&mut self) -> super::attribute_context::RequestMut<'_> {
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
  pub fn set_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::attribute_context::Request>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // context_extensions: repeated message envoy.service.auth.v3.AttributeContext.ContextExtensionsEntry
  pub fn context_extensions(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn context_extensions_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_context_extensions(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // metadata_context: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_metadata_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn metadata_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata_context().then(|| self.metadata_context())
  }
  pub fn metadata_context(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_context_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // route_metadata_context: optional message envoy.config.core.v3.Metadata
  pub fn has_route_metadata_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_route_metadata_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn route_metadata_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_route_metadata_context().then(|| self.route_metadata_context())
  }
  pub fn route_metadata_context(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn route_metadata_context_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_route_metadata_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // tls_session: optional message envoy.service.auth.v3.AttributeContext.TLSSession
  pub fn has_tls_session(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_tls_session(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn tls_session_opt(&self) -> ::std::option::Option<super::attribute_context::TLSSessionView<'_>> {
    self.has_tls_session().then(|| self.tls_session())
  }
  pub fn tls_session(&self) -> super::attribute_context::TLSSessionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::TLSSessionView::default())
  }
  pub fn tls_session_mut(&mut self) -> super::attribute_context::TLSSessionMut<'_> {
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
  pub fn set_tls_session(&mut self,
    val: impl ::protobuf::IntoProxied<super::attribute_context::TLSSession>) {

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
// - `AttributeContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AttributeContextMut<'_> {}

// SAFETY:
// - `AttributeContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AttributeContextMut<'_> {}

impl<'msg> ::protobuf::AsView for AttributeContextMut<'msg> {
  type Proxied = AttributeContext;
  fn as_view(&self) -> ::protobuf::View<'_, AttributeContext> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AttributeContextMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AttributeContext>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AttributeContextMut<'msg> {
  type MutProxied = AttributeContext;
  fn as_mut(&mut self) -> AttributeContextMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AttributeContextMut<'msg> {
  fn into_mut<'shorter>(self) -> AttributeContextMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AttributeContext {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AttributeContext> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AttributeContextView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AttributeContextMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // source: optional message envoy.service.auth.v3.AttributeContext.Peer
  pub fn has_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn source_opt(&self) -> ::std::option::Option<super::attribute_context::PeerView<'_>> {
    self.has_source().then(|| self.source())
  }
  pub fn source(&self) -> super::attribute_context::PeerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::PeerView::default())
  }
  pub fn source_mut(&mut self) -> super::attribute_context::PeerMut<'_> {
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
  pub fn set_source(&mut self,
    val: impl ::protobuf::IntoProxied<super::attribute_context::Peer>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // destination: optional message envoy.service.auth.v3.AttributeContext.Peer
  pub fn has_destination(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_destination(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn destination_opt(&self) -> ::std::option::Option<super::attribute_context::PeerView<'_>> {
    self.has_destination().then(|| self.destination())
  }
  pub fn destination(&self) -> super::attribute_context::PeerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::PeerView::default())
  }
  pub fn destination_mut(&mut self) -> super::attribute_context::PeerMut<'_> {
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
  pub fn set_destination(&mut self,
    val: impl ::protobuf::IntoProxied<super::attribute_context::Peer>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // request: optional message envoy.service.auth.v3.AttributeContext.Request
  pub fn has_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn request_opt(&self) -> ::std::option::Option<super::attribute_context::RequestView<'_>> {
    self.has_request().then(|| self.request())
  }
  pub fn request(&self) -> super::attribute_context::RequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::RequestView::default())
  }
  pub fn request_mut(&mut self) -> super::attribute_context::RequestMut<'_> {
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
  pub fn set_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::attribute_context::Request>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // context_extensions: repeated message envoy.service.auth.v3.AttributeContext.ContextExtensionsEntry
  pub fn context_extensions(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn context_extensions_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_context_extensions(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // metadata_context: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_metadata_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn metadata_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata_context().then(|| self.metadata_context())
  }
  pub fn metadata_context(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_context_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // route_metadata_context: optional message envoy.config.core.v3.Metadata
  pub fn has_route_metadata_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_route_metadata_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn route_metadata_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_route_metadata_context().then(|| self.route_metadata_context())
  }
  pub fn route_metadata_context(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn route_metadata_context_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_route_metadata_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // tls_session: optional message envoy.service.auth.v3.AttributeContext.TLSSession
  pub fn has_tls_session(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_tls_session(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn tls_session_opt(&self) -> ::std::option::Option<super::attribute_context::TLSSessionView<'_>> {
    self.has_tls_session().then(|| self.tls_session())
  }
  pub fn tls_session(&self) -> super::attribute_context::TLSSessionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::attribute_context::TLSSessionView::default())
  }
  pub fn tls_session_mut(&mut self) -> super::attribute_context::TLSSessionMut<'_> {
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
  pub fn set_tls_session(&mut self,
    val: impl ::protobuf::IntoProxied<super::attribute_context::TLSSession>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}  // impl AttributeContext

impl ::std::ops::Drop for AttributeContext {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AttributeContext {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AttributeContext {
  type Proxied = Self;
  fn as_view(&self) -> AttributeContextView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AttributeContext {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AttributeContextMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AttributeContext {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__auth__v3__AttributeContext_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33a3eG333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__auth__v3__AttributeContext_msg_init.0, &[<super::attribute_context::Peer as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::attribute_context::Peer as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::attribute_context::Request as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::attribute_context::ContextExtensionsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::attribute_context::TLSSession as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__auth__v3__AttributeContext_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AttributeContext {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AttributeContext {
  type Msg = AttributeContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AttributeContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AttributeContext {
  type Msg = AttributeContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AttributeContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AttributeContextMut<'_> {
  type Msg = AttributeContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AttributeContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AttributeContextMut<'_> {
  type Msg = AttributeContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AttributeContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AttributeContextView<'_> {
  type Msg = AttributeContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AttributeContext> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AttributeContextMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod attribute_context {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__AttributeContext__Peer_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Peer {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Peer>
}

impl ::protobuf::Message for Peer {
  type MessageView<'msg> = PeerView<'msg>;
  type MessageMut<'msg> = PeerMut<'msg>;
}

impl ::std::default::Default for Peer {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Peer {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Peer` is `Sync` because it does not implement interior mutability.
//    Neither does `PeerMut`.
unsafe impl ::std::marker::Sync for Peer {}

// SAFETY:
// - `Peer` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Peer {}

impl ::protobuf::Proxied for Peer {
  type View<'msg> = PeerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Peer {}

impl ::protobuf::MutProxied for Peer {
  type Mut<'msg> = PeerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PeerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Peer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PeerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PeerView<'msg> {
  type Message = Peer;
}

impl ::std::fmt::Debug for PeerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PeerView<'_> {
  fn default() -> PeerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Peer>> for PeerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Peer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PeerView<'msg> {

  pub fn to_owned(&self) -> Peer {
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

  // service: optional string
  pub fn service(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // labels: repeated message envoy.service.auth.v3.AttributeContext.Peer.LabelsEntry
  pub fn labels(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(2)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // principal: optional string
  pub fn principal(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // certificate: optional string
  pub fn certificate(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `PeerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PeerView<'_> {}

// SAFETY:
// - `PeerView` is `Send` because while its alive a `PeerMut` cannot.
// - `PeerView` does not use thread-local data.
unsafe impl ::std::marker::Send for PeerView<'_> {}

impl<'msg> ::protobuf::AsView for PeerView<'msg> {
  type Proxied = Peer;
  fn as_view(&self) -> ::protobuf::View<'msg, Peer> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PeerView<'msg> {
  fn into_view<'shorter>(self) -> PeerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Peer> for PeerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Peer {
    let mut dst = Peer::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Peer> for PeerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Peer {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Peer {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PeerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PeerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PeerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Peer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PeerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PeerMut<'msg> {
  type Message = Peer;
}

impl ::std::fmt::Debug for PeerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Peer>> for PeerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Peer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PeerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Peer> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Peer {
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

  // service: optional string
  pub fn service(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // labels: repeated message envoy.service.auth.v3.AttributeContext.Peer.LabelsEntry
  pub fn labels(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(2)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn labels_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          2, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // principal: optional string
  pub fn principal(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_principal(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // certificate: optional string
  pub fn certificate(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_certificate(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

}

// SAFETY:
// - `PeerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PeerMut<'_> {}

// SAFETY:
// - `PeerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PeerMut<'_> {}

impl<'msg> ::protobuf::AsView for PeerMut<'msg> {
  type Proxied = Peer;
  fn as_view(&self) -> ::protobuf::View<'_, Peer> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PeerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Peer>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PeerMut<'msg> {
  type MutProxied = Peer;
  fn as_mut(&mut self) -> PeerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PeerMut<'msg> {
  fn into_mut<'shorter>(self) -> PeerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Peer {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Peer> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PeerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PeerMut<'_> {
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

  // service: optional string
  pub fn service(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // labels: repeated message envoy.service.auth.v3.AttributeContext.Peer.LabelsEntry
  pub fn labels(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(2)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn labels_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          2, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // principal: optional string
  pub fn principal(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_principal(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // certificate: optional string
  pub fn certificate(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_certificate(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

}  // impl Peer

impl ::std::ops::Drop for Peer {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Peer {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Peer {
  type Proxied = Self;
  fn as_view(&self) -> PeerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Peer {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PeerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Peer {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::attribute_context::envoy__service__auth__v3__AttributeContext__Peer_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31XG1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::attribute_context::envoy__service__auth__v3__AttributeContext__Peer_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::attribute_context::peer::LabelsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::attribute_context::envoy__service__auth__v3__AttributeContext__Peer_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Peer {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Peer {
  type Msg = Peer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Peer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Peer {
  type Msg = Peer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Peer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PeerMut<'_> {
  type Msg = Peer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Peer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PeerMut<'_> {
  type Msg = Peer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Peer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PeerView<'_> {
  type Msg = Peer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Peer> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PeerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod peer {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__AttributeContext__Peer__LabelsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct LabelsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LabelsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::attribute_context::peer::envoy__service__auth__v3__AttributeContext__Peer__LabelsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::attribute_context::peer::envoy__service__auth__v3__AttributeContext__Peer__LabelsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::attribute_context::peer::envoy__service__auth__v3__AttributeContext__Peer__LabelsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod peer

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__AttributeContext__Request_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Request {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Request>
}

impl ::protobuf::Message for Request {
  type MessageView<'msg> = RequestView<'msg>;
  type MessageMut<'msg> = RequestMut<'msg>;
}

impl ::std::default::Default for Request {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Request {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Request` is `Sync` because it does not implement interior mutability.
//    Neither does `RequestMut`.
unsafe impl ::std::marker::Sync for Request {}

// SAFETY:
// - `Request` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Request {}

impl ::protobuf::Proxied for Request {
  type View<'msg> = RequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Request {}

impl ::protobuf::MutProxied for Request {
  type Mut<'msg> = RequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Request>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RequestView<'msg> {
  type Message = Request;
}

impl ::std::fmt::Debug for RequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RequestView<'_> {
  fn default() -> RequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Request>> for RequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Request>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RequestView<'msg> {

  pub fn to_owned(&self) -> Request {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // time: optional message google.protobuf.Timestamp
  pub fn has_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn time_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_time().then(|| self.time())
  }
  pub fn time(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // http: optional message envoy.service.auth.v3.AttributeContext.HttpRequest
  pub fn has_http(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn http_opt(self) -> ::std::option::Option<super::super::attribute_context::HttpRequestView<'msg>> {
    self.has_http().then(|| self.http())
  }
  pub fn http(self) -> super::super::attribute_context::HttpRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::attribute_context::HttpRequestView::default())
  }

}

// SAFETY:
// - `RequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RequestView<'_> {}

// SAFETY:
// - `RequestView` is `Send` because while its alive a `RequestMut` cannot.
// - `RequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for RequestView<'_> {}

impl<'msg> ::protobuf::AsView for RequestView<'msg> {
  type Proxied = Request;
  fn as_view(&self) -> ::protobuf::View<'msg, Request> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RequestView<'msg> {
  fn into_view<'shorter>(self) -> RequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Request> for RequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Request {
    let mut dst = Request::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Request> for RequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Request {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Request {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Request>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RequestMut<'msg> {
  type Message = Request;
}

impl ::std::fmt::Debug for RequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Request>> for RequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Request>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Request> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Request {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // time: optional message google.protobuf.Timestamp
  pub fn has_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn time_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_time().then(|| self.time())
  }
  pub fn time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // http: optional message envoy.service.auth.v3.AttributeContext.HttpRequest
  pub fn has_http(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_http(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn http_opt(&self) -> ::std::option::Option<super::super::attribute_context::HttpRequestView<'_>> {
    self.has_http().then(|| self.http())
  }
  pub fn http(&self) -> super::super::attribute_context::HttpRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::attribute_context::HttpRequestView::default())
  }
  pub fn http_mut(&mut self) -> super::super::attribute_context::HttpRequestMut<'_> {
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
  pub fn set_http(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::attribute_context::HttpRequest>) {

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
// - `RequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RequestMut<'_> {}

// SAFETY:
// - `RequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RequestMut<'_> {}

impl<'msg> ::protobuf::AsView for RequestMut<'msg> {
  type Proxied = Request;
  fn as_view(&self) -> ::protobuf::View<'_, Request> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Request>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RequestMut<'msg> {
  type MutProxied = Request;
  fn as_mut(&mut self) -> RequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RequestMut<'msg> {
  fn into_mut<'shorter>(self) -> RequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Request {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Request> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // time: optional message google.protobuf.Timestamp
  pub fn has_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn time_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_time().then(|| self.time())
  }
  pub fn time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // http: optional message envoy.service.auth.v3.AttributeContext.HttpRequest
  pub fn has_http(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_http(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn http_opt(&self) -> ::std::option::Option<super::super::attribute_context::HttpRequestView<'_>> {
    self.has_http().then(|| self.http())
  }
  pub fn http(&self) -> super::super::attribute_context::HttpRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::attribute_context::HttpRequestView::default())
  }
  pub fn http_mut(&mut self) -> super::super::attribute_context::HttpRequestMut<'_> {
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
  pub fn set_http(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::attribute_context::HttpRequest>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl Request

impl ::std::ops::Drop for Request {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Request {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Request {
  type Proxied = Self;
  fn as_view(&self) -> RequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Request {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Request {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::attribute_context::envoy__service__auth__v3__AttributeContext__Request_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::attribute_context::envoy__service__auth__v3__AttributeContext__Request_msg_init.0, &[<::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::attribute_context::HttpRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::attribute_context::envoy__service__auth__v3__AttributeContext__Request_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Request {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Request {
  type Msg = Request;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Request> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Request {
  type Msg = Request;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Request> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RequestMut<'_> {
  type Msg = Request;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Request> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RequestMut<'_> {
  type Msg = Request;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Request> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RequestView<'_> {
  type Msg = Request;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Request> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__AttributeContext__HttpRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpRequest>
}

impl ::protobuf::Message for HttpRequest {
  type MessageView<'msg> = HttpRequestView<'msg>;
  type MessageMut<'msg> = HttpRequestMut<'msg>;
}

impl ::std::default::Default for HttpRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpRequestMut`.
unsafe impl ::std::marker::Sync for HttpRequest {}

// SAFETY:
// - `HttpRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpRequest {}

impl ::protobuf::Proxied for HttpRequest {
  type View<'msg> = HttpRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpRequest {}

impl ::protobuf::MutProxied for HttpRequest {
  type Mut<'msg> = HttpRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpRequestView<'msg> {
  type Message = HttpRequest;
}

impl ::std::fmt::Debug for HttpRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpRequestView<'_> {
  fn default() -> HttpRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequest>> for HttpRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRequestView<'msg> {

  pub fn to_owned(&self) -> HttpRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional string
  pub fn id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // method: optional string
  pub fn method(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // headers: repeated message envoy.service.auth.v3.AttributeContext.HttpRequest.HeadersEntry
  pub fn headers(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(2)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // header_map: optional message envoy.config.core.v3.HeaderMap
  pub fn has_header_map(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn header_map_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'msg>> {
    self.has_header_map().then(|| self.header_map())
  }
  pub fn header_map(self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }

  // path: optional string
  pub fn path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // host: optional string
  pub fn host(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // scheme: optional string
  pub fn scheme(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // query: optional string
  pub fn query(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // fragment: optional string
  pub fn fragment(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // size: optional int64
  pub fn size(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        8, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // protocol: optional string
  pub fn protocol(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // body: optional string
  pub fn body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // raw_body: optional bytes
  pub fn raw_body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `HttpRequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpRequestView<'_> {}

// SAFETY:
// - `HttpRequestView` is `Send` because while its alive a `HttpRequestMut` cannot.
// - `HttpRequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpRequestView<'_> {}

impl<'msg> ::protobuf::AsView for HttpRequestView<'msg> {
  type Proxied = HttpRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRequestView<'msg> {
  fn into_view<'shorter>(self) -> HttpRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRequest> for HttpRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRequest {
    let mut dst = HttpRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRequest> for HttpRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpRequest {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpRequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpRequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpRequestMut<'msg> {
  type Message = HttpRequest;
}

impl ::std::fmt::Debug for HttpRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequest>> for HttpRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequest> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // method: optional string
  pub fn method(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_method(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // headers: repeated message envoy.service.auth.v3.AttributeContext.HttpRequest.HeadersEntry
  pub fn headers(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(2)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn headers_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          2, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_headers(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // header_map: optional message envoy.config.core.v3.HeaderMap
  pub fn has_header_map(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_header_map(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn header_map_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_>> {
    self.has_header_map().then(|| self.header_map())
  }
  pub fn header_map(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }
  pub fn header_map_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapMut<'_> {
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
  pub fn set_header_map(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // host: optional string
  pub fn host(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_host(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // scheme: optional string
  pub fn scheme(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_scheme(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // query: optional string
  pub fn query(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_query(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // fragment: optional string
  pub fn fragment(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_fragment(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // size: optional int64
  pub fn size(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        8, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_size(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        8, val.into()
      )
    }
  }

  // protocol: optional string
  pub fn protocol(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_protocol(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // body: optional string
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

  // raw_body: optional bytes
  pub fn raw_body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_raw_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val);
    }
  }

}

// SAFETY:
// - `HttpRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpRequestMut<'_> {}

// SAFETY:
// - `HttpRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpRequestMut<'msg> {
  type Proxied = HttpRequest;
  fn as_view(&self) -> ::protobuf::View<'_, HttpRequest> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpRequest>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpRequestMut<'msg> {
  type MutProxied = HttpRequest;
  fn as_mut(&mut self) -> HttpRequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // method: optional string
  pub fn method(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_method(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // headers: repeated message envoy.service.auth.v3.AttributeContext.HttpRequest.HeadersEntry
  pub fn headers(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(2)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn headers_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          2, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_headers(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // header_map: optional message envoy.config.core.v3.HeaderMap
  pub fn has_header_map(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_header_map(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn header_map_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_>> {
    self.has_header_map().then(|| self.header_map())
  }
  pub fn header_map(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }
  pub fn header_map_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapMut<'_> {
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
  pub fn set_header_map(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // host: optional string
  pub fn host(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_host(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // scheme: optional string
  pub fn scheme(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_scheme(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // query: optional string
  pub fn query(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_query(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // fragment: optional string
  pub fn fragment(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_fragment(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // size: optional int64
  pub fn size(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        8, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_size(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        8, val.into()
      )
    }
  }

  // protocol: optional string
  pub fn protocol(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_protocol(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // body: optional string
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

  // raw_body: optional bytes
  pub fn raw_body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_raw_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val);
    }
  }

}  // impl HttpRequest

impl ::std::ops::Drop for HttpRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpRequest {
  type Proxied = Self;
  fn as_view(&self) -> HttpRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::attribute_context::envoy__service__auth__v3__AttributeContext__HttpRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1XG1X1X1X1X1X+P1X1X0P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::attribute_context::envoy__service__auth__v3__AttributeContext__HttpRequest_msg_init.0, &[<super::super::attribute_context::http_request::HeadersEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderMap as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::attribute_context::envoy__service__auth__v3__AttributeContext__HttpRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRequest {
  type Msg = HttpRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequest {
  type Msg = HttpRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRequestMut<'_> {
  type Msg = HttpRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestMut<'_> {
  type Msg = HttpRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestView<'_> {
  type Msg = HttpRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod http_request {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__AttributeContext__HttpRequest__HeadersEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct HeadersEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeadersEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::attribute_context::http_request::envoy__service__auth__v3__AttributeContext__HttpRequest__HeadersEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::attribute_context::http_request::envoy__service__auth__v3__AttributeContext__HttpRequest__HeadersEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::attribute_context::http_request::envoy__service__auth__v3__AttributeContext__HttpRequest__HeadersEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod http_request

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__AttributeContext__TLSSession_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TLSSession {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TLSSession>
}

impl ::protobuf::Message for TLSSession {
  type MessageView<'msg> = TLSSessionView<'msg>;
  type MessageMut<'msg> = TLSSessionMut<'msg>;
}

impl ::std::default::Default for TLSSession {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TLSSession {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TLSSession` is `Sync` because it does not implement interior mutability.
//    Neither does `TLSSessionMut`.
unsafe impl ::std::marker::Sync for TLSSession {}

// SAFETY:
// - `TLSSession` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TLSSession {}

impl ::protobuf::Proxied for TLSSession {
  type View<'msg> = TLSSessionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TLSSession {}

impl ::protobuf::MutProxied for TLSSession {
  type Mut<'msg> = TLSSessionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TLSSessionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TLSSession>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TLSSessionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TLSSessionView<'msg> {
  type Message = TLSSession;
}

impl ::std::fmt::Debug for TLSSessionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TLSSessionView<'_> {
  fn default() -> TLSSessionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TLSSession>> for TLSSessionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TLSSession>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TLSSessionView<'msg> {

  pub fn to_owned(&self) -> TLSSession {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // sni: optional string
  pub fn sni(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `TLSSessionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TLSSessionView<'_> {}

// SAFETY:
// - `TLSSessionView` is `Send` because while its alive a `TLSSessionMut` cannot.
// - `TLSSessionView` does not use thread-local data.
unsafe impl ::std::marker::Send for TLSSessionView<'_> {}

impl<'msg> ::protobuf::AsView for TLSSessionView<'msg> {
  type Proxied = TLSSession;
  fn as_view(&self) -> ::protobuf::View<'msg, TLSSession> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TLSSessionView<'msg> {
  fn into_view<'shorter>(self) -> TLSSessionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TLSSession> for TLSSessionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TLSSession {
    let mut dst = TLSSession::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TLSSession> for TLSSessionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TLSSession {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TLSSession {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TLSSessionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TLSSessionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TLSSessionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TLSSession>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TLSSessionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TLSSessionMut<'msg> {
  type Message = TLSSession;
}

impl ::std::fmt::Debug for TLSSessionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TLSSession>> for TLSSessionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TLSSession>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TLSSessionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TLSSession> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TLSSession {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // sni: optional string
  pub fn sni(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_sni(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `TLSSessionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TLSSessionMut<'_> {}

// SAFETY:
// - `TLSSessionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TLSSessionMut<'_> {}

impl<'msg> ::protobuf::AsView for TLSSessionMut<'msg> {
  type Proxied = TLSSession;
  fn as_view(&self) -> ::protobuf::View<'_, TLSSession> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TLSSessionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TLSSession>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TLSSessionMut<'msg> {
  type MutProxied = TLSSession;
  fn as_mut(&mut self) -> TLSSessionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TLSSessionMut<'msg> {
  fn into_mut<'shorter>(self) -> TLSSessionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TLSSession {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TLSSession> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TLSSessionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TLSSessionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // sni: optional string
  pub fn sni(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_sni(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl TLSSession

impl ::std::ops::Drop for TLSSession {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TLSSession {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TLSSession {
  type Proxied = Self;
  fn as_view(&self) -> TLSSessionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TLSSession {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TLSSessionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TLSSession {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::attribute_context::envoy__service__auth__v3__AttributeContext__TLSSession_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::attribute_context::envoy__service__auth__v3__AttributeContext__TLSSession_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::attribute_context::envoy__service__auth__v3__AttributeContext__TLSSession_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TLSSession {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TLSSession {
  type Msg = TLSSession;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TLSSession> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TLSSession {
  type Msg = TLSSession;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TLSSession> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TLSSessionMut<'_> {
  type Msg = TLSSession;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TLSSession> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TLSSessionMut<'_> {
  type Msg = TLSSession;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TLSSession> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TLSSessionView<'_> {
  type Msg = TLSSession;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TLSSession> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TLSSessionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__AttributeContext__ContextExtensionsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct ContextExtensionsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ContextExtensionsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::attribute_context::envoy__service__auth__v3__AttributeContext__ContextExtensionsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::attribute_context::envoy__service__auth__v3__AttributeContext__ContextExtensionsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::attribute_context::envoy__service__auth__v3__AttributeContext__ContextExtensionsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod attribute_context


