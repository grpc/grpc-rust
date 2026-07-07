const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__CircuitBreakers_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CircuitBreakers {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CircuitBreakers>
}

impl ::protobuf::Message for CircuitBreakers {
  type MessageView<'msg> = CircuitBreakersView<'msg>;
  type MessageMut<'msg> = CircuitBreakersMut<'msg>;
}

impl ::std::default::Default for CircuitBreakers {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CircuitBreakers {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CircuitBreakers` is `Sync` because it does not implement interior mutability.
//    Neither does `CircuitBreakersMut`.
unsafe impl ::std::marker::Sync for CircuitBreakers {}

// SAFETY:
// - `CircuitBreakers` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CircuitBreakers {}

impl ::protobuf::Proxied for CircuitBreakers {
  type View<'msg> = CircuitBreakersView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CircuitBreakers {}

impl ::protobuf::MutProxied for CircuitBreakers {
  type Mut<'msg> = CircuitBreakersMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CircuitBreakersView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CircuitBreakers>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CircuitBreakersView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CircuitBreakersView<'msg> {
  type Message = CircuitBreakers;
}

impl ::std::fmt::Debug for CircuitBreakersView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CircuitBreakersView<'_> {
  fn default() -> CircuitBreakersView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CircuitBreakers>> for CircuitBreakersView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CircuitBreakers>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CircuitBreakersView<'msg> {

  pub fn to_owned(&self) -> CircuitBreakers {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // thresholds: repeated message envoy.config.cluster.v3.CircuitBreakers.Thresholds
  pub fn thresholds(self) -> ::protobuf::RepeatedView<'msg, super::circuit_breakers::Thresholds> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::circuit_breakers::Thresholds>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // per_host_thresholds: repeated message envoy.config.cluster.v3.CircuitBreakers.Thresholds
  pub fn per_host_thresholds(self) -> ::protobuf::RepeatedView<'msg, super::circuit_breakers::Thresholds> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::circuit_breakers::Thresholds>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `CircuitBreakersView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CircuitBreakersView<'_> {}

// SAFETY:
// - `CircuitBreakersView` is `Send` because while its alive a `CircuitBreakersMut` cannot.
// - `CircuitBreakersView` does not use thread-local data.
unsafe impl ::std::marker::Send for CircuitBreakersView<'_> {}

impl<'msg> ::protobuf::AsView for CircuitBreakersView<'msg> {
  type Proxied = CircuitBreakers;
  fn as_view(&self) -> ::protobuf::View<'msg, CircuitBreakers> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CircuitBreakersView<'msg> {
  fn into_view<'shorter>(self) -> CircuitBreakersView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CircuitBreakers> for CircuitBreakersView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CircuitBreakers {
    let mut dst = CircuitBreakers::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CircuitBreakers> for CircuitBreakersMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CircuitBreakers {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CircuitBreakers {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CircuitBreakersView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CircuitBreakersMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CircuitBreakersMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CircuitBreakers>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CircuitBreakersMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CircuitBreakersMut<'msg> {
  type Message = CircuitBreakers;
}

impl ::std::fmt::Debug for CircuitBreakersMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CircuitBreakers>> for CircuitBreakersMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CircuitBreakers>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CircuitBreakersMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CircuitBreakers> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CircuitBreakers {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // thresholds: repeated message envoy.config.cluster.v3.CircuitBreakers.Thresholds
  pub fn thresholds(&self) -> ::protobuf::RepeatedView<'_, super::circuit_breakers::Thresholds> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::circuit_breakers::Thresholds>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn thresholds_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::circuit_breakers::Thresholds> {
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
  pub fn set_thresholds(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::circuit_breakers::Thresholds>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // per_host_thresholds: repeated message envoy.config.cluster.v3.CircuitBreakers.Thresholds
  pub fn per_host_thresholds(&self) -> ::protobuf::RepeatedView<'_, super::circuit_breakers::Thresholds> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::circuit_breakers::Thresholds>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn per_host_thresholds_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::circuit_breakers::Thresholds> {
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
  pub fn set_per_host_thresholds(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::circuit_breakers::Thresholds>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `CircuitBreakersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CircuitBreakersMut<'_> {}

// SAFETY:
// - `CircuitBreakersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CircuitBreakersMut<'_> {}

impl<'msg> ::protobuf::AsView for CircuitBreakersMut<'msg> {
  type Proxied = CircuitBreakers;
  fn as_view(&self) -> ::protobuf::View<'_, CircuitBreakers> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CircuitBreakersMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CircuitBreakers>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CircuitBreakersMut<'msg> {
  type MutProxied = CircuitBreakers;
  fn as_mut(&mut self) -> CircuitBreakersMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CircuitBreakersMut<'msg> {
  fn into_mut<'shorter>(self) -> CircuitBreakersMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CircuitBreakers {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CircuitBreakers> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CircuitBreakersView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CircuitBreakersMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // thresholds: repeated message envoy.config.cluster.v3.CircuitBreakers.Thresholds
  pub fn thresholds(&self) -> ::protobuf::RepeatedView<'_, super::circuit_breakers::Thresholds> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::circuit_breakers::Thresholds>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn thresholds_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::circuit_breakers::Thresholds> {
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
  pub fn set_thresholds(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::circuit_breakers::Thresholds>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // per_host_thresholds: repeated message envoy.config.cluster.v3.CircuitBreakers.Thresholds
  pub fn per_host_thresholds(&self) -> ::protobuf::RepeatedView<'_, super::circuit_breakers::Thresholds> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::circuit_breakers::Thresholds>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn per_host_thresholds_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::circuit_breakers::Thresholds> {
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
  pub fn set_per_host_thresholds(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::circuit_breakers::Thresholds>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl CircuitBreakers

impl ::std::ops::Drop for CircuitBreakers {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CircuitBreakers {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CircuitBreakers {
  type Proxied = Self;
  fn as_view(&self) -> CircuitBreakersView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CircuitBreakers {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CircuitBreakersMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CircuitBreakers {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__cluster__v3__CircuitBreakers_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$GG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__cluster__v3__CircuitBreakers_msg_init.0, &[<super::circuit_breakers::Thresholds as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::circuit_breakers::Thresholds as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__cluster__v3__CircuitBreakers_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CircuitBreakers {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CircuitBreakers {
  type Msg = CircuitBreakers;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CircuitBreakers> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CircuitBreakers {
  type Msg = CircuitBreakers;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CircuitBreakers> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CircuitBreakersMut<'_> {
  type Msg = CircuitBreakers;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CircuitBreakers> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CircuitBreakersMut<'_> {
  type Msg = CircuitBreakers;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CircuitBreakers> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CircuitBreakersView<'_> {
  type Msg = CircuitBreakers;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CircuitBreakers> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CircuitBreakersMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod circuit_breakers {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__CircuitBreakers__Thresholds_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Thresholds {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Thresholds>
}

impl ::protobuf::Message for Thresholds {
  type MessageView<'msg> = ThresholdsView<'msg>;
  type MessageMut<'msg> = ThresholdsMut<'msg>;
}

impl ::std::default::Default for Thresholds {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Thresholds {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Thresholds` is `Sync` because it does not implement interior mutability.
//    Neither does `ThresholdsMut`.
unsafe impl ::std::marker::Sync for Thresholds {}

// SAFETY:
// - `Thresholds` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Thresholds {}

impl ::protobuf::Proxied for Thresholds {
  type View<'msg> = ThresholdsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Thresholds {}

impl ::protobuf::MutProxied for Thresholds {
  type Mut<'msg> = ThresholdsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ThresholdsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Thresholds>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ThresholdsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ThresholdsView<'msg> {
  type Message = Thresholds;
}

impl ::std::fmt::Debug for ThresholdsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ThresholdsView<'_> {
  fn default() -> ThresholdsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Thresholds>> for ThresholdsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Thresholds>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ThresholdsView<'msg> {

  pub fn to_owned(&self) -> Thresholds {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // priority: optional enum envoy.config.core.v3.RoutingPriority
  pub fn priority(self) -> crate::xds::generated::envoy::config::core::v3::base::RoutingPriority {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (crate::xds::generated::envoy::config::core::v3::base::RoutingPriority::Default).into()
      ).try_into().unwrap()
    }
  }

  // max_connections: optional message google.protobuf.UInt32Value
  pub fn has_max_connections(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn max_connections_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_connections().then(|| self.max_connections())
  }
  pub fn max_connections(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // max_pending_requests: optional message google.protobuf.UInt32Value
  pub fn has_max_pending_requests(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn max_pending_requests_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_pending_requests().then(|| self.max_pending_requests())
  }
  pub fn max_pending_requests(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // max_requests: optional message google.protobuf.UInt32Value
  pub fn has_max_requests(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn max_requests_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_requests().then(|| self.max_requests())
  }
  pub fn max_requests(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // max_retries: optional message google.protobuf.UInt32Value
  pub fn has_max_retries(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn max_retries_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_retries().then(|| self.max_retries())
  }
  pub fn max_retries(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // retry_budget: optional message envoy.config.cluster.v3.CircuitBreakers.Thresholds.RetryBudget
  pub fn has_retry_budget(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn retry_budget_opt(self) -> ::std::option::Option<super::super::circuit_breakers::thresholds::RetryBudgetView<'msg>> {
    self.has_retry_budget().then(|| self.retry_budget())
  }
  pub fn retry_budget(self) -> super::super::circuit_breakers::thresholds::RetryBudgetView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::circuit_breakers::thresholds::RetryBudgetView::default())
  }

  // track_remaining: optional bool
  pub fn track_remaining(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }

  // max_connection_pools: optional message google.protobuf.UInt32Value
  pub fn has_max_connection_pools(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn max_connection_pools_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_connection_pools().then(|| self.max_connection_pools())
  }
  pub fn max_connection_pools(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `ThresholdsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ThresholdsView<'_> {}

// SAFETY:
// - `ThresholdsView` is `Send` because while its alive a `ThresholdsMut` cannot.
// - `ThresholdsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ThresholdsView<'_> {}

impl<'msg> ::protobuf::AsView for ThresholdsView<'msg> {
  type Proxied = Thresholds;
  fn as_view(&self) -> ::protobuf::View<'msg, Thresholds> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ThresholdsView<'msg> {
  fn into_view<'shorter>(self) -> ThresholdsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Thresholds> for ThresholdsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Thresholds {
    let mut dst = Thresholds::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Thresholds> for ThresholdsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Thresholds {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Thresholds {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ThresholdsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ThresholdsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ThresholdsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Thresholds>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ThresholdsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ThresholdsMut<'msg> {
  type Message = Thresholds;
}

impl ::std::fmt::Debug for ThresholdsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Thresholds>> for ThresholdsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Thresholds>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ThresholdsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Thresholds> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Thresholds {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // priority: optional enum envoy.config.core.v3.RoutingPriority
  pub fn priority(&self) -> crate::xds::generated::envoy::config::core::v3::base::RoutingPriority {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (crate::xds::generated::envoy::config::core::v3::base::RoutingPriority::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_priority(&mut self, val: crate::xds::generated::envoy::config::core::v3::base::RoutingPriority) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // max_connections: optional message google.protobuf.UInt32Value
  pub fn has_max_connections(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_connections(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_connections_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_connections().then(|| self.max_connections())
  }
  pub fn max_connections(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_connections_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_connections(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // max_pending_requests: optional message google.protobuf.UInt32Value
  pub fn has_max_pending_requests(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_max_pending_requests(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn max_pending_requests_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_pending_requests().then(|| self.max_pending_requests())
  }
  pub fn max_pending_requests(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_pending_requests_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_pending_requests(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // max_requests: optional message google.protobuf.UInt32Value
  pub fn has_max_requests(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_max_requests(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn max_requests_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_requests().then(|| self.max_requests())
  }
  pub fn max_requests(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_requests_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_requests(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // max_retries: optional message google.protobuf.UInt32Value
  pub fn has_max_retries(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_max_retries(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn max_retries_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_retries().then(|| self.max_retries())
  }
  pub fn max_retries(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_retries_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_retries(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // retry_budget: optional message envoy.config.cluster.v3.CircuitBreakers.Thresholds.RetryBudget
  pub fn has_retry_budget(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_retry_budget(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn retry_budget_opt(&self) -> ::std::option::Option<super::super::circuit_breakers::thresholds::RetryBudgetView<'_>> {
    self.has_retry_budget().then(|| self.retry_budget())
  }
  pub fn retry_budget(&self) -> super::super::circuit_breakers::thresholds::RetryBudgetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::circuit_breakers::thresholds::RetryBudgetView::default())
  }
  pub fn retry_budget_mut(&mut self) -> super::super::circuit_breakers::thresholds::RetryBudgetMut<'_> {
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
  pub fn set_retry_budget(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::circuit_breakers::thresholds::RetryBudget>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // track_remaining: optional bool
  pub fn track_remaining(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_track_remaining(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // max_connection_pools: optional message google.protobuf.UInt32Value
  pub fn has_max_connection_pools(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_max_connection_pools(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn max_connection_pools_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_connection_pools().then(|| self.max_connection_pools())
  }
  pub fn max_connection_pools(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_connection_pools_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_connection_pools(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

}

// SAFETY:
// - `ThresholdsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ThresholdsMut<'_> {}

// SAFETY:
// - `ThresholdsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ThresholdsMut<'_> {}

impl<'msg> ::protobuf::AsView for ThresholdsMut<'msg> {
  type Proxied = Thresholds;
  fn as_view(&self) -> ::protobuf::View<'_, Thresholds> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ThresholdsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Thresholds>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ThresholdsMut<'msg> {
  type MutProxied = Thresholds;
  fn as_mut(&mut self) -> ThresholdsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ThresholdsMut<'msg> {
  fn into_mut<'shorter>(self) -> ThresholdsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Thresholds {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Thresholds> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ThresholdsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ThresholdsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // priority: optional enum envoy.config.core.v3.RoutingPriority
  pub fn priority(&self) -> crate::xds::generated::envoy::config::core::v3::base::RoutingPriority {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (crate::xds::generated::envoy::config::core::v3::base::RoutingPriority::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_priority(&mut self, val: crate::xds::generated::envoy::config::core::v3::base::RoutingPriority) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // max_connections: optional message google.protobuf.UInt32Value
  pub fn has_max_connections(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_connections(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_connections_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_connections().then(|| self.max_connections())
  }
  pub fn max_connections(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_connections_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_connections(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // max_pending_requests: optional message google.protobuf.UInt32Value
  pub fn has_max_pending_requests(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_max_pending_requests(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn max_pending_requests_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_pending_requests().then(|| self.max_pending_requests())
  }
  pub fn max_pending_requests(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_pending_requests_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_pending_requests(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // max_requests: optional message google.protobuf.UInt32Value
  pub fn has_max_requests(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_max_requests(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn max_requests_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_requests().then(|| self.max_requests())
  }
  pub fn max_requests(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_requests_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_requests(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // max_retries: optional message google.protobuf.UInt32Value
  pub fn has_max_retries(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_max_retries(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn max_retries_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_retries().then(|| self.max_retries())
  }
  pub fn max_retries(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_retries_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_retries(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // retry_budget: optional message envoy.config.cluster.v3.CircuitBreakers.Thresholds.RetryBudget
  pub fn has_retry_budget(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_retry_budget(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn retry_budget_opt(&self) -> ::std::option::Option<super::super::circuit_breakers::thresholds::RetryBudgetView<'_>> {
    self.has_retry_budget().then(|| self.retry_budget())
  }
  pub fn retry_budget(&self) -> super::super::circuit_breakers::thresholds::RetryBudgetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::circuit_breakers::thresholds::RetryBudgetView::default())
  }
  pub fn retry_budget_mut(&mut self) -> super::super::circuit_breakers::thresholds::RetryBudgetMut<'_> {
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
  pub fn set_retry_budget(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::circuit_breakers::thresholds::RetryBudget>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // track_remaining: optional bool
  pub fn track_remaining(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_track_remaining(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // max_connection_pools: optional message google.protobuf.UInt32Value
  pub fn has_max_connection_pools(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_max_connection_pools(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn max_connection_pools_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_connection_pools().then(|| self.max_connection_pools())
  }
  pub fn max_connection_pools(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_connection_pools_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_connection_pools(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

}  // impl Thresholds

impl ::std::ops::Drop for Thresholds {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Thresholds {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Thresholds {
  type Proxied = Self;
  fn as_view(&self) -> ThresholdsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Thresholds {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ThresholdsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Thresholds {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::circuit_breakers::envoy__config__cluster__v3__CircuitBreakers__Thresholds_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P3333/P33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::circuit_breakers::envoy__config__cluster__v3__CircuitBreakers__Thresholds_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::circuit_breakers::thresholds::RetryBudget as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::circuit_breakers::envoy__config__cluster__v3__CircuitBreakers__Thresholds_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Thresholds {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Thresholds {
  type Msg = Thresholds;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Thresholds> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Thresholds {
  type Msg = Thresholds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Thresholds> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ThresholdsMut<'_> {
  type Msg = Thresholds;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Thresholds> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ThresholdsMut<'_> {
  type Msg = Thresholds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Thresholds> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ThresholdsView<'_> {
  type Msg = Thresholds;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Thresholds> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ThresholdsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod thresholds {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__CircuitBreakers__Thresholds__RetryBudget_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RetryBudget {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RetryBudget>
}

impl ::protobuf::Message for RetryBudget {
  type MessageView<'msg> = RetryBudgetView<'msg>;
  type MessageMut<'msg> = RetryBudgetMut<'msg>;
}

impl ::std::default::Default for RetryBudget {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RetryBudget {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RetryBudget` is `Sync` because it does not implement interior mutability.
//    Neither does `RetryBudgetMut`.
unsafe impl ::std::marker::Sync for RetryBudget {}

// SAFETY:
// - `RetryBudget` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RetryBudget {}

impl ::protobuf::Proxied for RetryBudget {
  type View<'msg> = RetryBudgetView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RetryBudget {}

impl ::protobuf::MutProxied for RetryBudget {
  type Mut<'msg> = RetryBudgetMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RetryBudgetView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RetryBudget>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RetryBudgetView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RetryBudgetView<'msg> {
  type Message = RetryBudget;
}

impl ::std::fmt::Debug for RetryBudgetView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RetryBudgetView<'_> {
  fn default() -> RetryBudgetView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RetryBudget>> for RetryBudgetView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RetryBudget>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RetryBudgetView<'msg> {

  pub fn to_owned(&self) -> RetryBudget {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // budget_percent: optional message envoy.type.v3.Percent
  pub fn has_budget_percent(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn budget_percent_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_budget_percent().then(|| self.budget_percent())
  }
  pub fn budget_percent(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

  // min_retry_concurrency: optional message google.protobuf.UInt32Value
  pub fn has_min_retry_concurrency(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn min_retry_concurrency_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_min_retry_concurrency().then(|| self.min_retry_concurrency())
  }
  pub fn min_retry_concurrency(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `RetryBudgetView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RetryBudgetView<'_> {}

// SAFETY:
// - `RetryBudgetView` is `Send` because while its alive a `RetryBudgetMut` cannot.
// - `RetryBudgetView` does not use thread-local data.
unsafe impl ::std::marker::Send for RetryBudgetView<'_> {}

impl<'msg> ::protobuf::AsView for RetryBudgetView<'msg> {
  type Proxied = RetryBudget;
  fn as_view(&self) -> ::protobuf::View<'msg, RetryBudget> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RetryBudgetView<'msg> {
  fn into_view<'shorter>(self) -> RetryBudgetView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RetryBudget> for RetryBudgetView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RetryBudget {
    let mut dst = RetryBudget::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RetryBudget> for RetryBudgetMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RetryBudget {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RetryBudget {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RetryBudgetView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RetryBudgetMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RetryBudgetMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryBudget>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RetryBudgetMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RetryBudgetMut<'msg> {
  type Message = RetryBudget;
}

impl ::std::fmt::Debug for RetryBudgetMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RetryBudget>> for RetryBudgetMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryBudget>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RetryBudgetMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryBudget> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RetryBudget {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // budget_percent: optional message envoy.type.v3.Percent
  pub fn has_budget_percent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_budget_percent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn budget_percent_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_budget_percent().then(|| self.budget_percent())
  }
  pub fn budget_percent(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn budget_percent_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_budget_percent(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // min_retry_concurrency: optional message google.protobuf.UInt32Value
  pub fn has_min_retry_concurrency(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_retry_concurrency(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_retry_concurrency_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_min_retry_concurrency().then(|| self.min_retry_concurrency())
  }
  pub fn min_retry_concurrency(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn min_retry_concurrency_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_min_retry_concurrency(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

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
// - `RetryBudgetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RetryBudgetMut<'_> {}

// SAFETY:
// - `RetryBudgetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RetryBudgetMut<'_> {}

impl<'msg> ::protobuf::AsView for RetryBudgetMut<'msg> {
  type Proxied = RetryBudget;
  fn as_view(&self) -> ::protobuf::View<'_, RetryBudget> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RetryBudgetMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RetryBudget>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RetryBudgetMut<'msg> {
  type MutProxied = RetryBudget;
  fn as_mut(&mut self) -> RetryBudgetMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RetryBudgetMut<'msg> {
  fn into_mut<'shorter>(self) -> RetryBudgetMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RetryBudget {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RetryBudget> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RetryBudgetView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RetryBudgetMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // budget_percent: optional message envoy.type.v3.Percent
  pub fn has_budget_percent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_budget_percent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn budget_percent_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_budget_percent().then(|| self.budget_percent())
  }
  pub fn budget_percent(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn budget_percent_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_budget_percent(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // min_retry_concurrency: optional message google.protobuf.UInt32Value
  pub fn has_min_retry_concurrency(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_retry_concurrency(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_retry_concurrency_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_min_retry_concurrency().then(|| self.min_retry_concurrency())
  }
  pub fn min_retry_concurrency(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn min_retry_concurrency_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_min_retry_concurrency(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl RetryBudget

impl ::std::ops::Drop for RetryBudget {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RetryBudget {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RetryBudget {
  type Proxied = Self;
  fn as_view(&self) -> RetryBudgetView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RetryBudget {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RetryBudgetMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RetryBudget {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::circuit_breakers::thresholds::envoy__config__cluster__v3__CircuitBreakers__Thresholds__RetryBudget_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::circuit_breakers::thresholds::envoy__config__cluster__v3__CircuitBreakers__Thresholds__RetryBudget_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::circuit_breakers::thresholds::envoy__config__cluster__v3__CircuitBreakers__Thresholds__RetryBudget_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RetryBudget {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RetryBudget {
  type Msg = RetryBudget;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryBudget> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryBudget {
  type Msg = RetryBudget;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryBudget> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RetryBudgetMut<'_> {
  type Msg = RetryBudget;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryBudget> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryBudgetMut<'_> {
  type Msg = RetryBudget;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryBudget> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryBudgetView<'_> {
  type Msg = RetryBudget;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryBudget> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RetryBudgetMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod thresholds


}  // pub mod circuit_breakers


