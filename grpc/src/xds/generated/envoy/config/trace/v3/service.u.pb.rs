const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__trace__v3__TraceServiceConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TraceServiceConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TraceServiceConfig>
}

impl ::protobuf::Message for TraceServiceConfig {
  type MessageView<'msg> = TraceServiceConfigView<'msg>;
  type MessageMut<'msg> = TraceServiceConfigMut<'msg>;
}

impl ::std::default::Default for TraceServiceConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TraceServiceConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TraceServiceConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `TraceServiceConfigMut`.
unsafe impl ::std::marker::Sync for TraceServiceConfig {}

// SAFETY:
// - `TraceServiceConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TraceServiceConfig {}

impl ::protobuf::Proxied for TraceServiceConfig {
  type View<'msg> = TraceServiceConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TraceServiceConfig {}

impl ::protobuf::MutProxied for TraceServiceConfig {
  type Mut<'msg> = TraceServiceConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TraceServiceConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TraceServiceConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TraceServiceConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TraceServiceConfigView<'msg> {
  type Message = TraceServiceConfig;
}

impl ::std::fmt::Debug for TraceServiceConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TraceServiceConfigView<'_> {
  fn default() -> TraceServiceConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TraceServiceConfig>> for TraceServiceConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TraceServiceConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TraceServiceConfigView<'msg> {

  pub fn to_owned(&self) -> TraceServiceConfig {
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

}

// SAFETY:
// - `TraceServiceConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TraceServiceConfigView<'_> {}

// SAFETY:
// - `TraceServiceConfigView` is `Send` because while its alive a `TraceServiceConfigMut` cannot.
// - `TraceServiceConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for TraceServiceConfigView<'_> {}

impl<'msg> ::protobuf::AsView for TraceServiceConfigView<'msg> {
  type Proxied = TraceServiceConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, TraceServiceConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TraceServiceConfigView<'msg> {
  fn into_view<'shorter>(self) -> TraceServiceConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TraceServiceConfig> for TraceServiceConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TraceServiceConfig {
    let mut dst = TraceServiceConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TraceServiceConfig> for TraceServiceConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TraceServiceConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TraceServiceConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TraceServiceConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TraceServiceConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TraceServiceConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TraceServiceConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TraceServiceConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TraceServiceConfigMut<'msg> {
  type Message = TraceServiceConfig;
}

impl ::std::fmt::Debug for TraceServiceConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TraceServiceConfig>> for TraceServiceConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TraceServiceConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TraceServiceConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TraceServiceConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TraceServiceConfig {
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

}

// SAFETY:
// - `TraceServiceConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TraceServiceConfigMut<'_> {}

// SAFETY:
// - `TraceServiceConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TraceServiceConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for TraceServiceConfigMut<'msg> {
  type Proxied = TraceServiceConfig;
  fn as_view(&self) -> ::protobuf::View<'_, TraceServiceConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TraceServiceConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TraceServiceConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TraceServiceConfigMut<'msg> {
  type MutProxied = TraceServiceConfig;
  fn as_mut(&mut self) -> TraceServiceConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TraceServiceConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> TraceServiceConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TraceServiceConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TraceServiceConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TraceServiceConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TraceServiceConfigMut<'_> {
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

}  // impl TraceServiceConfig

impl ::std::ops::Drop for TraceServiceConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TraceServiceConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TraceServiceConfig {
  type Proxied = Self;
  fn as_view(&self) -> TraceServiceConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TraceServiceConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TraceServiceConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TraceServiceConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__trace__v3__TraceServiceConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__trace__v3__TraceServiceConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__trace__v3__TraceServiceConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TraceServiceConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TraceServiceConfig {
  type Msg = TraceServiceConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TraceServiceConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TraceServiceConfig {
  type Msg = TraceServiceConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TraceServiceConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TraceServiceConfigMut<'_> {
  type Msg = TraceServiceConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TraceServiceConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TraceServiceConfigMut<'_> {
  type Msg = TraceServiceConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TraceServiceConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TraceServiceConfigView<'_> {
  type Msg = TraceServiceConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TraceServiceConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TraceServiceConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



