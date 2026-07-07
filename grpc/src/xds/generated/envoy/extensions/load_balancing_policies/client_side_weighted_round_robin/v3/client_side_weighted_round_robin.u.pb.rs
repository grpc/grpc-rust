const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__client_0side_0weighted_0round_0robin__v3__ClientSideWeightedRoundRobin_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClientSideWeightedRoundRobin {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClientSideWeightedRoundRobin>
}

impl ::protobuf::Message for ClientSideWeightedRoundRobin {
  type MessageView<'msg> = ClientSideWeightedRoundRobinView<'msg>;
  type MessageMut<'msg> = ClientSideWeightedRoundRobinMut<'msg>;
}

impl ::std::default::Default for ClientSideWeightedRoundRobin {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClientSideWeightedRoundRobin {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClientSideWeightedRoundRobin` is `Sync` because it does not implement interior mutability.
//    Neither does `ClientSideWeightedRoundRobinMut`.
unsafe impl ::std::marker::Sync for ClientSideWeightedRoundRobin {}

// SAFETY:
// - `ClientSideWeightedRoundRobin` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ClientSideWeightedRoundRobin {}

impl ::protobuf::Proxied for ClientSideWeightedRoundRobin {
  type View<'msg> = ClientSideWeightedRoundRobinView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClientSideWeightedRoundRobin {}

impl ::protobuf::MutProxied for ClientSideWeightedRoundRobin {
  type Mut<'msg> = ClientSideWeightedRoundRobinMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClientSideWeightedRoundRobinView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClientSideWeightedRoundRobin>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientSideWeightedRoundRobinView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClientSideWeightedRoundRobinView<'msg> {
  type Message = ClientSideWeightedRoundRobin;
}

impl ::std::fmt::Debug for ClientSideWeightedRoundRobinView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClientSideWeightedRoundRobinView<'_> {
  fn default() -> ClientSideWeightedRoundRobinView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClientSideWeightedRoundRobin>> for ClientSideWeightedRoundRobinView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClientSideWeightedRoundRobin>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientSideWeightedRoundRobinView<'msg> {

  pub fn to_owned(&self) -> ClientSideWeightedRoundRobin {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // enable_oob_load_report: optional message google.protobuf.BoolValue
  pub fn has_enable_oob_load_report(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn enable_oob_load_report_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_enable_oob_load_report().then(|| self.enable_oob_load_report())
  }
  pub fn enable_oob_load_report(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // oob_reporting_period: optional message google.protobuf.Duration
  pub fn has_oob_reporting_period(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn oob_reporting_period_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_oob_reporting_period().then(|| self.oob_reporting_period())
  }
  pub fn oob_reporting_period(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // blackout_period: optional message google.protobuf.Duration
  pub fn has_blackout_period(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn blackout_period_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_blackout_period().then(|| self.blackout_period())
  }
  pub fn blackout_period(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // weight_expiration_period: optional message google.protobuf.Duration
  pub fn has_weight_expiration_period(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn weight_expiration_period_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_weight_expiration_period().then(|| self.weight_expiration_period())
  }
  pub fn weight_expiration_period(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // weight_update_period: optional message google.protobuf.Duration
  pub fn has_weight_update_period(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn weight_update_period_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_weight_update_period().then(|| self.weight_update_period())
  }
  pub fn weight_update_period(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // error_utilization_penalty: optional message google.protobuf.FloatValue
  pub fn has_error_utilization_penalty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn error_utilization_penalty_opt(self) -> ::std::option::Option<::protobuf_well_known_types::FloatValueView<'msg>> {
    self.has_error_utilization_penalty().then(|| self.error_utilization_penalty())
  }
  pub fn error_utilization_penalty(self) -> ::protobuf_well_known_types::FloatValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FloatValueView::default())
  }

  // metric_names_for_computing_utilization: repeated string
  pub fn metric_names_for_computing_utilization(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // slow_start_config: optional message envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig
  pub fn has_slow_start_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn slow_start_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'msg>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView::default())
  }

}

// SAFETY:
// - `ClientSideWeightedRoundRobinView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClientSideWeightedRoundRobinView<'_> {}

// SAFETY:
// - `ClientSideWeightedRoundRobinView` is `Send` because while its alive a `ClientSideWeightedRoundRobinMut` cannot.
// - `ClientSideWeightedRoundRobinView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClientSideWeightedRoundRobinView<'_> {}

impl<'msg> ::protobuf::AsView for ClientSideWeightedRoundRobinView<'msg> {
  type Proxied = ClientSideWeightedRoundRobin;
  fn as_view(&self) -> ::protobuf::View<'msg, ClientSideWeightedRoundRobin> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientSideWeightedRoundRobinView<'msg> {
  fn into_view<'shorter>(self) -> ClientSideWeightedRoundRobinView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClientSideWeightedRoundRobin> for ClientSideWeightedRoundRobinView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClientSideWeightedRoundRobin {
    let mut dst = ClientSideWeightedRoundRobin::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClientSideWeightedRoundRobin> for ClientSideWeightedRoundRobinMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClientSideWeightedRoundRobin {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ClientSideWeightedRoundRobin {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClientSideWeightedRoundRobinView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClientSideWeightedRoundRobinMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClientSideWeightedRoundRobinMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientSideWeightedRoundRobin>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientSideWeightedRoundRobinMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClientSideWeightedRoundRobinMut<'msg> {
  type Message = ClientSideWeightedRoundRobin;
}

impl ::std::fmt::Debug for ClientSideWeightedRoundRobinMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClientSideWeightedRoundRobin>> for ClientSideWeightedRoundRobinMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientSideWeightedRoundRobin>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientSideWeightedRoundRobinMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientSideWeightedRoundRobin> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ClientSideWeightedRoundRobin {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // enable_oob_load_report: optional message google.protobuf.BoolValue
  pub fn has_enable_oob_load_report(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_enable_oob_load_report(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn enable_oob_load_report_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enable_oob_load_report().then(|| self.enable_oob_load_report())
  }
  pub fn enable_oob_load_report(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enable_oob_load_report_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_enable_oob_load_report(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // oob_reporting_period: optional message google.protobuf.Duration
  pub fn has_oob_reporting_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_oob_reporting_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn oob_reporting_period_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_oob_reporting_period().then(|| self.oob_reporting_period())
  }
  pub fn oob_reporting_period(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn oob_reporting_period_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_oob_reporting_period(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // blackout_period: optional message google.protobuf.Duration
  pub fn has_blackout_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_blackout_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn blackout_period_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_blackout_period().then(|| self.blackout_period())
  }
  pub fn blackout_period(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn blackout_period_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_blackout_period(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // weight_expiration_period: optional message google.protobuf.Duration
  pub fn has_weight_expiration_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_weight_expiration_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn weight_expiration_period_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_weight_expiration_period().then(|| self.weight_expiration_period())
  }
  pub fn weight_expiration_period(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn weight_expiration_period_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_weight_expiration_period(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // weight_update_period: optional message google.protobuf.Duration
  pub fn has_weight_update_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_weight_update_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn weight_update_period_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_weight_update_period().then(|| self.weight_update_period())
  }
  pub fn weight_update_period(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn weight_update_period_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_weight_update_period(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // error_utilization_penalty: optional message google.protobuf.FloatValue
  pub fn has_error_utilization_penalty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_error_utilization_penalty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn error_utilization_penalty_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::FloatValueView<'_>> {
    self.has_error_utilization_penalty().then(|| self.error_utilization_penalty())
  }
  pub fn error_utilization_penalty(&self) -> ::protobuf_well_known_types::FloatValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FloatValueView::default())
  }
  pub fn error_utilization_penalty_mut(&mut self) -> ::protobuf_well_known_types::FloatValueMut<'_> {
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
  pub fn set_error_utilization_penalty(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::FloatValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // metric_names_for_computing_utilization: repeated string
  pub fn metric_names_for_computing_utilization(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn metric_names_for_computing_utilization_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
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
  pub fn set_metric_names_for_computing_utilization(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // slow_start_config: optional message envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig
  pub fn has_slow_start_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_slow_start_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn slow_start_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView::default())
  }
  pub fn slow_start_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigMut<'_> {
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
  pub fn set_slow_start_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfig>) {

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
// - `ClientSideWeightedRoundRobinMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClientSideWeightedRoundRobinMut<'_> {}

// SAFETY:
// - `ClientSideWeightedRoundRobinMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClientSideWeightedRoundRobinMut<'_> {}

impl<'msg> ::protobuf::AsView for ClientSideWeightedRoundRobinMut<'msg> {
  type Proxied = ClientSideWeightedRoundRobin;
  fn as_view(&self) -> ::protobuf::View<'_, ClientSideWeightedRoundRobin> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientSideWeightedRoundRobinMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClientSideWeightedRoundRobin>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClientSideWeightedRoundRobinMut<'msg> {
  type MutProxied = ClientSideWeightedRoundRobin;
  fn as_mut(&mut self) -> ClientSideWeightedRoundRobinMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClientSideWeightedRoundRobinMut<'msg> {
  fn into_mut<'shorter>(self) -> ClientSideWeightedRoundRobinMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClientSideWeightedRoundRobin {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClientSideWeightedRoundRobin> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClientSideWeightedRoundRobinView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClientSideWeightedRoundRobinMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // enable_oob_load_report: optional message google.protobuf.BoolValue
  pub fn has_enable_oob_load_report(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_enable_oob_load_report(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn enable_oob_load_report_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enable_oob_load_report().then(|| self.enable_oob_load_report())
  }
  pub fn enable_oob_load_report(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enable_oob_load_report_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_enable_oob_load_report(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // oob_reporting_period: optional message google.protobuf.Duration
  pub fn has_oob_reporting_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_oob_reporting_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn oob_reporting_period_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_oob_reporting_period().then(|| self.oob_reporting_period())
  }
  pub fn oob_reporting_period(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn oob_reporting_period_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_oob_reporting_period(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // blackout_period: optional message google.protobuf.Duration
  pub fn has_blackout_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_blackout_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn blackout_period_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_blackout_period().then(|| self.blackout_period())
  }
  pub fn blackout_period(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn blackout_period_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_blackout_period(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // weight_expiration_period: optional message google.protobuf.Duration
  pub fn has_weight_expiration_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_weight_expiration_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn weight_expiration_period_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_weight_expiration_period().then(|| self.weight_expiration_period())
  }
  pub fn weight_expiration_period(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn weight_expiration_period_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_weight_expiration_period(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // weight_update_period: optional message google.protobuf.Duration
  pub fn has_weight_update_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_weight_update_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn weight_update_period_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_weight_update_period().then(|| self.weight_update_period())
  }
  pub fn weight_update_period(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn weight_update_period_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_weight_update_period(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // error_utilization_penalty: optional message google.protobuf.FloatValue
  pub fn has_error_utilization_penalty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_error_utilization_penalty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn error_utilization_penalty_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::FloatValueView<'_>> {
    self.has_error_utilization_penalty().then(|| self.error_utilization_penalty())
  }
  pub fn error_utilization_penalty(&self) -> ::protobuf_well_known_types::FloatValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FloatValueView::default())
  }
  pub fn error_utilization_penalty_mut(&mut self) -> ::protobuf_well_known_types::FloatValueMut<'_> {
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
  pub fn set_error_utilization_penalty(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::FloatValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // metric_names_for_computing_utilization: repeated string
  pub fn metric_names_for_computing_utilization(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn metric_names_for_computing_utilization_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
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
  pub fn set_metric_names_for_computing_utilization(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // slow_start_config: optional message envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig
  pub fn has_slow_start_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_slow_start_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn slow_start_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView::default())
  }
  pub fn slow_start_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigMut<'_> {
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
  pub fn set_slow_start_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

}  // impl ClientSideWeightedRoundRobin

impl ::std::ops::Drop for ClientSideWeightedRoundRobin {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClientSideWeightedRoundRobin {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClientSideWeightedRoundRobin {
  type Proxied = Self;
  fn as_view(&self) -> ClientSideWeightedRoundRobinView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClientSideWeightedRoundRobin {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClientSideWeightedRoundRobinMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClientSideWeightedRoundRobin {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__load_0balancing_0policies__client_0side_0weighted_0round_0robin__v3__ClientSideWeightedRoundRobin_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333333ET3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__load_0balancing_0policies__client_0side_0weighted_0round_0robin__v3__ClientSideWeightedRoundRobin_msg_init.0, &[<::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::FloatValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__load_0balancing_0policies__client_0side_0weighted_0round_0robin__v3__ClientSideWeightedRoundRobin_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientSideWeightedRoundRobin {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientSideWeightedRoundRobin {
  type Msg = ClientSideWeightedRoundRobin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientSideWeightedRoundRobin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientSideWeightedRoundRobin {
  type Msg = ClientSideWeightedRoundRobin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientSideWeightedRoundRobin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientSideWeightedRoundRobinMut<'_> {
  type Msg = ClientSideWeightedRoundRobin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientSideWeightedRoundRobin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientSideWeightedRoundRobinMut<'_> {
  type Msg = ClientSideWeightedRoundRobin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientSideWeightedRoundRobin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientSideWeightedRoundRobinView<'_> {
  type Msg = ClientSideWeightedRoundRobin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientSideWeightedRoundRobin> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientSideWeightedRoundRobinMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



