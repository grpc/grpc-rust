const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__EventServiceConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EventServiceConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EventServiceConfig>
}

impl ::protobuf::Message for EventServiceConfig {
  type MessageView<'msg> = EventServiceConfigView<'msg>;
  type MessageMut<'msg> = EventServiceConfigMut<'msg>;
}

impl ::std::default::Default for EventServiceConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EventServiceConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EventServiceConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `EventServiceConfigMut`.
unsafe impl ::std::marker::Sync for EventServiceConfig {}

// SAFETY:
// - `EventServiceConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for EventServiceConfig {}

impl ::protobuf::Proxied for EventServiceConfig {
  type View<'msg> = EventServiceConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EventServiceConfig {}

impl ::protobuf::MutProxied for EventServiceConfig {
  type Mut<'msg> = EventServiceConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EventServiceConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EventServiceConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EventServiceConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EventServiceConfigView<'msg> {
  type Message = EventServiceConfig;
}

impl ::std::fmt::Debug for EventServiceConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EventServiceConfigView<'_> {
  fn default() -> EventServiceConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EventServiceConfig>> for EventServiceConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EventServiceConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EventServiceConfigView<'msg> {

  pub fn to_owned(&self) -> EventServiceConfig {
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

  pub fn config_source_specifier(self) -> super::event_service_config::ConfigSourceSpecifierOneof<'msg> {
    match self.config_source_specifier_case() {
      super::event_service_config::ConfigSourceSpecifierCase::GrpcService =>
          super::event_service_config::ConfigSourceSpecifierOneof::GrpcService(self.grpc_service()),
      _ => super::event_service_config::ConfigSourceSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_source_specifier_case(self) -> super::event_service_config::ConfigSourceSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::event_service_config::ConfigSourceSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `EventServiceConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EventServiceConfigView<'_> {}

// SAFETY:
// - `EventServiceConfigView` is `Send` because while its alive a `EventServiceConfigMut` cannot.
// - `EventServiceConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for EventServiceConfigView<'_> {}

impl<'msg> ::protobuf::AsView for EventServiceConfigView<'msg> {
  type Proxied = EventServiceConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, EventServiceConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EventServiceConfigView<'msg> {
  fn into_view<'shorter>(self) -> EventServiceConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EventServiceConfig> for EventServiceConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EventServiceConfig {
    let mut dst = EventServiceConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EventServiceConfig> for EventServiceConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EventServiceConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for EventServiceConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EventServiceConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EventServiceConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EventServiceConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EventServiceConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EventServiceConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EventServiceConfigMut<'msg> {
  type Message = EventServiceConfig;
}

impl ::std::fmt::Debug for EventServiceConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EventServiceConfig>> for EventServiceConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EventServiceConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EventServiceConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EventServiceConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> EventServiceConfig {
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

  pub fn config_source_specifier(&self) -> super::event_service_config::ConfigSourceSpecifierOneof<'_> {
    match &self.config_source_specifier_case() {
      super::event_service_config::ConfigSourceSpecifierCase::GrpcService =>
          super::event_service_config::ConfigSourceSpecifierOneof::GrpcService(self.grpc_service()),
      _ => super::event_service_config::ConfigSourceSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_source_specifier_case(&self) -> super::event_service_config::ConfigSourceSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::event_service_config::ConfigSourceSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `EventServiceConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EventServiceConfigMut<'_> {}

// SAFETY:
// - `EventServiceConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EventServiceConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for EventServiceConfigMut<'msg> {
  type Proxied = EventServiceConfig;
  fn as_view(&self) -> ::protobuf::View<'_, EventServiceConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EventServiceConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EventServiceConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EventServiceConfigMut<'msg> {
  type MutProxied = EventServiceConfig;
  fn as_mut(&mut self) -> EventServiceConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EventServiceConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> EventServiceConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EventServiceConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EventServiceConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EventServiceConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EventServiceConfigMut<'_> {
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

  pub fn config_source_specifier(&self) -> super::event_service_config::ConfigSourceSpecifierOneof<'_> {
    match &self.config_source_specifier_case() {
      super::event_service_config::ConfigSourceSpecifierCase::GrpcService =>
          super::event_service_config::ConfigSourceSpecifierOneof::GrpcService(self.grpc_service()),
      _ => super::event_service_config::ConfigSourceSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_source_specifier_case(&self) -> super::event_service_config::ConfigSourceSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::event_service_config::ConfigSourceSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl EventServiceConfig

impl ::std::ops::Drop for EventServiceConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EventServiceConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EventServiceConfig {
  type Proxied = Self;
  fn as_view(&self) -> EventServiceConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EventServiceConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EventServiceConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EventServiceConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__EventServiceConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3^!");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__EventServiceConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__EventServiceConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EventServiceConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EventServiceConfig {
  type Msg = EventServiceConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EventServiceConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EventServiceConfig {
  type Msg = EventServiceConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EventServiceConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EventServiceConfigMut<'_> {
  type Msg = EventServiceConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EventServiceConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EventServiceConfigMut<'_> {
  type Msg = EventServiceConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EventServiceConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EventServiceConfigView<'_> {
  type Msg = EventServiceConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EventServiceConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EventServiceConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod event_service_config {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigSourceSpecifierOneof<'msg> {
  GrpcService(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::grpc_service::GrpcService>) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigSourceSpecifierCase {
  GrpcService = 1,

  not_set = 0
}

impl ConfigSourceSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigSourceSpecifierCase> {
    match v {
      0 => Some(ConfigSourceSpecifierCase::not_set),
      1 => Some(ConfigSourceSpecifierCase::GrpcService),
      _ => None
    }
  }
}
}  // pub mod event_service_config


