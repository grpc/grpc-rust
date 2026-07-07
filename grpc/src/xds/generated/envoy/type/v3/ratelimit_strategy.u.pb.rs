const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__v3__RateLimitStrategy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RateLimitStrategy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RateLimitStrategy>
}

impl ::protobuf::Message for RateLimitStrategy {
  type MessageView<'msg> = RateLimitStrategyView<'msg>;
  type MessageMut<'msg> = RateLimitStrategyMut<'msg>;
}

impl ::std::default::Default for RateLimitStrategy {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RateLimitStrategy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RateLimitStrategy` is `Sync` because it does not implement interior mutability.
//    Neither does `RateLimitStrategyMut`.
unsafe impl ::std::marker::Sync for RateLimitStrategy {}

// SAFETY:
// - `RateLimitStrategy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitStrategy {}

impl ::protobuf::Proxied for RateLimitStrategy {
  type View<'msg> = RateLimitStrategyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RateLimitStrategy {}

impl ::protobuf::MutProxied for RateLimitStrategy {
  type Mut<'msg> = RateLimitStrategyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RateLimitStrategyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitStrategy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitStrategyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RateLimitStrategyView<'msg> {
  type Message = RateLimitStrategy;
}

impl ::std::fmt::Debug for RateLimitStrategyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RateLimitStrategyView<'_> {
  fn default() -> RateLimitStrategyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitStrategy>> for RateLimitStrategyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RateLimitStrategy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitStrategyView<'msg> {

  pub fn to_owned(&self) -> RateLimitStrategy {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // blanket_rule: optional enum envoy.type.v3.RateLimitStrategy.BlanketRule
  pub fn has_blanket_rule(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn blanket_rule_opt(self) -> ::std::option::Option<super::rate_limit_strategy::BlanketRule> {
    self.has_blanket_rule().then(|| self.blanket_rule())
  }
  pub fn blanket_rule(self) -> super::rate_limit_strategy::BlanketRule {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::rate_limit_strategy::BlanketRule::AllowAll).into()
      ).try_into().unwrap()
    }
  }

  // requests_per_time_unit: optional message envoy.type.v3.RateLimitStrategy.RequestsPerTimeUnit
  pub fn has_requests_per_time_unit(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn requests_per_time_unit_opt(self) -> ::std::option::Option<super::rate_limit_strategy::RequestsPerTimeUnitView<'msg>> {
    self.has_requests_per_time_unit().then(|| self.requests_per_time_unit())
  }
  pub fn requests_per_time_unit(self) -> super::rate_limit_strategy::RequestsPerTimeUnitView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_strategy::RequestsPerTimeUnitView::default())
  }

  // token_bucket: optional message envoy.type.v3.TokenBucket
  pub fn has_token_bucket(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn token_bucket_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucketView<'msg>> {
    self.has_token_bucket().then(|| self.token_bucket())
  }
  pub fn token_bucket(self) -> crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucketView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucketView::default())
  }

  pub fn strategy(self) -> super::rate_limit_strategy::StrategyOneof<'msg> {
    match self.strategy_case() {
      super::rate_limit_strategy::StrategyCase::BlanketRule =>
          super::rate_limit_strategy::StrategyOneof::BlanketRule(self.blanket_rule()),
      super::rate_limit_strategy::StrategyCase::RequestsPerTimeUnit =>
          super::rate_limit_strategy::StrategyOneof::RequestsPerTimeUnit(self.requests_per_time_unit()),
      super::rate_limit_strategy::StrategyCase::TokenBucket =>
          super::rate_limit_strategy::StrategyOneof::TokenBucket(self.token_bucket()),
      _ => super::rate_limit_strategy::StrategyOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn strategy_case(self) -> super::rate_limit_strategy::StrategyCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::rate_limit_strategy::StrategyCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RateLimitStrategyView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RateLimitStrategyView<'_> {}

// SAFETY:
// - `RateLimitStrategyView` is `Send` because while its alive a `RateLimitStrategyMut` cannot.
// - `RateLimitStrategyView` does not use thread-local data.
unsafe impl ::std::marker::Send for RateLimitStrategyView<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitStrategyView<'msg> {
  type Proxied = RateLimitStrategy;
  fn as_view(&self) -> ::protobuf::View<'msg, RateLimitStrategy> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitStrategyView<'msg> {
  fn into_view<'shorter>(self) -> RateLimitStrategyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitStrategy> for RateLimitStrategyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitStrategy {
    let mut dst = RateLimitStrategy::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RateLimitStrategy> for RateLimitStrategyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RateLimitStrategy {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RateLimitStrategy {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitStrategyView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RateLimitStrategyMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RateLimitStrategyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitStrategy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RateLimitStrategyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RateLimitStrategyMut<'msg> {
  type Message = RateLimitStrategy;
}

impl ::std::fmt::Debug for RateLimitStrategyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitStrategy>> for RateLimitStrategyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitStrategy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RateLimitStrategyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RateLimitStrategy> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RateLimitStrategy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // blanket_rule: optional enum envoy.type.v3.RateLimitStrategy.BlanketRule
  pub fn has_blanket_rule(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_blanket_rule(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn blanket_rule_opt(&self) -> ::std::option::Option<super::rate_limit_strategy::BlanketRule> {
    self.has_blanket_rule().then(|| self.blanket_rule())
  }
  pub fn blanket_rule(&self) -> super::rate_limit_strategy::BlanketRule {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::rate_limit_strategy::BlanketRule::AllowAll).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_blanket_rule(&mut self, val: super::rate_limit_strategy::BlanketRule) {
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

  // requests_per_time_unit: optional message envoy.type.v3.RateLimitStrategy.RequestsPerTimeUnit
  pub fn has_requests_per_time_unit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_requests_per_time_unit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn requests_per_time_unit_opt(&self) -> ::std::option::Option<super::rate_limit_strategy::RequestsPerTimeUnitView<'_>> {
    self.has_requests_per_time_unit().then(|| self.requests_per_time_unit())
  }
  pub fn requests_per_time_unit(&self) -> super::rate_limit_strategy::RequestsPerTimeUnitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_strategy::RequestsPerTimeUnitView::default())
  }
  pub fn requests_per_time_unit_mut(&mut self) -> super::rate_limit_strategy::RequestsPerTimeUnitMut<'_> {
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
  pub fn set_requests_per_time_unit(&mut self,
    val: impl ::protobuf::IntoProxied<super::rate_limit_strategy::RequestsPerTimeUnit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // token_bucket: optional message envoy.type.v3.TokenBucket
  pub fn has_token_bucket(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_token_bucket(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn token_bucket_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucketView<'_>> {
    self.has_token_bucket().then(|| self.token_bucket())
  }
  pub fn token_bucket(&self) -> crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucketView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucketView::default())
  }
  pub fn token_bucket_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucketMut<'_> {
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
  pub fn set_token_bucket(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucket>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn strategy(&self) -> super::rate_limit_strategy::StrategyOneof<'_> {
    match &self.strategy_case() {
      super::rate_limit_strategy::StrategyCase::BlanketRule =>
          super::rate_limit_strategy::StrategyOneof::BlanketRule(self.blanket_rule()),
      super::rate_limit_strategy::StrategyCase::RequestsPerTimeUnit =>
          super::rate_limit_strategy::StrategyOneof::RequestsPerTimeUnit(self.requests_per_time_unit()),
      super::rate_limit_strategy::StrategyCase::TokenBucket =>
          super::rate_limit_strategy::StrategyOneof::TokenBucket(self.token_bucket()),
      _ => super::rate_limit_strategy::StrategyOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn strategy_case(&self) -> super::rate_limit_strategy::StrategyCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::rate_limit_strategy::StrategyCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RateLimitStrategyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RateLimitStrategyMut<'_> {}

// SAFETY:
// - `RateLimitStrategyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RateLimitStrategyMut<'_> {}

impl<'msg> ::protobuf::AsView for RateLimitStrategyMut<'msg> {
  type Proxied = RateLimitStrategy;
  fn as_view(&self) -> ::protobuf::View<'_, RateLimitStrategy> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RateLimitStrategyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RateLimitStrategy>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RateLimitStrategyMut<'msg> {
  type MutProxied = RateLimitStrategy;
  fn as_mut(&mut self) -> RateLimitStrategyMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RateLimitStrategyMut<'msg> {
  fn into_mut<'shorter>(self) -> RateLimitStrategyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RateLimitStrategy {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RateLimitStrategy> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RateLimitStrategyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RateLimitStrategyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // blanket_rule: optional enum envoy.type.v3.RateLimitStrategy.BlanketRule
  pub fn has_blanket_rule(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_blanket_rule(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn blanket_rule_opt(&self) -> ::std::option::Option<super::rate_limit_strategy::BlanketRule> {
    self.has_blanket_rule().then(|| self.blanket_rule())
  }
  pub fn blanket_rule(&self) -> super::rate_limit_strategy::BlanketRule {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::rate_limit_strategy::BlanketRule::AllowAll).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_blanket_rule(&mut self, val: super::rate_limit_strategy::BlanketRule) {
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

  // requests_per_time_unit: optional message envoy.type.v3.RateLimitStrategy.RequestsPerTimeUnit
  pub fn has_requests_per_time_unit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_requests_per_time_unit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn requests_per_time_unit_opt(&self) -> ::std::option::Option<super::rate_limit_strategy::RequestsPerTimeUnitView<'_>> {
    self.has_requests_per_time_unit().then(|| self.requests_per_time_unit())
  }
  pub fn requests_per_time_unit(&self) -> super::rate_limit_strategy::RequestsPerTimeUnitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::rate_limit_strategy::RequestsPerTimeUnitView::default())
  }
  pub fn requests_per_time_unit_mut(&mut self) -> super::rate_limit_strategy::RequestsPerTimeUnitMut<'_> {
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
  pub fn set_requests_per_time_unit(&mut self,
    val: impl ::protobuf::IntoProxied<super::rate_limit_strategy::RequestsPerTimeUnit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // token_bucket: optional message envoy.type.v3.TokenBucket
  pub fn has_token_bucket(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_token_bucket(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn token_bucket_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucketView<'_>> {
    self.has_token_bucket().then(|| self.token_bucket())
  }
  pub fn token_bucket(&self) -> crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucketView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucketView::default())
  }
  pub fn token_bucket_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucketMut<'_> {
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
  pub fn set_token_bucket(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucket>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn strategy(&self) -> super::rate_limit_strategy::StrategyOneof<'_> {
    match &self.strategy_case() {
      super::rate_limit_strategy::StrategyCase::BlanketRule =>
          super::rate_limit_strategy::StrategyOneof::BlanketRule(self.blanket_rule()),
      super::rate_limit_strategy::StrategyCase::RequestsPerTimeUnit =>
          super::rate_limit_strategy::StrategyOneof::RequestsPerTimeUnit(self.requests_per_time_unit()),
      super::rate_limit_strategy::StrategyCase::TokenBucket =>
          super::rate_limit_strategy::StrategyOneof::TokenBucket(self.token_bucket()),
      _ => super::rate_limit_strategy::StrategyOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn strategy_case(&self) -> super::rate_limit_strategy::StrategyCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::rate_limit_strategy::StrategyCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl RateLimitStrategy

impl ::std::ops::Drop for RateLimitStrategy {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RateLimitStrategy {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RateLimitStrategy {
  type Proxied = Self;
  fn as_view(&self) -> RateLimitStrategyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RateLimitStrategy {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RateLimitStrategyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RateLimitStrategy {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__v3__RateLimitStrategy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.33^!|#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__v3__RateLimitStrategy_msg_init.0, &[<super::rate_limit_strategy::RequestsPerTimeUnit as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucket as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__v3__RateLimitStrategy_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitStrategy {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitStrategy {
  type Msg = RateLimitStrategy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitStrategy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitStrategy {
  type Msg = RateLimitStrategy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitStrategy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RateLimitStrategyMut<'_> {
  type Msg = RateLimitStrategy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitStrategy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitStrategyMut<'_> {
  type Msg = RateLimitStrategy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitStrategy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RateLimitStrategyView<'_> {
  type Msg = RateLimitStrategy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RateLimitStrategy> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RateLimitStrategyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod rate_limit_strategy {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__v3__RateLimitStrategy__RequestsPerTimeUnit_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RequestsPerTimeUnit {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RequestsPerTimeUnit>
}

impl ::protobuf::Message for RequestsPerTimeUnit {
  type MessageView<'msg> = RequestsPerTimeUnitView<'msg>;
  type MessageMut<'msg> = RequestsPerTimeUnitMut<'msg>;
}

impl ::std::default::Default for RequestsPerTimeUnit {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RequestsPerTimeUnit {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RequestsPerTimeUnit` is `Sync` because it does not implement interior mutability.
//    Neither does `RequestsPerTimeUnitMut`.
unsafe impl ::std::marker::Sync for RequestsPerTimeUnit {}

// SAFETY:
// - `RequestsPerTimeUnit` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RequestsPerTimeUnit {}

impl ::protobuf::Proxied for RequestsPerTimeUnit {
  type View<'msg> = RequestsPerTimeUnitView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RequestsPerTimeUnit {}

impl ::protobuf::MutProxied for RequestsPerTimeUnit {
  type Mut<'msg> = RequestsPerTimeUnitMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RequestsPerTimeUnitView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RequestsPerTimeUnit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RequestsPerTimeUnitView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RequestsPerTimeUnitView<'msg> {
  type Message = RequestsPerTimeUnit;
}

impl ::std::fmt::Debug for RequestsPerTimeUnitView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RequestsPerTimeUnitView<'_> {
  fn default() -> RequestsPerTimeUnitView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RequestsPerTimeUnit>> for RequestsPerTimeUnitView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RequestsPerTimeUnit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RequestsPerTimeUnitView<'msg> {

  pub fn to_owned(&self) -> RequestsPerTimeUnit {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // requests_per_time_unit: optional uint64
  pub fn requests_per_time_unit(self) -> u64 {
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

  // time_unit: optional enum envoy.type.v3.RateLimitUnit
  pub fn time_unit(self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_unit::RateLimitUnit {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (crate::xds::generated::envoy::r#type::v3::ratelimit_unit::RateLimitUnit::Unknown).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `RequestsPerTimeUnitView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RequestsPerTimeUnitView<'_> {}

// SAFETY:
// - `RequestsPerTimeUnitView` is `Send` because while its alive a `RequestsPerTimeUnitMut` cannot.
// - `RequestsPerTimeUnitView` does not use thread-local data.
unsafe impl ::std::marker::Send for RequestsPerTimeUnitView<'_> {}

impl<'msg> ::protobuf::AsView for RequestsPerTimeUnitView<'msg> {
  type Proxied = RequestsPerTimeUnit;
  fn as_view(&self) -> ::protobuf::View<'msg, RequestsPerTimeUnit> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RequestsPerTimeUnitView<'msg> {
  fn into_view<'shorter>(self) -> RequestsPerTimeUnitView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RequestsPerTimeUnit> for RequestsPerTimeUnitView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RequestsPerTimeUnit {
    let mut dst = RequestsPerTimeUnit::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RequestsPerTimeUnit> for RequestsPerTimeUnitMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RequestsPerTimeUnit {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RequestsPerTimeUnit {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RequestsPerTimeUnitView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RequestsPerTimeUnitMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RequestsPerTimeUnitMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RequestsPerTimeUnit>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RequestsPerTimeUnitMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RequestsPerTimeUnitMut<'msg> {
  type Message = RequestsPerTimeUnit;
}

impl ::std::fmt::Debug for RequestsPerTimeUnitMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RequestsPerTimeUnit>> for RequestsPerTimeUnitMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RequestsPerTimeUnit>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RequestsPerTimeUnitMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RequestsPerTimeUnit> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RequestsPerTimeUnit {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // requests_per_time_unit: optional uint64
  pub fn requests_per_time_unit(&self) -> u64 {
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
  pub fn set_requests_per_time_unit(&mut self, val: u64) {
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

  // time_unit: optional enum envoy.type.v3.RateLimitUnit
  pub fn time_unit(&self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_unit::RateLimitUnit {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (crate::xds::generated::envoy::r#type::v3::ratelimit_unit::RateLimitUnit::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_time_unit(&mut self, val: crate::xds::generated::envoy::r#type::v3::ratelimit_unit::RateLimitUnit) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `RequestsPerTimeUnitMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RequestsPerTimeUnitMut<'_> {}

// SAFETY:
// - `RequestsPerTimeUnitMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RequestsPerTimeUnitMut<'_> {}

impl<'msg> ::protobuf::AsView for RequestsPerTimeUnitMut<'msg> {
  type Proxied = RequestsPerTimeUnit;
  fn as_view(&self) -> ::protobuf::View<'_, RequestsPerTimeUnit> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RequestsPerTimeUnitMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RequestsPerTimeUnit>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RequestsPerTimeUnitMut<'msg> {
  type MutProxied = RequestsPerTimeUnit;
  fn as_mut(&mut self) -> RequestsPerTimeUnitMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RequestsPerTimeUnitMut<'msg> {
  fn into_mut<'shorter>(self) -> RequestsPerTimeUnitMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RequestsPerTimeUnit {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RequestsPerTimeUnit> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RequestsPerTimeUnitView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RequestsPerTimeUnitMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // requests_per_time_unit: optional uint64
  pub fn requests_per_time_unit(&self) -> u64 {
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
  pub fn set_requests_per_time_unit(&mut self, val: u64) {
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

  // time_unit: optional enum envoy.type.v3.RateLimitUnit
  pub fn time_unit(&self) -> crate::xds::generated::envoy::r#type::v3::ratelimit_unit::RateLimitUnit {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (crate::xds::generated::envoy::r#type::v3::ratelimit_unit::RateLimitUnit::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_time_unit(&mut self, val: crate::xds::generated::envoy::r#type::v3::ratelimit_unit::RateLimitUnit) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}  // impl RequestsPerTimeUnit

impl ::std::ops::Drop for RequestsPerTimeUnit {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RequestsPerTimeUnit {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RequestsPerTimeUnit {
  type Proxied = Self;
  fn as_view(&self) -> RequestsPerTimeUnitView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RequestsPerTimeUnit {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RequestsPerTimeUnitMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RequestsPerTimeUnit {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::rate_limit_strategy::envoy__type__v3__RateLimitStrategy__RequestsPerTimeUnit_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$,P.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::rate_limit_strategy::envoy__type__v3__RateLimitStrategy__RequestsPerTimeUnit_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::rate_limit_strategy::envoy__type__v3__RateLimitStrategy__RequestsPerTimeUnit_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RequestsPerTimeUnit {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RequestsPerTimeUnit {
  type Msg = RequestsPerTimeUnit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RequestsPerTimeUnit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RequestsPerTimeUnit {
  type Msg = RequestsPerTimeUnit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RequestsPerTimeUnit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RequestsPerTimeUnitMut<'_> {
  type Msg = RequestsPerTimeUnit;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RequestsPerTimeUnit> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RequestsPerTimeUnitMut<'_> {
  type Msg = RequestsPerTimeUnit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RequestsPerTimeUnit> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RequestsPerTimeUnitView<'_> {
  type Msg = RequestsPerTimeUnit;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RequestsPerTimeUnit> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RequestsPerTimeUnitMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlanketRule(i32);

#[allow(non_upper_case_globals)]
impl BlanketRule {
  pub const AllowAll: BlanketRule = BlanketRule(0);
  pub const DenyAll: BlanketRule = BlanketRule(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "AllowAll",
      1 => "DenyAll",
      _ => return None
    })
  }
}

impl ::std::convert::From<BlanketRule> for i32 {
  fn from(val: BlanketRule) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for BlanketRule {
  fn from(val: i32) -> BlanketRule {
    Self(val)
  }
}

impl ::std::default::Default for BlanketRule {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for BlanketRule {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "BlanketRule::{}", constant_name)
    } else {
      write!(f, "BlanketRule::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for BlanketRule {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for BlanketRule {}

impl ::protobuf::Proxied for BlanketRule {
  type View<'a> = BlanketRule;
}

impl ::protobuf::AsView for BlanketRule {
  type Proxied = BlanketRule;

  fn as_view(&self) -> BlanketRule {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BlanketRule {
  fn into_view<'shorter>(self) -> BlanketRule where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for BlanketRule {
  const NAME: &'static str = "BlanketRule";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for BlanketRule {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum StrategyOneof<'msg> {
  BlanketRule(::protobuf::View<'msg, super::super::rate_limit_strategy::BlanketRule>) = 1,
  RequestsPerTimeUnit(::protobuf::View<'msg, super::super::rate_limit_strategy::RequestsPerTimeUnit>) = 2,
  TokenBucket(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::v3::token_bucket::TokenBucket>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum StrategyCase {
  BlanketRule = 1,
  RequestsPerTimeUnit = 2,
  TokenBucket = 3,

  not_set = 0
}

impl StrategyCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<StrategyCase> {
    match v {
      0 => Some(StrategyCase::not_set),
      1 => Some(StrategyCase::BlanketRule),
      2 => Some(StrategyCase::RequestsPerTimeUnit),
      3 => Some(StrategyCase::TokenBucket),
      _ => None
    }
  }
}
}  // pub mod rate_limit_strategy


