const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__wrr_0locality__v3__WrrLocality_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct WrrLocality {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<WrrLocality>
}

impl ::protobuf::Message for WrrLocality {
  type MessageView<'msg> = WrrLocalityView<'msg>;
  type MessageMut<'msg> = WrrLocalityMut<'msg>;
}

impl ::std::default::Default for WrrLocality {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for WrrLocality {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `WrrLocality` is `Sync` because it does not implement interior mutability.
//    Neither does `WrrLocalityMut`.
unsafe impl ::std::marker::Sync for WrrLocality {}

// SAFETY:
// - `WrrLocality` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for WrrLocality {}

impl ::protobuf::Proxied for WrrLocality {
  type View<'msg> = WrrLocalityView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for WrrLocality {}

impl ::protobuf::MutProxied for WrrLocality {
  type Mut<'msg> = WrrLocalityMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct WrrLocalityView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, WrrLocality>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WrrLocalityView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for WrrLocalityView<'msg> {
  type Message = WrrLocality;
}

impl ::std::fmt::Debug for WrrLocalityView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for WrrLocalityView<'_> {
  fn default() -> WrrLocalityView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, WrrLocality>> for WrrLocalityView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, WrrLocality>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WrrLocalityView<'msg> {

  pub fn to_owned(&self) -> WrrLocality {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // endpoint_picking_policy: optional message envoy.config.cluster.v3.LoadBalancingPolicy
  pub fn has_endpoint_picking_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn endpoint_picking_policy_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicyView<'msg>> {
    self.has_endpoint_picking_policy().then(|| self.endpoint_picking_policy())
  }
  pub fn endpoint_picking_policy(self) -> crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicyView::default())
  }

}

// SAFETY:
// - `WrrLocalityView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for WrrLocalityView<'_> {}

// SAFETY:
// - `WrrLocalityView` is `Send` because while its alive a `WrrLocalityMut` cannot.
// - `WrrLocalityView` does not use thread-local data.
unsafe impl ::std::marker::Send for WrrLocalityView<'_> {}

impl<'msg> ::protobuf::AsView for WrrLocalityView<'msg> {
  type Proxied = WrrLocality;
  fn as_view(&self) -> ::protobuf::View<'msg, WrrLocality> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WrrLocalityView<'msg> {
  fn into_view<'shorter>(self) -> WrrLocalityView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<WrrLocality> for WrrLocalityView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> WrrLocality {
    let mut dst = WrrLocality::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<WrrLocality> for WrrLocalityMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> WrrLocality {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for WrrLocality {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for WrrLocalityView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for WrrLocalityMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct WrrLocalityMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, WrrLocality>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WrrLocalityMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for WrrLocalityMut<'msg> {
  type Message = WrrLocality;
}

impl ::std::fmt::Debug for WrrLocalityMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, WrrLocality>> for WrrLocalityMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, WrrLocality>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WrrLocalityMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, WrrLocality> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> WrrLocality {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // endpoint_picking_policy: optional message envoy.config.cluster.v3.LoadBalancingPolicy
  pub fn has_endpoint_picking_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_endpoint_picking_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn endpoint_picking_policy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicyView<'_>> {
    self.has_endpoint_picking_policy().then(|| self.endpoint_picking_policy())
  }
  pub fn endpoint_picking_policy(&self) -> crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicyView::default())
  }
  pub fn endpoint_picking_policy_mut(&mut self) -> crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicyMut<'_> {
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
  pub fn set_endpoint_picking_policy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `WrrLocalityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for WrrLocalityMut<'_> {}

// SAFETY:
// - `WrrLocalityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for WrrLocalityMut<'_> {}

impl<'msg> ::protobuf::AsView for WrrLocalityMut<'msg> {
  type Proxied = WrrLocality;
  fn as_view(&self) -> ::protobuf::View<'_, WrrLocality> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WrrLocalityMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, WrrLocality>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for WrrLocalityMut<'msg> {
  type MutProxied = WrrLocality;
  fn as_mut(&mut self) -> WrrLocalityMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for WrrLocalityMut<'msg> {
  fn into_mut<'shorter>(self) -> WrrLocalityMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl WrrLocality {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, WrrLocality> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> WrrLocalityView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> WrrLocalityMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // endpoint_picking_policy: optional message envoy.config.cluster.v3.LoadBalancingPolicy
  pub fn has_endpoint_picking_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_endpoint_picking_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn endpoint_picking_policy_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicyView<'_>> {
    self.has_endpoint_picking_policy().then(|| self.endpoint_picking_policy())
  }
  pub fn endpoint_picking_policy(&self) -> crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicyView::default())
  }
  pub fn endpoint_picking_policy_mut(&mut self) -> crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicyMut<'_> {
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
  pub fn set_endpoint_picking_policy(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl WrrLocality

impl ::std::ops::Drop for WrrLocality {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for WrrLocality {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for WrrLocality {
  type Proxied = Self;
  fn as_view(&self) -> WrrLocalityView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for WrrLocality {
  type MutProxied = Self;
  fn as_mut(&mut self) -> WrrLocalityMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for WrrLocality {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__load_0balancing_0policies__wrr_0locality__v3__WrrLocality_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__load_0balancing_0policies__wrr_0locality__v3__WrrLocality_msg_init.0, &[<crate::xds::generated::envoy::config::cluster::v3::cluster::LoadBalancingPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__load_0balancing_0policies__wrr_0locality__v3__WrrLocality_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WrrLocality {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WrrLocality {
  type Msg = WrrLocality;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WrrLocality> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WrrLocality {
  type Msg = WrrLocality;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WrrLocality> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WrrLocalityMut<'_> {
  type Msg = WrrLocality;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WrrLocality> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WrrLocalityMut<'_> {
  type Msg = WrrLocality;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WrrLocality> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WrrLocalityView<'_> {
  type Msg = WrrLocality;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WrrLocality> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WrrLocalityMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



