const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HealthStatusSet_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HealthStatusSet {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HealthStatusSet>
}

impl ::protobuf::Message for HealthStatusSet {
  type MessageView<'msg> = HealthStatusSetView<'msg>;
  type MessageMut<'msg> = HealthStatusSetMut<'msg>;
}

impl ::std::default::Default for HealthStatusSet {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HealthStatusSet {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HealthStatusSet` is `Sync` because it does not implement interior mutability.
//    Neither does `HealthStatusSetMut`.
unsafe impl ::std::marker::Sync for HealthStatusSet {}

// SAFETY:
// - `HealthStatusSet` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HealthStatusSet {}

impl ::protobuf::Proxied for HealthStatusSet {
  type View<'msg> = HealthStatusSetView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HealthStatusSet {}

impl ::protobuf::MutProxied for HealthStatusSet {
  type Mut<'msg> = HealthStatusSetMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HealthStatusSetView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HealthStatusSet>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HealthStatusSetView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HealthStatusSetView<'msg> {
  type Message = HealthStatusSet;
}

impl ::std::fmt::Debug for HealthStatusSetView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HealthStatusSetView<'_> {
  fn default() -> HealthStatusSetView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HealthStatusSet>> for HealthStatusSetView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HealthStatusSet>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HealthStatusSetView<'msg> {

  pub fn to_owned(&self) -> HealthStatusSet {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // statuses: repeated enum envoy.config.core.v3.HealthStatus
  pub fn statuses(self) -> ::protobuf::RepeatedView<'msg, super::HealthStatus> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HealthStatus>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `HealthStatusSetView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HealthStatusSetView<'_> {}

// SAFETY:
// - `HealthStatusSetView` is `Send` because while its alive a `HealthStatusSetMut` cannot.
// - `HealthStatusSetView` does not use thread-local data.
unsafe impl ::std::marker::Send for HealthStatusSetView<'_> {}

impl<'msg> ::protobuf::AsView for HealthStatusSetView<'msg> {
  type Proxied = HealthStatusSet;
  fn as_view(&self) -> ::protobuf::View<'msg, HealthStatusSet> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HealthStatusSetView<'msg> {
  fn into_view<'shorter>(self) -> HealthStatusSetView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HealthStatusSet> for HealthStatusSetView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HealthStatusSet {
    let mut dst = HealthStatusSet::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HealthStatusSet> for HealthStatusSetMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HealthStatusSet {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HealthStatusSet {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HealthStatusSetView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HealthStatusSetMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HealthStatusSetMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HealthStatusSet>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HealthStatusSetMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HealthStatusSetMut<'msg> {
  type Message = HealthStatusSet;
}

impl ::std::fmt::Debug for HealthStatusSetMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HealthStatusSet>> for HealthStatusSetMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HealthStatusSet>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HealthStatusSetMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HealthStatusSet> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HealthStatusSet {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // statuses: repeated enum envoy.config.core.v3.HealthStatus
  pub fn statuses(&self) -> ::protobuf::RepeatedView<'_, super::HealthStatus> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HealthStatus>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn statuses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HealthStatus> {
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
  pub fn set_statuses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HealthStatus>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `HealthStatusSetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HealthStatusSetMut<'_> {}

// SAFETY:
// - `HealthStatusSetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HealthStatusSetMut<'_> {}

impl<'msg> ::protobuf::AsView for HealthStatusSetMut<'msg> {
  type Proxied = HealthStatusSet;
  fn as_view(&self) -> ::protobuf::View<'_, HealthStatusSet> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HealthStatusSetMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HealthStatusSet>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HealthStatusSetMut<'msg> {
  type MutProxied = HealthStatusSet;
  fn as_mut(&mut self) -> HealthStatusSetMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HealthStatusSetMut<'msg> {
  fn into_mut<'shorter>(self) -> HealthStatusSetMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HealthStatusSet {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HealthStatusSet> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HealthStatusSetView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HealthStatusSetMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // statuses: repeated enum envoy.config.core.v3.HealthStatus
  pub fn statuses(&self) -> ::protobuf::RepeatedView<'_, super::HealthStatus> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HealthStatus>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn statuses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HealthStatus> {
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
  pub fn set_statuses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HealthStatus>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl HealthStatusSet

impl ::std::ops::Drop for HealthStatusSet {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HealthStatusSet {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HealthStatusSet {
  type Proxied = Self;
  fn as_view(&self) -> HealthStatusSetView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HealthStatusSet {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HealthStatusSetMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HealthStatusSet {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__HealthStatusSet_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$NB");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__HealthStatusSet_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__HealthStatusSet_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HealthStatusSet {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HealthStatusSet {
  type Msg = HealthStatusSet;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthStatusSet> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HealthStatusSet {
  type Msg = HealthStatusSet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthStatusSet> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HealthStatusSetMut<'_> {
  type Msg = HealthStatusSet;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthStatusSet> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HealthStatusSetMut<'_> {
  type Msg = HealthStatusSet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthStatusSet> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HealthStatusSetView<'_> {
  type Msg = HealthStatusSet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthStatusSet> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HealthStatusSetMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HealthCheck_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HealthCheck {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HealthCheck>
}

impl ::protobuf::Message for HealthCheck {
  type MessageView<'msg> = HealthCheckView<'msg>;
  type MessageMut<'msg> = HealthCheckMut<'msg>;
}

impl ::std::default::Default for HealthCheck {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HealthCheck {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HealthCheck` is `Sync` because it does not implement interior mutability.
//    Neither does `HealthCheckMut`.
unsafe impl ::std::marker::Sync for HealthCheck {}

// SAFETY:
// - `HealthCheck` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HealthCheck {}

impl ::protobuf::Proxied for HealthCheck {
  type View<'msg> = HealthCheckView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HealthCheck {}

impl ::protobuf::MutProxied for HealthCheck {
  type Mut<'msg> = HealthCheckMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HealthCheckView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HealthCheckView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HealthCheckView<'msg> {
  type Message = HealthCheck;
}

impl ::std::fmt::Debug for HealthCheckView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HealthCheckView<'_> {
  fn default() -> HealthCheckView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HealthCheck>> for HealthCheckView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HealthCheckView<'msg> {

  pub fn to_owned(&self) -> HealthCheck {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // interval: optional message google.protobuf.Duration
  pub fn has_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_interval().then(|| self.interval())
  }
  pub fn interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // initial_jitter: optional message google.protobuf.Duration
  pub fn has_initial_jitter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn initial_jitter_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_initial_jitter().then(|| self.initial_jitter())
  }
  pub fn initial_jitter(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // interval_jitter: optional message google.protobuf.Duration
  pub fn has_interval_jitter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn interval_jitter_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_interval_jitter().then(|| self.interval_jitter())
  }
  pub fn interval_jitter(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // interval_jitter_percent: optional uint32
  pub fn interval_jitter_percent(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        16, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // unhealthy_threshold: optional message google.protobuf.UInt32Value
  pub fn has_unhealthy_threshold(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn unhealthy_threshold_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_unhealthy_threshold().then(|| self.unhealthy_threshold())
  }
  pub fn unhealthy_threshold(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // healthy_threshold: optional message google.protobuf.UInt32Value
  pub fn has_healthy_threshold(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn healthy_threshold_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_healthy_threshold().then(|| self.healthy_threshold())
  }
  pub fn healthy_threshold(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // alt_port: optional message google.protobuf.UInt32Value
  pub fn has_alt_port(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn alt_port_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_alt_port().then(|| self.alt_port())
  }
  pub fn alt_port(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // reuse_connection: optional message google.protobuf.BoolValue
  pub fn has_reuse_connection(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn reuse_connection_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_reuse_connection().then(|| self.reuse_connection())
  }
  pub fn reuse_connection(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // http_health_check: optional message envoy.config.core.v3.HealthCheck.HttpHealthCheck
  pub fn has_http_health_check(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn http_health_check_opt(self) -> ::std::option::Option<super::health_check::HttpHealthCheckView<'msg>> {
    self.has_http_health_check().then(|| self.http_health_check())
  }
  pub fn http_health_check(self) -> super::health_check::HttpHealthCheckView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::HttpHealthCheckView::default())
  }

  // tcp_health_check: optional message envoy.config.core.v3.HealthCheck.TcpHealthCheck
  pub fn has_tcp_health_check(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn tcp_health_check_opt(self) -> ::std::option::Option<super::health_check::TcpHealthCheckView<'msg>> {
    self.has_tcp_health_check().then(|| self.tcp_health_check())
  }
  pub fn tcp_health_check(self) -> super::health_check::TcpHealthCheckView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::TcpHealthCheckView::default())
  }

  // grpc_health_check: optional message envoy.config.core.v3.HealthCheck.GrpcHealthCheck
  pub fn has_grpc_health_check(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn grpc_health_check_opt(self) -> ::std::option::Option<super::health_check::GrpcHealthCheckView<'msg>> {
    self.has_grpc_health_check().then(|| self.grpc_health_check())
  }
  pub fn grpc_health_check(self) -> super::health_check::GrpcHealthCheckView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::GrpcHealthCheckView::default())
  }

  // custom_health_check: optional message envoy.config.core.v3.HealthCheck.CustomHealthCheck
  pub fn has_custom_health_check(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn custom_health_check_opt(self) -> ::std::option::Option<super::health_check::CustomHealthCheckView<'msg>> {
    self.has_custom_health_check().then(|| self.custom_health_check())
  }
  pub fn custom_health_check(self) -> super::health_check::CustomHealthCheckView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::CustomHealthCheckView::default())
  }

  // no_traffic_interval: optional message google.protobuf.Duration
  pub fn has_no_traffic_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn no_traffic_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_no_traffic_interval().then(|| self.no_traffic_interval())
  }
  pub fn no_traffic_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // no_traffic_healthy_interval: optional message google.protobuf.Duration
  pub fn has_no_traffic_healthy_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn no_traffic_healthy_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_no_traffic_healthy_interval().then(|| self.no_traffic_healthy_interval())
  }
  pub fn no_traffic_healthy_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // unhealthy_interval: optional message google.protobuf.Duration
  pub fn has_unhealthy_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn unhealthy_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_unhealthy_interval().then(|| self.unhealthy_interval())
  }
  pub fn unhealthy_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // unhealthy_edge_interval: optional message google.protobuf.Duration
  pub fn has_unhealthy_edge_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn unhealthy_edge_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_unhealthy_edge_interval().then(|| self.unhealthy_edge_interval())
  }
  pub fn unhealthy_edge_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // healthy_edge_interval: optional message google.protobuf.Duration
  pub fn has_healthy_edge_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn healthy_edge_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_healthy_edge_interval().then(|| self.healthy_edge_interval())
  }
  pub fn healthy_edge_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // event_log_path: optional string
  pub fn event_log_path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        15, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // event_logger: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn event_logger(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        23
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // event_service: optional message envoy.config.core.v3.EventServiceConfig
  pub fn has_event_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn event_service_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'msg>> {
    self.has_event_service().then(|| self.event_service())
  }
  pub fn event_service(self) -> crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView::default())
  }

  // always_log_health_check_failures: optional bool
  pub fn always_log_health_check_failures(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }

  // always_log_health_check_success: optional bool
  pub fn always_log_health_check_success(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        24, (false).into()
      ).try_into().unwrap()
    }
  }

  // tls_options: optional message envoy.config.core.v3.HealthCheck.TlsOptions
  pub fn has_tls_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn tls_options_opt(self) -> ::std::option::Option<super::health_check::TlsOptionsView<'msg>> {
    self.has_tls_options().then(|| self.tls_options())
  }
  pub fn tls_options(self) -> super::health_check::TlsOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::TlsOptionsView::default())
  }

  // transport_socket_match_criteria: optional message google.protobuf.Struct
  pub fn has_transport_socket_match_criteria(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn transport_socket_match_criteria_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_transport_socket_match_criteria().then(|| self.transport_socket_match_criteria())
  }
  pub fn transport_socket_match_criteria(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  pub fn health_checker(self) -> super::health_check::HealthCheckerOneof<'msg> {
    match self.health_checker_case() {
      super::health_check::HealthCheckerCase::HttpHealthCheck =>
          super::health_check::HealthCheckerOneof::HttpHealthCheck(self.http_health_check()),
      super::health_check::HealthCheckerCase::TcpHealthCheck =>
          super::health_check::HealthCheckerOneof::TcpHealthCheck(self.tcp_health_check()),
      super::health_check::HealthCheckerCase::GrpcHealthCheck =>
          super::health_check::HealthCheckerOneof::GrpcHealthCheck(self.grpc_health_check()),
      super::health_check::HealthCheckerCase::CustomHealthCheck =>
          super::health_check::HealthCheckerOneof::CustomHealthCheck(self.custom_health_check()),
      _ => super::health_check::HealthCheckerOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn health_checker_case(self) -> super::health_check::HealthCheckerCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(7);
      super::health_check::HealthCheckerCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HealthCheckView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HealthCheckView<'_> {}

// SAFETY:
// - `HealthCheckView` is `Send` because while its alive a `HealthCheckMut` cannot.
// - `HealthCheckView` does not use thread-local data.
unsafe impl ::std::marker::Send for HealthCheckView<'_> {}

impl<'msg> ::protobuf::AsView for HealthCheckView<'msg> {
  type Proxied = HealthCheck;
  fn as_view(&self) -> ::protobuf::View<'msg, HealthCheck> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HealthCheckView<'msg> {
  fn into_view<'shorter>(self) -> HealthCheckView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HealthCheck> for HealthCheckView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HealthCheck {
    let mut dst = HealthCheck::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HealthCheck> for HealthCheckMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HealthCheck {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HealthCheck {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HealthCheckView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HealthCheckMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HealthCheckMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HealthCheckMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HealthCheckMut<'msg> {
  type Message = HealthCheck;
}

impl ::std::fmt::Debug for HealthCheckMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HealthCheck>> for HealthCheckMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HealthCheckMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HealthCheck> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HealthCheck {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // interval: optional message google.protobuf.Duration
  pub fn has_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_interval().then(|| self.interval())
  }
  pub fn interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // initial_jitter: optional message google.protobuf.Duration
  pub fn has_initial_jitter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_initial_jitter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn initial_jitter_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_initial_jitter().then(|| self.initial_jitter())
  }
  pub fn initial_jitter(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn initial_jitter_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_initial_jitter(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
    }
  }

  // interval_jitter: optional message google.protobuf.Duration
  pub fn has_interval_jitter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_interval_jitter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn interval_jitter_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_interval_jitter().then(|| self.interval_jitter())
  }
  pub fn interval_jitter(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn interval_jitter_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_interval_jitter(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // interval_jitter_percent: optional uint32
  pub fn interval_jitter_percent(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        16, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_interval_jitter_percent(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        16, val.into()
      )
    }
  }

  // unhealthy_threshold: optional message google.protobuf.UInt32Value
  pub fn has_unhealthy_threshold(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_unhealthy_threshold(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn unhealthy_threshold_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_unhealthy_threshold().then(|| self.unhealthy_threshold())
  }
  pub fn unhealthy_threshold(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn unhealthy_threshold_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_unhealthy_threshold(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // healthy_threshold: optional message google.protobuf.UInt32Value
  pub fn has_healthy_threshold(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_healthy_threshold(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn healthy_threshold_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_healthy_threshold().then(|| self.healthy_threshold())
  }
  pub fn healthy_threshold(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn healthy_threshold_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_healthy_threshold(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // alt_port: optional message google.protobuf.UInt32Value
  pub fn has_alt_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_alt_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn alt_port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_alt_port().then(|| self.alt_port())
  }
  pub fn alt_port(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn alt_port_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_alt_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // reuse_connection: optional message google.protobuf.BoolValue
  pub fn has_reuse_connection(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_reuse_connection(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn reuse_connection_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_reuse_connection().then(|| self.reuse_connection())
  }
  pub fn reuse_connection(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn reuse_connection_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_reuse_connection(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // http_health_check: optional message envoy.config.core.v3.HealthCheck.HttpHealthCheck
  pub fn has_http_health_check(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_http_health_check(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn http_health_check_opt(&self) -> ::std::option::Option<super::health_check::HttpHealthCheckView<'_>> {
    self.has_http_health_check().then(|| self.http_health_check())
  }
  pub fn http_health_check(&self) -> super::health_check::HttpHealthCheckView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::HttpHealthCheckView::default())
  }
  pub fn http_health_check_mut(&mut self) -> super::health_check::HttpHealthCheckMut<'_> {
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
  pub fn set_http_health_check(&mut self,
    val: impl ::protobuf::IntoProxied<super::health_check::HttpHealthCheck>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // tcp_health_check: optional message envoy.config.core.v3.HealthCheck.TcpHealthCheck
  pub fn has_tcp_health_check(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_tcp_health_check(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn tcp_health_check_opt(&self) -> ::std::option::Option<super::health_check::TcpHealthCheckView<'_>> {
    self.has_tcp_health_check().then(|| self.tcp_health_check())
  }
  pub fn tcp_health_check(&self) -> super::health_check::TcpHealthCheckView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::TcpHealthCheckView::default())
  }
  pub fn tcp_health_check_mut(&mut self) -> super::health_check::TcpHealthCheckMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tcp_health_check(&mut self,
    val: impl ::protobuf::IntoProxied<super::health_check::TcpHealthCheck>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // grpc_health_check: optional message envoy.config.core.v3.HealthCheck.GrpcHealthCheck
  pub fn has_grpc_health_check(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_grpc_health_check(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn grpc_health_check_opt(&self) -> ::std::option::Option<super::health_check::GrpcHealthCheckView<'_>> {
    self.has_grpc_health_check().then(|| self.grpc_health_check())
  }
  pub fn grpc_health_check(&self) -> super::health_check::GrpcHealthCheckView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::GrpcHealthCheckView::default())
  }
  pub fn grpc_health_check_mut(&mut self) -> super::health_check::GrpcHealthCheckMut<'_> {
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
  pub fn set_grpc_health_check(&mut self,
    val: impl ::protobuf::IntoProxied<super::health_check::GrpcHealthCheck>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // custom_health_check: optional message envoy.config.core.v3.HealthCheck.CustomHealthCheck
  pub fn has_custom_health_check(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_custom_health_check(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn custom_health_check_opt(&self) -> ::std::option::Option<super::health_check::CustomHealthCheckView<'_>> {
    self.has_custom_health_check().then(|| self.custom_health_check())
  }
  pub fn custom_health_check(&self) -> super::health_check::CustomHealthCheckView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::CustomHealthCheckView::default())
  }
  pub fn custom_health_check_mut(&mut self) -> super::health_check::CustomHealthCheckMut<'_> {
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
  pub fn set_custom_health_check(&mut self,
    val: impl ::protobuf::IntoProxied<super::health_check::CustomHealthCheck>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // no_traffic_interval: optional message google.protobuf.Duration
  pub fn has_no_traffic_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_no_traffic_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn no_traffic_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_no_traffic_interval().then(|| self.no_traffic_interval())
  }
  pub fn no_traffic_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn no_traffic_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_no_traffic_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // no_traffic_healthy_interval: optional message google.protobuf.Duration
  pub fn has_no_traffic_healthy_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_no_traffic_healthy_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn no_traffic_healthy_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_no_traffic_healthy_interval().then(|| self.no_traffic_healthy_interval())
  }
  pub fn no_traffic_healthy_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn no_traffic_healthy_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_no_traffic_healthy_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // unhealthy_interval: optional message google.protobuf.Duration
  pub fn has_unhealthy_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_unhealthy_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn unhealthy_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_unhealthy_interval().then(|| self.unhealthy_interval())
  }
  pub fn unhealthy_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn unhealthy_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_unhealthy_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // unhealthy_edge_interval: optional message google.protobuf.Duration
  pub fn has_unhealthy_edge_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_unhealthy_edge_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn unhealthy_edge_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_unhealthy_edge_interval().then(|| self.unhealthy_edge_interval())
  }
  pub fn unhealthy_edge_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn unhealthy_edge_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_unhealthy_edge_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // healthy_edge_interval: optional message google.protobuf.Duration
  pub fn has_healthy_edge_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_healthy_edge_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn healthy_edge_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_healthy_edge_interval().then(|| self.healthy_edge_interval())
  }
  pub fn healthy_edge_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn healthy_edge_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_healthy_edge_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // event_log_path: optional string
  pub fn event_log_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        15, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_event_log_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val);
    }
  }

  // event_logger: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn event_logger(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        23
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn event_logger_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        23,
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
  pub fn set_event_logger(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        src);
    }
  }

  // event_service: optional message envoy.config.core.v3.EventServiceConfig
  pub fn has_event_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn clear_event_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        20
      );
    }
  }
  pub fn event_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'_>> {
    self.has_event_service().then(|| self.event_service())
  }
  pub fn event_service(&self) -> crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView::default())
  }
  pub fn event_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         20, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_event_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        val
      );
    }
  }

  // always_log_health_check_failures: optional bool
  pub fn always_log_health_check_failures(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_always_log_health_check_failures(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        17, val.into()
      )
    }
  }

  // always_log_health_check_success: optional bool
  pub fn always_log_health_check_success(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        24, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_always_log_health_check_success(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        24, val.into()
      )
    }
  }

  // tls_options: optional message envoy.config.core.v3.HealthCheck.TlsOptions
  pub fn has_tls_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn clear_tls_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        19
      );
    }
  }
  pub fn tls_options_opt(&self) -> ::std::option::Option<super::health_check::TlsOptionsView<'_>> {
    self.has_tls_options().then(|| self.tls_options())
  }
  pub fn tls_options(&self) -> super::health_check::TlsOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::TlsOptionsView::default())
  }
  pub fn tls_options_mut(&mut self) -> super::health_check::TlsOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         19, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tls_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::health_check::TlsOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        val
      );
    }
  }

  // transport_socket_match_criteria: optional message google.protobuf.Struct
  pub fn has_transport_socket_match_criteria(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_transport_socket_match_criteria(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn transport_socket_match_criteria_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_transport_socket_match_criteria().then(|| self.transport_socket_match_criteria())
  }
  pub fn transport_socket_match_criteria(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn transport_socket_match_criteria_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_transport_socket_match_criteria(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val
      );
    }
  }

  pub fn health_checker(&self) -> super::health_check::HealthCheckerOneof<'_> {
    match &self.health_checker_case() {
      super::health_check::HealthCheckerCase::HttpHealthCheck =>
          super::health_check::HealthCheckerOneof::HttpHealthCheck(self.http_health_check()),
      super::health_check::HealthCheckerCase::TcpHealthCheck =>
          super::health_check::HealthCheckerOneof::TcpHealthCheck(self.tcp_health_check()),
      super::health_check::HealthCheckerCase::GrpcHealthCheck =>
          super::health_check::HealthCheckerOneof::GrpcHealthCheck(self.grpc_health_check()),
      super::health_check::HealthCheckerCase::CustomHealthCheck =>
          super::health_check::HealthCheckerOneof::CustomHealthCheck(self.custom_health_check()),
      _ => super::health_check::HealthCheckerOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn health_checker_case(&self) -> super::health_check::HealthCheckerCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(7);
      super::health_check::HealthCheckerCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HealthCheckMut<'_> {}

// SAFETY:
// - `HealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HealthCheckMut<'_> {}

impl<'msg> ::protobuf::AsView for HealthCheckMut<'msg> {
  type Proxied = HealthCheck;
  fn as_view(&self) -> ::protobuf::View<'_, HealthCheck> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HealthCheckMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HealthCheck>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HealthCheckMut<'msg> {
  type MutProxied = HealthCheck;
  fn as_mut(&mut self) -> HealthCheckMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HealthCheckMut<'msg> {
  fn into_mut<'shorter>(self) -> HealthCheckMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HealthCheck {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HealthCheck> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HealthCheckView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HealthCheckMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // interval: optional message google.protobuf.Duration
  pub fn has_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_interval().then(|| self.interval())
  }
  pub fn interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // initial_jitter: optional message google.protobuf.Duration
  pub fn has_initial_jitter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_initial_jitter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn initial_jitter_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_initial_jitter().then(|| self.initial_jitter())
  }
  pub fn initial_jitter(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn initial_jitter_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_initial_jitter(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
    }
  }

  // interval_jitter: optional message google.protobuf.Duration
  pub fn has_interval_jitter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_interval_jitter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn interval_jitter_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_interval_jitter().then(|| self.interval_jitter())
  }
  pub fn interval_jitter(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn interval_jitter_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_interval_jitter(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // interval_jitter_percent: optional uint32
  pub fn interval_jitter_percent(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        16, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_interval_jitter_percent(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        16, val.into()
      )
    }
  }

  // unhealthy_threshold: optional message google.protobuf.UInt32Value
  pub fn has_unhealthy_threshold(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_unhealthy_threshold(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn unhealthy_threshold_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_unhealthy_threshold().then(|| self.unhealthy_threshold())
  }
  pub fn unhealthy_threshold(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn unhealthy_threshold_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_unhealthy_threshold(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // healthy_threshold: optional message google.protobuf.UInt32Value
  pub fn has_healthy_threshold(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_healthy_threshold(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn healthy_threshold_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_healthy_threshold().then(|| self.healthy_threshold())
  }
  pub fn healthy_threshold(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn healthy_threshold_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_healthy_threshold(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // alt_port: optional message google.protobuf.UInt32Value
  pub fn has_alt_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_alt_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn alt_port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_alt_port().then(|| self.alt_port())
  }
  pub fn alt_port(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn alt_port_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_alt_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // reuse_connection: optional message google.protobuf.BoolValue
  pub fn has_reuse_connection(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_reuse_connection(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn reuse_connection_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_reuse_connection().then(|| self.reuse_connection())
  }
  pub fn reuse_connection(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn reuse_connection_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_reuse_connection(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // http_health_check: optional message envoy.config.core.v3.HealthCheck.HttpHealthCheck
  pub fn has_http_health_check(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_http_health_check(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn http_health_check_opt(&self) -> ::std::option::Option<super::health_check::HttpHealthCheckView<'_>> {
    self.has_http_health_check().then(|| self.http_health_check())
  }
  pub fn http_health_check(&self) -> super::health_check::HttpHealthCheckView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::HttpHealthCheckView::default())
  }
  pub fn http_health_check_mut(&mut self) -> super::health_check::HttpHealthCheckMut<'_> {
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
  pub fn set_http_health_check(&mut self,
    val: impl ::protobuf::IntoProxied<super::health_check::HttpHealthCheck>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // tcp_health_check: optional message envoy.config.core.v3.HealthCheck.TcpHealthCheck
  pub fn has_tcp_health_check(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_tcp_health_check(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn tcp_health_check_opt(&self) -> ::std::option::Option<super::health_check::TcpHealthCheckView<'_>> {
    self.has_tcp_health_check().then(|| self.tcp_health_check())
  }
  pub fn tcp_health_check(&self) -> super::health_check::TcpHealthCheckView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::TcpHealthCheckView::default())
  }
  pub fn tcp_health_check_mut(&mut self) -> super::health_check::TcpHealthCheckMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tcp_health_check(&mut self,
    val: impl ::protobuf::IntoProxied<super::health_check::TcpHealthCheck>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // grpc_health_check: optional message envoy.config.core.v3.HealthCheck.GrpcHealthCheck
  pub fn has_grpc_health_check(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_grpc_health_check(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn grpc_health_check_opt(&self) -> ::std::option::Option<super::health_check::GrpcHealthCheckView<'_>> {
    self.has_grpc_health_check().then(|| self.grpc_health_check())
  }
  pub fn grpc_health_check(&self) -> super::health_check::GrpcHealthCheckView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::GrpcHealthCheckView::default())
  }
  pub fn grpc_health_check_mut(&mut self) -> super::health_check::GrpcHealthCheckMut<'_> {
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
  pub fn set_grpc_health_check(&mut self,
    val: impl ::protobuf::IntoProxied<super::health_check::GrpcHealthCheck>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // custom_health_check: optional message envoy.config.core.v3.HealthCheck.CustomHealthCheck
  pub fn has_custom_health_check(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_custom_health_check(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn custom_health_check_opt(&self) -> ::std::option::Option<super::health_check::CustomHealthCheckView<'_>> {
    self.has_custom_health_check().then(|| self.custom_health_check())
  }
  pub fn custom_health_check(&self) -> super::health_check::CustomHealthCheckView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::CustomHealthCheckView::default())
  }
  pub fn custom_health_check_mut(&mut self) -> super::health_check::CustomHealthCheckMut<'_> {
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
  pub fn set_custom_health_check(&mut self,
    val: impl ::protobuf::IntoProxied<super::health_check::CustomHealthCheck>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // no_traffic_interval: optional message google.protobuf.Duration
  pub fn has_no_traffic_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_no_traffic_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn no_traffic_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_no_traffic_interval().then(|| self.no_traffic_interval())
  }
  pub fn no_traffic_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn no_traffic_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_no_traffic_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // no_traffic_healthy_interval: optional message google.protobuf.Duration
  pub fn has_no_traffic_healthy_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_no_traffic_healthy_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn no_traffic_healthy_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_no_traffic_healthy_interval().then(|| self.no_traffic_healthy_interval())
  }
  pub fn no_traffic_healthy_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn no_traffic_healthy_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_no_traffic_healthy_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // unhealthy_interval: optional message google.protobuf.Duration
  pub fn has_unhealthy_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_unhealthy_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn unhealthy_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_unhealthy_interval().then(|| self.unhealthy_interval())
  }
  pub fn unhealthy_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn unhealthy_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_unhealthy_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // unhealthy_edge_interval: optional message google.protobuf.Duration
  pub fn has_unhealthy_edge_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_unhealthy_edge_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn unhealthy_edge_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_unhealthy_edge_interval().then(|| self.unhealthy_edge_interval())
  }
  pub fn unhealthy_edge_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn unhealthy_edge_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_unhealthy_edge_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // healthy_edge_interval: optional message google.protobuf.Duration
  pub fn has_healthy_edge_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_healthy_edge_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn healthy_edge_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_healthy_edge_interval().then(|| self.healthy_edge_interval())
  }
  pub fn healthy_edge_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn healthy_edge_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_healthy_edge_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // event_log_path: optional string
  pub fn event_log_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        15, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_event_log_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val);
    }
  }

  // event_logger: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn event_logger(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        23
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn event_logger_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        23,
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
  pub fn set_event_logger(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        src);
    }
  }

  // event_service: optional message envoy.config.core.v3.EventServiceConfig
  pub fn has_event_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn clear_event_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        20
      );
    }
  }
  pub fn event_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'_>> {
    self.has_event_service().then(|| self.event_service())
  }
  pub fn event_service(&self) -> crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView::default())
  }
  pub fn event_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         20, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_event_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        val
      );
    }
  }

  // always_log_health_check_failures: optional bool
  pub fn always_log_health_check_failures(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_always_log_health_check_failures(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        17, val.into()
      )
    }
  }

  // always_log_health_check_success: optional bool
  pub fn always_log_health_check_success(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        24, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_always_log_health_check_success(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        24, val.into()
      )
    }
  }

  // tls_options: optional message envoy.config.core.v3.HealthCheck.TlsOptions
  pub fn has_tls_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn clear_tls_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        19
      );
    }
  }
  pub fn tls_options_opt(&self) -> ::std::option::Option<super::health_check::TlsOptionsView<'_>> {
    self.has_tls_options().then(|| self.tls_options())
  }
  pub fn tls_options(&self) -> super::health_check::TlsOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::health_check::TlsOptionsView::default())
  }
  pub fn tls_options_mut(&mut self) -> super::health_check::TlsOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         19, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tls_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::health_check::TlsOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        val
      );
    }
  }

  // transport_socket_match_criteria: optional message google.protobuf.Struct
  pub fn has_transport_socket_match_criteria(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_transport_socket_match_criteria(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn transport_socket_match_criteria_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_transport_socket_match_criteria().then(|| self.transport_socket_match_criteria())
  }
  pub fn transport_socket_match_criteria(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn transport_socket_match_criteria_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_transport_socket_match_criteria(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val
      );
    }
  }

  pub fn health_checker(&self) -> super::health_check::HealthCheckerOneof<'_> {
    match &self.health_checker_case() {
      super::health_check::HealthCheckerCase::HttpHealthCheck =>
          super::health_check::HealthCheckerOneof::HttpHealthCheck(self.http_health_check()),
      super::health_check::HealthCheckerCase::TcpHealthCheck =>
          super::health_check::HealthCheckerOneof::TcpHealthCheck(self.tcp_health_check()),
      super::health_check::HealthCheckerCase::GrpcHealthCheck =>
          super::health_check::HealthCheckerOneof::GrpcHealthCheck(self.grpc_health_check()),
      super::health_check::HealthCheckerCase::CustomHealthCheck =>
          super::health_check::HealthCheckerOneof::CustomHealthCheck(self.custom_health_check()),
      _ => super::health_check::HealthCheckerOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn health_checker_case(&self) -> super::health_check::HealthCheckerCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(7);
      super::health_check::HealthCheckerCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl HealthCheck

impl ::std::ops::Drop for HealthCheck {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HealthCheck {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HealthCheck {
  type Proxied = Self;
  fn as_view(&self) -> HealthCheckView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HealthCheck {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HealthCheckMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HealthCheck {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__HealthCheck_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333333333a3333331X)P/P33333G/P^*|+|-|/");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__HealthCheck_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::health_check::HttpHealthCheck as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::health_check::TcpHealthCheck as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::health_check::GrpcHealthCheck as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::health_check::CustomHealthCheck as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::health_check::TlsOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__HealthCheck_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HealthCheck {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HealthCheck {
  type Msg = HealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HealthCheck {
  type Msg = HealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HealthCheckMut<'_> {
  type Msg = HealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HealthCheckMut<'_> {
  type Msg = HealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HealthCheckView<'_> {
  type Msg = HealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HealthCheck> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HealthCheckMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod health_check {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HealthCheck__Payload_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Payload {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Payload>
}

impl ::protobuf::Message for Payload {
  type MessageView<'msg> = PayloadView<'msg>;
  type MessageMut<'msg> = PayloadMut<'msg>;
}

impl ::std::default::Default for Payload {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Payload {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Payload` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadMut`.
unsafe impl ::std::marker::Sync for Payload {}

// SAFETY:
// - `Payload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Payload {}

impl ::protobuf::Proxied for Payload {
  type View<'msg> = PayloadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Payload {}

impl ::protobuf::MutProxied for Payload {
  type Mut<'msg> = PayloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Payload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadView<'msg> {
  type Message = Payload;
}

impl ::std::fmt::Debug for PayloadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PayloadView<'_> {
  fn default() -> PayloadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Payload>> for PayloadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Payload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PayloadView<'msg> {

  pub fn to_owned(&self) -> Payload {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // text: optional string
  pub fn has_text(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn text_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_text().then(|| self.text())
  }
  pub fn text(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // binary: optional bytes
  pub fn has_binary(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn binary_opt(self) -> ::std::option::Option<&'msg [u8]> {
    self.has_binary().then(|| self.binary())
  }
  pub fn binary(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  pub fn payload(self) -> super::super::health_check::payload::PayloadOneof<'msg> {
    match self.payload_case() {
      super::super::health_check::payload::PayloadCase::Text =>
          super::super::health_check::payload::PayloadOneof::Text(self.text()),
      super::super::health_check::payload::PayloadCase::Binary =>
          super::super::health_check::payload::PayloadOneof::Binary(self.binary()),
      _ => super::super::health_check::payload::PayloadOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn payload_case(self) -> super::super::health_check::payload::PayloadCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::health_check::payload::PayloadCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PayloadView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PayloadView<'_> {}

// SAFETY:
// - `PayloadView` is `Send` because while its alive a `PayloadMut` cannot.
// - `PayloadView` does not use thread-local data.
unsafe impl ::std::marker::Send for PayloadView<'_> {}

impl<'msg> ::protobuf::AsView for PayloadView<'msg> {
  type Proxied = Payload;
  fn as_view(&self) -> ::protobuf::View<'msg, Payload> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadView<'msg> {
  fn into_view<'shorter>(self) -> PayloadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Payload> for PayloadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Payload {
    let mut dst = Payload::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Payload> for PayloadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Payload {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Payload {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PayloadView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PayloadMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Payload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadMut<'msg> {
  type Message = Payload;
}

impl ::std::fmt::Debug for PayloadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Payload>> for PayloadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Payload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PayloadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Payload> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Payload {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // text: optional string
  pub fn has_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn text_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_text().then(|| self.text())
  }
  pub fn text(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_text(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // binary: optional bytes
  pub fn has_binary(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_binary(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn binary_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_binary().then(|| self.binary())
  }
  pub fn binary(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_binary(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn payload(&self) -> super::super::health_check::payload::PayloadOneof<'_> {
    match &self.payload_case() {
      super::super::health_check::payload::PayloadCase::Text =>
          super::super::health_check::payload::PayloadOneof::Text(self.text()),
      super::super::health_check::payload::PayloadCase::Binary =>
          super::super::health_check::payload::PayloadOneof::Binary(self.binary()),
      _ => super::super::health_check::payload::PayloadOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn payload_case(&self) -> super::super::health_check::payload::PayloadCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::health_check::payload::PayloadCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PayloadMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PayloadMut<'_> {}

// SAFETY:
// - `PayloadMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PayloadMut<'_> {}

impl<'msg> ::protobuf::AsView for PayloadMut<'msg> {
  type Proxied = Payload;
  fn as_view(&self) -> ::protobuf::View<'_, Payload> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Payload>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PayloadMut<'msg> {
  type MutProxied = Payload;
  fn as_mut(&mut self) -> PayloadMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Payload {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Payload> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PayloadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PayloadMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // text: optional string
  pub fn has_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn text_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_text().then(|| self.text())
  }
  pub fn text(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_text(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // binary: optional bytes
  pub fn has_binary(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_binary(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn binary_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_binary().then(|| self.binary())
  }
  pub fn binary(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_binary(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn payload(&self) -> super::super::health_check::payload::PayloadOneof<'_> {
    match &self.payload_case() {
      super::super::health_check::payload::PayloadCase::Text =>
          super::super::health_check::payload::PayloadOneof::Text(self.text()),
      super::super::health_check::payload::PayloadCase::Binary =>
          super::super::health_check::payload::PayloadOneof::Binary(self.binary()),
      _ => super::super::health_check::payload::PayloadOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn payload_case(&self) -> super::super::health_check::payload::PayloadCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::health_check::payload::PayloadCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Payload

impl ::std::ops::Drop for Payload {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Payload {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Payload {
  type Proxied = Self;
  fn as_view(&self) -> PayloadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Payload {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Payload {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::health_check::envoy__config__core__v3__HealthCheck__Payload_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T0^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::health_check::envoy__config__core__v3__HealthCheck__Payload_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::health_check::envoy__config__core__v3__HealthCheck__Payload_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Payload {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Payload {
  type Msg = Payload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Payload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Payload {
  type Msg = Payload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Payload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadMut<'_> {
  type Msg = Payload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Payload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadMut<'_> {
  type Msg = Payload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Payload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadView<'_> {
  type Msg = Payload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Payload> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod payload {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum PayloadOneof<'msg> {
  Text(&'msg ::protobuf::ProtoStr) = 1,
  Binary(&'msg [u8]) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum PayloadCase {
  Text = 1,
  Binary = 2,

  not_set = 0
}

impl PayloadCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<PayloadCase> {
    match v {
      0 => Some(PayloadCase::not_set),
      1 => Some(PayloadCase::Text),
      2 => Some(PayloadCase::Binary),
      _ => None
    }
  }
}
}  // pub mod payload

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HealthCheck__HttpHealthCheck_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpHealthCheck {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpHealthCheck>
}

impl ::protobuf::Message for HttpHealthCheck {
  type MessageView<'msg> = HttpHealthCheckView<'msg>;
  type MessageMut<'msg> = HttpHealthCheckMut<'msg>;
}

impl ::std::default::Default for HttpHealthCheck {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpHealthCheck {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpHealthCheck` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpHealthCheckMut`.
unsafe impl ::std::marker::Sync for HttpHealthCheck {}

// SAFETY:
// - `HttpHealthCheck` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpHealthCheck {}

impl ::protobuf::Proxied for HttpHealthCheck {
  type View<'msg> = HttpHealthCheckView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpHealthCheck {}

impl ::protobuf::MutProxied for HttpHealthCheck {
  type Mut<'msg> = HttpHealthCheckMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpHealthCheckView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpHealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpHealthCheckView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpHealthCheckView<'msg> {
  type Message = HttpHealthCheck;
}

impl ::std::fmt::Debug for HttpHealthCheckView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpHealthCheckView<'_> {
  fn default() -> HttpHealthCheckView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpHealthCheck>> for HttpHealthCheckView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpHealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpHealthCheckView<'msg> {

  pub fn to_owned(&self) -> HttpHealthCheck {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // host: optional string
  pub fn host(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // path: optional string
  pub fn path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // send: optional message envoy.config.core.v3.HealthCheck.Payload
  pub fn has_send(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn send_opt(self) -> ::std::option::Option<super::super::health_check::PayloadView<'msg>> {
    self.has_send().then(|| self.send())
  }
  pub fn send(self) -> super::super::health_check::PayloadView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::health_check::PayloadView::default())
  }

  // receive: repeated message envoy.config.core.v3.HealthCheck.Payload
  pub fn receive(self) -> ::protobuf::RepeatedView<'msg, super::super::health_check::Payload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::health_check::Payload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // response_buffer_size: optional message google.protobuf.UInt64Value
  pub fn has_response_buffer_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn response_buffer_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_response_buffer_size().then(|| self.response_buffer_size())
  }
  pub fn response_buffer_size(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

  // request_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // request_headers_to_remove: repeated string
  pub fn request_headers_to_remove(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // expected_statuses: repeated message envoy.type.v3.Int64Range
  pub fn expected_statuses(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::r#type::v3::range::Int64Range> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::v3::range::Int64Range>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // retriable_statuses: repeated message envoy.type.v3.Int64Range
  pub fn retriable_statuses(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::r#type::v3::range::Int64Range> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::v3::range::Int64Range>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // codec_client_type: optional enum envoy.type.v3.CodecClientType
  pub fn codec_client_type(self) -> crate::xds::generated::envoy::r#type::v3::http::CodecClientType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (crate::xds::generated::envoy::r#type::v3::http::CodecClientType::Http1).into()
      ).try_into().unwrap()
    }
  }

  // service_name_matcher: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_service_name_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn service_name_matcher_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_service_name_matcher().then(|| self.service_name_matcher())
  }
  pub fn service_name_matcher(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }

  // method: optional enum envoy.config.core.v3.RequestMethod
  pub fn method(self) -> crate::xds::generated::envoy::config::core::v3::base::RequestMethod {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        10, (crate::xds::generated::envoy::config::core::v3::base::RequestMethod::MethodUnspecified).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `HttpHealthCheckView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpHealthCheckView<'_> {}

// SAFETY:
// - `HttpHealthCheckView` is `Send` because while its alive a `HttpHealthCheckMut` cannot.
// - `HttpHealthCheckView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpHealthCheckView<'_> {}

impl<'msg> ::protobuf::AsView for HttpHealthCheckView<'msg> {
  type Proxied = HttpHealthCheck;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpHealthCheck> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpHealthCheckView<'msg> {
  fn into_view<'shorter>(self) -> HttpHealthCheckView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpHealthCheck> for HttpHealthCheckView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpHealthCheck {
    let mut dst = HttpHealthCheck::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpHealthCheck> for HttpHealthCheckMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpHealthCheck {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpHealthCheck {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpHealthCheckView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpHealthCheckMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpHealthCheckMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpHealthCheckMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpHealthCheckMut<'msg> {
  type Message = HttpHealthCheck;
}

impl ::std::fmt::Debug for HttpHealthCheckMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHealthCheck>> for HttpHealthCheckMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpHealthCheckMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHealthCheck> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpHealthCheck {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // host: optional string
  pub fn host(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_host(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // send: optional message envoy.config.core.v3.HealthCheck.Payload
  pub fn has_send(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_send(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn send_opt(&self) -> ::std::option::Option<super::super::health_check::PayloadView<'_>> {
    self.has_send().then(|| self.send())
  }
  pub fn send(&self) -> super::super::health_check::PayloadView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::health_check::PayloadView::default())
  }
  pub fn send_mut(&mut self) -> super::super::health_check::PayloadMut<'_> {
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
  pub fn set_send(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::health_check::Payload>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // receive: repeated message envoy.config.core.v3.HealthCheck.Payload
  pub fn receive(&self) -> ::protobuf::RepeatedView<'_, super::super::health_check::Payload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::health_check::Payload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn receive_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::health_check::Payload> {
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
  pub fn set_receive(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::health_check::Payload>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // response_buffer_size: optional message google.protobuf.UInt64Value
  pub fn has_response_buffer_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_response_buffer_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn response_buffer_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_response_buffer_size().then(|| self.response_buffer_size())
  }
  pub fn response_buffer_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn response_buffer_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_response_buffer_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // request_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_request_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // request_headers_to_remove: repeated string
  pub fn request_headers_to_remove(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_headers_to_remove_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_request_headers_to_remove(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // expected_statuses: repeated message envoy.type.v3.Int64Range
  pub fn expected_statuses(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::v3::range::Int64Range> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::v3::range::Int64Range>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn expected_statuses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::v3::range::Int64Range> {
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
  pub fn set_expected_statuses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::v3::range::Int64Range>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // retriable_statuses: repeated message envoy.type.v3.Int64Range
  pub fn retriable_statuses(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::v3::range::Int64Range> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::v3::range::Int64Range>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn retriable_statuses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::v3::range::Int64Range> {
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
  pub fn set_retriable_statuses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::v3::range::Int64Range>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

  // codec_client_type: optional enum envoy.type.v3.CodecClientType
  pub fn codec_client_type(&self) -> crate::xds::generated::envoy::r#type::v3::http::CodecClientType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (crate::xds::generated::envoy::r#type::v3::http::CodecClientType::Http1).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_codec_client_type(&mut self, val: crate::xds::generated::envoy::r#type::v3::http::CodecClientType) {
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

  // service_name_matcher: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_service_name_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_service_name_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn service_name_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_service_name_matcher().then(|| self.service_name_matcher())
  }
  pub fn service_name_matcher(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn service_name_matcher_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_service_name_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // method: optional enum envoy.config.core.v3.RequestMethod
  pub fn method(&self) -> crate::xds::generated::envoy::config::core::v3::base::RequestMethod {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        10, (crate::xds::generated::envoy::config::core::v3::base::RequestMethod::MethodUnspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_method(&mut self, val: crate::xds::generated::envoy::config::core::v3::base::RequestMethod) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        10, val.into()
      )
    }
  }

}

// SAFETY:
// - `HttpHealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpHealthCheckMut<'_> {}

// SAFETY:
// - `HttpHealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpHealthCheckMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpHealthCheckMut<'msg> {
  type Proxied = HttpHealthCheck;
  fn as_view(&self) -> ::protobuf::View<'_, HttpHealthCheck> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpHealthCheckMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpHealthCheck>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpHealthCheckMut<'msg> {
  type MutProxied = HttpHealthCheck;
  fn as_mut(&mut self) -> HttpHealthCheckMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpHealthCheckMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpHealthCheckMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpHealthCheck {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpHealthCheck> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpHealthCheckView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpHealthCheckMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // host: optional string
  pub fn host(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_host(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // send: optional message envoy.config.core.v3.HealthCheck.Payload
  pub fn has_send(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_send(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn send_opt(&self) -> ::std::option::Option<super::super::health_check::PayloadView<'_>> {
    self.has_send().then(|| self.send())
  }
  pub fn send(&self) -> super::super::health_check::PayloadView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::health_check::PayloadView::default())
  }
  pub fn send_mut(&mut self) -> super::super::health_check::PayloadMut<'_> {
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
  pub fn set_send(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::health_check::Payload>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // receive: repeated message envoy.config.core.v3.HealthCheck.Payload
  pub fn receive(&self) -> ::protobuf::RepeatedView<'_, super::super::health_check::Payload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::health_check::Payload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn receive_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::health_check::Payload> {
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
  pub fn set_receive(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::health_check::Payload>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // response_buffer_size: optional message google.protobuf.UInt64Value
  pub fn has_response_buffer_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_response_buffer_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn response_buffer_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_response_buffer_size().then(|| self.response_buffer_size())
  }
  pub fn response_buffer_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn response_buffer_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_response_buffer_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // request_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_request_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // request_headers_to_remove: repeated string
  pub fn request_headers_to_remove(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_headers_to_remove_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_request_headers_to_remove(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // expected_statuses: repeated message envoy.type.v3.Int64Range
  pub fn expected_statuses(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::v3::range::Int64Range> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::v3::range::Int64Range>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn expected_statuses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::v3::range::Int64Range> {
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
  pub fn set_expected_statuses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::v3::range::Int64Range>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // retriable_statuses: repeated message envoy.type.v3.Int64Range
  pub fn retriable_statuses(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::v3::range::Int64Range> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::v3::range::Int64Range>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn retriable_statuses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::v3::range::Int64Range> {
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
  pub fn set_retriable_statuses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::v3::range::Int64Range>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

  // codec_client_type: optional enum envoy.type.v3.CodecClientType
  pub fn codec_client_type(&self) -> crate::xds::generated::envoy::r#type::v3::http::CodecClientType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (crate::xds::generated::envoy::r#type::v3::http::CodecClientType::Http1).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_codec_client_type(&mut self, val: crate::xds::generated::envoy::r#type::v3::http::CodecClientType) {
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

  // service_name_matcher: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_service_name_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_service_name_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn service_name_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_service_name_matcher().then(|| self.service_name_matcher())
  }
  pub fn service_name_matcher(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn service_name_matcher_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_service_name_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // method: optional enum envoy.config.core.v3.RequestMethod
  pub fn method(&self) -> crate::xds::generated::envoy::config::core::v3::base::RequestMethod {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        10, (crate::xds::generated::envoy::config::core::v3::base::RequestMethod::MethodUnspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_method(&mut self, val: crate::xds::generated::envoy::config::core::v3::base::RequestMethod) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        10, val.into()
      )
    }
  }

}  // impl HttpHealthCheck

impl ::std::ops::Drop for HttpHealthCheck {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpHealthCheck {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpHealthCheck {
  type Proxied = Self;
  fn as_view(&self) -> HttpHealthCheckView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpHealthCheck {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpHealthCheckMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpHealthCheck {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::health_check::envoy__config__core__v3__HealthCheck__HttpHealthCheck_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X3GaGaETG.P3G.P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::health_check::envoy__config__core__v3__HealthCheck__HttpHealthCheck_msg_init.0, &[<super::super::health_check::Payload as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::health_check::Payload as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::range::Int64Range as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::range::Int64Range as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::health_check::envoy__config__core__v3__HealthCheck__HttpHealthCheck_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpHealthCheck {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpHealthCheck {
  type Msg = HttpHealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpHealthCheck {
  type Msg = HttpHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpHealthCheckMut<'_> {
  type Msg = HttpHealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpHealthCheckMut<'_> {
  type Msg = HttpHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpHealthCheckView<'_> {
  type Msg = HttpHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHealthCheck> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpHealthCheckMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HealthCheck__TcpHealthCheck_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TcpHealthCheck {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TcpHealthCheck>
}

impl ::protobuf::Message for TcpHealthCheck {
  type MessageView<'msg> = TcpHealthCheckView<'msg>;
  type MessageMut<'msg> = TcpHealthCheckMut<'msg>;
}

impl ::std::default::Default for TcpHealthCheck {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TcpHealthCheck {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TcpHealthCheck` is `Sync` because it does not implement interior mutability.
//    Neither does `TcpHealthCheckMut`.
unsafe impl ::std::marker::Sync for TcpHealthCheck {}

// SAFETY:
// - `TcpHealthCheck` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TcpHealthCheck {}

impl ::protobuf::Proxied for TcpHealthCheck {
  type View<'msg> = TcpHealthCheckView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TcpHealthCheck {}

impl ::protobuf::MutProxied for TcpHealthCheck {
  type Mut<'msg> = TcpHealthCheckMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TcpHealthCheckView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TcpHealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TcpHealthCheckView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TcpHealthCheckView<'msg> {
  type Message = TcpHealthCheck;
}

impl ::std::fmt::Debug for TcpHealthCheckView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TcpHealthCheckView<'_> {
  fn default() -> TcpHealthCheckView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TcpHealthCheck>> for TcpHealthCheckView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TcpHealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TcpHealthCheckView<'msg> {

  pub fn to_owned(&self) -> TcpHealthCheck {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // send: optional message envoy.config.core.v3.HealthCheck.Payload
  pub fn has_send(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn send_opt(self) -> ::std::option::Option<super::super::health_check::PayloadView<'msg>> {
    self.has_send().then(|| self.send())
  }
  pub fn send(self) -> super::super::health_check::PayloadView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::health_check::PayloadView::default())
  }

  // receive: repeated message envoy.config.core.v3.HealthCheck.Payload
  pub fn receive(self) -> ::protobuf::RepeatedView<'msg, super::super::health_check::Payload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::health_check::Payload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // proxy_protocol_config: optional message envoy.config.core.v3.ProxyProtocolConfig
  pub fn has_proxy_protocol_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn proxy_protocol_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfigView<'msg>> {
    self.has_proxy_protocol_config().then(|| self.proxy_protocol_config())
  }
  pub fn proxy_protocol_config(self) -> crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfigView::default())
  }

}

// SAFETY:
// - `TcpHealthCheckView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TcpHealthCheckView<'_> {}

// SAFETY:
// - `TcpHealthCheckView` is `Send` because while its alive a `TcpHealthCheckMut` cannot.
// - `TcpHealthCheckView` does not use thread-local data.
unsafe impl ::std::marker::Send for TcpHealthCheckView<'_> {}

impl<'msg> ::protobuf::AsView for TcpHealthCheckView<'msg> {
  type Proxied = TcpHealthCheck;
  fn as_view(&self) -> ::protobuf::View<'msg, TcpHealthCheck> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TcpHealthCheckView<'msg> {
  fn into_view<'shorter>(self) -> TcpHealthCheckView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TcpHealthCheck> for TcpHealthCheckView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TcpHealthCheck {
    let mut dst = TcpHealthCheck::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TcpHealthCheck> for TcpHealthCheckMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TcpHealthCheck {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TcpHealthCheck {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TcpHealthCheckView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TcpHealthCheckMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TcpHealthCheckMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TcpHealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TcpHealthCheckMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TcpHealthCheckMut<'msg> {
  type Message = TcpHealthCheck;
}

impl ::std::fmt::Debug for TcpHealthCheckMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TcpHealthCheck>> for TcpHealthCheckMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TcpHealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TcpHealthCheckMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TcpHealthCheck> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TcpHealthCheck {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // send: optional message envoy.config.core.v3.HealthCheck.Payload
  pub fn has_send(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_send(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn send_opt(&self) -> ::std::option::Option<super::super::health_check::PayloadView<'_>> {
    self.has_send().then(|| self.send())
  }
  pub fn send(&self) -> super::super::health_check::PayloadView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::health_check::PayloadView::default())
  }
  pub fn send_mut(&mut self) -> super::super::health_check::PayloadMut<'_> {
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
  pub fn set_send(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::health_check::Payload>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // receive: repeated message envoy.config.core.v3.HealthCheck.Payload
  pub fn receive(&self) -> ::protobuf::RepeatedView<'_, super::super::health_check::Payload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::health_check::Payload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn receive_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::health_check::Payload> {
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
  pub fn set_receive(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::health_check::Payload>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // proxy_protocol_config: optional message envoy.config.core.v3.ProxyProtocolConfig
  pub fn has_proxy_protocol_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_proxy_protocol_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn proxy_protocol_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfigView<'_>> {
    self.has_proxy_protocol_config().then(|| self.proxy_protocol_config())
  }
  pub fn proxy_protocol_config(&self) -> crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfigView::default())
  }
  pub fn proxy_protocol_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfigMut<'_> {
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
  pub fn set_proxy_protocol_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfig>) {

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
// - `TcpHealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TcpHealthCheckMut<'_> {}

// SAFETY:
// - `TcpHealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TcpHealthCheckMut<'_> {}

impl<'msg> ::protobuf::AsView for TcpHealthCheckMut<'msg> {
  type Proxied = TcpHealthCheck;
  fn as_view(&self) -> ::protobuf::View<'_, TcpHealthCheck> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TcpHealthCheckMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TcpHealthCheck>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TcpHealthCheckMut<'msg> {
  type MutProxied = TcpHealthCheck;
  fn as_mut(&mut self) -> TcpHealthCheckMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TcpHealthCheckMut<'msg> {
  fn into_mut<'shorter>(self) -> TcpHealthCheckMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TcpHealthCheck {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TcpHealthCheck> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TcpHealthCheckView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TcpHealthCheckMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // send: optional message envoy.config.core.v3.HealthCheck.Payload
  pub fn has_send(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_send(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn send_opt(&self) -> ::std::option::Option<super::super::health_check::PayloadView<'_>> {
    self.has_send().then(|| self.send())
  }
  pub fn send(&self) -> super::super::health_check::PayloadView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::health_check::PayloadView::default())
  }
  pub fn send_mut(&mut self) -> super::super::health_check::PayloadMut<'_> {
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
  pub fn set_send(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::health_check::Payload>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // receive: repeated message envoy.config.core.v3.HealthCheck.Payload
  pub fn receive(&self) -> ::protobuf::RepeatedView<'_, super::super::health_check::Payload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::health_check::Payload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn receive_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::health_check::Payload> {
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
  pub fn set_receive(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::health_check::Payload>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // proxy_protocol_config: optional message envoy.config.core.v3.ProxyProtocolConfig
  pub fn has_proxy_protocol_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_proxy_protocol_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn proxy_protocol_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfigView<'_>> {
    self.has_proxy_protocol_config().then(|| self.proxy_protocol_config())
  }
  pub fn proxy_protocol_config(&self) -> crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfigView::default())
  }
  pub fn proxy_protocol_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfigMut<'_> {
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
  pub fn set_proxy_protocol_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl TcpHealthCheck

impl ::std::ops::Drop for TcpHealthCheck {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TcpHealthCheck {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TcpHealthCheck {
  type Proxied = Self;
  fn as_view(&self) -> TcpHealthCheckView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TcpHealthCheck {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TcpHealthCheckMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TcpHealthCheck {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::health_check::envoy__config__core__v3__HealthCheck__TcpHealthCheck_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::health_check::envoy__config__core__v3__HealthCheck__TcpHealthCheck_msg_init.0, &[<super::super::health_check::Payload as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::health_check::Payload as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::proxy_protocol::ProxyProtocolConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::health_check::envoy__config__core__v3__HealthCheck__TcpHealthCheck_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TcpHealthCheck {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TcpHealthCheck {
  type Msg = TcpHealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpHealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TcpHealthCheck {
  type Msg = TcpHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpHealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TcpHealthCheckMut<'_> {
  type Msg = TcpHealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpHealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TcpHealthCheckMut<'_> {
  type Msg = TcpHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpHealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TcpHealthCheckView<'_> {
  type Msg = TcpHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpHealthCheck> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TcpHealthCheckMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HealthCheck__RedisHealthCheck_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RedisHealthCheck {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RedisHealthCheck>
}

impl ::protobuf::Message for RedisHealthCheck {
  type MessageView<'msg> = RedisHealthCheckView<'msg>;
  type MessageMut<'msg> = RedisHealthCheckMut<'msg>;
}

impl ::std::default::Default for RedisHealthCheck {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RedisHealthCheck {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RedisHealthCheck` is `Sync` because it does not implement interior mutability.
//    Neither does `RedisHealthCheckMut`.
unsafe impl ::std::marker::Sync for RedisHealthCheck {}

// SAFETY:
// - `RedisHealthCheck` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RedisHealthCheck {}

impl ::protobuf::Proxied for RedisHealthCheck {
  type View<'msg> = RedisHealthCheckView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RedisHealthCheck {}

impl ::protobuf::MutProxied for RedisHealthCheck {
  type Mut<'msg> = RedisHealthCheckMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RedisHealthCheckView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RedisHealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RedisHealthCheckView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RedisHealthCheckView<'msg> {
  type Message = RedisHealthCheck;
}

impl ::std::fmt::Debug for RedisHealthCheckView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RedisHealthCheckView<'_> {
  fn default() -> RedisHealthCheckView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RedisHealthCheck>> for RedisHealthCheckView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RedisHealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RedisHealthCheckView<'msg> {

  pub fn to_owned(&self) -> RedisHealthCheck {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key: optional string
  pub fn key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `RedisHealthCheckView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RedisHealthCheckView<'_> {}

// SAFETY:
// - `RedisHealthCheckView` is `Send` because while its alive a `RedisHealthCheckMut` cannot.
// - `RedisHealthCheckView` does not use thread-local data.
unsafe impl ::std::marker::Send for RedisHealthCheckView<'_> {}

impl<'msg> ::protobuf::AsView for RedisHealthCheckView<'msg> {
  type Proxied = RedisHealthCheck;
  fn as_view(&self) -> ::protobuf::View<'msg, RedisHealthCheck> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RedisHealthCheckView<'msg> {
  fn into_view<'shorter>(self) -> RedisHealthCheckView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RedisHealthCheck> for RedisHealthCheckView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RedisHealthCheck {
    let mut dst = RedisHealthCheck::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RedisHealthCheck> for RedisHealthCheckMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RedisHealthCheck {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RedisHealthCheck {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RedisHealthCheckView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RedisHealthCheckMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RedisHealthCheckMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RedisHealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RedisHealthCheckMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RedisHealthCheckMut<'msg> {
  type Message = RedisHealthCheck;
}

impl ::std::fmt::Debug for RedisHealthCheckMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RedisHealthCheck>> for RedisHealthCheckMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RedisHealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RedisHealthCheckMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RedisHealthCheck> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RedisHealthCheck {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `RedisHealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RedisHealthCheckMut<'_> {}

// SAFETY:
// - `RedisHealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RedisHealthCheckMut<'_> {}

impl<'msg> ::protobuf::AsView for RedisHealthCheckMut<'msg> {
  type Proxied = RedisHealthCheck;
  fn as_view(&self) -> ::protobuf::View<'_, RedisHealthCheck> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RedisHealthCheckMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RedisHealthCheck>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RedisHealthCheckMut<'msg> {
  type MutProxied = RedisHealthCheck;
  fn as_mut(&mut self) -> RedisHealthCheckMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RedisHealthCheckMut<'msg> {
  fn into_mut<'shorter>(self) -> RedisHealthCheckMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RedisHealthCheck {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RedisHealthCheck> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RedisHealthCheckView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RedisHealthCheckMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl RedisHealthCheck

impl ::std::ops::Drop for RedisHealthCheck {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RedisHealthCheck {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RedisHealthCheck {
  type Proxied = Self;
  fn as_view(&self) -> RedisHealthCheckView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RedisHealthCheck {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RedisHealthCheckMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RedisHealthCheck {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::health_check::envoy__config__core__v3__HealthCheck__RedisHealthCheck_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::health_check::envoy__config__core__v3__HealthCheck__RedisHealthCheck_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::health_check::envoy__config__core__v3__HealthCheck__RedisHealthCheck_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RedisHealthCheck {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RedisHealthCheck {
  type Msg = RedisHealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RedisHealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RedisHealthCheck {
  type Msg = RedisHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RedisHealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RedisHealthCheckMut<'_> {
  type Msg = RedisHealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RedisHealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RedisHealthCheckMut<'_> {
  type Msg = RedisHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RedisHealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RedisHealthCheckView<'_> {
  type Msg = RedisHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RedisHealthCheck> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RedisHealthCheckMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HealthCheck__GrpcHealthCheck_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GrpcHealthCheck {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GrpcHealthCheck>
}

impl ::protobuf::Message for GrpcHealthCheck {
  type MessageView<'msg> = GrpcHealthCheckView<'msg>;
  type MessageMut<'msg> = GrpcHealthCheckMut<'msg>;
}

impl ::std::default::Default for GrpcHealthCheck {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GrpcHealthCheck {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GrpcHealthCheck` is `Sync` because it does not implement interior mutability.
//    Neither does `GrpcHealthCheckMut`.
unsafe impl ::std::marker::Sync for GrpcHealthCheck {}

// SAFETY:
// - `GrpcHealthCheck` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GrpcHealthCheck {}

impl ::protobuf::Proxied for GrpcHealthCheck {
  type View<'msg> = GrpcHealthCheckView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GrpcHealthCheck {}

impl ::protobuf::MutProxied for GrpcHealthCheck {
  type Mut<'msg> = GrpcHealthCheckMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GrpcHealthCheckView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcHealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcHealthCheckView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GrpcHealthCheckView<'msg> {
  type Message = GrpcHealthCheck;
}

impl ::std::fmt::Debug for GrpcHealthCheckView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GrpcHealthCheckView<'_> {
  fn default() -> GrpcHealthCheckView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcHealthCheck>> for GrpcHealthCheckView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcHealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcHealthCheckView<'msg> {

  pub fn to_owned(&self) -> GrpcHealthCheck {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // service_name: optional string
  pub fn service_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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

  // initial_metadata: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn initial_metadata(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `GrpcHealthCheckView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GrpcHealthCheckView<'_> {}

// SAFETY:
// - `GrpcHealthCheckView` is `Send` because while its alive a `GrpcHealthCheckMut` cannot.
// - `GrpcHealthCheckView` does not use thread-local data.
unsafe impl ::std::marker::Send for GrpcHealthCheckView<'_> {}

impl<'msg> ::protobuf::AsView for GrpcHealthCheckView<'msg> {
  type Proxied = GrpcHealthCheck;
  fn as_view(&self) -> ::protobuf::View<'msg, GrpcHealthCheck> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcHealthCheckView<'msg> {
  fn into_view<'shorter>(self) -> GrpcHealthCheckView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcHealthCheck> for GrpcHealthCheckView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcHealthCheck {
    let mut dst = GrpcHealthCheck::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcHealthCheck> for GrpcHealthCheckMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcHealthCheck {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GrpcHealthCheck {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcHealthCheckView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcHealthCheckMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GrpcHealthCheckMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcHealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcHealthCheckMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GrpcHealthCheckMut<'msg> {
  type Message = GrpcHealthCheck;
}

impl ::std::fmt::Debug for GrpcHealthCheckMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcHealthCheck>> for GrpcHealthCheckMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcHealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcHealthCheckMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcHealthCheck> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GrpcHealthCheck {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // service_name: optional string
  pub fn service_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
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

  // initial_metadata: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn initial_metadata(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn initial_metadata_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_initial_metadata(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `GrpcHealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GrpcHealthCheckMut<'_> {}

// SAFETY:
// - `GrpcHealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GrpcHealthCheckMut<'_> {}

impl<'msg> ::protobuf::AsView for GrpcHealthCheckMut<'msg> {
  type Proxied = GrpcHealthCheck;
  fn as_view(&self) -> ::protobuf::View<'_, GrpcHealthCheck> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcHealthCheckMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GrpcHealthCheck>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GrpcHealthCheckMut<'msg> {
  type MutProxied = GrpcHealthCheck;
  fn as_mut(&mut self) -> GrpcHealthCheckMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GrpcHealthCheckMut<'msg> {
  fn into_mut<'shorter>(self) -> GrpcHealthCheckMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GrpcHealthCheck {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GrpcHealthCheck> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GrpcHealthCheckView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GrpcHealthCheckMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // service_name: optional string
  pub fn service_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
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

  // initial_metadata: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn initial_metadata(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn initial_metadata_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_initial_metadata(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl GrpcHealthCheck

impl ::std::ops::Drop for GrpcHealthCheck {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GrpcHealthCheck {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GrpcHealthCheck {
  type Proxied = Self;
  fn as_view(&self) -> GrpcHealthCheckView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GrpcHealthCheck {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GrpcHealthCheckMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GrpcHealthCheck {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::health_check::envoy__config__core__v3__HealthCheck__GrpcHealthCheck_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1XG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::health_check::envoy__config__core__v3__HealthCheck__GrpcHealthCheck_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::health_check::envoy__config__core__v3__HealthCheck__GrpcHealthCheck_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcHealthCheck {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcHealthCheck {
  type Msg = GrpcHealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcHealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcHealthCheck {
  type Msg = GrpcHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcHealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcHealthCheckMut<'_> {
  type Msg = GrpcHealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcHealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcHealthCheckMut<'_> {
  type Msg = GrpcHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcHealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcHealthCheckView<'_> {
  type Msg = GrpcHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcHealthCheck> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcHealthCheckMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HealthCheck__CustomHealthCheck_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CustomHealthCheck {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CustomHealthCheck>
}

impl ::protobuf::Message for CustomHealthCheck {
  type MessageView<'msg> = CustomHealthCheckView<'msg>;
  type MessageMut<'msg> = CustomHealthCheckMut<'msg>;
}

impl ::std::default::Default for CustomHealthCheck {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CustomHealthCheck {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CustomHealthCheck` is `Sync` because it does not implement interior mutability.
//    Neither does `CustomHealthCheckMut`.
unsafe impl ::std::marker::Sync for CustomHealthCheck {}

// SAFETY:
// - `CustomHealthCheck` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CustomHealthCheck {}

impl ::protobuf::Proxied for CustomHealthCheck {
  type View<'msg> = CustomHealthCheckView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CustomHealthCheck {}

impl ::protobuf::MutProxied for CustomHealthCheck {
  type Mut<'msg> = CustomHealthCheckMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CustomHealthCheckView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CustomHealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CustomHealthCheckView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CustomHealthCheckView<'msg> {
  type Message = CustomHealthCheck;
}

impl ::std::fmt::Debug for CustomHealthCheckView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CustomHealthCheckView<'_> {
  fn default() -> CustomHealthCheckView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CustomHealthCheck>> for CustomHealthCheckView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CustomHealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CustomHealthCheckView<'msg> {

  pub fn to_owned(&self) -> CustomHealthCheck {
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

  pub fn config_type(self) -> super::super::health_check::custom_health_check::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::super::health_check::custom_health_check::ConfigTypeCase::TypedConfig =>
          super::super::health_check::custom_health_check::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::health_check::custom_health_check::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::super::health_check::custom_health_check::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::health_check::custom_health_check::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CustomHealthCheckView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CustomHealthCheckView<'_> {}

// SAFETY:
// - `CustomHealthCheckView` is `Send` because while its alive a `CustomHealthCheckMut` cannot.
// - `CustomHealthCheckView` does not use thread-local data.
unsafe impl ::std::marker::Send for CustomHealthCheckView<'_> {}

impl<'msg> ::protobuf::AsView for CustomHealthCheckView<'msg> {
  type Proxied = CustomHealthCheck;
  fn as_view(&self) -> ::protobuf::View<'msg, CustomHealthCheck> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CustomHealthCheckView<'msg> {
  fn into_view<'shorter>(self) -> CustomHealthCheckView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CustomHealthCheck> for CustomHealthCheckView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CustomHealthCheck {
    let mut dst = CustomHealthCheck::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CustomHealthCheck> for CustomHealthCheckMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CustomHealthCheck {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CustomHealthCheck {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CustomHealthCheckView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CustomHealthCheckMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CustomHealthCheckMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomHealthCheck>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CustomHealthCheckMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CustomHealthCheckMut<'msg> {
  type Message = CustomHealthCheck;
}

impl ::std::fmt::Debug for CustomHealthCheckMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CustomHealthCheck>> for CustomHealthCheckMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomHealthCheck>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CustomHealthCheckMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomHealthCheck> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CustomHealthCheck {
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

  pub fn config_type(&self) -> super::super::health_check::custom_health_check::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::super::health_check::custom_health_check::ConfigTypeCase::TypedConfig =>
          super::super::health_check::custom_health_check::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::health_check::custom_health_check::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::super::health_check::custom_health_check::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::health_check::custom_health_check::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CustomHealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CustomHealthCheckMut<'_> {}

// SAFETY:
// - `CustomHealthCheckMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CustomHealthCheckMut<'_> {}

impl<'msg> ::protobuf::AsView for CustomHealthCheckMut<'msg> {
  type Proxied = CustomHealthCheck;
  fn as_view(&self) -> ::protobuf::View<'_, CustomHealthCheck> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CustomHealthCheckMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CustomHealthCheck>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CustomHealthCheckMut<'msg> {
  type MutProxied = CustomHealthCheck;
  fn as_mut(&mut self) -> CustomHealthCheckMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CustomHealthCheckMut<'msg> {
  fn into_mut<'shorter>(self) -> CustomHealthCheckMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CustomHealthCheck {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CustomHealthCheck> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CustomHealthCheckView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CustomHealthCheckMut<'_> {
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

  pub fn config_type(&self) -> super::super::health_check::custom_health_check::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::super::health_check::custom_health_check::ConfigTypeCase::TypedConfig =>
          super::super::health_check::custom_health_check::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::health_check::custom_health_check::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::super::health_check::custom_health_check::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::health_check::custom_health_check::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl CustomHealthCheck

impl ::std::ops::Drop for CustomHealthCheck {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CustomHealthCheck {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CustomHealthCheck {
  type Proxied = Self;
  fn as_view(&self) -> CustomHealthCheckView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CustomHealthCheck {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CustomHealthCheckMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CustomHealthCheck {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::health_check::envoy__config__core__v3__HealthCheck__CustomHealthCheck_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xa3^$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::health_check::envoy__config__core__v3__HealthCheck__CustomHealthCheck_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::health_check::envoy__config__core__v3__HealthCheck__CustomHealthCheck_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CustomHealthCheck {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CustomHealthCheck {
  type Msg = CustomHealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomHealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomHealthCheck {
  type Msg = CustomHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomHealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CustomHealthCheckMut<'_> {
  type Msg = CustomHealthCheck;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomHealthCheck> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomHealthCheckMut<'_> {
  type Msg = CustomHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomHealthCheck> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomHealthCheckView<'_> {
  type Msg = CustomHealthCheck;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomHealthCheck> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CustomHealthCheckMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod custom_health_check {

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
}  // pub mod custom_health_check

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HealthCheck__TlsOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TlsOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TlsOptions>
}

impl ::protobuf::Message for TlsOptions {
  type MessageView<'msg> = TlsOptionsView<'msg>;
  type MessageMut<'msg> = TlsOptionsMut<'msg>;
}

impl ::std::default::Default for TlsOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TlsOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TlsOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `TlsOptionsMut`.
unsafe impl ::std::marker::Sync for TlsOptions {}

// SAFETY:
// - `TlsOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TlsOptions {}

impl ::protobuf::Proxied for TlsOptions {
  type View<'msg> = TlsOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TlsOptions {}

impl ::protobuf::MutProxied for TlsOptions {
  type Mut<'msg> = TlsOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TlsOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TlsOptionsView<'msg> {
  type Message = TlsOptions;
}

impl ::std::fmt::Debug for TlsOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TlsOptionsView<'_> {
  fn default() -> TlsOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TlsOptions>> for TlsOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsOptionsView<'msg> {

  pub fn to_owned(&self) -> TlsOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // alpn_protocols: repeated string
  pub fn alpn_protocols(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `TlsOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TlsOptionsView<'_> {}

// SAFETY:
// - `TlsOptionsView` is `Send` because while its alive a `TlsOptionsMut` cannot.
// - `TlsOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for TlsOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for TlsOptionsView<'msg> {
  type Proxied = TlsOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, TlsOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsOptionsView<'msg> {
  fn into_view<'shorter>(self) -> TlsOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsOptions> for TlsOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsOptions {
    let mut dst = TlsOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsOptions> for TlsOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TlsOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TlsOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TlsOptionsMut<'msg> {
  type Message = TlsOptions;
}

impl ::std::fmt::Debug for TlsOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TlsOptions>> for TlsOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TlsOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // alpn_protocols: repeated string
  pub fn alpn_protocols(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn alpn_protocols_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_alpn_protocols(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `TlsOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TlsOptionsMut<'_> {}

// SAFETY:
// - `TlsOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TlsOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for TlsOptionsMut<'msg> {
  type Proxied = TlsOptions;
  fn as_view(&self) -> ::protobuf::View<'_, TlsOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TlsOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TlsOptionsMut<'msg> {
  type MutProxied = TlsOptions;
  fn as_mut(&mut self) -> TlsOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TlsOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> TlsOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TlsOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TlsOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TlsOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TlsOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // alpn_protocols: repeated string
  pub fn alpn_protocols(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn alpn_protocols_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_alpn_protocols(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl TlsOptions

impl ::std::ops::Drop for TlsOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TlsOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TlsOptions {
  type Proxied = Self;
  fn as_view(&self) -> TlsOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TlsOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TlsOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TlsOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::health_check::envoy__config__core__v3__HealthCheck__TlsOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ME");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::health_check::envoy__config__core__v3__HealthCheck__TlsOptions_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::health_check::envoy__config__core__v3__HealthCheck__TlsOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsOptions {
  type Msg = TlsOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsOptions {
  type Msg = TlsOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsOptionsMut<'_> {
  type Msg = TlsOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsOptionsMut<'_> {
  type Msg = TlsOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsOptionsView<'_> {
  type Msg = TlsOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum HealthCheckerOneof<'msg> {
  HttpHealthCheck(::protobuf::View<'msg, super::super::health_check::HttpHealthCheck>) = 8,
  TcpHealthCheck(::protobuf::View<'msg, super::super::health_check::TcpHealthCheck>) = 9,
  GrpcHealthCheck(::protobuf::View<'msg, super::super::health_check::GrpcHealthCheck>) = 11,
  CustomHealthCheck(::protobuf::View<'msg, super::super::health_check::CustomHealthCheck>) = 13,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum HealthCheckerCase {
  HttpHealthCheck = 8,
  TcpHealthCheck = 9,
  GrpcHealthCheck = 11,
  CustomHealthCheck = 13,

  not_set = 0
}

impl HealthCheckerCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<HealthCheckerCase> {
    match v {
      0 => Some(HealthCheckerCase::not_set),
      8 => Some(HealthCheckerCase::HttpHealthCheck),
      9 => Some(HealthCheckerCase::TcpHealthCheck),
      11 => Some(HealthCheckerCase::GrpcHealthCheck),
      13 => Some(HealthCheckerCase::CustomHealthCheck),
      _ => None
    }
  }
}
}  // pub mod health_check


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HealthStatus(i32);

#[allow(non_upper_case_globals)]
impl HealthStatus {
  pub const Unknown: HealthStatus = HealthStatus(0);
  pub const Healthy: HealthStatus = HealthStatus(1);
  pub const Unhealthy: HealthStatus = HealthStatus(2);
  pub const Draining: HealthStatus = HealthStatus(3);
  pub const Timeout: HealthStatus = HealthStatus(4);
  pub const Degraded: HealthStatus = HealthStatus(5);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unknown",
      1 => "Healthy",
      2 => "Unhealthy",
      3 => "Draining",
      4 => "Timeout",
      5 => "Degraded",
      _ => return None
    })
  }
}

impl ::std::convert::From<HealthStatus> for i32 {
  fn from(val: HealthStatus) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for HealthStatus {
  fn from(val: i32) -> HealthStatus {
    Self(val)
  }
}

impl ::std::default::Default for HealthStatus {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for HealthStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "HealthStatus::{}", constant_name)
    } else {
      write!(f, "HealthStatus::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for HealthStatus {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for HealthStatus {}

impl ::protobuf::Proxied for HealthStatus {
  type View<'a> = HealthStatus;
}

impl ::protobuf::AsView for HealthStatus {
  type Proxied = HealthStatus;

  fn as_view(&self) -> HealthStatus {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HealthStatus {
  fn into_view<'shorter>(self) -> HealthStatus where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for HealthStatus {
  const NAME: &'static str = "HealthStatus";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5)
  }
}

impl ::protobuf::__internal::EntityType for HealthStatus {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


