const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__ring_0hash__v3__RingHash_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RingHash {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RingHash>
}

impl ::protobuf::Message for RingHash {
  type MessageView<'msg> = RingHashView<'msg>;
  type MessageMut<'msg> = RingHashMut<'msg>;
}

impl ::std::default::Default for RingHash {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RingHash {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RingHash` is `Sync` because it does not implement interior mutability.
//    Neither does `RingHashMut`.
unsafe impl ::std::marker::Sync for RingHash {}

// SAFETY:
// - `RingHash` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RingHash {}

impl ::protobuf::Proxied for RingHash {
  type View<'msg> = RingHashView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RingHash {}

impl ::protobuf::MutProxied for RingHash {
  type Mut<'msg> = RingHashMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RingHashView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RingHash>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RingHashView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RingHashView<'msg> {
  type Message = RingHash;
}

impl ::std::fmt::Debug for RingHashView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RingHashView<'_> {
  fn default() -> RingHashView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RingHash>> for RingHashView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RingHash>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RingHashView<'msg> {

  pub fn to_owned(&self) -> RingHash {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // hash_function: optional enum envoy.extensions.load_balancing_policies.ring_hash.v3.RingHash.HashFunction
  pub fn hash_function(self) -> super::ring_hash::HashFunction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ring_hash::HashFunction::DefaultHash).into()
      ).try_into().unwrap()
    }
  }

  // minimum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_minimum_ring_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn minimum_ring_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_minimum_ring_size().then(|| self.minimum_ring_size())
  }
  pub fn minimum_ring_size(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

  // maximum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_maximum_ring_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn maximum_ring_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_maximum_ring_size().then(|| self.maximum_ring_size())
  }
  pub fn maximum_ring_size(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

  // use_hostname_for_hashing: optional bool
  pub fn use_hostname_for_hashing(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

  // hash_balance_factor: optional message google.protobuf.UInt32Value
  pub fn has_hash_balance_factor(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn hash_balance_factor_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_hash_balance_factor().then(|| self.hash_balance_factor())
  }
  pub fn hash_balance_factor(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // consistent_hashing_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.ConsistentHashingLbConfig
  pub fn has_consistent_hashing_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn consistent_hashing_lb_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfigView<'msg>> {
    self.has_consistent_hashing_lb_config().then(|| self.consistent_hashing_lb_config())
  }
  pub fn consistent_hashing_lb_config(self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfigView::default())
  }

  // locality_weighted_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.LocalityWeightedLbConfig
  pub fn has_locality_weighted_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn locality_weighted_lb_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfigView<'msg>> {
    self.has_locality_weighted_lb_config().then(|| self.locality_weighted_lb_config())
  }
  pub fn locality_weighted_lb_config(self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfigView::default())
  }

}

// SAFETY:
// - `RingHashView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RingHashView<'_> {}

// SAFETY:
// - `RingHashView` is `Send` because while its alive a `RingHashMut` cannot.
// - `RingHashView` does not use thread-local data.
unsafe impl ::std::marker::Send for RingHashView<'_> {}

impl<'msg> ::protobuf::AsView for RingHashView<'msg> {
  type Proxied = RingHash;
  fn as_view(&self) -> ::protobuf::View<'msg, RingHash> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RingHashView<'msg> {
  fn into_view<'shorter>(self) -> RingHashView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RingHash> for RingHashView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RingHash {
    let mut dst = RingHash::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RingHash> for RingHashMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RingHash {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RingHash {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RingHashView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RingHashMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RingHashMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RingHash>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RingHashMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RingHashMut<'msg> {
  type Message = RingHash;
}

impl ::std::fmt::Debug for RingHashMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RingHash>> for RingHashMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RingHash>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RingHashMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RingHash> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RingHash {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // hash_function: optional enum envoy.extensions.load_balancing_policies.ring_hash.v3.RingHash.HashFunction
  pub fn hash_function(&self) -> super::ring_hash::HashFunction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ring_hash::HashFunction::DefaultHash).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_hash_function(&mut self, val: super::ring_hash::HashFunction) {
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

  // minimum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_minimum_ring_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_minimum_ring_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn minimum_ring_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_minimum_ring_size().then(|| self.minimum_ring_size())
  }
  pub fn minimum_ring_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn minimum_ring_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_minimum_ring_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // maximum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_maximum_ring_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_maximum_ring_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn maximum_ring_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_maximum_ring_size().then(|| self.maximum_ring_size())
  }
  pub fn maximum_ring_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn maximum_ring_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_maximum_ring_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // use_hostname_for_hashing: optional bool
  pub fn use_hostname_for_hashing(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_use_hostname_for_hashing(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // hash_balance_factor: optional message google.protobuf.UInt32Value
  pub fn has_hash_balance_factor(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_hash_balance_factor(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn hash_balance_factor_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_hash_balance_factor().then(|| self.hash_balance_factor())
  }
  pub fn hash_balance_factor(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn hash_balance_factor_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_hash_balance_factor(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // consistent_hashing_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.ConsistentHashingLbConfig
  pub fn has_consistent_hashing_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_consistent_hashing_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn consistent_hashing_lb_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfigView<'_>> {
    self.has_consistent_hashing_lb_config().then(|| self.consistent_hashing_lb_config())
  }
  pub fn consistent_hashing_lb_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfigView::default())
  }
  pub fn consistent_hashing_lb_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfigMut<'_> {
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
  pub fn set_consistent_hashing_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // locality_weighted_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.LocalityWeightedLbConfig
  pub fn has_locality_weighted_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_locality_weighted_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn locality_weighted_lb_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfigView<'_>> {
    self.has_locality_weighted_lb_config().then(|| self.locality_weighted_lb_config())
  }
  pub fn locality_weighted_lb_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfigView::default())
  }
  pub fn locality_weighted_lb_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfigMut<'_> {
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
  pub fn set_locality_weighted_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfig>) {

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
// - `RingHashMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RingHashMut<'_> {}

// SAFETY:
// - `RingHashMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RingHashMut<'_> {}

impl<'msg> ::protobuf::AsView for RingHashMut<'msg> {
  type Proxied = RingHash;
  fn as_view(&self) -> ::protobuf::View<'_, RingHash> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RingHashMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RingHash>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RingHashMut<'msg> {
  type MutProxied = RingHash;
  fn as_mut(&mut self) -> RingHashMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RingHashMut<'msg> {
  fn into_mut<'shorter>(self) -> RingHashMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RingHash {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RingHash> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RingHashView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RingHashMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // hash_function: optional enum envoy.extensions.load_balancing_policies.ring_hash.v3.RingHash.HashFunction
  pub fn hash_function(&self) -> super::ring_hash::HashFunction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ring_hash::HashFunction::DefaultHash).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_hash_function(&mut self, val: super::ring_hash::HashFunction) {
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

  // minimum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_minimum_ring_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_minimum_ring_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn minimum_ring_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_minimum_ring_size().then(|| self.minimum_ring_size())
  }
  pub fn minimum_ring_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn minimum_ring_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_minimum_ring_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // maximum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_maximum_ring_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_maximum_ring_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn maximum_ring_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_maximum_ring_size().then(|| self.maximum_ring_size())
  }
  pub fn maximum_ring_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn maximum_ring_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_maximum_ring_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // use_hostname_for_hashing: optional bool
  pub fn use_hostname_for_hashing(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_use_hostname_for_hashing(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // hash_balance_factor: optional message google.protobuf.UInt32Value
  pub fn has_hash_balance_factor(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_hash_balance_factor(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn hash_balance_factor_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_hash_balance_factor().then(|| self.hash_balance_factor())
  }
  pub fn hash_balance_factor(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn hash_balance_factor_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_hash_balance_factor(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // consistent_hashing_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.ConsistentHashingLbConfig
  pub fn has_consistent_hashing_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_consistent_hashing_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn consistent_hashing_lb_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfigView<'_>> {
    self.has_consistent_hashing_lb_config().then(|| self.consistent_hashing_lb_config())
  }
  pub fn consistent_hashing_lb_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfigView::default())
  }
  pub fn consistent_hashing_lb_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfigMut<'_> {
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
  pub fn set_consistent_hashing_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // locality_weighted_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.LocalityWeightedLbConfig
  pub fn has_locality_weighted_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_locality_weighted_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn locality_weighted_lb_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfigView<'_>> {
    self.has_locality_weighted_lb_config().then(|| self.locality_weighted_lb_config())
  }
  pub fn locality_weighted_lb_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfigView::default())
  }
  pub fn locality_weighted_lb_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfigMut<'_> {
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
  pub fn set_locality_weighted_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

}  // impl RingHash

impl ::std::ops::Drop for RingHash {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RingHash {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RingHash {
  type Proxied = Self;
  fn as_view(&self) -> RingHashView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RingHash {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RingHashMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RingHash {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__load_0balancing_0policies__ring_0hash__v3__RingHash_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P33/P333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__load_0balancing_0policies__ring_0hash__v3__RingHash_msg_init.0, &[<::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::ConsistentHashingLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::locality_lb_config::LocalityWeightedLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__load_0balancing_0policies__ring_0hash__v3__RingHash_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RingHash {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RingHash {
  type Msg = RingHash;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RingHash> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RingHash {
  type Msg = RingHash;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RingHash> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RingHashMut<'_> {
  type Msg = RingHash;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RingHash> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RingHashMut<'_> {
  type Msg = RingHash;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RingHash> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RingHashView<'_> {
  type Msg = RingHash;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RingHash> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RingHashMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod ring_hash {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HashFunction(i32);

#[allow(non_upper_case_globals)]
impl HashFunction {
  pub const DefaultHash: HashFunction = HashFunction(0);
  pub const XxHash: HashFunction = HashFunction(1);
  pub const MurmurHash2: HashFunction = HashFunction(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "DefaultHash",
      1 => "XxHash",
      2 => "MurmurHash2",
      _ => return None
    })
  }
}

impl ::std::convert::From<HashFunction> for i32 {
  fn from(val: HashFunction) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for HashFunction {
  fn from(val: i32) -> HashFunction {
    Self(val)
  }
}

impl ::std::default::Default for HashFunction {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for HashFunction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "HashFunction::{}", constant_name)
    } else {
      write!(f, "HashFunction::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for HashFunction {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for HashFunction {}

impl ::protobuf::Proxied for HashFunction {
  type View<'a> = HashFunction;
}

impl ::protobuf::AsView for HashFunction {
  type Proxied = HashFunction;

  fn as_view(&self) -> HashFunction {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HashFunction {
  fn into_view<'shorter>(self) -> HashFunction where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for HashFunction {
  const NAME: &'static str = "HashFunction";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for HashFunction {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod ring_hash


