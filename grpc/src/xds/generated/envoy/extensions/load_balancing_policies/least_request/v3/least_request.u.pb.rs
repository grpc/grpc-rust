const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__least_0request__v3__LeastRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LeastRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LeastRequest>
}

impl ::protobuf::Message for LeastRequest {
  type MessageView<'msg> = LeastRequestView<'msg>;
  type MessageMut<'msg> = LeastRequestMut<'msg>;
}

impl ::std::default::Default for LeastRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LeastRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LeastRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `LeastRequestMut`.
unsafe impl ::std::marker::Sync for LeastRequest {}

// SAFETY:
// - `LeastRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LeastRequest {}

impl ::protobuf::Proxied for LeastRequest {
  type View<'msg> = LeastRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LeastRequest {}

impl ::protobuf::MutProxied for LeastRequest {
  type Mut<'msg> = LeastRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LeastRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LeastRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LeastRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LeastRequestView<'msg> {
  type Message = LeastRequest;
}

impl ::std::fmt::Debug for LeastRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LeastRequestView<'_> {
  fn default() -> LeastRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LeastRequest>> for LeastRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LeastRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LeastRequestView<'msg> {

  pub fn to_owned(&self) -> LeastRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // choice_count: optional message google.protobuf.UInt32Value
  pub fn has_choice_count(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn choice_count_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_choice_count().then(|| self.choice_count())
  }
  pub fn choice_count(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // active_request_bias: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_active_request_bias(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn active_request_bias_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'msg>> {
    self.has_active_request_bias().then(|| self.active_request_bias())
  }
  pub fn active_request_bias(self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }

  // slow_start_config: optional message envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig
  pub fn has_slow_start_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn slow_start_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'msg>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView::default())
  }

  // locality_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig
  pub fn has_locality_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn locality_lb_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'msg>> {
    self.has_locality_lb_config().then(|| self.locality_lb_config())
  }
  pub fn locality_lb_config(self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView::default())
  }

  // enable_full_scan: optional message google.protobuf.BoolValue
  pub fn has_enable_full_scan(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn enable_full_scan_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_enable_full_scan().then(|| self.enable_full_scan())
  }
  pub fn enable_full_scan(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // selection_method: optional enum envoy.extensions.load_balancing_policies.least_request.v3.LeastRequest.SelectionMethod
  pub fn selection_method(self) -> super::least_request::SelectionMethod {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::least_request::SelectionMethod::NChoices).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `LeastRequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LeastRequestView<'_> {}

// SAFETY:
// - `LeastRequestView` is `Send` because while its alive a `LeastRequestMut` cannot.
// - `LeastRequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for LeastRequestView<'_> {}

impl<'msg> ::protobuf::AsView for LeastRequestView<'msg> {
  type Proxied = LeastRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, LeastRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LeastRequestView<'msg> {
  fn into_view<'shorter>(self) -> LeastRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LeastRequest> for LeastRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LeastRequest {
    let mut dst = LeastRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LeastRequest> for LeastRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LeastRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LeastRequest {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LeastRequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LeastRequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LeastRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LeastRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LeastRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LeastRequestMut<'msg> {
  type Message = LeastRequest;
}

impl ::std::fmt::Debug for LeastRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LeastRequest>> for LeastRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LeastRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LeastRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LeastRequest> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LeastRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // choice_count: optional message google.protobuf.UInt32Value
  pub fn has_choice_count(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_choice_count(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn choice_count_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_choice_count().then(|| self.choice_count())
  }
  pub fn choice_count(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn choice_count_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_choice_count(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // active_request_bias: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_active_request_bias(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_active_request_bias(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn active_request_bias_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_>> {
    self.has_active_request_bias().then(|| self.active_request_bias())
  }
  pub fn active_request_bias(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }
  pub fn active_request_bias_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleMut<'_> {
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
  pub fn set_active_request_bias(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // slow_start_config: optional message envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig
  pub fn has_slow_start_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_slow_start_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn slow_start_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView::default())
  }
  pub fn slow_start_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigMut<'_> {
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
  pub fn set_slow_start_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // locality_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig
  pub fn has_locality_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_locality_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn locality_lb_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'_>> {
    self.has_locality_lb_config().then(|| self.locality_lb_config())
  }
  pub fn locality_lb_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView::default())
  }
  pub fn locality_lb_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigMut<'_> {
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
  pub fn set_locality_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // enable_full_scan: optional message google.protobuf.BoolValue
  pub fn has_enable_full_scan(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_enable_full_scan(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn enable_full_scan_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enable_full_scan().then(|| self.enable_full_scan())
  }
  pub fn enable_full_scan(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enable_full_scan_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_enable_full_scan(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // selection_method: optional enum envoy.extensions.load_balancing_policies.least_request.v3.LeastRequest.SelectionMethod
  pub fn selection_method(&self) -> super::least_request::SelectionMethod {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::least_request::SelectionMethod::NChoices).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_selection_method(&mut self, val: super::least_request::SelectionMethod) {
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

}

// SAFETY:
// - `LeastRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LeastRequestMut<'_> {}

// SAFETY:
// - `LeastRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LeastRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for LeastRequestMut<'msg> {
  type Proxied = LeastRequest;
  fn as_view(&self) -> ::protobuf::View<'_, LeastRequest> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LeastRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LeastRequest>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LeastRequestMut<'msg> {
  type MutProxied = LeastRequest;
  fn as_mut(&mut self) -> LeastRequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LeastRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> LeastRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LeastRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LeastRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LeastRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LeastRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // choice_count: optional message google.protobuf.UInt32Value
  pub fn has_choice_count(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_choice_count(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn choice_count_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_choice_count().then(|| self.choice_count())
  }
  pub fn choice_count(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn choice_count_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_choice_count(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // active_request_bias: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_active_request_bias(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_active_request_bias(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn active_request_bias_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_>> {
    self.has_active_request_bias().then(|| self.active_request_bias())
  }
  pub fn active_request_bias(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }
  pub fn active_request_bias_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleMut<'_> {
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
  pub fn set_active_request_bias(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // slow_start_config: optional message envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig
  pub fn has_slow_start_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_slow_start_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn slow_start_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView::default())
  }
  pub fn slow_start_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigMut<'_> {
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
  pub fn set_slow_start_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // locality_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig
  pub fn has_locality_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_locality_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn locality_lb_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'_>> {
    self.has_locality_lb_config().then(|| self.locality_lb_config())
  }
  pub fn locality_lb_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView::default())
  }
  pub fn locality_lb_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigMut<'_> {
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
  pub fn set_locality_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // enable_full_scan: optional message google.protobuf.BoolValue
  pub fn has_enable_full_scan(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_enable_full_scan(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn enable_full_scan_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enable_full_scan().then(|| self.enable_full_scan())
  }
  pub fn enable_full_scan(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enable_full_scan_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_enable_full_scan(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // selection_method: optional enum envoy.extensions.load_balancing_policies.least_request.v3.LeastRequest.SelectionMethod
  pub fn selection_method(&self) -> super::least_request::SelectionMethod {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::least_request::SelectionMethod::NChoices).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_selection_method(&mut self, val: super::least_request::SelectionMethod) {
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

}  // impl LeastRequest

impl ::std::ops::Drop for LeastRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LeastRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LeastRequest {
  type Proxied = Self;
  fn as_view(&self) -> LeastRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LeastRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LeastRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LeastRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__load_0balancing_0policies__least_0request__v3__LeastRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33333.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__load_0balancing_0policies__least_0request__v3__LeastRequest_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__load_0balancing_0policies__least_0request__v3__LeastRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LeastRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LeastRequest {
  type Msg = LeastRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LeastRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LeastRequest {
  type Msg = LeastRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LeastRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LeastRequestMut<'_> {
  type Msg = LeastRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LeastRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LeastRequestMut<'_> {
  type Msg = LeastRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LeastRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LeastRequestView<'_> {
  type Msg = LeastRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LeastRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LeastRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod least_request {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SelectionMethod(i32);

#[allow(non_upper_case_globals)]
impl SelectionMethod {
  pub const NChoices: SelectionMethod = SelectionMethod(0);
  pub const FullScan: SelectionMethod = SelectionMethod(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "NChoices",
      1 => "FullScan",
      _ => return None
    })
  }
}

impl ::std::convert::From<SelectionMethod> for i32 {
  fn from(val: SelectionMethod) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for SelectionMethod {
  fn from(val: i32) -> SelectionMethod {
    Self(val)
  }
}

impl ::std::default::Default for SelectionMethod {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for SelectionMethod {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "SelectionMethod::{}", constant_name)
    } else {
      write!(f, "SelectionMethod::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for SelectionMethod {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for SelectionMethod {}

impl ::protobuf::Proxied for SelectionMethod {
  type View<'a> = SelectionMethod;
}

impl ::protobuf::AsView for SelectionMethod {
  type Proxied = SelectionMethod;

  fn as_view(&self) -> SelectionMethod {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SelectionMethod {
  fn into_view<'shorter>(self) -> SelectionMethod where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for SelectionMethod {
  const NAME: &'static str = "SelectionMethod";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for SelectionMethod {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod least_request


