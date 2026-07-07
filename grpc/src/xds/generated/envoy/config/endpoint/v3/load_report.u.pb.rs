const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__UpstreamLocalityStats_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UpstreamLocalityStats {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UpstreamLocalityStats>
}

impl ::protobuf::Message for UpstreamLocalityStats {
  type MessageView<'msg> = UpstreamLocalityStatsView<'msg>;
  type MessageMut<'msg> = UpstreamLocalityStatsMut<'msg>;
}

impl ::std::default::Default for UpstreamLocalityStats {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UpstreamLocalityStats {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UpstreamLocalityStats` is `Sync` because it does not implement interior mutability.
//    Neither does `UpstreamLocalityStatsMut`.
unsafe impl ::std::marker::Sync for UpstreamLocalityStats {}

// SAFETY:
// - `UpstreamLocalityStats` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamLocalityStats {}

impl ::protobuf::Proxied for UpstreamLocalityStats {
  type View<'msg> = UpstreamLocalityStatsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UpstreamLocalityStats {}

impl ::protobuf::MutProxied for UpstreamLocalityStats {
  type Mut<'msg> = UpstreamLocalityStatsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpstreamLocalityStatsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamLocalityStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamLocalityStatsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpstreamLocalityStatsView<'msg> {
  type Message = UpstreamLocalityStats;
}

impl ::std::fmt::Debug for UpstreamLocalityStatsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpstreamLocalityStatsView<'_> {
  fn default() -> UpstreamLocalityStatsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamLocalityStats>> for UpstreamLocalityStatsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamLocalityStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamLocalityStatsView<'msg> {

  pub fn to_owned(&self) -> UpstreamLocalityStats {
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

  // total_successful_requests: optional uint64
  pub fn total_successful_requests(self) -> u64 {
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

  // total_requests_in_progress: optional uint64
  pub fn total_requests_in_progress(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // total_error_requests: optional uint64
  pub fn total_error_requests(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // total_issued_requests: optional uint64
  pub fn total_issued_requests(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        7, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // total_active_connections: optional uint64
  pub fn total_active_connections(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        8, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // total_new_connections: optional uint64
  pub fn total_new_connections(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        9, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // total_fail_connections: optional uint64
  pub fn total_fail_connections(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        10, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // cpu_utilization: optional message envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats
  pub fn has_cpu_utilization(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn cpu_utilization_opt(self) -> ::std::option::Option<super::UnnamedEndpointLoadMetricStatsView<'msg>> {
    self.has_cpu_utilization().then(|| self.cpu_utilization())
  }
  pub fn cpu_utilization(self) -> super::UnnamedEndpointLoadMetricStatsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UnnamedEndpointLoadMetricStatsView::default())
  }

  // mem_utilization: optional message envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats
  pub fn has_mem_utilization(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn mem_utilization_opt(self) -> ::std::option::Option<super::UnnamedEndpointLoadMetricStatsView<'msg>> {
    self.has_mem_utilization().then(|| self.mem_utilization())
  }
  pub fn mem_utilization(self) -> super::UnnamedEndpointLoadMetricStatsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UnnamedEndpointLoadMetricStatsView::default())
  }

  // application_utilization: optional message envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats
  pub fn has_application_utilization(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn application_utilization_opt(self) -> ::std::option::Option<super::UnnamedEndpointLoadMetricStatsView<'msg>> {
    self.has_application_utilization().then(|| self.application_utilization())
  }
  pub fn application_utilization(self) -> super::UnnamedEndpointLoadMetricStatsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UnnamedEndpointLoadMetricStatsView::default())
  }

  // load_metric_stats: repeated message envoy.config.endpoint.v3.EndpointLoadMetricStats
  pub fn load_metric_stats(self) -> ::protobuf::RepeatedView<'msg, super::EndpointLoadMetricStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EndpointLoadMetricStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // upstream_endpoint_stats: repeated message envoy.config.endpoint.v3.UpstreamEndpointStats
  pub fn upstream_endpoint_stats(self) -> ::protobuf::RepeatedView<'msg, super::UpstreamEndpointStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::UpstreamEndpointStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
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
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `UpstreamLocalityStatsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UpstreamLocalityStatsView<'_> {}

// SAFETY:
// - `UpstreamLocalityStatsView` is `Send` because while its alive a `UpstreamLocalityStatsMut` cannot.
// - `UpstreamLocalityStatsView` does not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamLocalityStatsView<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamLocalityStatsView<'msg> {
  type Proxied = UpstreamLocalityStats;
  fn as_view(&self) -> ::protobuf::View<'msg, UpstreamLocalityStats> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamLocalityStatsView<'msg> {
  fn into_view<'shorter>(self) -> UpstreamLocalityStatsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamLocalityStats> for UpstreamLocalityStatsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamLocalityStats {
    let mut dst = UpstreamLocalityStats::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamLocalityStats> for UpstreamLocalityStatsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamLocalityStats {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UpstreamLocalityStats {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamLocalityStatsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamLocalityStatsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpstreamLocalityStatsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamLocalityStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamLocalityStatsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpstreamLocalityStatsMut<'msg> {
  type Message = UpstreamLocalityStats;
}

impl ::std::fmt::Debug for UpstreamLocalityStatsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamLocalityStats>> for UpstreamLocalityStatsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamLocalityStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamLocalityStatsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamLocalityStats> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UpstreamLocalityStats {
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

  // total_successful_requests: optional uint64
  pub fn total_successful_requests(&self) -> u64 {
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
  pub fn set_total_successful_requests(&mut self, val: u64) {
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

  // total_requests_in_progress: optional uint64
  pub fn total_requests_in_progress(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_requests_in_progress(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // total_error_requests: optional uint64
  pub fn total_error_requests(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_error_requests(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // total_issued_requests: optional uint64
  pub fn total_issued_requests(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        7, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_issued_requests(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        7, val.into()
      )
    }
  }

  // total_active_connections: optional uint64
  pub fn total_active_connections(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        8, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_active_connections(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        8, val.into()
      )
    }
  }

  // total_new_connections: optional uint64
  pub fn total_new_connections(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        9, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_new_connections(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        9, val.into()
      )
    }
  }

  // total_fail_connections: optional uint64
  pub fn total_fail_connections(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        10, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_fail_connections(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        10, val.into()
      )
    }
  }

  // cpu_utilization: optional message envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats
  pub fn has_cpu_utilization(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_cpu_utilization(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn cpu_utilization_opt(&self) -> ::std::option::Option<super::UnnamedEndpointLoadMetricStatsView<'_>> {
    self.has_cpu_utilization().then(|| self.cpu_utilization())
  }
  pub fn cpu_utilization(&self) -> super::UnnamedEndpointLoadMetricStatsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UnnamedEndpointLoadMetricStatsView::default())
  }
  pub fn cpu_utilization_mut(&mut self) -> super::UnnamedEndpointLoadMetricStatsMut<'_> {
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
  pub fn set_cpu_utilization(&mut self,
    val: impl ::protobuf::IntoProxied<super::UnnamedEndpointLoadMetricStats>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // mem_utilization: optional message envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats
  pub fn has_mem_utilization(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_mem_utilization(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn mem_utilization_opt(&self) -> ::std::option::Option<super::UnnamedEndpointLoadMetricStatsView<'_>> {
    self.has_mem_utilization().then(|| self.mem_utilization())
  }
  pub fn mem_utilization(&self) -> super::UnnamedEndpointLoadMetricStatsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UnnamedEndpointLoadMetricStatsView::default())
  }
  pub fn mem_utilization_mut(&mut self) -> super::UnnamedEndpointLoadMetricStatsMut<'_> {
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
  pub fn set_mem_utilization(&mut self,
    val: impl ::protobuf::IntoProxied<super::UnnamedEndpointLoadMetricStats>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // application_utilization: optional message envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats
  pub fn has_application_utilization(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_application_utilization(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn application_utilization_opt(&self) -> ::std::option::Option<super::UnnamedEndpointLoadMetricStatsView<'_>> {
    self.has_application_utilization().then(|| self.application_utilization())
  }
  pub fn application_utilization(&self) -> super::UnnamedEndpointLoadMetricStatsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UnnamedEndpointLoadMetricStatsView::default())
  }
  pub fn application_utilization_mut(&mut self) -> super::UnnamedEndpointLoadMetricStatsMut<'_> {
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
  pub fn set_application_utilization(&mut self,
    val: impl ::protobuf::IntoProxied<super::UnnamedEndpointLoadMetricStats>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // load_metric_stats: repeated message envoy.config.endpoint.v3.EndpointLoadMetricStats
  pub fn load_metric_stats(&self) -> ::protobuf::RepeatedView<'_, super::EndpointLoadMetricStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EndpointLoadMetricStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn load_metric_stats_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::EndpointLoadMetricStats> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_load_metric_stats(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::EndpointLoadMetricStats>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // upstream_endpoint_stats: repeated message envoy.config.endpoint.v3.UpstreamEndpointStats
  pub fn upstream_endpoint_stats(&self) -> ::protobuf::RepeatedView<'_, super::UpstreamEndpointStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::UpstreamEndpointStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn upstream_endpoint_stats_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::UpstreamEndpointStats> {
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
  pub fn set_upstream_endpoint_stats(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::UpstreamEndpointStats>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
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
        5, (0u32).into()
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
        5, val.into()
      )
    }
  }

}

// SAFETY:
// - `UpstreamLocalityStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UpstreamLocalityStatsMut<'_> {}

// SAFETY:
// - `UpstreamLocalityStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UpstreamLocalityStatsMut<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamLocalityStatsMut<'msg> {
  type Proxied = UpstreamLocalityStats;
  fn as_view(&self) -> ::protobuf::View<'_, UpstreamLocalityStats> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamLocalityStatsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UpstreamLocalityStats>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UpstreamLocalityStatsMut<'msg> {
  type MutProxied = UpstreamLocalityStats;
  fn as_mut(&mut self) -> UpstreamLocalityStatsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpstreamLocalityStatsMut<'msg> {
  fn into_mut<'shorter>(self) -> UpstreamLocalityStatsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UpstreamLocalityStats {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UpstreamLocalityStats> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpstreamLocalityStatsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpstreamLocalityStatsMut<'_> {
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

  // total_successful_requests: optional uint64
  pub fn total_successful_requests(&self) -> u64 {
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
  pub fn set_total_successful_requests(&mut self, val: u64) {
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

  // total_requests_in_progress: optional uint64
  pub fn total_requests_in_progress(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_requests_in_progress(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // total_error_requests: optional uint64
  pub fn total_error_requests(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_error_requests(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // total_issued_requests: optional uint64
  pub fn total_issued_requests(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        7, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_issued_requests(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        7, val.into()
      )
    }
  }

  // total_active_connections: optional uint64
  pub fn total_active_connections(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        8, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_active_connections(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        8, val.into()
      )
    }
  }

  // total_new_connections: optional uint64
  pub fn total_new_connections(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        9, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_new_connections(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        9, val.into()
      )
    }
  }

  // total_fail_connections: optional uint64
  pub fn total_fail_connections(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        10, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_fail_connections(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        10, val.into()
      )
    }
  }

  // cpu_utilization: optional message envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats
  pub fn has_cpu_utilization(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_cpu_utilization(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn cpu_utilization_opt(&self) -> ::std::option::Option<super::UnnamedEndpointLoadMetricStatsView<'_>> {
    self.has_cpu_utilization().then(|| self.cpu_utilization())
  }
  pub fn cpu_utilization(&self) -> super::UnnamedEndpointLoadMetricStatsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UnnamedEndpointLoadMetricStatsView::default())
  }
  pub fn cpu_utilization_mut(&mut self) -> super::UnnamedEndpointLoadMetricStatsMut<'_> {
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
  pub fn set_cpu_utilization(&mut self,
    val: impl ::protobuf::IntoProxied<super::UnnamedEndpointLoadMetricStats>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // mem_utilization: optional message envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats
  pub fn has_mem_utilization(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_mem_utilization(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn mem_utilization_opt(&self) -> ::std::option::Option<super::UnnamedEndpointLoadMetricStatsView<'_>> {
    self.has_mem_utilization().then(|| self.mem_utilization())
  }
  pub fn mem_utilization(&self) -> super::UnnamedEndpointLoadMetricStatsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UnnamedEndpointLoadMetricStatsView::default())
  }
  pub fn mem_utilization_mut(&mut self) -> super::UnnamedEndpointLoadMetricStatsMut<'_> {
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
  pub fn set_mem_utilization(&mut self,
    val: impl ::protobuf::IntoProxied<super::UnnamedEndpointLoadMetricStats>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // application_utilization: optional message envoy.config.endpoint.v3.UnnamedEndpointLoadMetricStats
  pub fn has_application_utilization(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_application_utilization(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn application_utilization_opt(&self) -> ::std::option::Option<super::UnnamedEndpointLoadMetricStatsView<'_>> {
    self.has_application_utilization().then(|| self.application_utilization())
  }
  pub fn application_utilization(&self) -> super::UnnamedEndpointLoadMetricStatsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UnnamedEndpointLoadMetricStatsView::default())
  }
  pub fn application_utilization_mut(&mut self) -> super::UnnamedEndpointLoadMetricStatsMut<'_> {
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
  pub fn set_application_utilization(&mut self,
    val: impl ::protobuf::IntoProxied<super::UnnamedEndpointLoadMetricStats>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // load_metric_stats: repeated message envoy.config.endpoint.v3.EndpointLoadMetricStats
  pub fn load_metric_stats(&self) -> ::protobuf::RepeatedView<'_, super::EndpointLoadMetricStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EndpointLoadMetricStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn load_metric_stats_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::EndpointLoadMetricStats> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_load_metric_stats(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::EndpointLoadMetricStats>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // upstream_endpoint_stats: repeated message envoy.config.endpoint.v3.UpstreamEndpointStats
  pub fn upstream_endpoint_stats(&self) -> ::protobuf::RepeatedView<'_, super::UpstreamEndpointStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::UpstreamEndpointStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn upstream_endpoint_stats_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::UpstreamEndpointStats> {
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
  pub fn set_upstream_endpoint_stats(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::UpstreamEndpointStats>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
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
        5, (0u32).into()
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
        5, val.into()
      )
    }
  }

}  // impl UpstreamLocalityStats

impl ::std::ops::Drop for UpstreamLocalityStats {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UpstreamLocalityStats {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UpstreamLocalityStats {
  type Proxied = Self;
  fn as_view(&self) -> UpstreamLocalityStatsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UpstreamLocalityStats {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpstreamLocalityStatsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UpstreamLocalityStats {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__endpoint__v3__UpstreamLocalityStats_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3,P,P,PG)PG,P,P,P,P333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__endpoint__v3__UpstreamLocalityStats_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::Locality as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::EndpointLoadMetricStats as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::UpstreamEndpointStats as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::UnnamedEndpointLoadMetricStats as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::UnnamedEndpointLoadMetricStats as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::UnnamedEndpointLoadMetricStats as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__endpoint__v3__UpstreamLocalityStats_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamLocalityStats {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamLocalityStats {
  type Msg = UpstreamLocalityStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamLocalityStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamLocalityStats {
  type Msg = UpstreamLocalityStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamLocalityStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamLocalityStatsMut<'_> {
  type Msg = UpstreamLocalityStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamLocalityStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamLocalityStatsMut<'_> {
  type Msg = UpstreamLocalityStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamLocalityStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamLocalityStatsView<'_> {
  type Msg = UpstreamLocalityStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamLocalityStats> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamLocalityStatsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__UpstreamEndpointStats_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UpstreamEndpointStats {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UpstreamEndpointStats>
}

impl ::protobuf::Message for UpstreamEndpointStats {
  type MessageView<'msg> = UpstreamEndpointStatsView<'msg>;
  type MessageMut<'msg> = UpstreamEndpointStatsMut<'msg>;
}

impl ::std::default::Default for UpstreamEndpointStats {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UpstreamEndpointStats {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UpstreamEndpointStats` is `Sync` because it does not implement interior mutability.
//    Neither does `UpstreamEndpointStatsMut`.
unsafe impl ::std::marker::Sync for UpstreamEndpointStats {}

// SAFETY:
// - `UpstreamEndpointStats` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamEndpointStats {}

impl ::protobuf::Proxied for UpstreamEndpointStats {
  type View<'msg> = UpstreamEndpointStatsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UpstreamEndpointStats {}

impl ::protobuf::MutProxied for UpstreamEndpointStats {
  type Mut<'msg> = UpstreamEndpointStatsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpstreamEndpointStatsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamEndpointStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamEndpointStatsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpstreamEndpointStatsView<'msg> {
  type Message = UpstreamEndpointStats;
}

impl ::std::fmt::Debug for UpstreamEndpointStatsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpstreamEndpointStatsView<'_> {
  fn default() -> UpstreamEndpointStatsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamEndpointStats>> for UpstreamEndpointStatsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamEndpointStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamEndpointStatsView<'msg> {

  pub fn to_owned(&self) -> UpstreamEndpointStats {
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

  // metadata: optional message google.protobuf.Struct
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // total_successful_requests: optional uint64
  pub fn total_successful_requests(self) -> u64 {
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

  // total_requests_in_progress: optional uint64
  pub fn total_requests_in_progress(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // total_error_requests: optional uint64
  pub fn total_error_requests(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // total_issued_requests: optional uint64
  pub fn total_issued_requests(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        6, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // load_metric_stats: repeated message envoy.config.endpoint.v3.EndpointLoadMetricStats
  pub fn load_metric_stats(self) -> ::protobuf::RepeatedView<'msg, super::EndpointLoadMetricStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EndpointLoadMetricStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `UpstreamEndpointStatsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UpstreamEndpointStatsView<'_> {}

// SAFETY:
// - `UpstreamEndpointStatsView` is `Send` because while its alive a `UpstreamEndpointStatsMut` cannot.
// - `UpstreamEndpointStatsView` does not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamEndpointStatsView<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamEndpointStatsView<'msg> {
  type Proxied = UpstreamEndpointStats;
  fn as_view(&self) -> ::protobuf::View<'msg, UpstreamEndpointStats> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamEndpointStatsView<'msg> {
  fn into_view<'shorter>(self) -> UpstreamEndpointStatsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamEndpointStats> for UpstreamEndpointStatsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamEndpointStats {
    let mut dst = UpstreamEndpointStats::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamEndpointStats> for UpstreamEndpointStatsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamEndpointStats {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UpstreamEndpointStats {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamEndpointStatsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamEndpointStatsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpstreamEndpointStatsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamEndpointStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamEndpointStatsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpstreamEndpointStatsMut<'msg> {
  type Message = UpstreamEndpointStats;
}

impl ::std::fmt::Debug for UpstreamEndpointStatsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamEndpointStats>> for UpstreamEndpointStatsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamEndpointStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamEndpointStatsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamEndpointStats> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UpstreamEndpointStats {
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

  // metadata: optional message google.protobuf.Struct
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
  pub fn metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // total_successful_requests: optional uint64
  pub fn total_successful_requests(&self) -> u64 {
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
  pub fn set_total_successful_requests(&mut self, val: u64) {
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

  // total_requests_in_progress: optional uint64
  pub fn total_requests_in_progress(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_requests_in_progress(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // total_error_requests: optional uint64
  pub fn total_error_requests(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_error_requests(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // total_issued_requests: optional uint64
  pub fn total_issued_requests(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        6, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_issued_requests(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        6, val.into()
      )
    }
  }

  // load_metric_stats: repeated message envoy.config.endpoint.v3.EndpointLoadMetricStats
  pub fn load_metric_stats(&self) -> ::protobuf::RepeatedView<'_, super::EndpointLoadMetricStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EndpointLoadMetricStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn load_metric_stats_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::EndpointLoadMetricStats> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_load_metric_stats(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::EndpointLoadMetricStats>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

}

// SAFETY:
// - `UpstreamEndpointStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UpstreamEndpointStatsMut<'_> {}

// SAFETY:
// - `UpstreamEndpointStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UpstreamEndpointStatsMut<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamEndpointStatsMut<'msg> {
  type Proxied = UpstreamEndpointStats;
  fn as_view(&self) -> ::protobuf::View<'_, UpstreamEndpointStats> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamEndpointStatsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UpstreamEndpointStats>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UpstreamEndpointStatsMut<'msg> {
  type MutProxied = UpstreamEndpointStats;
  fn as_mut(&mut self) -> UpstreamEndpointStatsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpstreamEndpointStatsMut<'msg> {
  fn into_mut<'shorter>(self) -> UpstreamEndpointStatsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UpstreamEndpointStats {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UpstreamEndpointStats> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpstreamEndpointStatsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpstreamEndpointStatsMut<'_> {
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

  // metadata: optional message google.protobuf.Struct
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
  pub fn metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // total_successful_requests: optional uint64
  pub fn total_successful_requests(&self) -> u64 {
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
  pub fn set_total_successful_requests(&mut self, val: u64) {
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

  // total_requests_in_progress: optional uint64
  pub fn total_requests_in_progress(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_requests_in_progress(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // total_error_requests: optional uint64
  pub fn total_error_requests(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_error_requests(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // total_issued_requests: optional uint64
  pub fn total_issued_requests(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        6, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_issued_requests(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        6, val.into()
      )
    }
  }

  // load_metric_stats: repeated message envoy.config.endpoint.v3.EndpointLoadMetricStats
  pub fn load_metric_stats(&self) -> ::protobuf::RepeatedView<'_, super::EndpointLoadMetricStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EndpointLoadMetricStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn load_metric_stats_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::EndpointLoadMetricStats> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_load_metric_stats(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::EndpointLoadMetricStats>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

}  // impl UpstreamEndpointStats

impl ::std::ops::Drop for UpstreamEndpointStats {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UpstreamEndpointStats {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UpstreamEndpointStats {
  type Proxied = Self;
  fn as_view(&self) -> UpstreamEndpointStatsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UpstreamEndpointStats {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpstreamEndpointStatsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UpstreamEndpointStats {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__endpoint__v3__UpstreamEndpointStats_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3,P,P,PG3,P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__endpoint__v3__UpstreamEndpointStats_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::EndpointLoadMetricStats as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__endpoint__v3__UpstreamEndpointStats_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamEndpointStats {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamEndpointStats {
  type Msg = UpstreamEndpointStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamEndpointStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamEndpointStats {
  type Msg = UpstreamEndpointStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamEndpointStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamEndpointStatsMut<'_> {
  type Msg = UpstreamEndpointStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamEndpointStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamEndpointStatsMut<'_> {
  type Msg = UpstreamEndpointStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamEndpointStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamEndpointStatsView<'_> {
  type Msg = UpstreamEndpointStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamEndpointStats> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamEndpointStatsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__EndpointLoadMetricStats_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EndpointLoadMetricStats {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EndpointLoadMetricStats>
}

impl ::protobuf::Message for EndpointLoadMetricStats {
  type MessageView<'msg> = EndpointLoadMetricStatsView<'msg>;
  type MessageMut<'msg> = EndpointLoadMetricStatsMut<'msg>;
}

impl ::std::default::Default for EndpointLoadMetricStats {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EndpointLoadMetricStats {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EndpointLoadMetricStats` is `Sync` because it does not implement interior mutability.
//    Neither does `EndpointLoadMetricStatsMut`.
unsafe impl ::std::marker::Sync for EndpointLoadMetricStats {}

// SAFETY:
// - `EndpointLoadMetricStats` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for EndpointLoadMetricStats {}

impl ::protobuf::Proxied for EndpointLoadMetricStats {
  type View<'msg> = EndpointLoadMetricStatsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EndpointLoadMetricStats {}

impl ::protobuf::MutProxied for EndpointLoadMetricStats {
  type Mut<'msg> = EndpointLoadMetricStatsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EndpointLoadMetricStatsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EndpointLoadMetricStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EndpointLoadMetricStatsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EndpointLoadMetricStatsView<'msg> {
  type Message = EndpointLoadMetricStats;
}

impl ::std::fmt::Debug for EndpointLoadMetricStatsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EndpointLoadMetricStatsView<'_> {
  fn default() -> EndpointLoadMetricStatsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EndpointLoadMetricStats>> for EndpointLoadMetricStatsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EndpointLoadMetricStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EndpointLoadMetricStatsView<'msg> {

  pub fn to_owned(&self) -> EndpointLoadMetricStats {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // metric_name: optional string
  pub fn metric_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // num_requests_finished_with_metric: optional uint64
  pub fn num_requests_finished_with_metric(self) -> u64 {
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

  // total_metric_value: optional double
  pub fn total_metric_value(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `EndpointLoadMetricStatsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EndpointLoadMetricStatsView<'_> {}

// SAFETY:
// - `EndpointLoadMetricStatsView` is `Send` because while its alive a `EndpointLoadMetricStatsMut` cannot.
// - `EndpointLoadMetricStatsView` does not use thread-local data.
unsafe impl ::std::marker::Send for EndpointLoadMetricStatsView<'_> {}

impl<'msg> ::protobuf::AsView for EndpointLoadMetricStatsView<'msg> {
  type Proxied = EndpointLoadMetricStats;
  fn as_view(&self) -> ::protobuf::View<'msg, EndpointLoadMetricStats> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EndpointLoadMetricStatsView<'msg> {
  fn into_view<'shorter>(self) -> EndpointLoadMetricStatsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EndpointLoadMetricStats> for EndpointLoadMetricStatsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EndpointLoadMetricStats {
    let mut dst = EndpointLoadMetricStats::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EndpointLoadMetricStats> for EndpointLoadMetricStatsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EndpointLoadMetricStats {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for EndpointLoadMetricStats {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EndpointLoadMetricStatsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EndpointLoadMetricStatsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EndpointLoadMetricStatsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EndpointLoadMetricStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EndpointLoadMetricStatsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EndpointLoadMetricStatsMut<'msg> {
  type Message = EndpointLoadMetricStats;
}

impl ::std::fmt::Debug for EndpointLoadMetricStatsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EndpointLoadMetricStats>> for EndpointLoadMetricStatsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EndpointLoadMetricStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EndpointLoadMetricStatsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EndpointLoadMetricStats> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> EndpointLoadMetricStats {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // metric_name: optional string
  pub fn metric_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_metric_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // num_requests_finished_with_metric: optional uint64
  pub fn num_requests_finished_with_metric(&self) -> u64 {
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
  pub fn set_num_requests_finished_with_metric(&mut self, val: u64) {
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

  // total_metric_value: optional double
  pub fn total_metric_value(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_metric_value(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `EndpointLoadMetricStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EndpointLoadMetricStatsMut<'_> {}

// SAFETY:
// - `EndpointLoadMetricStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EndpointLoadMetricStatsMut<'_> {}

impl<'msg> ::protobuf::AsView for EndpointLoadMetricStatsMut<'msg> {
  type Proxied = EndpointLoadMetricStats;
  fn as_view(&self) -> ::protobuf::View<'_, EndpointLoadMetricStats> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EndpointLoadMetricStatsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EndpointLoadMetricStats>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EndpointLoadMetricStatsMut<'msg> {
  type MutProxied = EndpointLoadMetricStats;
  fn as_mut(&mut self) -> EndpointLoadMetricStatsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EndpointLoadMetricStatsMut<'msg> {
  fn into_mut<'shorter>(self) -> EndpointLoadMetricStatsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EndpointLoadMetricStats {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EndpointLoadMetricStats> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EndpointLoadMetricStatsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EndpointLoadMetricStatsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // metric_name: optional string
  pub fn metric_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_metric_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // num_requests_finished_with_metric: optional uint64
  pub fn num_requests_finished_with_metric(&self) -> u64 {
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
  pub fn set_num_requests_finished_with_metric(&mut self, val: u64) {
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

  // total_metric_value: optional double
  pub fn total_metric_value(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_metric_value(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        2, val.into()
      )
    }
  }

}  // impl EndpointLoadMetricStats

impl ::std::ops::Drop for EndpointLoadMetricStats {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EndpointLoadMetricStats {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EndpointLoadMetricStats {
  type Proxied = Self;
  fn as_view(&self) -> EndpointLoadMetricStatsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EndpointLoadMetricStats {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EndpointLoadMetricStatsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EndpointLoadMetricStats {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__endpoint__v3__EndpointLoadMetricStats_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X,P P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__endpoint__v3__EndpointLoadMetricStats_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__endpoint__v3__EndpointLoadMetricStats_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EndpointLoadMetricStats {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EndpointLoadMetricStats {
  type Msg = EndpointLoadMetricStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EndpointLoadMetricStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EndpointLoadMetricStats {
  type Msg = EndpointLoadMetricStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EndpointLoadMetricStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EndpointLoadMetricStatsMut<'_> {
  type Msg = EndpointLoadMetricStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EndpointLoadMetricStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EndpointLoadMetricStatsMut<'_> {
  type Msg = EndpointLoadMetricStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EndpointLoadMetricStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EndpointLoadMetricStatsView<'_> {
  type Msg = EndpointLoadMetricStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EndpointLoadMetricStats> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EndpointLoadMetricStatsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__UnnamedEndpointLoadMetricStats_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UnnamedEndpointLoadMetricStats {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UnnamedEndpointLoadMetricStats>
}

impl ::protobuf::Message for UnnamedEndpointLoadMetricStats {
  type MessageView<'msg> = UnnamedEndpointLoadMetricStatsView<'msg>;
  type MessageMut<'msg> = UnnamedEndpointLoadMetricStatsMut<'msg>;
}

impl ::std::default::Default for UnnamedEndpointLoadMetricStats {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UnnamedEndpointLoadMetricStats {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UnnamedEndpointLoadMetricStats` is `Sync` because it does not implement interior mutability.
//    Neither does `UnnamedEndpointLoadMetricStatsMut`.
unsafe impl ::std::marker::Sync for UnnamedEndpointLoadMetricStats {}

// SAFETY:
// - `UnnamedEndpointLoadMetricStats` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UnnamedEndpointLoadMetricStats {}

impl ::protobuf::Proxied for UnnamedEndpointLoadMetricStats {
  type View<'msg> = UnnamedEndpointLoadMetricStatsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UnnamedEndpointLoadMetricStats {}

impl ::protobuf::MutProxied for UnnamedEndpointLoadMetricStats {
  type Mut<'msg> = UnnamedEndpointLoadMetricStatsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UnnamedEndpointLoadMetricStatsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UnnamedEndpointLoadMetricStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UnnamedEndpointLoadMetricStatsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UnnamedEndpointLoadMetricStatsView<'msg> {
  type Message = UnnamedEndpointLoadMetricStats;
}

impl ::std::fmt::Debug for UnnamedEndpointLoadMetricStatsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UnnamedEndpointLoadMetricStatsView<'_> {
  fn default() -> UnnamedEndpointLoadMetricStatsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UnnamedEndpointLoadMetricStats>> for UnnamedEndpointLoadMetricStatsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UnnamedEndpointLoadMetricStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UnnamedEndpointLoadMetricStatsView<'msg> {

  pub fn to_owned(&self) -> UnnamedEndpointLoadMetricStats {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // num_requests_finished_with_metric: optional uint64
  pub fn num_requests_finished_with_metric(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // total_metric_value: optional double
  pub fn total_metric_value(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `UnnamedEndpointLoadMetricStatsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UnnamedEndpointLoadMetricStatsView<'_> {}

// SAFETY:
// - `UnnamedEndpointLoadMetricStatsView` is `Send` because while its alive a `UnnamedEndpointLoadMetricStatsMut` cannot.
// - `UnnamedEndpointLoadMetricStatsView` does not use thread-local data.
unsafe impl ::std::marker::Send for UnnamedEndpointLoadMetricStatsView<'_> {}

impl<'msg> ::protobuf::AsView for UnnamedEndpointLoadMetricStatsView<'msg> {
  type Proxied = UnnamedEndpointLoadMetricStats;
  fn as_view(&self) -> ::protobuf::View<'msg, UnnamedEndpointLoadMetricStats> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UnnamedEndpointLoadMetricStatsView<'msg> {
  fn into_view<'shorter>(self) -> UnnamedEndpointLoadMetricStatsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UnnamedEndpointLoadMetricStats> for UnnamedEndpointLoadMetricStatsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UnnamedEndpointLoadMetricStats {
    let mut dst = UnnamedEndpointLoadMetricStats::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UnnamedEndpointLoadMetricStats> for UnnamedEndpointLoadMetricStatsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UnnamedEndpointLoadMetricStats {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UnnamedEndpointLoadMetricStats {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UnnamedEndpointLoadMetricStatsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UnnamedEndpointLoadMetricStatsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UnnamedEndpointLoadMetricStatsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UnnamedEndpointLoadMetricStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UnnamedEndpointLoadMetricStatsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UnnamedEndpointLoadMetricStatsMut<'msg> {
  type Message = UnnamedEndpointLoadMetricStats;
}

impl ::std::fmt::Debug for UnnamedEndpointLoadMetricStatsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UnnamedEndpointLoadMetricStats>> for UnnamedEndpointLoadMetricStatsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UnnamedEndpointLoadMetricStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UnnamedEndpointLoadMetricStatsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UnnamedEndpointLoadMetricStats> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UnnamedEndpointLoadMetricStats {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // num_requests_finished_with_metric: optional uint64
  pub fn num_requests_finished_with_metric(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_num_requests_finished_with_metric(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // total_metric_value: optional double
  pub fn total_metric_value(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_metric_value(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `UnnamedEndpointLoadMetricStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UnnamedEndpointLoadMetricStatsMut<'_> {}

// SAFETY:
// - `UnnamedEndpointLoadMetricStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UnnamedEndpointLoadMetricStatsMut<'_> {}

impl<'msg> ::protobuf::AsView for UnnamedEndpointLoadMetricStatsMut<'msg> {
  type Proxied = UnnamedEndpointLoadMetricStats;
  fn as_view(&self) -> ::protobuf::View<'_, UnnamedEndpointLoadMetricStats> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UnnamedEndpointLoadMetricStatsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UnnamedEndpointLoadMetricStats>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UnnamedEndpointLoadMetricStatsMut<'msg> {
  type MutProxied = UnnamedEndpointLoadMetricStats;
  fn as_mut(&mut self) -> UnnamedEndpointLoadMetricStatsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UnnamedEndpointLoadMetricStatsMut<'msg> {
  fn into_mut<'shorter>(self) -> UnnamedEndpointLoadMetricStatsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UnnamedEndpointLoadMetricStats {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UnnamedEndpointLoadMetricStats> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UnnamedEndpointLoadMetricStatsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UnnamedEndpointLoadMetricStatsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // num_requests_finished_with_metric: optional uint64
  pub fn num_requests_finished_with_metric(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_num_requests_finished_with_metric(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // total_metric_value: optional double
  pub fn total_metric_value(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_metric_value(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

}  // impl UnnamedEndpointLoadMetricStats

impl ::std::ops::Drop for UnnamedEndpointLoadMetricStats {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UnnamedEndpointLoadMetricStats {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UnnamedEndpointLoadMetricStats {
  type Proxied = Self;
  fn as_view(&self) -> UnnamedEndpointLoadMetricStatsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UnnamedEndpointLoadMetricStats {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UnnamedEndpointLoadMetricStatsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UnnamedEndpointLoadMetricStats {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__endpoint__v3__UnnamedEndpointLoadMetricStats_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$,P P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__endpoint__v3__UnnamedEndpointLoadMetricStats_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__endpoint__v3__UnnamedEndpointLoadMetricStats_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UnnamedEndpointLoadMetricStats {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UnnamedEndpointLoadMetricStats {
  type Msg = UnnamedEndpointLoadMetricStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UnnamedEndpointLoadMetricStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UnnamedEndpointLoadMetricStats {
  type Msg = UnnamedEndpointLoadMetricStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UnnamedEndpointLoadMetricStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UnnamedEndpointLoadMetricStatsMut<'_> {
  type Msg = UnnamedEndpointLoadMetricStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UnnamedEndpointLoadMetricStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UnnamedEndpointLoadMetricStatsMut<'_> {
  type Msg = UnnamedEndpointLoadMetricStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UnnamedEndpointLoadMetricStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UnnamedEndpointLoadMetricStatsView<'_> {
  type Msg = UnnamedEndpointLoadMetricStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UnnamedEndpointLoadMetricStats> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UnnamedEndpointLoadMetricStatsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__ClusterStats_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClusterStats {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClusterStats>
}

impl ::protobuf::Message for ClusterStats {
  type MessageView<'msg> = ClusterStatsView<'msg>;
  type MessageMut<'msg> = ClusterStatsMut<'msg>;
}

impl ::std::default::Default for ClusterStats {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClusterStats {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClusterStats` is `Sync` because it does not implement interior mutability.
//    Neither does `ClusterStatsMut`.
unsafe impl ::std::marker::Sync for ClusterStats {}

// SAFETY:
// - `ClusterStats` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ClusterStats {}

impl ::protobuf::Proxied for ClusterStats {
  type View<'msg> = ClusterStatsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClusterStats {}

impl ::protobuf::MutProxied for ClusterStats {
  type Mut<'msg> = ClusterStatsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClusterStatsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterStatsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClusterStatsView<'msg> {
  type Message = ClusterStats;
}

impl ::std::fmt::Debug for ClusterStatsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClusterStatsView<'_> {
  fn default() -> ClusterStatsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterStats>> for ClusterStatsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterStatsView<'msg> {

  pub fn to_owned(&self) -> ClusterStats {
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

  // cluster_service_name: optional string
  pub fn cluster_service_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // upstream_locality_stats: repeated message envoy.config.endpoint.v3.UpstreamLocalityStats
  pub fn upstream_locality_stats(self) -> ::protobuf::RepeatedView<'msg, super::UpstreamLocalityStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::UpstreamLocalityStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // total_dropped_requests: optional uint64
  pub fn total_dropped_requests(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // dropped_requests: repeated message envoy.config.endpoint.v3.ClusterStats.DroppedRequests
  pub fn dropped_requests(self) -> ::protobuf::RepeatedView<'msg, super::cluster_stats::DroppedRequests> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::cluster_stats::DroppedRequests>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // load_report_interval: optional message google.protobuf.Duration
  pub fn has_load_report_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn load_report_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_load_report_interval().then(|| self.load_report_interval())
  }
  pub fn load_report_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `ClusterStatsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClusterStatsView<'_> {}

// SAFETY:
// - `ClusterStatsView` is `Send` because while its alive a `ClusterStatsMut` cannot.
// - `ClusterStatsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClusterStatsView<'_> {}

impl<'msg> ::protobuf::AsView for ClusterStatsView<'msg> {
  type Proxied = ClusterStats;
  fn as_view(&self) -> ::protobuf::View<'msg, ClusterStats> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterStatsView<'msg> {
  fn into_view<'shorter>(self) -> ClusterStatsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClusterStats> for ClusterStatsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClusterStats {
    let mut dst = ClusterStats::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClusterStats> for ClusterStatsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClusterStats {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ClusterStats {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterStatsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterStatsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClusterStatsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterStatsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClusterStatsMut<'msg> {
  type Message = ClusterStats;
}

impl ::std::fmt::Debug for ClusterStatsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterStats>> for ClusterStatsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterStatsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterStats> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ClusterStats {
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

  // cluster_service_name: optional string
  pub fn cluster_service_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster_service_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // upstream_locality_stats: repeated message envoy.config.endpoint.v3.UpstreamLocalityStats
  pub fn upstream_locality_stats(&self) -> ::protobuf::RepeatedView<'_, super::UpstreamLocalityStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::UpstreamLocalityStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn upstream_locality_stats_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::UpstreamLocalityStats> {
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
  pub fn set_upstream_locality_stats(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::UpstreamLocalityStats>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // total_dropped_requests: optional uint64
  pub fn total_dropped_requests(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_dropped_requests(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // dropped_requests: repeated message envoy.config.endpoint.v3.ClusterStats.DroppedRequests
  pub fn dropped_requests(&self) -> ::protobuf::RepeatedView<'_, super::cluster_stats::DroppedRequests> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::cluster_stats::DroppedRequests>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dropped_requests_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::cluster_stats::DroppedRequests> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_dropped_requests(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::cluster_stats::DroppedRequests>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // load_report_interval: optional message google.protobuf.Duration
  pub fn has_load_report_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_load_report_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn load_report_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_load_report_interval().then(|| self.load_report_interval())
  }
  pub fn load_report_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn load_report_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_load_report_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}

// SAFETY:
// - `ClusterStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClusterStatsMut<'_> {}

// SAFETY:
// - `ClusterStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClusterStatsMut<'_> {}

impl<'msg> ::protobuf::AsView for ClusterStatsMut<'msg> {
  type Proxied = ClusterStats;
  fn as_view(&self) -> ::protobuf::View<'_, ClusterStats> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterStatsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClusterStats>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClusterStatsMut<'msg> {
  type MutProxied = ClusterStats;
  fn as_mut(&mut self) -> ClusterStatsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClusterStatsMut<'msg> {
  fn into_mut<'shorter>(self) -> ClusterStatsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClusterStats {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClusterStats> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClusterStatsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClusterStatsMut<'_> {
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

  // cluster_service_name: optional string
  pub fn cluster_service_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster_service_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // upstream_locality_stats: repeated message envoy.config.endpoint.v3.UpstreamLocalityStats
  pub fn upstream_locality_stats(&self) -> ::protobuf::RepeatedView<'_, super::UpstreamLocalityStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::UpstreamLocalityStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn upstream_locality_stats_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::UpstreamLocalityStats> {
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
  pub fn set_upstream_locality_stats(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::UpstreamLocalityStats>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // total_dropped_requests: optional uint64
  pub fn total_dropped_requests(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_total_dropped_requests(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // dropped_requests: repeated message envoy.config.endpoint.v3.ClusterStats.DroppedRequests
  pub fn dropped_requests(&self) -> ::protobuf::RepeatedView<'_, super::cluster_stats::DroppedRequests> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::cluster_stats::DroppedRequests>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dropped_requests_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::cluster_stats::DroppedRequests> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_dropped_requests(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::cluster_stats::DroppedRequests>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // load_report_interval: optional message google.protobuf.Duration
  pub fn has_load_report_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_load_report_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn load_report_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_load_report_interval().then(|| self.load_report_interval())
  }
  pub fn load_report_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn load_report_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_load_report_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}  // impl ClusterStats

impl ::std::ops::Drop for ClusterStats {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClusterStats {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClusterStats {
  type Proxied = Self;
  fn as_view(&self) -> ClusterStatsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClusterStats {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClusterStatsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClusterStats {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__endpoint__v3__ClusterStats_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG,P3G1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__endpoint__v3__ClusterStats_msg_init.0, &[<super::UpstreamLocalityStats as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster_stats::DroppedRequests as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__endpoint__v3__ClusterStats_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterStats {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterStats {
  type Msg = ClusterStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterStats {
  type Msg = ClusterStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterStatsMut<'_> {
  type Msg = ClusterStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterStatsMut<'_> {
  type Msg = ClusterStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterStatsView<'_> {
  type Msg = ClusterStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterStats> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterStatsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod cluster_stats {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__ClusterStats__DroppedRequests_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DroppedRequests {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DroppedRequests>
}

impl ::protobuf::Message for DroppedRequests {
  type MessageView<'msg> = DroppedRequestsView<'msg>;
  type MessageMut<'msg> = DroppedRequestsMut<'msg>;
}

impl ::std::default::Default for DroppedRequests {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DroppedRequests {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DroppedRequests` is `Sync` because it does not implement interior mutability.
//    Neither does `DroppedRequestsMut`.
unsafe impl ::std::marker::Sync for DroppedRequests {}

// SAFETY:
// - `DroppedRequests` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DroppedRequests {}

impl ::protobuf::Proxied for DroppedRequests {
  type View<'msg> = DroppedRequestsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DroppedRequests {}

impl ::protobuf::MutProxied for DroppedRequests {
  type Mut<'msg> = DroppedRequestsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DroppedRequestsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DroppedRequests>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DroppedRequestsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DroppedRequestsView<'msg> {
  type Message = DroppedRequests;
}

impl ::std::fmt::Debug for DroppedRequestsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DroppedRequestsView<'_> {
  fn default() -> DroppedRequestsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DroppedRequests>> for DroppedRequestsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DroppedRequests>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DroppedRequestsView<'msg> {

  pub fn to_owned(&self) -> DroppedRequests {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // category: optional string
  pub fn category(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // dropped_count: optional uint64
  pub fn dropped_count(self) -> u64 {
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
// - `DroppedRequestsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DroppedRequestsView<'_> {}

// SAFETY:
// - `DroppedRequestsView` is `Send` because while its alive a `DroppedRequestsMut` cannot.
// - `DroppedRequestsView` does not use thread-local data.
unsafe impl ::std::marker::Send for DroppedRequestsView<'_> {}

impl<'msg> ::protobuf::AsView for DroppedRequestsView<'msg> {
  type Proxied = DroppedRequests;
  fn as_view(&self) -> ::protobuf::View<'msg, DroppedRequests> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DroppedRequestsView<'msg> {
  fn into_view<'shorter>(self) -> DroppedRequestsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DroppedRequests> for DroppedRequestsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DroppedRequests {
    let mut dst = DroppedRequests::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DroppedRequests> for DroppedRequestsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DroppedRequests {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DroppedRequests {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DroppedRequestsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DroppedRequestsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DroppedRequestsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DroppedRequests>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DroppedRequestsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DroppedRequestsMut<'msg> {
  type Message = DroppedRequests;
}

impl ::std::fmt::Debug for DroppedRequestsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DroppedRequests>> for DroppedRequestsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DroppedRequests>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DroppedRequestsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DroppedRequests> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DroppedRequests {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // category: optional string
  pub fn category(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_category(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // dropped_count: optional uint64
  pub fn dropped_count(&self) -> u64 {
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
  pub fn set_dropped_count(&mut self, val: u64) {
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
// - `DroppedRequestsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DroppedRequestsMut<'_> {}

// SAFETY:
// - `DroppedRequestsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DroppedRequestsMut<'_> {}

impl<'msg> ::protobuf::AsView for DroppedRequestsMut<'msg> {
  type Proxied = DroppedRequests;
  fn as_view(&self) -> ::protobuf::View<'_, DroppedRequests> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DroppedRequestsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DroppedRequests>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DroppedRequestsMut<'msg> {
  type MutProxied = DroppedRequests;
  fn as_mut(&mut self) -> DroppedRequestsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DroppedRequestsMut<'msg> {
  fn into_mut<'shorter>(self) -> DroppedRequestsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DroppedRequests {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DroppedRequests> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DroppedRequestsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DroppedRequestsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // category: optional string
  pub fn category(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_category(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // dropped_count: optional uint64
  pub fn dropped_count(&self) -> u64 {
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
  pub fn set_dropped_count(&mut self, val: u64) {
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

}  // impl DroppedRequests

impl ::std::ops::Drop for DroppedRequests {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DroppedRequests {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DroppedRequests {
  type Proxied = Self;
  fn as_view(&self) -> DroppedRequestsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DroppedRequests {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DroppedRequestsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DroppedRequests {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster_stats::envoy__config__endpoint__v3__ClusterStats__DroppedRequests_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X,P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster_stats::envoy__config__endpoint__v3__ClusterStats__DroppedRequests_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster_stats::envoy__config__endpoint__v3__ClusterStats__DroppedRequests_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DroppedRequests {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DroppedRequests {
  type Msg = DroppedRequests;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DroppedRequests> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DroppedRequests {
  type Msg = DroppedRequests;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DroppedRequests> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DroppedRequestsMut<'_> {
  type Msg = DroppedRequests;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DroppedRequests> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DroppedRequestsMut<'_> {
  type Msg = DroppedRequests;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DroppedRequests> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DroppedRequestsView<'_> {
  type Msg = DroppedRequests;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DroppedRequests> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DroppedRequestsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod cluster_stats


