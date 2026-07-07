const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GrpcService {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GrpcService>
}

impl ::protobuf::Message for GrpcService {
  type MessageView<'msg> = GrpcServiceView<'msg>;
  type MessageMut<'msg> = GrpcServiceMut<'msg>;
}

impl ::std::default::Default for GrpcService {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GrpcService {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GrpcService` is `Sync` because it does not implement interior mutability.
//    Neither does `GrpcServiceMut`.
unsafe impl ::std::marker::Sync for GrpcService {}

// SAFETY:
// - `GrpcService` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GrpcService {}

impl ::protobuf::Proxied for GrpcService {
  type View<'msg> = GrpcServiceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GrpcService {}

impl ::protobuf::MutProxied for GrpcService {
  type Mut<'msg> = GrpcServiceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GrpcServiceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcService>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcServiceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GrpcServiceView<'msg> {
  type Message = GrpcService;
}

impl ::std::fmt::Debug for GrpcServiceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GrpcServiceView<'_> {
  fn default() -> GrpcServiceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcService>> for GrpcServiceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcService>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcServiceView<'msg> {

  pub fn to_owned(&self) -> GrpcService {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // envoy_grpc: optional message envoy.config.core.v3.GrpcService.EnvoyGrpc
  pub fn has_envoy_grpc(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn envoy_grpc_opt(self) -> ::std::option::Option<super::grpc_service::EnvoyGrpcView<'msg>> {
    self.has_envoy_grpc().then(|| self.envoy_grpc())
  }
  pub fn envoy_grpc(self) -> super::grpc_service::EnvoyGrpcView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::grpc_service::EnvoyGrpcView::default())
  }

  // google_grpc: optional message envoy.config.core.v3.GrpcService.GoogleGrpc
  pub fn has_google_grpc(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn google_grpc_opt(self) -> ::std::option::Option<super::grpc_service::GoogleGrpcView<'msg>> {
    self.has_google_grpc().then(|| self.google_grpc())
  }
  pub fn google_grpc(self) -> super::grpc_service::GoogleGrpcView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::grpc_service::GoogleGrpcView::default())
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // initial_metadata: repeated message envoy.config.core.v3.HeaderValue
  pub fn initial_metadata(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn retry_policy_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'msg>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }

  pub fn target_specifier(self) -> super::grpc_service::TargetSpecifierOneof<'msg> {
    match self.target_specifier_case() {
      super::grpc_service::TargetSpecifierCase::EnvoyGrpc =>
          super::grpc_service::TargetSpecifierOneof::EnvoyGrpc(self.envoy_grpc()),
      super::grpc_service::TargetSpecifierCase::GoogleGrpc =>
          super::grpc_service::TargetSpecifierOneof::GoogleGrpc(self.google_grpc()),
      _ => super::grpc_service::TargetSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn target_specifier_case(self) -> super::grpc_service::TargetSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::grpc_service::TargetSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `GrpcServiceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GrpcServiceView<'_> {}

// SAFETY:
// - `GrpcServiceView` is `Send` because while its alive a `GrpcServiceMut` cannot.
// - `GrpcServiceView` does not use thread-local data.
unsafe impl ::std::marker::Send for GrpcServiceView<'_> {}

impl<'msg> ::protobuf::AsView for GrpcServiceView<'msg> {
  type Proxied = GrpcService;
  fn as_view(&self) -> ::protobuf::View<'msg, GrpcService> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcServiceView<'msg> {
  fn into_view<'shorter>(self) -> GrpcServiceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcService> for GrpcServiceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcService {
    let mut dst = GrpcService::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcService> for GrpcServiceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcService {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GrpcService {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcServiceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcServiceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GrpcServiceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcService>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcServiceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GrpcServiceMut<'msg> {
  type Message = GrpcService;
}

impl ::std::fmt::Debug for GrpcServiceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcService>> for GrpcServiceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcService>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcServiceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcService> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GrpcService {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // envoy_grpc: optional message envoy.config.core.v3.GrpcService.EnvoyGrpc
  pub fn has_envoy_grpc(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_envoy_grpc(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn envoy_grpc_opt(&self) -> ::std::option::Option<super::grpc_service::EnvoyGrpcView<'_>> {
    self.has_envoy_grpc().then(|| self.envoy_grpc())
  }
  pub fn envoy_grpc(&self) -> super::grpc_service::EnvoyGrpcView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::grpc_service::EnvoyGrpcView::default())
  }
  pub fn envoy_grpc_mut(&mut self) -> super::grpc_service::EnvoyGrpcMut<'_> {
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
  pub fn set_envoy_grpc(&mut self,
    val: impl ::protobuf::IntoProxied<super::grpc_service::EnvoyGrpc>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // google_grpc: optional message envoy.config.core.v3.GrpcService.GoogleGrpc
  pub fn has_google_grpc(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_google_grpc(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn google_grpc_opt(&self) -> ::std::option::Option<super::grpc_service::GoogleGrpcView<'_>> {
    self.has_google_grpc().then(|| self.google_grpc())
  }
  pub fn google_grpc(&self) -> super::grpc_service::GoogleGrpcView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::grpc_service::GoogleGrpcView::default())
  }
  pub fn google_grpc_mut(&mut self) -> super::grpc_service::GoogleGrpcMut<'_> {
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
  pub fn set_google_grpc(&mut self,
    val: impl ::protobuf::IntoProxied<super::grpc_service::GoogleGrpc>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // initial_metadata: repeated message envoy.config.core.v3.HeaderValue
  pub fn initial_metadata(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn initial_metadata_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
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
  pub fn set_initial_metadata(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(&self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RetryPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn target_specifier(&self) -> super::grpc_service::TargetSpecifierOneof<'_> {
    match &self.target_specifier_case() {
      super::grpc_service::TargetSpecifierCase::EnvoyGrpc =>
          super::grpc_service::TargetSpecifierOneof::EnvoyGrpc(self.envoy_grpc()),
      super::grpc_service::TargetSpecifierCase::GoogleGrpc =>
          super::grpc_service::TargetSpecifierOneof::GoogleGrpc(self.google_grpc()),
      _ => super::grpc_service::TargetSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn target_specifier_case(&self) -> super::grpc_service::TargetSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::grpc_service::TargetSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `GrpcServiceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GrpcServiceMut<'_> {}

// SAFETY:
// - `GrpcServiceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GrpcServiceMut<'_> {}

impl<'msg> ::protobuf::AsView for GrpcServiceMut<'msg> {
  type Proxied = GrpcService;
  fn as_view(&self) -> ::protobuf::View<'_, GrpcService> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcServiceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GrpcService>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GrpcServiceMut<'msg> {
  type MutProxied = GrpcService;
  fn as_mut(&mut self) -> GrpcServiceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GrpcServiceMut<'msg> {
  fn into_mut<'shorter>(self) -> GrpcServiceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GrpcService {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GrpcService> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GrpcServiceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GrpcServiceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // envoy_grpc: optional message envoy.config.core.v3.GrpcService.EnvoyGrpc
  pub fn has_envoy_grpc(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_envoy_grpc(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn envoy_grpc_opt(&self) -> ::std::option::Option<super::grpc_service::EnvoyGrpcView<'_>> {
    self.has_envoy_grpc().then(|| self.envoy_grpc())
  }
  pub fn envoy_grpc(&self) -> super::grpc_service::EnvoyGrpcView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::grpc_service::EnvoyGrpcView::default())
  }
  pub fn envoy_grpc_mut(&mut self) -> super::grpc_service::EnvoyGrpcMut<'_> {
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
  pub fn set_envoy_grpc(&mut self,
    val: impl ::protobuf::IntoProxied<super::grpc_service::EnvoyGrpc>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // google_grpc: optional message envoy.config.core.v3.GrpcService.GoogleGrpc
  pub fn has_google_grpc(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_google_grpc(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn google_grpc_opt(&self) -> ::std::option::Option<super::grpc_service::GoogleGrpcView<'_>> {
    self.has_google_grpc().then(|| self.google_grpc())
  }
  pub fn google_grpc(&self) -> super::grpc_service::GoogleGrpcView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::grpc_service::GoogleGrpcView::default())
  }
  pub fn google_grpc_mut(&mut self) -> super::grpc_service::GoogleGrpcMut<'_> {
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
  pub fn set_google_grpc(&mut self,
    val: impl ::protobuf::IntoProxied<super::grpc_service::GoogleGrpc>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // initial_metadata: repeated message envoy.config.core.v3.HeaderValue
  pub fn initial_metadata(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn initial_metadata_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValue> {
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
  pub fn set_initial_metadata(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(&self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RetryPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn target_specifier(&self) -> super::grpc_service::TargetSpecifierOneof<'_> {
    match &self.target_specifier_case() {
      super::grpc_service::TargetSpecifierCase::EnvoyGrpc =>
          super::grpc_service::TargetSpecifierOneof::EnvoyGrpc(self.envoy_grpc()),
      super::grpc_service::TargetSpecifierCase::GoogleGrpc =>
          super::grpc_service::TargetSpecifierOneof::GoogleGrpc(self.google_grpc()),
      _ => super::grpc_service::TargetSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn target_specifier_case(&self) -> super::grpc_service::TargetSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::grpc_service::TargetSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl GrpcService

impl ::std::ops::Drop for GrpcService {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GrpcService {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GrpcService {
  type Proxied = Self;
  fn as_view(&self) -> GrpcServiceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GrpcService {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GrpcServiceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GrpcService {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__GrpcService_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333aG3^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__GrpcService_msg_init.0, &[<super::grpc_service::EnvoyGrpc as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::grpc_service::GoogleGrpc as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RetryPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__GrpcService_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcService {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcService {
  type Msg = GrpcService;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcService> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcService {
  type Msg = GrpcService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcService> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcServiceMut<'_> {
  type Msg = GrpcService;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcService> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcServiceMut<'_> {
  type Msg = GrpcService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcService> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcServiceView<'_> {
  type Msg = GrpcService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcService> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcServiceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod grpc_service {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__EnvoyGrpc_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EnvoyGrpc {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EnvoyGrpc>
}

impl ::protobuf::Message for EnvoyGrpc {
  type MessageView<'msg> = EnvoyGrpcView<'msg>;
  type MessageMut<'msg> = EnvoyGrpcMut<'msg>;
}

impl ::std::default::Default for EnvoyGrpc {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EnvoyGrpc {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EnvoyGrpc` is `Sync` because it does not implement interior mutability.
//    Neither does `EnvoyGrpcMut`.
unsafe impl ::std::marker::Sync for EnvoyGrpc {}

// SAFETY:
// - `EnvoyGrpc` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for EnvoyGrpc {}

impl ::protobuf::Proxied for EnvoyGrpc {
  type View<'msg> = EnvoyGrpcView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EnvoyGrpc {}

impl ::protobuf::MutProxied for EnvoyGrpc {
  type Mut<'msg> = EnvoyGrpcMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EnvoyGrpcView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnvoyGrpc>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnvoyGrpcView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EnvoyGrpcView<'msg> {
  type Message = EnvoyGrpc;
}

impl ::std::fmt::Debug for EnvoyGrpcView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EnvoyGrpcView<'_> {
  fn default() -> EnvoyGrpcView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EnvoyGrpc>> for EnvoyGrpcView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnvoyGrpc>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnvoyGrpcView<'msg> {

  pub fn to_owned(&self) -> EnvoyGrpc {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // cluster_name: optional string
  pub fn cluster_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // authority: optional string
  pub fn authority(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn retry_policy_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'msg>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }

  // max_receive_message_length: optional message google.protobuf.UInt32Value
  pub fn has_max_receive_message_length(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn max_receive_message_length_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_receive_message_length().then(|| self.max_receive_message_length())
  }
  pub fn max_receive_message_length(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // skip_envoy_headers: optional bool
  pub fn skip_envoy_headers(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `EnvoyGrpcView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EnvoyGrpcView<'_> {}

// SAFETY:
// - `EnvoyGrpcView` is `Send` because while its alive a `EnvoyGrpcMut` cannot.
// - `EnvoyGrpcView` does not use thread-local data.
unsafe impl ::std::marker::Send for EnvoyGrpcView<'_> {}

impl<'msg> ::protobuf::AsView for EnvoyGrpcView<'msg> {
  type Proxied = EnvoyGrpc;
  fn as_view(&self) -> ::protobuf::View<'msg, EnvoyGrpc> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnvoyGrpcView<'msg> {
  fn into_view<'shorter>(self) -> EnvoyGrpcView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EnvoyGrpc> for EnvoyGrpcView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnvoyGrpc {
    let mut dst = EnvoyGrpc::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EnvoyGrpc> for EnvoyGrpcMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnvoyGrpc {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for EnvoyGrpc {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EnvoyGrpcView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EnvoyGrpcMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EnvoyGrpcMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyGrpc>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnvoyGrpcMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EnvoyGrpcMut<'msg> {
  type Message = EnvoyGrpc;
}

impl ::std::fmt::Debug for EnvoyGrpcMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyGrpc>> for EnvoyGrpcMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyGrpc>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnvoyGrpcMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyGrpc> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> EnvoyGrpc {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // cluster_name: optional string
  pub fn cluster_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // authority: optional string
  pub fn authority(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authority(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(&self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RetryPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // max_receive_message_length: optional message google.protobuf.UInt32Value
  pub fn has_max_receive_message_length(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_max_receive_message_length(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn max_receive_message_length_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_receive_message_length().then(|| self.max_receive_message_length())
  }
  pub fn max_receive_message_length(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_receive_message_length_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_receive_message_length(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // skip_envoy_headers: optional bool
  pub fn skip_envoy_headers(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_skip_envoy_headers(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

}

// SAFETY:
// - `EnvoyGrpcMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EnvoyGrpcMut<'_> {}

// SAFETY:
// - `EnvoyGrpcMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EnvoyGrpcMut<'_> {}

impl<'msg> ::protobuf::AsView for EnvoyGrpcMut<'msg> {
  type Proxied = EnvoyGrpc;
  fn as_view(&self) -> ::protobuf::View<'_, EnvoyGrpc> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnvoyGrpcMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EnvoyGrpc>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EnvoyGrpcMut<'msg> {
  type MutProxied = EnvoyGrpc;
  fn as_mut(&mut self) -> EnvoyGrpcMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EnvoyGrpcMut<'msg> {
  fn into_mut<'shorter>(self) -> EnvoyGrpcMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EnvoyGrpc {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EnvoyGrpc> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EnvoyGrpcView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EnvoyGrpcMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // cluster_name: optional string
  pub fn cluster_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // authority: optional string
  pub fn authority(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authority(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(&self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RetryPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // max_receive_message_length: optional message google.protobuf.UInt32Value
  pub fn has_max_receive_message_length(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_max_receive_message_length(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn max_receive_message_length_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_receive_message_length().then(|| self.max_receive_message_length())
  }
  pub fn max_receive_message_length(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_receive_message_length_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_receive_message_length(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // skip_envoy_headers: optional bool
  pub fn skip_envoy_headers(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_skip_envoy_headers(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

}  // impl EnvoyGrpc

impl ::std::ops::Drop for EnvoyGrpc {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EnvoyGrpc {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EnvoyGrpc {
  type Proxied = Self;
  fn as_view(&self) -> EnvoyGrpcView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EnvoyGrpc {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EnvoyGrpcMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EnvoyGrpc {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::grpc_service::envoy__config__core__v3__GrpcService__EnvoyGrpc_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X33/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::grpc_service::envoy__config__core__v3__GrpcService__EnvoyGrpc_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::RetryPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::grpc_service::envoy__config__core__v3__GrpcService__EnvoyGrpc_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnvoyGrpc {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnvoyGrpc {
  type Msg = EnvoyGrpc;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyGrpc> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnvoyGrpc {
  type Msg = EnvoyGrpc;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyGrpc> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnvoyGrpcMut<'_> {
  type Msg = EnvoyGrpc;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyGrpc> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnvoyGrpcMut<'_> {
  type Msg = EnvoyGrpc;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyGrpc> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnvoyGrpcView<'_> {
  type Msg = EnvoyGrpc;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyGrpc> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnvoyGrpcMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GoogleGrpc {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GoogleGrpc>
}

impl ::protobuf::Message for GoogleGrpc {
  type MessageView<'msg> = GoogleGrpcView<'msg>;
  type MessageMut<'msg> = GoogleGrpcMut<'msg>;
}

impl ::std::default::Default for GoogleGrpc {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GoogleGrpc {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GoogleGrpc` is `Sync` because it does not implement interior mutability.
//    Neither does `GoogleGrpcMut`.
unsafe impl ::std::marker::Sync for GoogleGrpc {}

// SAFETY:
// - `GoogleGrpc` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GoogleGrpc {}

impl ::protobuf::Proxied for GoogleGrpc {
  type View<'msg> = GoogleGrpcView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GoogleGrpc {}

impl ::protobuf::MutProxied for GoogleGrpc {
  type Mut<'msg> = GoogleGrpcMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GoogleGrpcView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleGrpc>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleGrpcView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GoogleGrpcView<'msg> {
  type Message = GoogleGrpc;
}

impl ::std::fmt::Debug for GoogleGrpcView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GoogleGrpcView<'_> {
  fn default() -> GoogleGrpcView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleGrpc>> for GoogleGrpcView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleGrpc>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleGrpcView<'msg> {

  pub fn to_owned(&self) -> GoogleGrpc {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // target_uri: optional string
  pub fn target_uri(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // channel_credentials: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelCredentials
  pub fn has_channel_credentials(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn channel_credentials_opt(self) -> ::std::option::Option<super::super::grpc_service::google_grpc::ChannelCredentialsView<'msg>> {
    self.has_channel_credentials().then(|| self.channel_credentials())
  }
  pub fn channel_credentials(self) -> super::super::grpc_service::google_grpc::ChannelCredentialsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::grpc_service::google_grpc::ChannelCredentialsView::default())
  }

  // channel_credentials_plugin: repeated message google.protobuf.Any
  pub fn channel_credentials_plugin(self) -> ::protobuf::RepeatedView<'msg, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // call_credentials: repeated message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials
  pub fn call_credentials(self) -> ::protobuf::RepeatedView<'msg, super::super::grpc_service::google_grpc::CallCredentials> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::grpc_service::google_grpc::CallCredentials>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // call_credentials_plugin: repeated message google.protobuf.Any
  pub fn call_credentials_plugin(self) -> ::protobuf::RepeatedView<'msg, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // stat_prefix: optional string
  pub fn stat_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // credentials_factory_name: optional string
  pub fn credentials_factory_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // config: optional message google.protobuf.Struct
  pub fn has_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // per_stream_buffer_limit_bytes: optional message google.protobuf.UInt32Value
  pub fn has_per_stream_buffer_limit_bytes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn per_stream_buffer_limit_bytes_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_per_stream_buffer_limit_bytes().then(|| self.per_stream_buffer_limit_bytes())
  }
  pub fn per_stream_buffer_limit_bytes(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // channel_args: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs
  pub fn has_channel_args(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn channel_args_opt(self) -> ::std::option::Option<super::super::grpc_service::google_grpc::ChannelArgsView<'msg>> {
    self.has_channel_args().then(|| self.channel_args())
  }
  pub fn channel_args(self) -> super::super::grpc_service::google_grpc::ChannelArgsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::grpc_service::google_grpc::ChannelArgsView::default())
  }

}

// SAFETY:
// - `GoogleGrpcView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GoogleGrpcView<'_> {}

// SAFETY:
// - `GoogleGrpcView` is `Send` because while its alive a `GoogleGrpcMut` cannot.
// - `GoogleGrpcView` does not use thread-local data.
unsafe impl ::std::marker::Send for GoogleGrpcView<'_> {}

impl<'msg> ::protobuf::AsView for GoogleGrpcView<'msg> {
  type Proxied = GoogleGrpc;
  fn as_view(&self) -> ::protobuf::View<'msg, GoogleGrpc> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleGrpcView<'msg> {
  fn into_view<'shorter>(self) -> GoogleGrpcView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleGrpc> for GoogleGrpcView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleGrpc {
    let mut dst = GoogleGrpc::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleGrpc> for GoogleGrpcMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleGrpc {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GoogleGrpc {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GoogleGrpcView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GoogleGrpcMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GoogleGrpcMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleGrpc>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleGrpcMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GoogleGrpcMut<'msg> {
  type Message = GoogleGrpc;
}

impl ::std::fmt::Debug for GoogleGrpcMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleGrpc>> for GoogleGrpcMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleGrpc>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleGrpcMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleGrpc> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GoogleGrpc {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // target_uri: optional string
  pub fn target_uri(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_target_uri(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // channel_credentials: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelCredentials
  pub fn has_channel_credentials(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_channel_credentials(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn channel_credentials_opt(&self) -> ::std::option::Option<super::super::grpc_service::google_grpc::ChannelCredentialsView<'_>> {
    self.has_channel_credentials().then(|| self.channel_credentials())
  }
  pub fn channel_credentials(&self) -> super::super::grpc_service::google_grpc::ChannelCredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::grpc_service::google_grpc::ChannelCredentialsView::default())
  }
  pub fn channel_credentials_mut(&mut self) -> super::super::grpc_service::google_grpc::ChannelCredentialsMut<'_> {
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
  pub fn set_channel_credentials(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::grpc_service::google_grpc::ChannelCredentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // channel_credentials_plugin: repeated message google.protobuf.Any
  pub fn channel_credentials_plugin(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn channel_credentials_plugin_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
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
  pub fn set_channel_credentials_plugin(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // call_credentials: repeated message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials
  pub fn call_credentials(&self) -> ::protobuf::RepeatedView<'_, super::super::grpc_service::google_grpc::CallCredentials> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::grpc_service::google_grpc::CallCredentials>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn call_credentials_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::grpc_service::google_grpc::CallCredentials> {
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
  pub fn set_call_credentials(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::grpc_service::google_grpc::CallCredentials>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // call_credentials_plugin: repeated message google.protobuf.Any
  pub fn call_credentials_plugin(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn call_credentials_plugin_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        9,
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
  pub fn set_call_credentials_plugin(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // credentials_factory_name: optional string
  pub fn credentials_factory_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_credentials_factory_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // config: optional message google.protobuf.Struct
  pub fn has_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn config_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // per_stream_buffer_limit_bytes: optional message google.protobuf.UInt32Value
  pub fn has_per_stream_buffer_limit_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_per_stream_buffer_limit_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn per_stream_buffer_limit_bytes_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_per_stream_buffer_limit_bytes().then(|| self.per_stream_buffer_limit_bytes())
  }
  pub fn per_stream_buffer_limit_bytes(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn per_stream_buffer_limit_bytes_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_per_stream_buffer_limit_bytes(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // channel_args: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs
  pub fn has_channel_args(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_channel_args(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn channel_args_opt(&self) -> ::std::option::Option<super::super::grpc_service::google_grpc::ChannelArgsView<'_>> {
    self.has_channel_args().then(|| self.channel_args())
  }
  pub fn channel_args(&self) -> super::super::grpc_service::google_grpc::ChannelArgsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::grpc_service::google_grpc::ChannelArgsView::default())
  }
  pub fn channel_args_mut(&mut self) -> super::super::grpc_service::google_grpc::ChannelArgsMut<'_> {
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
  pub fn set_channel_args(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::grpc_service::google_grpc::ChannelArgs>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

}

// SAFETY:
// - `GoogleGrpcMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GoogleGrpcMut<'_> {}

// SAFETY:
// - `GoogleGrpcMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GoogleGrpcMut<'_> {}

impl<'msg> ::protobuf::AsView for GoogleGrpcMut<'msg> {
  type Proxied = GoogleGrpc;
  fn as_view(&self) -> ::protobuf::View<'_, GoogleGrpc> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleGrpcMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GoogleGrpc>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GoogleGrpcMut<'msg> {
  type MutProxied = GoogleGrpc;
  fn as_mut(&mut self) -> GoogleGrpcMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GoogleGrpcMut<'msg> {
  fn into_mut<'shorter>(self) -> GoogleGrpcMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GoogleGrpc {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GoogleGrpc> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GoogleGrpcView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GoogleGrpcMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // target_uri: optional string
  pub fn target_uri(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_target_uri(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // channel_credentials: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelCredentials
  pub fn has_channel_credentials(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_channel_credentials(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn channel_credentials_opt(&self) -> ::std::option::Option<super::super::grpc_service::google_grpc::ChannelCredentialsView<'_>> {
    self.has_channel_credentials().then(|| self.channel_credentials())
  }
  pub fn channel_credentials(&self) -> super::super::grpc_service::google_grpc::ChannelCredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::grpc_service::google_grpc::ChannelCredentialsView::default())
  }
  pub fn channel_credentials_mut(&mut self) -> super::super::grpc_service::google_grpc::ChannelCredentialsMut<'_> {
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
  pub fn set_channel_credentials(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::grpc_service::google_grpc::ChannelCredentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // channel_credentials_plugin: repeated message google.protobuf.Any
  pub fn channel_credentials_plugin(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn channel_credentials_plugin_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
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
  pub fn set_channel_credentials_plugin(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // call_credentials: repeated message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials
  pub fn call_credentials(&self) -> ::protobuf::RepeatedView<'_, super::super::grpc_service::google_grpc::CallCredentials> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::grpc_service::google_grpc::CallCredentials>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn call_credentials_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::grpc_service::google_grpc::CallCredentials> {
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
  pub fn set_call_credentials(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::grpc_service::google_grpc::CallCredentials>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // call_credentials_plugin: repeated message google.protobuf.Any
  pub fn call_credentials_plugin(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn call_credentials_plugin_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        9,
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
  pub fn set_call_credentials_plugin(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

  // stat_prefix: optional string
  pub fn stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // credentials_factory_name: optional string
  pub fn credentials_factory_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_credentials_factory_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // config: optional message google.protobuf.Struct
  pub fn has_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn config_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // per_stream_buffer_limit_bytes: optional message google.protobuf.UInt32Value
  pub fn has_per_stream_buffer_limit_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_per_stream_buffer_limit_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn per_stream_buffer_limit_bytes_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_per_stream_buffer_limit_bytes().then(|| self.per_stream_buffer_limit_bytes())
  }
  pub fn per_stream_buffer_limit_bytes(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn per_stream_buffer_limit_bytes_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_per_stream_buffer_limit_bytes(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // channel_args: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs
  pub fn has_channel_args(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_channel_args(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn channel_args_opt(&self) -> ::std::option::Option<super::super::grpc_service::google_grpc::ChannelArgsView<'_>> {
    self.has_channel_args().then(|| self.channel_args())
  }
  pub fn channel_args(&self) -> super::super::grpc_service::google_grpc::ChannelArgsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::grpc_service::google_grpc::ChannelArgsView::default())
  }
  pub fn channel_args_mut(&mut self) -> super::super::grpc_service::google_grpc::ChannelArgsMut<'_> {
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
  pub fn set_channel_args(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::grpc_service::google_grpc::ChannelArgs>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

}  // impl GoogleGrpc

impl ::std::ops::Drop for GoogleGrpc {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GoogleGrpc {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GoogleGrpc {
  type Proxied = Self;
  fn as_view(&self) -> GoogleGrpcView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GoogleGrpc {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GoogleGrpcMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GoogleGrpc {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::grpc_service::envoy__config__core__v3__GrpcService__GoogleGrpc_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3G1X1X333GG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::grpc_service::envoy__config__core__v3__GrpcService__GoogleGrpc_msg_init.0, &[<super::super::grpc_service::google_grpc::ChannelCredentials as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::grpc_service::google_grpc::CallCredentials as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::grpc_service::google_grpc::ChannelArgs as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::grpc_service::envoy__config__core__v3__GrpcService__GoogleGrpc_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleGrpc {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleGrpc {
  type Msg = GoogleGrpc;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleGrpc> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleGrpc {
  type Msg = GoogleGrpc;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleGrpc> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleGrpcMut<'_> {
  type Msg = GoogleGrpc;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleGrpc> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleGrpcMut<'_> {
  type Msg = GoogleGrpc;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleGrpc> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleGrpcView<'_> {
  type Msg = GoogleGrpc;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleGrpc> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleGrpcMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod google_grpc {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc__SslCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SslCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SslCredentials>
}

impl ::protobuf::Message for SslCredentials {
  type MessageView<'msg> = SslCredentialsView<'msg>;
  type MessageMut<'msg> = SslCredentialsMut<'msg>;
}

impl ::std::default::Default for SslCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SslCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SslCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `SslCredentialsMut`.
unsafe impl ::std::marker::Sync for SslCredentials {}

// SAFETY:
// - `SslCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SslCredentials {}

impl ::protobuf::Proxied for SslCredentials {
  type View<'msg> = SslCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SslCredentials {}

impl ::protobuf::MutProxied for SslCredentials {
  type Mut<'msg> = SslCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SslCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SslCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SslCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SslCredentialsView<'msg> {
  type Message = SslCredentials;
}

impl ::std::fmt::Debug for SslCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SslCredentialsView<'_> {
  fn default() -> SslCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SslCredentials>> for SslCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SslCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SslCredentialsView<'msg> {

  pub fn to_owned(&self) -> SslCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // root_certs: optional message envoy.config.core.v3.DataSource
  pub fn has_root_certs(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn root_certs_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_root_certs().then(|| self.root_certs())
  }
  pub fn root_certs(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // private_key: optional message envoy.config.core.v3.DataSource
  pub fn has_private_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn private_key_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_private_key().then(|| self.private_key())
  }
  pub fn private_key(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // cert_chain: optional message envoy.config.core.v3.DataSource
  pub fn has_cert_chain(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn cert_chain_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_cert_chain().then(|| self.cert_chain())
  }
  pub fn cert_chain(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

}

// SAFETY:
// - `SslCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SslCredentialsView<'_> {}

// SAFETY:
// - `SslCredentialsView` is `Send` because while its alive a `SslCredentialsMut` cannot.
// - `SslCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for SslCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for SslCredentialsView<'msg> {
  type Proxied = SslCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, SslCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SslCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> SslCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SslCredentials> for SslCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SslCredentials {
    let mut dst = SslCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SslCredentials> for SslCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SslCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SslCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SslCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SslCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SslCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SslCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SslCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SslCredentialsMut<'msg> {
  type Message = SslCredentials;
}

impl ::std::fmt::Debug for SslCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SslCredentials>> for SslCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SslCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SslCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SslCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SslCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // root_certs: optional message envoy.config.core.v3.DataSource
  pub fn has_root_certs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_root_certs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn root_certs_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_root_certs().then(|| self.root_certs())
  }
  pub fn root_certs(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn root_certs_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_root_certs(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // private_key: optional message envoy.config.core.v3.DataSource
  pub fn has_private_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_private_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn private_key_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_private_key().then(|| self.private_key())
  }
  pub fn private_key(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn private_key_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_private_key(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cert_chain: optional message envoy.config.core.v3.DataSource
  pub fn has_cert_chain(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cert_chain(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cert_chain_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_cert_chain().then(|| self.cert_chain())
  }
  pub fn cert_chain(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn cert_chain_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_cert_chain(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

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
// - `SslCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SslCredentialsMut<'_> {}

// SAFETY:
// - `SslCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SslCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for SslCredentialsMut<'msg> {
  type Proxied = SslCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, SslCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SslCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SslCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SslCredentialsMut<'msg> {
  type MutProxied = SslCredentials;
  fn as_mut(&mut self) -> SslCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SslCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> SslCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SslCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SslCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SslCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SslCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // root_certs: optional message envoy.config.core.v3.DataSource
  pub fn has_root_certs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_root_certs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn root_certs_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_root_certs().then(|| self.root_certs())
  }
  pub fn root_certs(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn root_certs_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_root_certs(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // private_key: optional message envoy.config.core.v3.DataSource
  pub fn has_private_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_private_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn private_key_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_private_key().then(|| self.private_key())
  }
  pub fn private_key(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn private_key_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_private_key(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cert_chain: optional message envoy.config.core.v3.DataSource
  pub fn has_cert_chain(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cert_chain(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cert_chain_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_cert_chain().then(|| self.cert_chain())
  }
  pub fn cert_chain(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn cert_chain_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_cert_chain(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl SslCredentials

impl ::std::ops::Drop for SslCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SslCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SslCredentials {
  type Proxied = Self;
  fn as_view(&self) -> SslCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SslCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SslCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SslCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__SslCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__SslCredentials_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__SslCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SslCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SslCredentials {
  type Msg = SslCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SslCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SslCredentials {
  type Msg = SslCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SslCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SslCredentialsMut<'_> {
  type Msg = SslCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SslCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SslCredentialsMut<'_> {
  type Msg = SslCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SslCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SslCredentialsView<'_> {
  type Msg = SslCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SslCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SslCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc__GoogleLocalCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GoogleLocalCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GoogleLocalCredentials>
}

impl ::protobuf::Message for GoogleLocalCredentials {
  type MessageView<'msg> = GoogleLocalCredentialsView<'msg>;
  type MessageMut<'msg> = GoogleLocalCredentialsMut<'msg>;
}

impl ::std::default::Default for GoogleLocalCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GoogleLocalCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GoogleLocalCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `GoogleLocalCredentialsMut`.
unsafe impl ::std::marker::Sync for GoogleLocalCredentials {}

// SAFETY:
// - `GoogleLocalCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GoogleLocalCredentials {}

impl ::protobuf::Proxied for GoogleLocalCredentials {
  type View<'msg> = GoogleLocalCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GoogleLocalCredentials {}

impl ::protobuf::MutProxied for GoogleLocalCredentials {
  type Mut<'msg> = GoogleLocalCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GoogleLocalCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleLocalCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleLocalCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GoogleLocalCredentialsView<'msg> {
  type Message = GoogleLocalCredentials;
}

impl ::std::fmt::Debug for GoogleLocalCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GoogleLocalCredentialsView<'_> {
  fn default() -> GoogleLocalCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleLocalCredentials>> for GoogleLocalCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleLocalCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleLocalCredentialsView<'msg> {

  pub fn to_owned(&self) -> GoogleLocalCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `GoogleLocalCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GoogleLocalCredentialsView<'_> {}

// SAFETY:
// - `GoogleLocalCredentialsView` is `Send` because while its alive a `GoogleLocalCredentialsMut` cannot.
// - `GoogleLocalCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for GoogleLocalCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for GoogleLocalCredentialsView<'msg> {
  type Proxied = GoogleLocalCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, GoogleLocalCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleLocalCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> GoogleLocalCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleLocalCredentials> for GoogleLocalCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleLocalCredentials {
    let mut dst = GoogleLocalCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleLocalCredentials> for GoogleLocalCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleLocalCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GoogleLocalCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GoogleLocalCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GoogleLocalCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GoogleLocalCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleLocalCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleLocalCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GoogleLocalCredentialsMut<'msg> {
  type Message = GoogleLocalCredentials;
}

impl ::std::fmt::Debug for GoogleLocalCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleLocalCredentials>> for GoogleLocalCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleLocalCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleLocalCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleLocalCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GoogleLocalCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `GoogleLocalCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GoogleLocalCredentialsMut<'_> {}

// SAFETY:
// - `GoogleLocalCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GoogleLocalCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for GoogleLocalCredentialsMut<'msg> {
  type Proxied = GoogleLocalCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, GoogleLocalCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleLocalCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GoogleLocalCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GoogleLocalCredentialsMut<'msg> {
  type MutProxied = GoogleLocalCredentials;
  fn as_mut(&mut self) -> GoogleLocalCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GoogleLocalCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> GoogleLocalCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GoogleLocalCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GoogleLocalCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GoogleLocalCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GoogleLocalCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl GoogleLocalCredentials

impl ::std::ops::Drop for GoogleLocalCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GoogleLocalCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GoogleLocalCredentials {
  type Proxied = Self;
  fn as_view(&self) -> GoogleLocalCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GoogleLocalCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GoogleLocalCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GoogleLocalCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__GoogleLocalCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__GoogleLocalCredentials_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__GoogleLocalCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleLocalCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleLocalCredentials {
  type Msg = GoogleLocalCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleLocalCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleLocalCredentials {
  type Msg = GoogleLocalCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleLocalCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleLocalCredentialsMut<'_> {
  type Msg = GoogleLocalCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleLocalCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleLocalCredentialsMut<'_> {
  type Msg = GoogleLocalCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleLocalCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleLocalCredentialsView<'_> {
  type Msg = GoogleLocalCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleLocalCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleLocalCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ChannelCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ChannelCredentials>
}

impl ::protobuf::Message for ChannelCredentials {
  type MessageView<'msg> = ChannelCredentialsView<'msg>;
  type MessageMut<'msg> = ChannelCredentialsMut<'msg>;
}

impl ::std::default::Default for ChannelCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ChannelCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ChannelCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `ChannelCredentialsMut`.
unsafe impl ::std::marker::Sync for ChannelCredentials {}

// SAFETY:
// - `ChannelCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ChannelCredentials {}

impl ::protobuf::Proxied for ChannelCredentials {
  type View<'msg> = ChannelCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ChannelCredentials {}

impl ::protobuf::MutProxied for ChannelCredentials {
  type Mut<'msg> = ChannelCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ChannelCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ChannelCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ChannelCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ChannelCredentialsView<'msg> {
  type Message = ChannelCredentials;
}

impl ::std::fmt::Debug for ChannelCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ChannelCredentialsView<'_> {
  fn default() -> ChannelCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ChannelCredentials>> for ChannelCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ChannelCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ChannelCredentialsView<'msg> {

  pub fn to_owned(&self) -> ChannelCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // ssl_credentials: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.SslCredentials
  pub fn has_ssl_credentials(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn ssl_credentials_opt(self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::SslCredentialsView<'msg>> {
    self.has_ssl_credentials().then(|| self.ssl_credentials())
  }
  pub fn ssl_credentials(self) -> super::super::super::grpc_service::google_grpc::SslCredentialsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::SslCredentialsView::default())
  }

  // google_default: optional message google.protobuf.Empty
  pub fn has_google_default(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn google_default_opt(self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'msg>> {
    self.has_google_default().then(|| self.google_default())
  }
  pub fn google_default(self) -> ::protobuf_well_known_types::EmptyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }

  // local_credentials: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.GoogleLocalCredentials
  pub fn has_local_credentials(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn local_credentials_opt(self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::GoogleLocalCredentialsView<'msg>> {
    self.has_local_credentials().then(|| self.local_credentials())
  }
  pub fn local_credentials(self) -> super::super::super::grpc_service::google_grpc::GoogleLocalCredentialsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::GoogleLocalCredentialsView::default())
  }

  pub fn credential_specifier(self) -> super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof<'msg> {
    match self.credential_specifier_case() {
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::SslCredentials =>
          super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::SslCredentials(self.ssl_credentials()),
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::GoogleDefault =>
          super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::GoogleDefault(self.google_default()),
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::LocalCredentials =>
          super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::LocalCredentials(self.local_credentials()),
      _ => super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn credential_specifier_case(self) -> super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ChannelCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ChannelCredentialsView<'_> {}

// SAFETY:
// - `ChannelCredentialsView` is `Send` because while its alive a `ChannelCredentialsMut` cannot.
// - `ChannelCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ChannelCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for ChannelCredentialsView<'msg> {
  type Proxied = ChannelCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, ChannelCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ChannelCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> ChannelCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ChannelCredentials> for ChannelCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ChannelCredentials {
    let mut dst = ChannelCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ChannelCredentials> for ChannelCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ChannelCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ChannelCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ChannelCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ChannelCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ChannelCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ChannelCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ChannelCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ChannelCredentialsMut<'msg> {
  type Message = ChannelCredentials;
}

impl ::std::fmt::Debug for ChannelCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ChannelCredentials>> for ChannelCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ChannelCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ChannelCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ChannelCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ChannelCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // ssl_credentials: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.SslCredentials
  pub fn has_ssl_credentials(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_ssl_credentials(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn ssl_credentials_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::SslCredentialsView<'_>> {
    self.has_ssl_credentials().then(|| self.ssl_credentials())
  }
  pub fn ssl_credentials(&self) -> super::super::super::grpc_service::google_grpc::SslCredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::SslCredentialsView::default())
  }
  pub fn ssl_credentials_mut(&mut self) -> super::super::super::grpc_service::google_grpc::SslCredentialsMut<'_> {
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
  pub fn set_ssl_credentials(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::SslCredentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // google_default: optional message google.protobuf.Empty
  pub fn has_google_default(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_google_default(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn google_default_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'_>> {
    self.has_google_default().then(|| self.google_default())
  }
  pub fn google_default(&self) -> ::protobuf_well_known_types::EmptyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }
  pub fn google_default_mut(&mut self) -> ::protobuf_well_known_types::EmptyMut<'_> {
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
  pub fn set_google_default(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Empty>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // local_credentials: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.GoogleLocalCredentials
  pub fn has_local_credentials(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_local_credentials(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn local_credentials_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::GoogleLocalCredentialsView<'_>> {
    self.has_local_credentials().then(|| self.local_credentials())
  }
  pub fn local_credentials(&self) -> super::super::super::grpc_service::google_grpc::GoogleLocalCredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::GoogleLocalCredentialsView::default())
  }
  pub fn local_credentials_mut(&mut self) -> super::super::super::grpc_service::google_grpc::GoogleLocalCredentialsMut<'_> {
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
  pub fn set_local_credentials(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::GoogleLocalCredentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn credential_specifier(&self) -> super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof<'_> {
    match &self.credential_specifier_case() {
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::SslCredentials =>
          super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::SslCredentials(self.ssl_credentials()),
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::GoogleDefault =>
          super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::GoogleDefault(self.google_default()),
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::LocalCredentials =>
          super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::LocalCredentials(self.local_credentials()),
      _ => super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn credential_specifier_case(&self) -> super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ChannelCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ChannelCredentialsMut<'_> {}

// SAFETY:
// - `ChannelCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ChannelCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for ChannelCredentialsMut<'msg> {
  type Proxied = ChannelCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, ChannelCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ChannelCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ChannelCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ChannelCredentialsMut<'msg> {
  type MutProxied = ChannelCredentials;
  fn as_mut(&mut self) -> ChannelCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ChannelCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> ChannelCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ChannelCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ChannelCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ChannelCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ChannelCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // ssl_credentials: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.SslCredentials
  pub fn has_ssl_credentials(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_ssl_credentials(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn ssl_credentials_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::SslCredentialsView<'_>> {
    self.has_ssl_credentials().then(|| self.ssl_credentials())
  }
  pub fn ssl_credentials(&self) -> super::super::super::grpc_service::google_grpc::SslCredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::SslCredentialsView::default())
  }
  pub fn ssl_credentials_mut(&mut self) -> super::super::super::grpc_service::google_grpc::SslCredentialsMut<'_> {
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
  pub fn set_ssl_credentials(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::SslCredentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // google_default: optional message google.protobuf.Empty
  pub fn has_google_default(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_google_default(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn google_default_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'_>> {
    self.has_google_default().then(|| self.google_default())
  }
  pub fn google_default(&self) -> ::protobuf_well_known_types::EmptyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }
  pub fn google_default_mut(&mut self) -> ::protobuf_well_known_types::EmptyMut<'_> {
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
  pub fn set_google_default(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Empty>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // local_credentials: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.GoogleLocalCredentials
  pub fn has_local_credentials(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_local_credentials(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn local_credentials_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::GoogleLocalCredentialsView<'_>> {
    self.has_local_credentials().then(|| self.local_credentials())
  }
  pub fn local_credentials(&self) -> super::super::super::grpc_service::google_grpc::GoogleLocalCredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::GoogleLocalCredentialsView::default())
  }
  pub fn local_credentials_mut(&mut self) -> super::super::super::grpc_service::google_grpc::GoogleLocalCredentialsMut<'_> {
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
  pub fn set_local_credentials(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::GoogleLocalCredentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn credential_specifier(&self) -> super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof<'_> {
    match &self.credential_specifier_case() {
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::SslCredentials =>
          super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::SslCredentials(self.ssl_credentials()),
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::GoogleDefault =>
          super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::GoogleDefault(self.google_default()),
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::LocalCredentials =>
          super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::LocalCredentials(self.local_credentials()),
      _ => super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn credential_specifier_case(&self) -> super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::grpc_service::google_grpc::channel_credentials::CredentialSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ChannelCredentials

impl ::std::ops::Drop for ChannelCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ChannelCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ChannelCredentials {
  type Proxied = Self;
  fn as_view(&self) -> ChannelCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ChannelCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ChannelCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ChannelCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333^!|#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelCredentials_msg_init.0, &[<super::super::super::grpc_service::google_grpc::SslCredentials as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Empty as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::super::grpc_service::google_grpc::GoogleLocalCredentials as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ChannelCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ChannelCredentials {
  type Msg = ChannelCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChannelCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ChannelCredentials {
  type Msg = ChannelCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChannelCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ChannelCredentialsMut<'_> {
  type Msg = ChannelCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChannelCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ChannelCredentialsMut<'_> {
  type Msg = ChannelCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChannelCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ChannelCredentialsView<'_> {
  type Msg = ChannelCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChannelCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ChannelCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod channel_credentials {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum CredentialSpecifierOneof<'msg> {
  SslCredentials(::protobuf::View<'msg, super::super::super::super::grpc_service::google_grpc::SslCredentials>) = 1,
  GoogleDefault(::protobuf::View<'msg, ::protobuf_well_known_types::Empty>) = 2,
  LocalCredentials(::protobuf::View<'msg, super::super::super::super::grpc_service::google_grpc::GoogleLocalCredentials>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum CredentialSpecifierCase {
  SslCredentials = 1,
  GoogleDefault = 2,
  LocalCredentials = 3,

  not_set = 0
}

impl CredentialSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<CredentialSpecifierCase> {
    match v {
      0 => Some(CredentialSpecifierCase::not_set),
      1 => Some(CredentialSpecifierCase::SslCredentials),
      2 => Some(CredentialSpecifierCase::GoogleDefault),
      3 => Some(CredentialSpecifierCase::LocalCredentials),
      _ => None
    }
  }
}
}  // pub mod channel_credentials

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CallCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CallCredentials>
}

impl ::protobuf::Message for CallCredentials {
  type MessageView<'msg> = CallCredentialsView<'msg>;
  type MessageMut<'msg> = CallCredentialsMut<'msg>;
}

impl ::std::default::Default for CallCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CallCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CallCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `CallCredentialsMut`.
unsafe impl ::std::marker::Sync for CallCredentials {}

// SAFETY:
// - `CallCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CallCredentials {}

impl ::protobuf::Proxied for CallCredentials {
  type View<'msg> = CallCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CallCredentials {}

impl ::protobuf::MutProxied for CallCredentials {
  type Mut<'msg> = CallCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CallCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CallCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CallCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CallCredentialsView<'msg> {
  type Message = CallCredentials;
}

impl ::std::fmt::Debug for CallCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CallCredentialsView<'_> {
  fn default() -> CallCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CallCredentials>> for CallCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CallCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CallCredentialsView<'msg> {

  pub fn to_owned(&self) -> CallCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // access_token: optional string
  pub fn has_access_token(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn access_token_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_access_token().then(|| self.access_token())
  }
  pub fn access_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // google_compute_engine: optional message google.protobuf.Empty
  pub fn has_google_compute_engine(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn google_compute_engine_opt(self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'msg>> {
    self.has_google_compute_engine().then(|| self.google_compute_engine())
  }
  pub fn google_compute_engine(self) -> ::protobuf_well_known_types::EmptyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }

  // google_refresh_token: optional string
  pub fn has_google_refresh_token(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn google_refresh_token_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_google_refresh_token().then(|| self.google_refresh_token())
  }
  pub fn google_refresh_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // service_account_jwt_access: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.ServiceAccountJWTAccessCredentials
  pub fn has_service_account_jwt_access(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn service_account_jwt_access_opt(self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentialsView<'msg>> {
    self.has_service_account_jwt_access().then(|| self.service_account_jwt_access())
  }
  pub fn service_account_jwt_access(self) -> super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentialsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentialsView::default())
  }

  // google_iam: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.GoogleIAMCredentials
  pub fn has_google_iam(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn google_iam_opt(self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentialsView<'msg>> {
    self.has_google_iam().then(|| self.google_iam())
  }
  pub fn google_iam(self) -> super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentialsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentialsView::default())
  }

  // from_plugin: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.MetadataCredentialsFromPlugin
  pub fn has_from_plugin(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn from_plugin_opt(self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPluginView<'msg>> {
    self.has_from_plugin().then(|| self.from_plugin())
  }
  pub fn from_plugin(self) -> super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPluginView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPluginView::default())
  }

  // sts_service: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.StsService
  pub fn has_sts_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn sts_service_opt(self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::StsServiceView<'msg>> {
    self.has_sts_service().then(|| self.sts_service())
  }
  pub fn sts_service(self) -> super::super::super::grpc_service::google_grpc::call_credentials::StsServiceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::StsServiceView::default())
  }

  pub fn credential_specifier(self) -> super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof<'msg> {
    match self.credential_specifier_case() {
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::AccessToken =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::AccessToken(self.access_token()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::GoogleComputeEngine =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::GoogleComputeEngine(self.google_compute_engine()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::GoogleRefreshToken =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::GoogleRefreshToken(self.google_refresh_token()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::ServiceAccountJwtAccess =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::ServiceAccountJwtAccess(self.service_account_jwt_access()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::GoogleIam =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::GoogleIam(self.google_iam()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::FromPlugin =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::FromPlugin(self.from_plugin()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::StsService =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::StsService(self.sts_service()),
      _ => super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn credential_specifier_case(self) -> super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CallCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CallCredentialsView<'_> {}

// SAFETY:
// - `CallCredentialsView` is `Send` because while its alive a `CallCredentialsMut` cannot.
// - `CallCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for CallCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for CallCredentialsView<'msg> {
  type Proxied = CallCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, CallCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CallCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> CallCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CallCredentials> for CallCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CallCredentials {
    let mut dst = CallCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CallCredentials> for CallCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CallCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CallCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CallCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CallCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CallCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CallCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CallCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CallCredentialsMut<'msg> {
  type Message = CallCredentials;
}

impl ::std::fmt::Debug for CallCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CallCredentials>> for CallCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CallCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CallCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CallCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CallCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // access_token: optional string
  pub fn has_access_token(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_access_token(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn access_token_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_access_token().then(|| self.access_token())
  }
  pub fn access_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_access_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // google_compute_engine: optional message google.protobuf.Empty
  pub fn has_google_compute_engine(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_google_compute_engine(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn google_compute_engine_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'_>> {
    self.has_google_compute_engine().then(|| self.google_compute_engine())
  }
  pub fn google_compute_engine(&self) -> ::protobuf_well_known_types::EmptyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }
  pub fn google_compute_engine_mut(&mut self) -> ::protobuf_well_known_types::EmptyMut<'_> {
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
  pub fn set_google_compute_engine(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Empty>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // google_refresh_token: optional string
  pub fn has_google_refresh_token(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_google_refresh_token(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn google_refresh_token_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_google_refresh_token().then(|| self.google_refresh_token())
  }
  pub fn google_refresh_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_google_refresh_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // service_account_jwt_access: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.ServiceAccountJWTAccessCredentials
  pub fn has_service_account_jwt_access(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_service_account_jwt_access(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn service_account_jwt_access_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentialsView<'_>> {
    self.has_service_account_jwt_access().then(|| self.service_account_jwt_access())
  }
  pub fn service_account_jwt_access(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentialsView::default())
  }
  pub fn service_account_jwt_access_mut(&mut self) -> super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentialsMut<'_> {
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
  pub fn set_service_account_jwt_access(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // google_iam: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.GoogleIAMCredentials
  pub fn has_google_iam(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_google_iam(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn google_iam_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentialsView<'_>> {
    self.has_google_iam().then(|| self.google_iam())
  }
  pub fn google_iam(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentialsView::default())
  }
  pub fn google_iam_mut(&mut self) -> super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentialsMut<'_> {
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
  pub fn set_google_iam(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // from_plugin: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.MetadataCredentialsFromPlugin
  pub fn has_from_plugin(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_from_plugin(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn from_plugin_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPluginView<'_>> {
    self.has_from_plugin().then(|| self.from_plugin())
  }
  pub fn from_plugin(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPluginView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPluginView::default())
  }
  pub fn from_plugin_mut(&mut self) -> super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPluginMut<'_> {
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
  pub fn set_from_plugin(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPlugin>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // sts_service: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.StsService
  pub fn has_sts_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_sts_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn sts_service_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::StsServiceView<'_>> {
    self.has_sts_service().then(|| self.sts_service())
  }
  pub fn sts_service(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::StsServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::StsServiceView::default())
  }
  pub fn sts_service_mut(&mut self) -> super::super::super::grpc_service::google_grpc::call_credentials::StsServiceMut<'_> {
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
  pub fn set_sts_service(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::call_credentials::StsService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn credential_specifier(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof<'_> {
    match &self.credential_specifier_case() {
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::AccessToken =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::AccessToken(self.access_token()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::GoogleComputeEngine =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::GoogleComputeEngine(self.google_compute_engine()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::GoogleRefreshToken =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::GoogleRefreshToken(self.google_refresh_token()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::ServiceAccountJwtAccess =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::ServiceAccountJwtAccess(self.service_account_jwt_access()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::GoogleIam =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::GoogleIam(self.google_iam()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::FromPlugin =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::FromPlugin(self.from_plugin()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::StsService =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::StsService(self.sts_service()),
      _ => super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn credential_specifier_case(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CallCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CallCredentialsMut<'_> {}

// SAFETY:
// - `CallCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CallCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for CallCredentialsMut<'msg> {
  type Proxied = CallCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, CallCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CallCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CallCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CallCredentialsMut<'msg> {
  type MutProxied = CallCredentials;
  fn as_mut(&mut self) -> CallCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CallCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> CallCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CallCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CallCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CallCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CallCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // access_token: optional string
  pub fn has_access_token(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_access_token(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn access_token_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_access_token().then(|| self.access_token())
  }
  pub fn access_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_access_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // google_compute_engine: optional message google.protobuf.Empty
  pub fn has_google_compute_engine(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_google_compute_engine(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn google_compute_engine_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::EmptyView<'_>> {
    self.has_google_compute_engine().then(|| self.google_compute_engine())
  }
  pub fn google_compute_engine(&self) -> ::protobuf_well_known_types::EmptyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::EmptyView::default())
  }
  pub fn google_compute_engine_mut(&mut self) -> ::protobuf_well_known_types::EmptyMut<'_> {
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
  pub fn set_google_compute_engine(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Empty>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // google_refresh_token: optional string
  pub fn has_google_refresh_token(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_google_refresh_token(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn google_refresh_token_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_google_refresh_token().then(|| self.google_refresh_token())
  }
  pub fn google_refresh_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_google_refresh_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // service_account_jwt_access: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.ServiceAccountJWTAccessCredentials
  pub fn has_service_account_jwt_access(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_service_account_jwt_access(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn service_account_jwt_access_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentialsView<'_>> {
    self.has_service_account_jwt_access().then(|| self.service_account_jwt_access())
  }
  pub fn service_account_jwt_access(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentialsView::default())
  }
  pub fn service_account_jwt_access_mut(&mut self) -> super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentialsMut<'_> {
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
  pub fn set_service_account_jwt_access(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // google_iam: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.GoogleIAMCredentials
  pub fn has_google_iam(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_google_iam(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn google_iam_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentialsView<'_>> {
    self.has_google_iam().then(|| self.google_iam())
  }
  pub fn google_iam(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentialsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentialsView::default())
  }
  pub fn google_iam_mut(&mut self) -> super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentialsMut<'_> {
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
  pub fn set_google_iam(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentials>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // from_plugin: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.MetadataCredentialsFromPlugin
  pub fn has_from_plugin(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_from_plugin(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn from_plugin_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPluginView<'_>> {
    self.has_from_plugin().then(|| self.from_plugin())
  }
  pub fn from_plugin(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPluginView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPluginView::default())
  }
  pub fn from_plugin_mut(&mut self) -> super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPluginMut<'_> {
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
  pub fn set_from_plugin(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPlugin>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // sts_service: optional message envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.StsService
  pub fn has_sts_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_sts_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn sts_service_opt(&self) -> ::std::option::Option<super::super::super::grpc_service::google_grpc::call_credentials::StsServiceView<'_>> {
    self.has_sts_service().then(|| self.sts_service())
  }
  pub fn sts_service(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::StsServiceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::grpc_service::google_grpc::call_credentials::StsServiceView::default())
  }
  pub fn sts_service_mut(&mut self) -> super::super::super::grpc_service::google_grpc::call_credentials::StsServiceMut<'_> {
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
  pub fn set_sts_service(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::grpc_service::google_grpc::call_credentials::StsService>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn credential_specifier(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof<'_> {
    match &self.credential_specifier_case() {
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::AccessToken =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::AccessToken(self.access_token()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::GoogleComputeEngine =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::GoogleComputeEngine(self.google_compute_engine()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::GoogleRefreshToken =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::GoogleRefreshToken(self.google_refresh_token()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::ServiceAccountJwtAccess =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::ServiceAccountJwtAccess(self.service_account_jwt_access()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::GoogleIam =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::GoogleIam(self.google_iam()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::FromPlugin =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::FromPlugin(self.from_plugin()),
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::StsService =>
          super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::StsService(self.sts_service()),
      _ => super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn credential_specifier_case(&self) -> super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::grpc_service::google_grpc::call_credentials::CredentialSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl CallCredentials

impl ::std::ops::Drop for CallCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CallCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CallCredentials {
  type Proxied = Self;
  fn as_view(&self) -> CallCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CallCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CallCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CallCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T31T3333^!|#|$|%|&|(|)");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials_msg_init.0, &[<::protobuf_well_known_types::Empty as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentials as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentials as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPlugin as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::super::grpc_service::google_grpc::call_credentials::StsService as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CallCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CallCredentials {
  type Msg = CallCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CallCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CallCredentials {
  type Msg = CallCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CallCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CallCredentialsMut<'_> {
  type Msg = CallCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CallCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CallCredentialsMut<'_> {
  type Msg = CallCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CallCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CallCredentialsView<'_> {
  type Msg = CallCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CallCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CallCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod call_credentials {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__ServiceAccountJWTAccessCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ServiceAccountJWTAccessCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ServiceAccountJWTAccessCredentials>
}

impl ::protobuf::Message for ServiceAccountJWTAccessCredentials {
  type MessageView<'msg> = ServiceAccountJWTAccessCredentialsView<'msg>;
  type MessageMut<'msg> = ServiceAccountJWTAccessCredentialsMut<'msg>;
}

impl ::std::default::Default for ServiceAccountJWTAccessCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ServiceAccountJWTAccessCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ServiceAccountJWTAccessCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `ServiceAccountJWTAccessCredentialsMut`.
unsafe impl ::std::marker::Sync for ServiceAccountJWTAccessCredentials {}

// SAFETY:
// - `ServiceAccountJWTAccessCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ServiceAccountJWTAccessCredentials {}

impl ::protobuf::Proxied for ServiceAccountJWTAccessCredentials {
  type View<'msg> = ServiceAccountJWTAccessCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ServiceAccountJWTAccessCredentials {}

impl ::protobuf::MutProxied for ServiceAccountJWTAccessCredentials {
  type Mut<'msg> = ServiceAccountJWTAccessCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ServiceAccountJWTAccessCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ServiceAccountJWTAccessCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ServiceAccountJWTAccessCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ServiceAccountJWTAccessCredentialsView<'msg> {
  type Message = ServiceAccountJWTAccessCredentials;
}

impl ::std::fmt::Debug for ServiceAccountJWTAccessCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ServiceAccountJWTAccessCredentialsView<'_> {
  fn default() -> ServiceAccountJWTAccessCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ServiceAccountJWTAccessCredentials>> for ServiceAccountJWTAccessCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ServiceAccountJWTAccessCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ServiceAccountJWTAccessCredentialsView<'msg> {

  pub fn to_owned(&self) -> ServiceAccountJWTAccessCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // json_key: optional string
  pub fn json_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // token_lifetime_seconds: optional uint64
  pub fn token_lifetime_seconds(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ServiceAccountJWTAccessCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ServiceAccountJWTAccessCredentialsView<'_> {}

// SAFETY:
// - `ServiceAccountJWTAccessCredentialsView` is `Send` because while its alive a `ServiceAccountJWTAccessCredentialsMut` cannot.
// - `ServiceAccountJWTAccessCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ServiceAccountJWTAccessCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for ServiceAccountJWTAccessCredentialsView<'msg> {
  type Proxied = ServiceAccountJWTAccessCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, ServiceAccountJWTAccessCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ServiceAccountJWTAccessCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> ServiceAccountJWTAccessCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ServiceAccountJWTAccessCredentials> for ServiceAccountJWTAccessCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ServiceAccountJWTAccessCredentials {
    let mut dst = ServiceAccountJWTAccessCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ServiceAccountJWTAccessCredentials> for ServiceAccountJWTAccessCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ServiceAccountJWTAccessCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ServiceAccountJWTAccessCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ServiceAccountJWTAccessCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ServiceAccountJWTAccessCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ServiceAccountJWTAccessCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ServiceAccountJWTAccessCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ServiceAccountJWTAccessCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ServiceAccountJWTAccessCredentialsMut<'msg> {
  type Message = ServiceAccountJWTAccessCredentials;
}

impl ::std::fmt::Debug for ServiceAccountJWTAccessCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ServiceAccountJWTAccessCredentials>> for ServiceAccountJWTAccessCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ServiceAccountJWTAccessCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ServiceAccountJWTAccessCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ServiceAccountJWTAccessCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ServiceAccountJWTAccessCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // json_key: optional string
  pub fn json_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_json_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // token_lifetime_seconds: optional uint64
  pub fn token_lifetime_seconds(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_token_lifetime_seconds(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `ServiceAccountJWTAccessCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ServiceAccountJWTAccessCredentialsMut<'_> {}

// SAFETY:
// - `ServiceAccountJWTAccessCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ServiceAccountJWTAccessCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for ServiceAccountJWTAccessCredentialsMut<'msg> {
  type Proxied = ServiceAccountJWTAccessCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, ServiceAccountJWTAccessCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ServiceAccountJWTAccessCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ServiceAccountJWTAccessCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ServiceAccountJWTAccessCredentialsMut<'msg> {
  type MutProxied = ServiceAccountJWTAccessCredentials;
  fn as_mut(&mut self) -> ServiceAccountJWTAccessCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ServiceAccountJWTAccessCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> ServiceAccountJWTAccessCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ServiceAccountJWTAccessCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ServiceAccountJWTAccessCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ServiceAccountJWTAccessCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ServiceAccountJWTAccessCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // json_key: optional string
  pub fn json_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_json_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // token_lifetime_seconds: optional uint64
  pub fn token_lifetime_seconds(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_token_lifetime_seconds(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}  // impl ServiceAccountJWTAccessCredentials

impl ::std::ops::Drop for ServiceAccountJWTAccessCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ServiceAccountJWTAccessCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ServiceAccountJWTAccessCredentials {
  type Proxied = Self;
  fn as_view(&self) -> ServiceAccountJWTAccessCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ServiceAccountJWTAccessCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ServiceAccountJWTAccessCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ServiceAccountJWTAccessCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__ServiceAccountJWTAccessCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X,P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__ServiceAccountJWTAccessCredentials_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__ServiceAccountJWTAccessCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ServiceAccountJWTAccessCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ServiceAccountJWTAccessCredentials {
  type Msg = ServiceAccountJWTAccessCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ServiceAccountJWTAccessCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ServiceAccountJWTAccessCredentials {
  type Msg = ServiceAccountJWTAccessCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ServiceAccountJWTAccessCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ServiceAccountJWTAccessCredentialsMut<'_> {
  type Msg = ServiceAccountJWTAccessCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ServiceAccountJWTAccessCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ServiceAccountJWTAccessCredentialsMut<'_> {
  type Msg = ServiceAccountJWTAccessCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ServiceAccountJWTAccessCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ServiceAccountJWTAccessCredentialsView<'_> {
  type Msg = ServiceAccountJWTAccessCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ServiceAccountJWTAccessCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ServiceAccountJWTAccessCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__GoogleIAMCredentials_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GoogleIAMCredentials {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GoogleIAMCredentials>
}

impl ::protobuf::Message for GoogleIAMCredentials {
  type MessageView<'msg> = GoogleIAMCredentialsView<'msg>;
  type MessageMut<'msg> = GoogleIAMCredentialsMut<'msg>;
}

impl ::std::default::Default for GoogleIAMCredentials {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GoogleIAMCredentials {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GoogleIAMCredentials` is `Sync` because it does not implement interior mutability.
//    Neither does `GoogleIAMCredentialsMut`.
unsafe impl ::std::marker::Sync for GoogleIAMCredentials {}

// SAFETY:
// - `GoogleIAMCredentials` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GoogleIAMCredentials {}

impl ::protobuf::Proxied for GoogleIAMCredentials {
  type View<'msg> = GoogleIAMCredentialsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GoogleIAMCredentials {}

impl ::protobuf::MutProxied for GoogleIAMCredentials {
  type Mut<'msg> = GoogleIAMCredentialsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GoogleIAMCredentialsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleIAMCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleIAMCredentialsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GoogleIAMCredentialsView<'msg> {
  type Message = GoogleIAMCredentials;
}

impl ::std::fmt::Debug for GoogleIAMCredentialsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GoogleIAMCredentialsView<'_> {
  fn default() -> GoogleIAMCredentialsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleIAMCredentials>> for GoogleIAMCredentialsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleIAMCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleIAMCredentialsView<'msg> {

  pub fn to_owned(&self) -> GoogleIAMCredentials {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // authorization_token: optional string
  pub fn authorization_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // authority_selector: optional string
  pub fn authority_selector(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `GoogleIAMCredentialsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GoogleIAMCredentialsView<'_> {}

// SAFETY:
// - `GoogleIAMCredentialsView` is `Send` because while its alive a `GoogleIAMCredentialsMut` cannot.
// - `GoogleIAMCredentialsView` does not use thread-local data.
unsafe impl ::std::marker::Send for GoogleIAMCredentialsView<'_> {}

impl<'msg> ::protobuf::AsView for GoogleIAMCredentialsView<'msg> {
  type Proxied = GoogleIAMCredentials;
  fn as_view(&self) -> ::protobuf::View<'msg, GoogleIAMCredentials> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleIAMCredentialsView<'msg> {
  fn into_view<'shorter>(self) -> GoogleIAMCredentialsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleIAMCredentials> for GoogleIAMCredentialsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleIAMCredentials {
    let mut dst = GoogleIAMCredentials::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleIAMCredentials> for GoogleIAMCredentialsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleIAMCredentials {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GoogleIAMCredentials {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GoogleIAMCredentialsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GoogleIAMCredentialsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GoogleIAMCredentialsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleIAMCredentials>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleIAMCredentialsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GoogleIAMCredentialsMut<'msg> {
  type Message = GoogleIAMCredentials;
}

impl ::std::fmt::Debug for GoogleIAMCredentialsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleIAMCredentials>> for GoogleIAMCredentialsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleIAMCredentials>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleIAMCredentialsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleIAMCredentials> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GoogleIAMCredentials {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // authorization_token: optional string
  pub fn authorization_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authorization_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // authority_selector: optional string
  pub fn authority_selector(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authority_selector(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `GoogleIAMCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GoogleIAMCredentialsMut<'_> {}

// SAFETY:
// - `GoogleIAMCredentialsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GoogleIAMCredentialsMut<'_> {}

impl<'msg> ::protobuf::AsView for GoogleIAMCredentialsMut<'msg> {
  type Proxied = GoogleIAMCredentials;
  fn as_view(&self) -> ::protobuf::View<'_, GoogleIAMCredentials> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleIAMCredentialsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GoogleIAMCredentials>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GoogleIAMCredentialsMut<'msg> {
  type MutProxied = GoogleIAMCredentials;
  fn as_mut(&mut self) -> GoogleIAMCredentialsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GoogleIAMCredentialsMut<'msg> {
  fn into_mut<'shorter>(self) -> GoogleIAMCredentialsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GoogleIAMCredentials {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GoogleIAMCredentials> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GoogleIAMCredentialsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GoogleIAMCredentialsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // authorization_token: optional string
  pub fn authorization_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authorization_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // authority_selector: optional string
  pub fn authority_selector(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authority_selector(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl GoogleIAMCredentials

impl ::std::ops::Drop for GoogleIAMCredentials {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GoogleIAMCredentials {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GoogleIAMCredentials {
  type Proxied = Self;
  fn as_view(&self) -> GoogleIAMCredentialsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GoogleIAMCredentials {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GoogleIAMCredentialsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GoogleIAMCredentials {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__GoogleIAMCredentials_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__GoogleIAMCredentials_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__GoogleIAMCredentials_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleIAMCredentials {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleIAMCredentials {
  type Msg = GoogleIAMCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleIAMCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleIAMCredentials {
  type Msg = GoogleIAMCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleIAMCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleIAMCredentialsMut<'_> {
  type Msg = GoogleIAMCredentials;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleIAMCredentials> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleIAMCredentialsMut<'_> {
  type Msg = GoogleIAMCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleIAMCredentials> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleIAMCredentialsView<'_> {
  type Msg = GoogleIAMCredentials;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleIAMCredentials> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleIAMCredentialsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__MetadataCredentialsFromPlugin_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MetadataCredentialsFromPlugin {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MetadataCredentialsFromPlugin>
}

impl ::protobuf::Message for MetadataCredentialsFromPlugin {
  type MessageView<'msg> = MetadataCredentialsFromPluginView<'msg>;
  type MessageMut<'msg> = MetadataCredentialsFromPluginMut<'msg>;
}

impl ::std::default::Default for MetadataCredentialsFromPlugin {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MetadataCredentialsFromPlugin {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MetadataCredentialsFromPlugin` is `Sync` because it does not implement interior mutability.
//    Neither does `MetadataCredentialsFromPluginMut`.
unsafe impl ::std::marker::Sync for MetadataCredentialsFromPlugin {}

// SAFETY:
// - `MetadataCredentialsFromPlugin` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MetadataCredentialsFromPlugin {}

impl ::protobuf::Proxied for MetadataCredentialsFromPlugin {
  type View<'msg> = MetadataCredentialsFromPluginView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MetadataCredentialsFromPlugin {}

impl ::protobuf::MutProxied for MetadataCredentialsFromPlugin {
  type Mut<'msg> = MetadataCredentialsFromPluginMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MetadataCredentialsFromPluginView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataCredentialsFromPlugin>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataCredentialsFromPluginView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MetadataCredentialsFromPluginView<'msg> {
  type Message = MetadataCredentialsFromPlugin;
}

impl ::std::fmt::Debug for MetadataCredentialsFromPluginView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MetadataCredentialsFromPluginView<'_> {
  fn default() -> MetadataCredentialsFromPluginView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataCredentialsFromPlugin>> for MetadataCredentialsFromPluginView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataCredentialsFromPlugin>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataCredentialsFromPluginView<'msg> {

  pub fn to_owned(&self) -> MetadataCredentialsFromPlugin {
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

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn typed_config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  pub fn config_type(self) -> super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeCase::TypedConfig =>
          super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `MetadataCredentialsFromPluginView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MetadataCredentialsFromPluginView<'_> {}

// SAFETY:
// - `MetadataCredentialsFromPluginView` is `Send` because while its alive a `MetadataCredentialsFromPluginMut` cannot.
// - `MetadataCredentialsFromPluginView` does not use thread-local data.
unsafe impl ::std::marker::Send for MetadataCredentialsFromPluginView<'_> {}

impl<'msg> ::protobuf::AsView for MetadataCredentialsFromPluginView<'msg> {
  type Proxied = MetadataCredentialsFromPlugin;
  fn as_view(&self) -> ::protobuf::View<'msg, MetadataCredentialsFromPlugin> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataCredentialsFromPluginView<'msg> {
  fn into_view<'shorter>(self) -> MetadataCredentialsFromPluginView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataCredentialsFromPlugin> for MetadataCredentialsFromPluginView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataCredentialsFromPlugin {
    let mut dst = MetadataCredentialsFromPlugin::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataCredentialsFromPlugin> for MetadataCredentialsFromPluginMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataCredentialsFromPlugin {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MetadataCredentialsFromPlugin {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataCredentialsFromPluginView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataCredentialsFromPluginMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MetadataCredentialsFromPluginMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataCredentialsFromPlugin>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataCredentialsFromPluginMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MetadataCredentialsFromPluginMut<'msg> {
  type Message = MetadataCredentialsFromPlugin;
}

impl ::std::fmt::Debug for MetadataCredentialsFromPluginMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataCredentialsFromPlugin>> for MetadataCredentialsFromPluginMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataCredentialsFromPlugin>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataCredentialsFromPluginMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataCredentialsFromPlugin> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MetadataCredentialsFromPlugin {
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

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_typed_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn config_type(&self) -> super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeCase::TypedConfig =>
          super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `MetadataCredentialsFromPluginMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MetadataCredentialsFromPluginMut<'_> {}

// SAFETY:
// - `MetadataCredentialsFromPluginMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MetadataCredentialsFromPluginMut<'_> {}

impl<'msg> ::protobuf::AsView for MetadataCredentialsFromPluginMut<'msg> {
  type Proxied = MetadataCredentialsFromPlugin;
  fn as_view(&self) -> ::protobuf::View<'_, MetadataCredentialsFromPlugin> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataCredentialsFromPluginMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MetadataCredentialsFromPlugin>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MetadataCredentialsFromPluginMut<'msg> {
  type MutProxied = MetadataCredentialsFromPlugin;
  fn as_mut(&mut self) -> MetadataCredentialsFromPluginMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MetadataCredentialsFromPluginMut<'msg> {
  fn into_mut<'shorter>(self) -> MetadataCredentialsFromPluginMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MetadataCredentialsFromPlugin {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MetadataCredentialsFromPlugin> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MetadataCredentialsFromPluginView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MetadataCredentialsFromPluginMut<'_> {
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

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_typed_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn config_type(&self) -> super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeCase::TypedConfig =>
          super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::super::super::grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl MetadataCredentialsFromPlugin

impl ::std::ops::Drop for MetadataCredentialsFromPlugin {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MetadataCredentialsFromPlugin {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MetadataCredentialsFromPlugin {
  type Proxied = Self;
  fn as_view(&self) -> MetadataCredentialsFromPluginView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MetadataCredentialsFromPlugin {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MetadataCredentialsFromPluginMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MetadataCredentialsFromPlugin {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__MetadataCredentialsFromPlugin_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xa3^$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__MetadataCredentialsFromPlugin_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__MetadataCredentialsFromPlugin_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataCredentialsFromPlugin {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataCredentialsFromPlugin {
  type Msg = MetadataCredentialsFromPlugin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataCredentialsFromPlugin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataCredentialsFromPlugin {
  type Msg = MetadataCredentialsFromPlugin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataCredentialsFromPlugin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataCredentialsFromPluginMut<'_> {
  type Msg = MetadataCredentialsFromPlugin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataCredentialsFromPlugin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataCredentialsFromPluginMut<'_> {
  type Msg = MetadataCredentialsFromPlugin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataCredentialsFromPlugin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataCredentialsFromPluginView<'_> {
  type Msg = MetadataCredentialsFromPlugin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataCredentialsFromPlugin> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataCredentialsFromPluginMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod metadata_credentials_from_plugin {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigTypeOneof<'msg> {
  TypedConfig(::protobuf::View<'msg, ::protobuf_well_known_types::Any>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigTypeCase {
  TypedConfig = 3,

  not_set = 0
}

impl ConfigTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigTypeCase> {
    match v {
      0 => Some(ConfigTypeCase::not_set),
      3 => Some(ConfigTypeCase::TypedConfig),
      _ => None
    }
  }
}
}  // pub mod metadata_credentials_from_plugin

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__StsService_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StsService {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StsService>
}

impl ::protobuf::Message for StsService {
  type MessageView<'msg> = StsServiceView<'msg>;
  type MessageMut<'msg> = StsServiceMut<'msg>;
}

impl ::std::default::Default for StsService {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StsService {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StsService` is `Sync` because it does not implement interior mutability.
//    Neither does `StsServiceMut`.
unsafe impl ::std::marker::Sync for StsService {}

// SAFETY:
// - `StsService` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StsService {}

impl ::protobuf::Proxied for StsService {
  type View<'msg> = StsServiceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StsService {}

impl ::protobuf::MutProxied for StsService {
  type Mut<'msg> = StsServiceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StsServiceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StsService>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StsServiceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StsServiceView<'msg> {
  type Message = StsService;
}

impl ::std::fmt::Debug for StsServiceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StsServiceView<'_> {
  fn default() -> StsServiceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StsService>> for StsServiceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StsService>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StsServiceView<'msg> {

  pub fn to_owned(&self) -> StsService {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // token_exchange_service_uri: optional string
  pub fn token_exchange_service_uri(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // resource: optional string
  pub fn resource(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // audience: optional string
  pub fn audience(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // scope: optional string
  pub fn scope(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // requested_token_type: optional string
  pub fn requested_token_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // subject_token_path: optional string
  pub fn subject_token_path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // subject_token_type: optional string
  pub fn subject_token_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // actor_token_path: optional string
  pub fn actor_token_path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // actor_token_type: optional string
  pub fn actor_token_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `StsServiceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StsServiceView<'_> {}

// SAFETY:
// - `StsServiceView` is `Send` because while its alive a `StsServiceMut` cannot.
// - `StsServiceView` does not use thread-local data.
unsafe impl ::std::marker::Send for StsServiceView<'_> {}

impl<'msg> ::protobuf::AsView for StsServiceView<'msg> {
  type Proxied = StsService;
  fn as_view(&self) -> ::protobuf::View<'msg, StsService> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StsServiceView<'msg> {
  fn into_view<'shorter>(self) -> StsServiceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StsService> for StsServiceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StsService {
    let mut dst = StsService::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StsService> for StsServiceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StsService {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StsService {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StsServiceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StsServiceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StsServiceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StsService>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StsServiceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StsServiceMut<'msg> {
  type Message = StsService;
}

impl ::std::fmt::Debug for StsServiceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StsService>> for StsServiceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StsService>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StsServiceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StsService> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StsService {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // token_exchange_service_uri: optional string
  pub fn token_exchange_service_uri(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_token_exchange_service_uri(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // resource: optional string
  pub fn resource(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_resource(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // audience: optional string
  pub fn audience(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_audience(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // scope: optional string
  pub fn scope(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_scope(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // requested_token_type: optional string
  pub fn requested_token_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_requested_token_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // subject_token_path: optional string
  pub fn subject_token_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_subject_token_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // subject_token_type: optional string
  pub fn subject_token_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_subject_token_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // actor_token_path: optional string
  pub fn actor_token_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_actor_token_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // actor_token_type: optional string
  pub fn actor_token_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_actor_token_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

}

// SAFETY:
// - `StsServiceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StsServiceMut<'_> {}

// SAFETY:
// - `StsServiceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StsServiceMut<'_> {}

impl<'msg> ::protobuf::AsView for StsServiceMut<'msg> {
  type Proxied = StsService;
  fn as_view(&self) -> ::protobuf::View<'_, StsService> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StsServiceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StsService>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StsServiceMut<'msg> {
  type MutProxied = StsService;
  fn as_mut(&mut self) -> StsServiceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StsServiceMut<'msg> {
  fn into_mut<'shorter>(self) -> StsServiceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StsService {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StsService> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StsServiceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StsServiceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // token_exchange_service_uri: optional string
  pub fn token_exchange_service_uri(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_token_exchange_service_uri(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // resource: optional string
  pub fn resource(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_resource(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // audience: optional string
  pub fn audience(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_audience(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // scope: optional string
  pub fn scope(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_scope(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // requested_token_type: optional string
  pub fn requested_token_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_requested_token_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // subject_token_path: optional string
  pub fn subject_token_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_subject_token_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // subject_token_type: optional string
  pub fn subject_token_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_subject_token_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // actor_token_path: optional string
  pub fn actor_token_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_actor_token_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // actor_token_type: optional string
  pub fn actor_token_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_actor_token_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

}  // impl StsService

impl ::std::ops::Drop for StsService {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StsService {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StsService {
  type Proxied = Self;
  fn as_view(&self) -> StsServiceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StsService {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StsServiceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StsService {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__StsService_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P1P1P1P1P1P1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__StsService_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::grpc_service::google_grpc::call_credentials::envoy__config__core__v3__GrpcService__GoogleGrpc__CallCredentials__StsService_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StsService {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StsService {
  type Msg = StsService;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StsService> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StsService {
  type Msg = StsService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StsService> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StsServiceMut<'_> {
  type Msg = StsService;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StsService> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StsServiceMut<'_> {
  type Msg = StsService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StsService> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StsServiceView<'_> {
  type Msg = StsService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StsService> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StsServiceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum CredentialSpecifierOneof<'msg> {
  AccessToken(&'msg ::protobuf::ProtoStr) = 1,
  GoogleComputeEngine(::protobuf::View<'msg, ::protobuf_well_known_types::Empty>) = 2,
  GoogleRefreshToken(&'msg ::protobuf::ProtoStr) = 3,
  ServiceAccountJwtAccess(::protobuf::View<'msg, super::super::super::super::grpc_service::google_grpc::call_credentials::ServiceAccountJWTAccessCredentials>) = 4,
  GoogleIam(::protobuf::View<'msg, super::super::super::super::grpc_service::google_grpc::call_credentials::GoogleIAMCredentials>) = 5,
  FromPlugin(::protobuf::View<'msg, super::super::super::super::grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPlugin>) = 6,
  StsService(::protobuf::View<'msg, super::super::super::super::grpc_service::google_grpc::call_credentials::StsService>) = 7,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum CredentialSpecifierCase {
  AccessToken = 1,
  GoogleComputeEngine = 2,
  GoogleRefreshToken = 3,
  ServiceAccountJwtAccess = 4,
  GoogleIam = 5,
  FromPlugin = 6,
  StsService = 7,

  not_set = 0
}

impl CredentialSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<CredentialSpecifierCase> {
    match v {
      0 => Some(CredentialSpecifierCase::not_set),
      1 => Some(CredentialSpecifierCase::AccessToken),
      2 => Some(CredentialSpecifierCase::GoogleComputeEngine),
      3 => Some(CredentialSpecifierCase::GoogleRefreshToken),
      4 => Some(CredentialSpecifierCase::ServiceAccountJwtAccess),
      5 => Some(CredentialSpecifierCase::GoogleIam),
      6 => Some(CredentialSpecifierCase::FromPlugin),
      7 => Some(CredentialSpecifierCase::StsService),
      _ => None
    }
  }
}
}  // pub mod call_credentials

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ChannelArgs {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ChannelArgs>
}

impl ::protobuf::Message for ChannelArgs {
  type MessageView<'msg> = ChannelArgsView<'msg>;
  type MessageMut<'msg> = ChannelArgsMut<'msg>;
}

impl ::std::default::Default for ChannelArgs {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ChannelArgs {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ChannelArgs` is `Sync` because it does not implement interior mutability.
//    Neither does `ChannelArgsMut`.
unsafe impl ::std::marker::Sync for ChannelArgs {}

// SAFETY:
// - `ChannelArgs` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ChannelArgs {}

impl ::protobuf::Proxied for ChannelArgs {
  type View<'msg> = ChannelArgsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ChannelArgs {}

impl ::protobuf::MutProxied for ChannelArgs {
  type Mut<'msg> = ChannelArgsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ChannelArgsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ChannelArgs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ChannelArgsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ChannelArgsView<'msg> {
  type Message = ChannelArgs;
}

impl ::std::fmt::Debug for ChannelArgsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ChannelArgsView<'_> {
  fn default() -> ChannelArgsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ChannelArgs>> for ChannelArgsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ChannelArgs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ChannelArgsView<'msg> {

  pub fn to_owned(&self) -> ChannelArgs {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // args: repeated message envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs.ArgsEntry
  pub fn args(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, super::super::super::grpc_service::google_grpc::channel_args::Value> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::super::super::grpc_service::google_grpc::channel_args::Value>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `ChannelArgsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ChannelArgsView<'_> {}

// SAFETY:
// - `ChannelArgsView` is `Send` because while its alive a `ChannelArgsMut` cannot.
// - `ChannelArgsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ChannelArgsView<'_> {}

impl<'msg> ::protobuf::AsView for ChannelArgsView<'msg> {
  type Proxied = ChannelArgs;
  fn as_view(&self) -> ::protobuf::View<'msg, ChannelArgs> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ChannelArgsView<'msg> {
  fn into_view<'shorter>(self) -> ChannelArgsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ChannelArgs> for ChannelArgsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ChannelArgs {
    let mut dst = ChannelArgs::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ChannelArgs> for ChannelArgsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ChannelArgs {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ChannelArgs {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ChannelArgsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ChannelArgsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ChannelArgsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ChannelArgs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ChannelArgsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ChannelArgsMut<'msg> {
  type Message = ChannelArgs;
}

impl ::std::fmt::Debug for ChannelArgsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ChannelArgs>> for ChannelArgsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ChannelArgs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ChannelArgsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ChannelArgs> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ChannelArgs {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // args: repeated message envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs.ArgsEntry
  pub fn args(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::super::super::grpc_service::google_grpc::channel_args::Value> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::super::super::grpc_service::google_grpc::channel_args::Value>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn args_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::super::super::grpc_service::google_grpc::channel_args::Value> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_args(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::super::super::grpc_service::google_grpc::channel_args::Value>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ChannelArgsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ChannelArgsMut<'_> {}

// SAFETY:
// - `ChannelArgsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ChannelArgsMut<'_> {}

impl<'msg> ::protobuf::AsView for ChannelArgsMut<'msg> {
  type Proxied = ChannelArgs;
  fn as_view(&self) -> ::protobuf::View<'_, ChannelArgs> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ChannelArgsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ChannelArgs>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ChannelArgsMut<'msg> {
  type MutProxied = ChannelArgs;
  fn as_mut(&mut self) -> ChannelArgsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ChannelArgsMut<'msg> {
  fn into_mut<'shorter>(self) -> ChannelArgsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ChannelArgs {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ChannelArgs> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ChannelArgsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ChannelArgsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // args: repeated message envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs.ArgsEntry
  pub fn args(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::super::super::grpc_service::google_grpc::channel_args::Value> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::super::super::grpc_service::google_grpc::channel_args::Value>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn args_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::super::super::grpc_service::google_grpc::channel_args::Value> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_args(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::super::super::grpc_service::google_grpc::channel_args::Value>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ChannelArgs

impl ::std::ops::Drop for ChannelArgs {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ChannelArgs {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ChannelArgs {
  type Proxied = Self;
  fn as_view(&self) -> ChannelArgsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ChannelArgs {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ChannelArgsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ChannelArgs {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs_msg_init.0, &[<super::super::super::grpc_service::google_grpc::channel_args::ArgsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::grpc_service::google_grpc::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ChannelArgs {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ChannelArgs {
  type Msg = ChannelArgs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChannelArgs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ChannelArgs {
  type Msg = ChannelArgs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChannelArgs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ChannelArgsMut<'_> {
  type Msg = ChannelArgs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChannelArgs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ChannelArgsMut<'_> {
  type Msg = ChannelArgs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChannelArgs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ChannelArgsView<'_> {
  type Msg = ChannelArgs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChannelArgs> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ChannelArgsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod channel_args {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs__Value_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Value {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Value>
}

impl ::protobuf::Message for Value {
  type MessageView<'msg> = ValueView<'msg>;
  type MessageMut<'msg> = ValueMut<'msg>;
}

impl ::std::default::Default for Value {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Value {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Value` is `Sync` because it does not implement interior mutability.
//    Neither does `ValueMut`.
unsafe impl ::std::marker::Sync for Value {}

// SAFETY:
// - `Value` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Value {}

impl ::protobuf::Proxied for Value {
  type View<'msg> = ValueView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Value {}

impl ::protobuf::MutProxied for Value {
  type Mut<'msg> = ValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ValueView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Value>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValueView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ValueView<'msg> {
  type Message = Value;
}

impl ::std::fmt::Debug for ValueView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ValueView<'_> {
  fn default() -> ValueView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Value>> for ValueView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Value>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValueView<'msg> {

  pub fn to_owned(&self) -> Value {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // string_value: optional string
  pub fn has_string_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn string_value_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_string_value().then(|| self.string_value())
  }
  pub fn string_value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // int_value: optional int64
  pub fn has_int_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn int_value_opt(self) -> ::std::option::Option<i64> {
    self.has_int_value().then(|| self.int_value())
  }
  pub fn int_value(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }

  pub fn value_specifier(self) -> super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof<'msg> {
    match self.value_specifier_case() {
      super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase::StringValue =>
          super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof::StringValue(self.string_value()),
      super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase::IntValue =>
          super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof::IntValue(self.int_value()),
      _ => super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_specifier_case(self) -> super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ValueView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ValueView<'_> {}

// SAFETY:
// - `ValueView` is `Send` because while its alive a `ValueMut` cannot.
// - `ValueView` does not use thread-local data.
unsafe impl ::std::marker::Send for ValueView<'_> {}

impl<'msg> ::protobuf::AsView for ValueView<'msg> {
  type Proxied = Value;
  fn as_view(&self) -> ::protobuf::View<'msg, Value> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValueView<'msg> {
  fn into_view<'shorter>(self) -> ValueView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Value> for ValueView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Value {
    let mut dst = Value::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Value> for ValueMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Value {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Value {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ValueView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ValueMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ValueMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Value>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValueMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ValueMut<'msg> {
  type Message = Value;
}

impl ::std::fmt::Debug for ValueMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Value>> for ValueMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Value>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValueMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Value> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Value {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // string_value: optional string
  pub fn has_string_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_string_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn string_value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_string_value().then(|| self.string_value())
  }
  pub fn string_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_string_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // int_value: optional int64
  pub fn has_int_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_int_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn int_value_opt(&self) -> ::std::option::Option<i64> {
    self.has_int_value().then(|| self.int_value())
  }
  pub fn int_value(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_int_value(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  pub fn value_specifier(&self) -> super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof<'_> {
    match &self.value_specifier_case() {
      super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase::StringValue =>
          super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof::StringValue(self.string_value()),
      super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase::IntValue =>
          super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof::IntValue(self.int_value()),
      _ => super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_specifier_case(&self) -> super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ValueMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ValueMut<'_> {}

// SAFETY:
// - `ValueMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ValueMut<'_> {}

impl<'msg> ::protobuf::AsView for ValueMut<'msg> {
  type Proxied = Value;
  fn as_view(&self) -> ::protobuf::View<'_, Value> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValueMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Value>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ValueMut<'msg> {
  type MutProxied = Value;
  fn as_mut(&mut self) -> ValueMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ValueMut<'msg> {
  fn into_mut<'shorter>(self) -> ValueMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Value {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Value> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ValueView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ValueMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // string_value: optional string
  pub fn has_string_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_string_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn string_value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_string_value().then(|| self.string_value())
  }
  pub fn string_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_string_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // int_value: optional int64
  pub fn has_int_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_int_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn int_value_opt(&self) -> ::std::option::Option<i64> {
    self.has_int_value().then(|| self.int_value())
  }
  pub fn int_value(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_int_value(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  pub fn value_specifier(&self) -> super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof<'_> {
    match &self.value_specifier_case() {
      super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase::StringValue =>
          super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof::StringValue(self.string_value()),
      super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase::IntValue =>
          super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof::IntValue(self.int_value()),
      _ => super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn value_specifier_case(&self) -> super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::super::grpc_service::google_grpc::channel_args::value::ValueSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Value

impl ::std::ops::Drop for Value {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Value {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Value {
  type Proxied = Self;
  fn as_view(&self) -> ValueView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Value {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ValueMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Value {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::super::grpc_service::google_grpc::channel_args::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs__Value_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T+^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::super::grpc_service::google_grpc::channel_args::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs__Value_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::grpc_service::google_grpc::channel_args::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs__Value_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Value {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Value {
  type Msg = Value;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Value> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Value {
  type Msg = Value;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Value> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValueMut<'_> {
  type Msg = Value;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Value> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValueMut<'_> {
  type Msg = Value;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Value> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValueView<'_> {
  type Msg = Value;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Value> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValueMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod value {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ValueSpecifierOneof<'msg> {
  StringValue(&'msg ::protobuf::ProtoStr) = 1,
  IntValue(i64) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ValueSpecifierCase {
  StringValue = 1,
  IntValue = 2,

  not_set = 0
}

impl ValueSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ValueSpecifierCase> {
    match v {
      0 => Some(ValueSpecifierCase::not_set),
      1 => Some(ValueSpecifierCase::StringValue),
      2 => Some(ValueSpecifierCase::IntValue),
      _ => None
    }
  }
}
}  // pub mod value

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs__ArgsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct ArgsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ArgsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::super::grpc_service::google_grpc::channel_args::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs__ArgsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::super::grpc_service::google_grpc::channel_args::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs__ArgsEntry_msg_init.0, &[<super::super::super::super::grpc_service::google_grpc::channel_args::Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::grpc_service::google_grpc::channel_args::envoy__config__core__v3__GrpcService__GoogleGrpc__ChannelArgs__ArgsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod channel_args


}  // pub mod google_grpc


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TargetSpecifierOneof<'msg> {
  EnvoyGrpc(::protobuf::View<'msg, super::super::grpc_service::EnvoyGrpc>) = 1,
  GoogleGrpc(::protobuf::View<'msg, super::super::grpc_service::GoogleGrpc>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TargetSpecifierCase {
  EnvoyGrpc = 1,
  GoogleGrpc = 2,

  not_set = 0
}

impl TargetSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TargetSpecifierCase> {
    match v {
      0 => Some(TargetSpecifierCase::not_set),
      1 => Some(TargetSpecifierCase::EnvoyGrpc),
      2 => Some(TargetSpecifierCase::GoogleGrpc),
      _ => None
    }
  }
}
}  // pub mod grpc_service


