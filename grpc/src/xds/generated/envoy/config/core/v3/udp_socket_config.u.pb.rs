const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__UdpSocketConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UdpSocketConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UdpSocketConfig>
}

impl ::protobuf::Message for UdpSocketConfig {
  type MessageView<'msg> = UdpSocketConfigView<'msg>;
  type MessageMut<'msg> = UdpSocketConfigMut<'msg>;
}

impl ::std::default::Default for UdpSocketConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UdpSocketConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UdpSocketConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `UdpSocketConfigMut`.
unsafe impl ::std::marker::Sync for UdpSocketConfig {}

// SAFETY:
// - `UdpSocketConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UdpSocketConfig {}

impl ::protobuf::Proxied for UdpSocketConfig {
  type View<'msg> = UdpSocketConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UdpSocketConfig {}

impl ::protobuf::MutProxied for UdpSocketConfig {
  type Mut<'msg> = UdpSocketConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UdpSocketConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UdpSocketConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UdpSocketConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UdpSocketConfigView<'msg> {
  type Message = UdpSocketConfig;
}

impl ::std::fmt::Debug for UdpSocketConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UdpSocketConfigView<'_> {
  fn default() -> UdpSocketConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UdpSocketConfig>> for UdpSocketConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UdpSocketConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UdpSocketConfigView<'msg> {

  pub fn to_owned(&self) -> UdpSocketConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // max_rx_datagram_size: optional message google.protobuf.UInt64Value
  pub fn has_max_rx_datagram_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn max_rx_datagram_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_max_rx_datagram_size().then(|| self.max_rx_datagram_size())
  }
  pub fn max_rx_datagram_size(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

  // prefer_gro: optional message google.protobuf.BoolValue
  pub fn has_prefer_gro(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn prefer_gro_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_prefer_gro().then(|| self.prefer_gro())
  }
  pub fn prefer_gro(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

}

// SAFETY:
// - `UdpSocketConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UdpSocketConfigView<'_> {}

// SAFETY:
// - `UdpSocketConfigView` is `Send` because while its alive a `UdpSocketConfigMut` cannot.
// - `UdpSocketConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for UdpSocketConfigView<'_> {}

impl<'msg> ::protobuf::AsView for UdpSocketConfigView<'msg> {
  type Proxied = UdpSocketConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, UdpSocketConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UdpSocketConfigView<'msg> {
  fn into_view<'shorter>(self) -> UdpSocketConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UdpSocketConfig> for UdpSocketConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UdpSocketConfig {
    let mut dst = UdpSocketConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UdpSocketConfig> for UdpSocketConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UdpSocketConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UdpSocketConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UdpSocketConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UdpSocketConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UdpSocketConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UdpSocketConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UdpSocketConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UdpSocketConfigMut<'msg> {
  type Message = UdpSocketConfig;
}

impl ::std::fmt::Debug for UdpSocketConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UdpSocketConfig>> for UdpSocketConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UdpSocketConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UdpSocketConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UdpSocketConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UdpSocketConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // max_rx_datagram_size: optional message google.protobuf.UInt64Value
  pub fn has_max_rx_datagram_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_rx_datagram_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_rx_datagram_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_max_rx_datagram_size().then(|| self.max_rx_datagram_size())
  }
  pub fn max_rx_datagram_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn max_rx_datagram_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_max_rx_datagram_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // prefer_gro: optional message google.protobuf.BoolValue
  pub fn has_prefer_gro(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_prefer_gro(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn prefer_gro_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_prefer_gro().then(|| self.prefer_gro())
  }
  pub fn prefer_gro(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn prefer_gro_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_prefer_gro(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

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
// - `UdpSocketConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UdpSocketConfigMut<'_> {}

// SAFETY:
// - `UdpSocketConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UdpSocketConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for UdpSocketConfigMut<'msg> {
  type Proxied = UdpSocketConfig;
  fn as_view(&self) -> ::protobuf::View<'_, UdpSocketConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UdpSocketConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UdpSocketConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UdpSocketConfigMut<'msg> {
  type MutProxied = UdpSocketConfig;
  fn as_mut(&mut self) -> UdpSocketConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UdpSocketConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> UdpSocketConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UdpSocketConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UdpSocketConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UdpSocketConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UdpSocketConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // max_rx_datagram_size: optional message google.protobuf.UInt64Value
  pub fn has_max_rx_datagram_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_rx_datagram_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_rx_datagram_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_max_rx_datagram_size().then(|| self.max_rx_datagram_size())
  }
  pub fn max_rx_datagram_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn max_rx_datagram_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_max_rx_datagram_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // prefer_gro: optional message google.protobuf.BoolValue
  pub fn has_prefer_gro(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_prefer_gro(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn prefer_gro_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_prefer_gro().then(|| self.prefer_gro())
  }
  pub fn prefer_gro(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn prefer_gro_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_prefer_gro(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl UdpSocketConfig

impl ::std::ops::Drop for UdpSocketConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UdpSocketConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UdpSocketConfig {
  type Proxied = Self;
  fn as_view(&self) -> UdpSocketConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UdpSocketConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UdpSocketConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UdpSocketConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__UdpSocketConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__UdpSocketConfig_msg_init.0, &[<::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__UdpSocketConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UdpSocketConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UdpSocketConfig {
  type Msg = UdpSocketConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UdpSocketConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UdpSocketConfig {
  type Msg = UdpSocketConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UdpSocketConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UdpSocketConfigMut<'_> {
  type Msg = UdpSocketConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UdpSocketConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UdpSocketConfigMut<'_> {
  type Msg = UdpSocketConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UdpSocketConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UdpSocketConfigView<'_> {
  type Msg = UdpSocketConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UdpSocketConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UdpSocketConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



