const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__data__orca__v3__OrcaLoadReport_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OrcaLoadReport {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OrcaLoadReport>
}

impl ::protobuf::Message for OrcaLoadReport {
  type MessageView<'msg> = OrcaLoadReportView<'msg>;
  type MessageMut<'msg> = OrcaLoadReportMut<'msg>;
}

impl ::std::default::Default for OrcaLoadReport {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OrcaLoadReport {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OrcaLoadReport` is `Sync` because it does not implement interior mutability.
//    Neither does `OrcaLoadReportMut`.
unsafe impl ::std::marker::Sync for OrcaLoadReport {}

// SAFETY:
// - `OrcaLoadReport` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for OrcaLoadReport {}

impl ::protobuf::Proxied for OrcaLoadReport {
  type View<'msg> = OrcaLoadReportView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OrcaLoadReport {}

impl ::protobuf::MutProxied for OrcaLoadReport {
  type Mut<'msg> = OrcaLoadReportMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OrcaLoadReportView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OrcaLoadReport>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OrcaLoadReportView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OrcaLoadReportView<'msg> {
  type Message = OrcaLoadReport;
}

impl ::std::fmt::Debug for OrcaLoadReportView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OrcaLoadReportView<'_> {
  fn default() -> OrcaLoadReportView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OrcaLoadReport>> for OrcaLoadReportView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OrcaLoadReport>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OrcaLoadReportView<'msg> {

  pub fn to_owned(&self) -> OrcaLoadReport {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // cpu_utilization: optional double
  pub fn cpu_utilization(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // mem_utilization: optional double
  pub fn mem_utilization(self) -> f64 {
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

  // rps: optional uint64
  pub fn rps(self) -> u64 {
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

  // request_cost: repeated message xds.data.orca.v3.OrcaLoadReport.RequestCostEntry
  pub fn request_cost(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, f64> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, f64>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // utilization: repeated message xds.data.orca.v3.OrcaLoadReport.UtilizationEntry
  pub fn utilization(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, f64> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, f64>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // rps_fractional: optional double
  pub fn rps_fractional(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // eps: optional double
  pub fn eps(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        6, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // named_metrics: repeated message xds.data.orca.v3.OrcaLoadReport.NamedMetricsEntry
  pub fn named_metrics(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, f64> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(7)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, f64>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // application_utilization: optional double
  pub fn application_utilization(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        8, (0f64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `OrcaLoadReportView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OrcaLoadReportView<'_> {}

// SAFETY:
// - `OrcaLoadReportView` is `Send` because while its alive a `OrcaLoadReportMut` cannot.
// - `OrcaLoadReportView` does not use thread-local data.
unsafe impl ::std::marker::Send for OrcaLoadReportView<'_> {}

impl<'msg> ::protobuf::AsView for OrcaLoadReportView<'msg> {
  type Proxied = OrcaLoadReport;
  fn as_view(&self) -> ::protobuf::View<'msg, OrcaLoadReport> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OrcaLoadReportView<'msg> {
  fn into_view<'shorter>(self) -> OrcaLoadReportView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OrcaLoadReport> for OrcaLoadReportView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OrcaLoadReport {
    let mut dst = OrcaLoadReport::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OrcaLoadReport> for OrcaLoadReportMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OrcaLoadReport {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for OrcaLoadReport {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OrcaLoadReportView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OrcaLoadReportMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OrcaLoadReportMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OrcaLoadReport>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OrcaLoadReportMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OrcaLoadReportMut<'msg> {
  type Message = OrcaLoadReport;
}

impl ::std::fmt::Debug for OrcaLoadReportMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OrcaLoadReport>> for OrcaLoadReportMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OrcaLoadReport>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OrcaLoadReportMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OrcaLoadReport> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> OrcaLoadReport {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // cpu_utilization: optional double
  pub fn cpu_utilization(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_cpu_utilization(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        0, val.into()
      )
    }
  }

  // mem_utilization: optional double
  pub fn mem_utilization(&self) -> f64 {
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
  pub fn set_mem_utilization(&mut self, val: f64) {
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

  // rps: optional uint64
  pub fn rps(&self) -> u64 {
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
  pub fn set_rps(&mut self, val: u64) {
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

  // request_cost: repeated message xds.data.orca.v3.OrcaLoadReport.RequestCostEntry
  pub fn request_cost(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, f64>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn request_cost_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_request_cost(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // utilization: repeated message xds.data.orca.v3.OrcaLoadReport.UtilizationEntry
  pub fn utilization(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, f64>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn utilization_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          4, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_utilization(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // rps_fractional: optional double
  pub fn rps_fractional(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_rps_fractional(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        5, val.into()
      )
    }
  }

  // eps: optional double
  pub fn eps(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        6, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_eps(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        6, val.into()
      )
    }
  }

  // named_metrics: repeated message xds.data.orca.v3.OrcaLoadReport.NamedMetricsEntry
  pub fn named_metrics(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(7)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, f64>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn named_metrics_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          7, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_named_metrics(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // application_utilization: optional double
  pub fn application_utilization(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        8, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_application_utilization(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        8, val.into()
      )
    }
  }

}

// SAFETY:
// - `OrcaLoadReportMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OrcaLoadReportMut<'_> {}

// SAFETY:
// - `OrcaLoadReportMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OrcaLoadReportMut<'_> {}

impl<'msg> ::protobuf::AsView for OrcaLoadReportMut<'msg> {
  type Proxied = OrcaLoadReport;
  fn as_view(&self) -> ::protobuf::View<'_, OrcaLoadReport> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OrcaLoadReportMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OrcaLoadReport>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OrcaLoadReportMut<'msg> {
  type MutProxied = OrcaLoadReport;
  fn as_mut(&mut self) -> OrcaLoadReportMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OrcaLoadReportMut<'msg> {
  fn into_mut<'shorter>(self) -> OrcaLoadReportMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OrcaLoadReport {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OrcaLoadReport> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OrcaLoadReportView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OrcaLoadReportMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // cpu_utilization: optional double
  pub fn cpu_utilization(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_cpu_utilization(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        0, val.into()
      )
    }
  }

  // mem_utilization: optional double
  pub fn mem_utilization(&self) -> f64 {
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
  pub fn set_mem_utilization(&mut self, val: f64) {
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

  // rps: optional uint64
  pub fn rps(&self) -> u64 {
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
  pub fn set_rps(&mut self, val: u64) {
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

  // request_cost: repeated message xds.data.orca.v3.OrcaLoadReport.RequestCostEntry
  pub fn request_cost(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, f64>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn request_cost_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_request_cost(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // utilization: repeated message xds.data.orca.v3.OrcaLoadReport.UtilizationEntry
  pub fn utilization(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, f64>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn utilization_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          4, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_utilization(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // rps_fractional: optional double
  pub fn rps_fractional(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_rps_fractional(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        5, val.into()
      )
    }
  }

  // eps: optional double
  pub fn eps(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        6, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_eps(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        6, val.into()
      )
    }
  }

  // named_metrics: repeated message xds.data.orca.v3.OrcaLoadReport.NamedMetricsEntry
  pub fn named_metrics(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(7)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, f64>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn named_metrics_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, f64> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          7, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_named_metrics(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // application_utilization: optional double
  pub fn application_utilization(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        8, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_application_utilization(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        8, val.into()
      )
    }
  }

}  // impl OrcaLoadReport

impl ::std::ops::Drop for OrcaLoadReport {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OrcaLoadReport {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OrcaLoadReport {
  type Proxied = Self;
  fn as_view(&self) -> OrcaLoadReportView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OrcaLoadReport {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OrcaLoadReportMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OrcaLoadReport {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__data__orca__v3__OrcaLoadReport_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ P P,PGG P PG P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__data__orca__v3__OrcaLoadReport_msg_init.0, &[<super::orca_load_report::RequestCostEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::orca_load_report::UtilizationEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::orca_load_report::NamedMetricsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__data__orca__v3__OrcaLoadReport_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OrcaLoadReport {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OrcaLoadReport {
  type Msg = OrcaLoadReport;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrcaLoadReport> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrcaLoadReport {
  type Msg = OrcaLoadReport;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrcaLoadReport> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OrcaLoadReportMut<'_> {
  type Msg = OrcaLoadReport;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrcaLoadReport> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrcaLoadReportMut<'_> {
  type Msg = OrcaLoadReport;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrcaLoadReport> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrcaLoadReportView<'_> {
  type Msg = OrcaLoadReport;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrcaLoadReport> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OrcaLoadReportMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod orca_load_report {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__data__orca__v3__OrcaLoadReport__RequestCostEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct RequestCostEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RequestCostEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::orca_load_report::xds__data__orca__v3__OrcaLoadReport__RequestCostEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::orca_load_report::xds__data__orca__v3__OrcaLoadReport__RequestCostEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::orca_load_report::xds__data__orca__v3__OrcaLoadReport__RequestCostEntry_msg_init.0)
      }).0
    }
  }
}
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__data__orca__v3__OrcaLoadReport__UtilizationEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct UtilizationEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UtilizationEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::orca_load_report::xds__data__orca__v3__OrcaLoadReport__UtilizationEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::orca_load_report::xds__data__orca__v3__OrcaLoadReport__UtilizationEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::orca_load_report::xds__data__orca__v3__OrcaLoadReport__UtilizationEntry_msg_init.0)
      }).0
    }
  }
}
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__data__orca__v3__OrcaLoadReport__NamedMetricsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct NamedMetricsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NamedMetricsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::orca_load_report::xds__data__orca__v3__OrcaLoadReport__NamedMetricsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::orca_load_report::xds__data__orca__v3__OrcaLoadReport__NamedMetricsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::orca_load_report::xds__data__orca__v3__OrcaLoadReport__NamedMetricsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod orca_load_report


