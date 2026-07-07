const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__round_0robin__v3__RoundRobin_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RoundRobin {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RoundRobin>
}

impl ::protobuf::Message for RoundRobin {
  type MessageView<'msg> = RoundRobinView<'msg>;
  type MessageMut<'msg> = RoundRobinMut<'msg>;
}

impl ::std::default::Default for RoundRobin {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RoundRobin {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RoundRobin` is `Sync` because it does not implement interior mutability.
//    Neither does `RoundRobinMut`.
unsafe impl ::std::marker::Sync for RoundRobin {}

// SAFETY:
// - `RoundRobin` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RoundRobin {}

impl ::protobuf::Proxied for RoundRobin {
  type View<'msg> = RoundRobinView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RoundRobin {}

impl ::protobuf::MutProxied for RoundRobin {
  type Mut<'msg> = RoundRobinMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RoundRobinView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RoundRobin>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RoundRobinView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RoundRobinView<'msg> {
  type Message = RoundRobin;
}

impl ::std::fmt::Debug for RoundRobinView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RoundRobinView<'_> {
  fn default() -> RoundRobinView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RoundRobin>> for RoundRobinView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RoundRobin>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RoundRobinView<'msg> {

  pub fn to_owned(&self) -> RoundRobin {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // slow_start_config: optional message envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig
  pub fn has_slow_start_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn slow_start_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'msg>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView::default())
  }

  // locality_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig
  pub fn has_locality_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn locality_lb_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'msg>> {
    self.has_locality_lb_config().then(|| self.locality_lb_config())
  }
  pub fn locality_lb_config(self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView::default())
  }

}

// SAFETY:
// - `RoundRobinView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RoundRobinView<'_> {}

// SAFETY:
// - `RoundRobinView` is `Send` because while its alive a `RoundRobinMut` cannot.
// - `RoundRobinView` does not use thread-local data.
unsafe impl ::std::marker::Send for RoundRobinView<'_> {}

impl<'msg> ::protobuf::AsView for RoundRobinView<'msg> {
  type Proxied = RoundRobin;
  fn as_view(&self) -> ::protobuf::View<'msg, RoundRobin> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RoundRobinView<'msg> {
  fn into_view<'shorter>(self) -> RoundRobinView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RoundRobin> for RoundRobinView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RoundRobin {
    let mut dst = RoundRobin::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RoundRobin> for RoundRobinMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RoundRobin {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RoundRobin {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RoundRobinView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RoundRobinMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RoundRobinMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RoundRobin>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RoundRobinMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RoundRobinMut<'msg> {
  type Message = RoundRobin;
}

impl ::std::fmt::Debug for RoundRobinMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RoundRobin>> for RoundRobinMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RoundRobin>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RoundRobinMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RoundRobin> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RoundRobin {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // slow_start_config: optional message envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig
  pub fn has_slow_start_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_slow_start_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn slow_start_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView::default())
  }
  pub fn slow_start_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigMut<'_> {
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
  pub fn set_slow_start_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // locality_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig
  pub fn has_locality_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_locality_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn locality_lb_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'_>> {
    self.has_locality_lb_config().then(|| self.locality_lb_config())
  }
  pub fn locality_lb_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView::default())
  }
  pub fn locality_lb_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigMut<'_> {
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
  pub fn set_locality_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfig>) {

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
// - `RoundRobinMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RoundRobinMut<'_> {}

// SAFETY:
// - `RoundRobinMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RoundRobinMut<'_> {}

impl<'msg> ::protobuf::AsView for RoundRobinMut<'msg> {
  type Proxied = RoundRobin;
  fn as_view(&self) -> ::protobuf::View<'_, RoundRobin> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RoundRobinMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RoundRobin>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RoundRobinMut<'msg> {
  type MutProxied = RoundRobin;
  fn as_mut(&mut self) -> RoundRobinMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RoundRobinMut<'msg> {
  fn into_mut<'shorter>(self) -> RoundRobinMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RoundRobin {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RoundRobin> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RoundRobinView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RoundRobinMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // slow_start_config: optional message envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig
  pub fn has_slow_start_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_slow_start_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn slow_start_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigView::default())
  }
  pub fn slow_start_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfigMut<'_> {
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
  pub fn set_slow_start_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // locality_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig
  pub fn has_locality_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_locality_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn locality_lb_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'_>> {
    self.has_locality_lb_config().then(|| self.locality_lb_config())
  }
  pub fn locality_lb_config(&self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigView::default())
  }
  pub fn locality_lb_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfigMut<'_> {
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
  pub fn set_locality_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl RoundRobin

impl ::std::ops::Drop for RoundRobin {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RoundRobin {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RoundRobin {
  type Proxied = Self;
  fn as_view(&self) -> RoundRobinView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RoundRobin {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RoundRobinMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RoundRobin {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__load_0balancing_0policies__round_0robin__v3__RoundRobin_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__load_0balancing_0policies__round_0robin__v3__RoundRobin_msg_init.0, &[<crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::SlowStartConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::load_balancing_policies::common::v3::common::LocalityLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__load_0balancing_0policies__round_0robin__v3__RoundRobin_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RoundRobin {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RoundRobin {
  type Msg = RoundRobin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoundRobin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoundRobin {
  type Msg = RoundRobin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoundRobin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RoundRobinMut<'_> {
  type Msg = RoundRobin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoundRobin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoundRobinMut<'_> {
  type Msg = RoundRobin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoundRobin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoundRobinView<'_> {
  type Msg = RoundRobin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoundRobin> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RoundRobinMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



