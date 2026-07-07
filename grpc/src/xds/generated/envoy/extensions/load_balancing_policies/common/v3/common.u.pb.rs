const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LocalityLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LocalityLbConfig>
}

impl ::protobuf::Message for LocalityLbConfig {
  type MessageView<'msg> = LocalityLbConfigView<'msg>;
  type MessageMut<'msg> = LocalityLbConfigMut<'msg>;
}

impl ::std::default::Default for LocalityLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LocalityLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LocalityLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `LocalityLbConfigMut`.
unsafe impl ::std::marker::Sync for LocalityLbConfig {}

// SAFETY:
// - `LocalityLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LocalityLbConfig {}

impl ::protobuf::Proxied for LocalityLbConfig {
  type View<'msg> = LocalityLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LocalityLbConfig {}

impl ::protobuf::MutProxied for LocalityLbConfig {
  type Mut<'msg> = LocalityLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LocalityLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalityLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LocalityLbConfigView<'msg> {
  type Message = LocalityLbConfig;
}

impl ::std::fmt::Debug for LocalityLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LocalityLbConfigView<'_> {
  fn default() -> LocalityLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityLbConfig>> for LocalityLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalityLbConfigView<'msg> {

  pub fn to_owned(&self) -> LocalityLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // zone_aware_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig
  pub fn has_zone_aware_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn zone_aware_lb_config_opt(self) -> ::std::option::Option<super::locality_lb_config::ZoneAwareLbConfigView<'msg>> {
    self.has_zone_aware_lb_config().then(|| self.zone_aware_lb_config())
  }
  pub fn zone_aware_lb_config(self) -> super::locality_lb_config::ZoneAwareLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::locality_lb_config::ZoneAwareLbConfigView::default())
  }

  // locality_weighted_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.LocalityWeightedLbConfig
  pub fn has_locality_weighted_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn locality_weighted_lb_config_opt(self) -> ::std::option::Option<super::locality_lb_config::LocalityWeightedLbConfigView<'msg>> {
    self.has_locality_weighted_lb_config().then(|| self.locality_weighted_lb_config())
  }
  pub fn locality_weighted_lb_config(self) -> super::locality_lb_config::LocalityWeightedLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::locality_lb_config::LocalityWeightedLbConfigView::default())
  }

  pub fn locality_config_specifier(self) -> super::locality_lb_config::LocalityConfigSpecifierOneof<'msg> {
    match self.locality_config_specifier_case() {
      super::locality_lb_config::LocalityConfigSpecifierCase::ZoneAwareLbConfig =>
          super::locality_lb_config::LocalityConfigSpecifierOneof::ZoneAwareLbConfig(self.zone_aware_lb_config()),
      super::locality_lb_config::LocalityConfigSpecifierCase::LocalityWeightedLbConfig =>
          super::locality_lb_config::LocalityConfigSpecifierOneof::LocalityWeightedLbConfig(self.locality_weighted_lb_config()),
      _ => super::locality_lb_config::LocalityConfigSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn locality_config_specifier_case(self) -> super::locality_lb_config::LocalityConfigSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::locality_lb_config::LocalityConfigSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `LocalityLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LocalityLbConfigView<'_> {}

// SAFETY:
// - `LocalityLbConfigView` is `Send` because while its alive a `LocalityLbConfigMut` cannot.
// - `LocalityLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for LocalityLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for LocalityLbConfigView<'msg> {
  type Proxied = LocalityLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, LocalityLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalityLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> LocalityLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalityLbConfig> for LocalityLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalityLbConfig {
    let mut dst = LocalityLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalityLbConfig> for LocalityLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalityLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LocalityLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalityLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalityLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LocalityLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalityLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LocalityLbConfigMut<'msg> {
  type Message = LocalityLbConfig;
}

impl ::std::fmt::Debug for LocalityLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityLbConfig>> for LocalityLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalityLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LocalityLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // zone_aware_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig
  pub fn has_zone_aware_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_zone_aware_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn zone_aware_lb_config_opt(&self) -> ::std::option::Option<super::locality_lb_config::ZoneAwareLbConfigView<'_>> {
    self.has_zone_aware_lb_config().then(|| self.zone_aware_lb_config())
  }
  pub fn zone_aware_lb_config(&self) -> super::locality_lb_config::ZoneAwareLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::locality_lb_config::ZoneAwareLbConfigView::default())
  }
  pub fn zone_aware_lb_config_mut(&mut self) -> super::locality_lb_config::ZoneAwareLbConfigMut<'_> {
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
  pub fn set_zone_aware_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::locality_lb_config::ZoneAwareLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // locality_weighted_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.LocalityWeightedLbConfig
  pub fn has_locality_weighted_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_locality_weighted_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn locality_weighted_lb_config_opt(&self) -> ::std::option::Option<super::locality_lb_config::LocalityWeightedLbConfigView<'_>> {
    self.has_locality_weighted_lb_config().then(|| self.locality_weighted_lb_config())
  }
  pub fn locality_weighted_lb_config(&self) -> super::locality_lb_config::LocalityWeightedLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::locality_lb_config::LocalityWeightedLbConfigView::default())
  }
  pub fn locality_weighted_lb_config_mut(&mut self) -> super::locality_lb_config::LocalityWeightedLbConfigMut<'_> {
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
  pub fn set_locality_weighted_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::locality_lb_config::LocalityWeightedLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn locality_config_specifier(&self) -> super::locality_lb_config::LocalityConfigSpecifierOneof<'_> {
    match &self.locality_config_specifier_case() {
      super::locality_lb_config::LocalityConfigSpecifierCase::ZoneAwareLbConfig =>
          super::locality_lb_config::LocalityConfigSpecifierOneof::ZoneAwareLbConfig(self.zone_aware_lb_config()),
      super::locality_lb_config::LocalityConfigSpecifierCase::LocalityWeightedLbConfig =>
          super::locality_lb_config::LocalityConfigSpecifierOneof::LocalityWeightedLbConfig(self.locality_weighted_lb_config()),
      _ => super::locality_lb_config::LocalityConfigSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn locality_config_specifier_case(&self) -> super::locality_lb_config::LocalityConfigSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::locality_lb_config::LocalityConfigSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `LocalityLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LocalityLbConfigMut<'_> {}

// SAFETY:
// - `LocalityLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LocalityLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for LocalityLbConfigMut<'msg> {
  type Proxied = LocalityLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, LocalityLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalityLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LocalityLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LocalityLbConfigMut<'msg> {
  type MutProxied = LocalityLbConfig;
  fn as_mut(&mut self) -> LocalityLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LocalityLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> LocalityLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LocalityLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LocalityLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LocalityLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LocalityLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // zone_aware_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig
  pub fn has_zone_aware_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_zone_aware_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn zone_aware_lb_config_opt(&self) -> ::std::option::Option<super::locality_lb_config::ZoneAwareLbConfigView<'_>> {
    self.has_zone_aware_lb_config().then(|| self.zone_aware_lb_config())
  }
  pub fn zone_aware_lb_config(&self) -> super::locality_lb_config::ZoneAwareLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::locality_lb_config::ZoneAwareLbConfigView::default())
  }
  pub fn zone_aware_lb_config_mut(&mut self) -> super::locality_lb_config::ZoneAwareLbConfigMut<'_> {
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
  pub fn set_zone_aware_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::locality_lb_config::ZoneAwareLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // locality_weighted_lb_config: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.LocalityWeightedLbConfig
  pub fn has_locality_weighted_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_locality_weighted_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn locality_weighted_lb_config_opt(&self) -> ::std::option::Option<super::locality_lb_config::LocalityWeightedLbConfigView<'_>> {
    self.has_locality_weighted_lb_config().then(|| self.locality_weighted_lb_config())
  }
  pub fn locality_weighted_lb_config(&self) -> super::locality_lb_config::LocalityWeightedLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::locality_lb_config::LocalityWeightedLbConfigView::default())
  }
  pub fn locality_weighted_lb_config_mut(&mut self) -> super::locality_lb_config::LocalityWeightedLbConfigMut<'_> {
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
  pub fn set_locality_weighted_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::locality_lb_config::LocalityWeightedLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn locality_config_specifier(&self) -> super::locality_lb_config::LocalityConfigSpecifierOneof<'_> {
    match &self.locality_config_specifier_case() {
      super::locality_lb_config::LocalityConfigSpecifierCase::ZoneAwareLbConfig =>
          super::locality_lb_config::LocalityConfigSpecifierOneof::ZoneAwareLbConfig(self.zone_aware_lb_config()),
      super::locality_lb_config::LocalityConfigSpecifierCase::LocalityWeightedLbConfig =>
          super::locality_lb_config::LocalityConfigSpecifierOneof::LocalityWeightedLbConfig(self.locality_weighted_lb_config()),
      _ => super::locality_lb_config::LocalityConfigSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn locality_config_specifier_case(&self) -> super::locality_lb_config::LocalityConfigSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::locality_lb_config::LocalityConfigSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl LocalityLbConfig

impl ::std::ops::Drop for LocalityLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LocalityLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LocalityLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> LocalityLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LocalityLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LocalityLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LocalityLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig_msg_init.0, &[<super::locality_lb_config::ZoneAwareLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::locality_lb_config::LocalityWeightedLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalityLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalityLbConfig {
  type Msg = LocalityLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityLbConfig {
  type Msg = LocalityLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalityLbConfigMut<'_> {
  type Msg = LocalityLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityLbConfigMut<'_> {
  type Msg = LocalityLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityLbConfigView<'_> {
  type Msg = LocalityLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalityLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod locality_lb_config {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__ZoneAwareLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ZoneAwareLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ZoneAwareLbConfig>
}

impl ::protobuf::Message for ZoneAwareLbConfig {
  type MessageView<'msg> = ZoneAwareLbConfigView<'msg>;
  type MessageMut<'msg> = ZoneAwareLbConfigMut<'msg>;
}

impl ::std::default::Default for ZoneAwareLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ZoneAwareLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ZoneAwareLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ZoneAwareLbConfigMut`.
unsafe impl ::std::marker::Sync for ZoneAwareLbConfig {}

// SAFETY:
// - `ZoneAwareLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ZoneAwareLbConfig {}

impl ::protobuf::Proxied for ZoneAwareLbConfig {
  type View<'msg> = ZoneAwareLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ZoneAwareLbConfig {}

impl ::protobuf::MutProxied for ZoneAwareLbConfig {
  type Mut<'msg> = ZoneAwareLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ZoneAwareLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ZoneAwareLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ZoneAwareLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ZoneAwareLbConfigView<'msg> {
  type Message = ZoneAwareLbConfig;
}

impl ::std::fmt::Debug for ZoneAwareLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ZoneAwareLbConfigView<'_> {
  fn default() -> ZoneAwareLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ZoneAwareLbConfig>> for ZoneAwareLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ZoneAwareLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ZoneAwareLbConfigView<'msg> {

  pub fn to_owned(&self) -> ZoneAwareLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // routing_enabled: optional message envoy.type.v3.Percent
  pub fn has_routing_enabled(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn routing_enabled_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_routing_enabled().then(|| self.routing_enabled())
  }
  pub fn routing_enabled(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

  // min_cluster_size: optional message google.protobuf.UInt64Value
  pub fn has_min_cluster_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn min_cluster_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_min_cluster_size().then(|| self.min_cluster_size())
  }
  pub fn min_cluster_size(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

  // fail_traffic_on_panic: optional bool
  pub fn fail_traffic_on_panic(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }

  // force_locality_direct_routing: optional bool
  pub fn force_locality_direct_routing(self) -> bool {
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

  // force_local_zone: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig.ForceLocalZone
  pub fn has_force_local_zone(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn force_local_zone_opt(self) -> ::std::option::Option<super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZoneView<'msg>> {
    self.has_force_local_zone().then(|| self.force_local_zone())
  }
  pub fn force_local_zone(self) -> super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZoneView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZoneView::default())
  }

  // locality_basis: optional enum envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig.LocalityBasis
  pub fn locality_basis(self) -> super::super::locality_lb_config::zone_aware_lb_config::LocalityBasis {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::locality_lb_config::zone_aware_lb_config::LocalityBasis::HealthyHostsNum).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ZoneAwareLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ZoneAwareLbConfigView<'_> {}

// SAFETY:
// - `ZoneAwareLbConfigView` is `Send` because while its alive a `ZoneAwareLbConfigMut` cannot.
// - `ZoneAwareLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ZoneAwareLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ZoneAwareLbConfigView<'msg> {
  type Proxied = ZoneAwareLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ZoneAwareLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ZoneAwareLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> ZoneAwareLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ZoneAwareLbConfig> for ZoneAwareLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ZoneAwareLbConfig {
    let mut dst = ZoneAwareLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ZoneAwareLbConfig> for ZoneAwareLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ZoneAwareLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ZoneAwareLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ZoneAwareLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ZoneAwareLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ZoneAwareLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ZoneAwareLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ZoneAwareLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ZoneAwareLbConfigMut<'msg> {
  type Message = ZoneAwareLbConfig;
}

impl ::std::fmt::Debug for ZoneAwareLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ZoneAwareLbConfig>> for ZoneAwareLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ZoneAwareLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ZoneAwareLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ZoneAwareLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ZoneAwareLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // routing_enabled: optional message envoy.type.v3.Percent
  pub fn has_routing_enabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_routing_enabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn routing_enabled_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_routing_enabled().then(|| self.routing_enabled())
  }
  pub fn routing_enabled(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn routing_enabled_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_routing_enabled(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // min_cluster_size: optional message google.protobuf.UInt64Value
  pub fn has_min_cluster_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_cluster_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_cluster_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_min_cluster_size().then(|| self.min_cluster_size())
  }
  pub fn min_cluster_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn min_cluster_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_min_cluster_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // fail_traffic_on_panic: optional bool
  pub fn fail_traffic_on_panic(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fail_traffic_on_panic(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // force_locality_direct_routing: optional bool
  pub fn force_locality_direct_routing(&self) -> bool {
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
  pub fn set_force_locality_direct_routing(&mut self, val: bool) {
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

  // force_local_zone: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig.ForceLocalZone
  pub fn has_force_local_zone(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_force_local_zone(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn force_local_zone_opt(&self) -> ::std::option::Option<super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZoneView<'_>> {
    self.has_force_local_zone().then(|| self.force_local_zone())
  }
  pub fn force_local_zone(&self) -> super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZoneView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZoneView::default())
  }
  pub fn force_local_zone_mut(&mut self) -> super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZoneMut<'_> {
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
  pub fn set_force_local_zone(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZone>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // locality_basis: optional enum envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig.LocalityBasis
  pub fn locality_basis(&self) -> super::super::locality_lb_config::zone_aware_lb_config::LocalityBasis {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::locality_lb_config::zone_aware_lb_config::LocalityBasis::HealthyHostsNum).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_locality_basis(&mut self, val: super::super::locality_lb_config::zone_aware_lb_config::LocalityBasis) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

}

// SAFETY:
// - `ZoneAwareLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ZoneAwareLbConfigMut<'_> {}

// SAFETY:
// - `ZoneAwareLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ZoneAwareLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ZoneAwareLbConfigMut<'msg> {
  type Proxied = ZoneAwareLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ZoneAwareLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ZoneAwareLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ZoneAwareLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ZoneAwareLbConfigMut<'msg> {
  type MutProxied = ZoneAwareLbConfig;
  fn as_mut(&mut self) -> ZoneAwareLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ZoneAwareLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ZoneAwareLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ZoneAwareLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ZoneAwareLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ZoneAwareLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ZoneAwareLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // routing_enabled: optional message envoy.type.v3.Percent
  pub fn has_routing_enabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_routing_enabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn routing_enabled_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_routing_enabled().then(|| self.routing_enabled())
  }
  pub fn routing_enabled(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn routing_enabled_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_routing_enabled(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // min_cluster_size: optional message google.protobuf.UInt64Value
  pub fn has_min_cluster_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_cluster_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_cluster_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_min_cluster_size().then(|| self.min_cluster_size())
  }
  pub fn min_cluster_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn min_cluster_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_min_cluster_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // fail_traffic_on_panic: optional bool
  pub fn fail_traffic_on_panic(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fail_traffic_on_panic(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // force_locality_direct_routing: optional bool
  pub fn force_locality_direct_routing(&self) -> bool {
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
  pub fn set_force_locality_direct_routing(&mut self, val: bool) {
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

  // force_local_zone: optional message envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig.ForceLocalZone
  pub fn has_force_local_zone(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_force_local_zone(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn force_local_zone_opt(&self) -> ::std::option::Option<super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZoneView<'_>> {
    self.has_force_local_zone().then(|| self.force_local_zone())
  }
  pub fn force_local_zone(&self) -> super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZoneView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZoneView::default())
  }
  pub fn force_local_zone_mut(&mut self) -> super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZoneMut<'_> {
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
  pub fn set_force_local_zone(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZone>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // locality_basis: optional enum envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig.LocalityBasis
  pub fn locality_basis(&self) -> super::super::locality_lb_config::zone_aware_lb_config::LocalityBasis {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::locality_lb_config::zone_aware_lb_config::LocalityBasis::HealthyHostsNum).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_locality_basis(&mut self, val: super::super::locality_lb_config::zone_aware_lb_config::LocalityBasis) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

}  // impl ZoneAwareLbConfig

impl ::std::ops::Drop for ZoneAwareLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ZoneAwareLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ZoneAwareLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> ZoneAwareLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ZoneAwareLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ZoneAwareLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ZoneAwareLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::locality_lb_config::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__ZoneAwareLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33/P/P3.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::locality_lb_config::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__ZoneAwareLbConfig_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::locality_lb_config::zone_aware_lb_config::ForceLocalZone as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::locality_lb_config::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__ZoneAwareLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ZoneAwareLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ZoneAwareLbConfig {
  type Msg = ZoneAwareLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZoneAwareLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ZoneAwareLbConfig {
  type Msg = ZoneAwareLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZoneAwareLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ZoneAwareLbConfigMut<'_> {
  type Msg = ZoneAwareLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZoneAwareLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ZoneAwareLbConfigMut<'_> {
  type Msg = ZoneAwareLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZoneAwareLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ZoneAwareLbConfigView<'_> {
  type Msg = ZoneAwareLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZoneAwareLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ZoneAwareLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod zone_aware_lb_config {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__ZoneAwareLbConfig__ForceLocalZone_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ForceLocalZone {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ForceLocalZone>
}

impl ::protobuf::Message for ForceLocalZone {
  type MessageView<'msg> = ForceLocalZoneView<'msg>;
  type MessageMut<'msg> = ForceLocalZoneMut<'msg>;
}

impl ::std::default::Default for ForceLocalZone {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ForceLocalZone {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ForceLocalZone` is `Sync` because it does not implement interior mutability.
//    Neither does `ForceLocalZoneMut`.
unsafe impl ::std::marker::Sync for ForceLocalZone {}

// SAFETY:
// - `ForceLocalZone` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ForceLocalZone {}

impl ::protobuf::Proxied for ForceLocalZone {
  type View<'msg> = ForceLocalZoneView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ForceLocalZone {}

impl ::protobuf::MutProxied for ForceLocalZone {
  type Mut<'msg> = ForceLocalZoneMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ForceLocalZoneView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ForceLocalZone>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ForceLocalZoneView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ForceLocalZoneView<'msg> {
  type Message = ForceLocalZone;
}

impl ::std::fmt::Debug for ForceLocalZoneView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ForceLocalZoneView<'_> {
  fn default() -> ForceLocalZoneView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ForceLocalZone>> for ForceLocalZoneView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ForceLocalZone>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ForceLocalZoneView<'msg> {

  pub fn to_owned(&self) -> ForceLocalZone {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // min_size: optional message google.protobuf.UInt32Value
  pub fn has_min_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn min_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_min_size().then(|| self.min_size())
  }
  pub fn min_size(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `ForceLocalZoneView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ForceLocalZoneView<'_> {}

// SAFETY:
// - `ForceLocalZoneView` is `Send` because while its alive a `ForceLocalZoneMut` cannot.
// - `ForceLocalZoneView` does not use thread-local data.
unsafe impl ::std::marker::Send for ForceLocalZoneView<'_> {}

impl<'msg> ::protobuf::AsView for ForceLocalZoneView<'msg> {
  type Proxied = ForceLocalZone;
  fn as_view(&self) -> ::protobuf::View<'msg, ForceLocalZone> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ForceLocalZoneView<'msg> {
  fn into_view<'shorter>(self) -> ForceLocalZoneView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ForceLocalZone> for ForceLocalZoneView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ForceLocalZone {
    let mut dst = ForceLocalZone::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ForceLocalZone> for ForceLocalZoneMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ForceLocalZone {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ForceLocalZone {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ForceLocalZoneView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ForceLocalZoneMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ForceLocalZoneMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ForceLocalZone>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ForceLocalZoneMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ForceLocalZoneMut<'msg> {
  type Message = ForceLocalZone;
}

impl ::std::fmt::Debug for ForceLocalZoneMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ForceLocalZone>> for ForceLocalZoneMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ForceLocalZone>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ForceLocalZoneMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ForceLocalZone> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ForceLocalZone {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // min_size: optional message google.protobuf.UInt32Value
  pub fn has_min_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_min_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn min_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_min_size().then(|| self.min_size())
  }
  pub fn min_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn min_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_min_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

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
// - `ForceLocalZoneMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ForceLocalZoneMut<'_> {}

// SAFETY:
// - `ForceLocalZoneMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ForceLocalZoneMut<'_> {}

impl<'msg> ::protobuf::AsView for ForceLocalZoneMut<'msg> {
  type Proxied = ForceLocalZone;
  fn as_view(&self) -> ::protobuf::View<'_, ForceLocalZone> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ForceLocalZoneMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ForceLocalZone>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ForceLocalZoneMut<'msg> {
  type MutProxied = ForceLocalZone;
  fn as_mut(&mut self) -> ForceLocalZoneMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ForceLocalZoneMut<'msg> {
  fn into_mut<'shorter>(self) -> ForceLocalZoneMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ForceLocalZone {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ForceLocalZone> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ForceLocalZoneView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ForceLocalZoneMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // min_size: optional message google.protobuf.UInt32Value
  pub fn has_min_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_min_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn min_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_min_size().then(|| self.min_size())
  }
  pub fn min_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn min_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_min_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl ForceLocalZone

impl ::std::ops::Drop for ForceLocalZone {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ForceLocalZone {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ForceLocalZone {
  type Proxied = Self;
  fn as_view(&self) -> ForceLocalZoneView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ForceLocalZone {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ForceLocalZoneMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ForceLocalZone {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::locality_lb_config::zone_aware_lb_config::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__ZoneAwareLbConfig__ForceLocalZone_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::locality_lb_config::zone_aware_lb_config::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__ZoneAwareLbConfig__ForceLocalZone_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::locality_lb_config::zone_aware_lb_config::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__ZoneAwareLbConfig__ForceLocalZone_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ForceLocalZone {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ForceLocalZone {
  type Msg = ForceLocalZone;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ForceLocalZone> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ForceLocalZone {
  type Msg = ForceLocalZone;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ForceLocalZone> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ForceLocalZoneMut<'_> {
  type Msg = ForceLocalZone;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ForceLocalZone> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ForceLocalZoneMut<'_> {
  type Msg = ForceLocalZone;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ForceLocalZone> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ForceLocalZoneView<'_> {
  type Msg = ForceLocalZone;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ForceLocalZone> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ForceLocalZoneMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LocalityBasis(i32);

#[allow(non_upper_case_globals)]
impl LocalityBasis {
  pub const HealthyHostsNum: LocalityBasis = LocalityBasis(0);
  pub const HealthyHostsWeight: LocalityBasis = LocalityBasis(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "HealthyHostsNum",
      1 => "HealthyHostsWeight",
      _ => return None
    })
  }
}

impl ::std::convert::From<LocalityBasis> for i32 {
  fn from(val: LocalityBasis) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for LocalityBasis {
  fn from(val: i32) -> LocalityBasis {
    Self(val)
  }
}

impl ::std::default::Default for LocalityBasis {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for LocalityBasis {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "LocalityBasis::{}", constant_name)
    } else {
      write!(f, "LocalityBasis::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for LocalityBasis {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for LocalityBasis {}

impl ::protobuf::Proxied for LocalityBasis {
  type View<'a> = LocalityBasis;
}

impl ::protobuf::AsView for LocalityBasis {
  type Proxied = LocalityBasis;

  fn as_view(&self) -> LocalityBasis {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalityBasis {
  fn into_view<'shorter>(self) -> LocalityBasis where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for LocalityBasis {
  const NAME: &'static str = "LocalityBasis";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for LocalityBasis {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod zone_aware_lb_config

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__LocalityWeightedLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LocalityWeightedLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LocalityWeightedLbConfig>
}

impl ::protobuf::Message for LocalityWeightedLbConfig {
  type MessageView<'msg> = LocalityWeightedLbConfigView<'msg>;
  type MessageMut<'msg> = LocalityWeightedLbConfigMut<'msg>;
}

impl ::std::default::Default for LocalityWeightedLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LocalityWeightedLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LocalityWeightedLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `LocalityWeightedLbConfigMut`.
unsafe impl ::std::marker::Sync for LocalityWeightedLbConfig {}

// SAFETY:
// - `LocalityWeightedLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LocalityWeightedLbConfig {}

impl ::protobuf::Proxied for LocalityWeightedLbConfig {
  type View<'msg> = LocalityWeightedLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LocalityWeightedLbConfig {}

impl ::protobuf::MutProxied for LocalityWeightedLbConfig {
  type Mut<'msg> = LocalityWeightedLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LocalityWeightedLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityWeightedLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalityWeightedLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LocalityWeightedLbConfigView<'msg> {
  type Message = LocalityWeightedLbConfig;
}

impl ::std::fmt::Debug for LocalityWeightedLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LocalityWeightedLbConfigView<'_> {
  fn default() -> LocalityWeightedLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityWeightedLbConfig>> for LocalityWeightedLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityWeightedLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalityWeightedLbConfigView<'msg> {

  pub fn to_owned(&self) -> LocalityWeightedLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `LocalityWeightedLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LocalityWeightedLbConfigView<'_> {}

// SAFETY:
// - `LocalityWeightedLbConfigView` is `Send` because while its alive a `LocalityWeightedLbConfigMut` cannot.
// - `LocalityWeightedLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for LocalityWeightedLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for LocalityWeightedLbConfigView<'msg> {
  type Proxied = LocalityWeightedLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, LocalityWeightedLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalityWeightedLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> LocalityWeightedLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalityWeightedLbConfig> for LocalityWeightedLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalityWeightedLbConfig {
    let mut dst = LocalityWeightedLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalityWeightedLbConfig> for LocalityWeightedLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalityWeightedLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LocalityWeightedLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalityWeightedLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalityWeightedLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LocalityWeightedLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityWeightedLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalityWeightedLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LocalityWeightedLbConfigMut<'msg> {
  type Message = LocalityWeightedLbConfig;
}

impl ::std::fmt::Debug for LocalityWeightedLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityWeightedLbConfig>> for LocalityWeightedLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityWeightedLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalityWeightedLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityWeightedLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LocalityWeightedLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `LocalityWeightedLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LocalityWeightedLbConfigMut<'_> {}

// SAFETY:
// - `LocalityWeightedLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LocalityWeightedLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for LocalityWeightedLbConfigMut<'msg> {
  type Proxied = LocalityWeightedLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, LocalityWeightedLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalityWeightedLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LocalityWeightedLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LocalityWeightedLbConfigMut<'msg> {
  type MutProxied = LocalityWeightedLbConfig;
  fn as_mut(&mut self) -> LocalityWeightedLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LocalityWeightedLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> LocalityWeightedLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LocalityWeightedLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LocalityWeightedLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LocalityWeightedLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LocalityWeightedLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl LocalityWeightedLbConfig

impl ::std::ops::Drop for LocalityWeightedLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LocalityWeightedLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LocalityWeightedLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> LocalityWeightedLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LocalityWeightedLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LocalityWeightedLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LocalityWeightedLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::locality_lb_config::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__LocalityWeightedLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::locality_lb_config::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__LocalityWeightedLbConfig_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::locality_lb_config::envoy__extensions__load_0balancing_0policies__common__v3__LocalityLbConfig__LocalityWeightedLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalityWeightedLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalityWeightedLbConfig {
  type Msg = LocalityWeightedLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityWeightedLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityWeightedLbConfig {
  type Msg = LocalityWeightedLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityWeightedLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalityWeightedLbConfigMut<'_> {
  type Msg = LocalityWeightedLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityWeightedLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityWeightedLbConfigMut<'_> {
  type Msg = LocalityWeightedLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityWeightedLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityWeightedLbConfigView<'_> {
  type Msg = LocalityWeightedLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityWeightedLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalityWeightedLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum LocalityConfigSpecifierOneof<'msg> {
  ZoneAwareLbConfig(::protobuf::View<'msg, super::super::locality_lb_config::ZoneAwareLbConfig>) = 1,
  LocalityWeightedLbConfig(::protobuf::View<'msg, super::super::locality_lb_config::LocalityWeightedLbConfig>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum LocalityConfigSpecifierCase {
  ZoneAwareLbConfig = 1,
  LocalityWeightedLbConfig = 2,

  not_set = 0
}

impl LocalityConfigSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<LocalityConfigSpecifierCase> {
    match v {
      0 => Some(LocalityConfigSpecifierCase::not_set),
      1 => Some(LocalityConfigSpecifierCase::ZoneAwareLbConfig),
      2 => Some(LocalityConfigSpecifierCase::LocalityWeightedLbConfig),
      _ => None
    }
  }
}
}  // pub mod locality_lb_config


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__common__v3__SlowStartConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SlowStartConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SlowStartConfig>
}

impl ::protobuf::Message for SlowStartConfig {
  type MessageView<'msg> = SlowStartConfigView<'msg>;
  type MessageMut<'msg> = SlowStartConfigMut<'msg>;
}

impl ::std::default::Default for SlowStartConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SlowStartConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SlowStartConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `SlowStartConfigMut`.
unsafe impl ::std::marker::Sync for SlowStartConfig {}

// SAFETY:
// - `SlowStartConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SlowStartConfig {}

impl ::protobuf::Proxied for SlowStartConfig {
  type View<'msg> = SlowStartConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SlowStartConfig {}

impl ::protobuf::MutProxied for SlowStartConfig {
  type Mut<'msg> = SlowStartConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SlowStartConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SlowStartConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SlowStartConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SlowStartConfigView<'msg> {
  type Message = SlowStartConfig;
}

impl ::std::fmt::Debug for SlowStartConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SlowStartConfigView<'_> {
  fn default() -> SlowStartConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SlowStartConfig>> for SlowStartConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SlowStartConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SlowStartConfigView<'msg> {

  pub fn to_owned(&self) -> SlowStartConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // slow_start_window: optional message google.protobuf.Duration
  pub fn has_slow_start_window(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn slow_start_window_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_slow_start_window().then(|| self.slow_start_window())
  }
  pub fn slow_start_window(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // aggression: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_aggression(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn aggression_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'msg>> {
    self.has_aggression().then(|| self.aggression())
  }
  pub fn aggression(self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }

  // min_weight_percent: optional message envoy.type.v3.Percent
  pub fn has_min_weight_percent(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn min_weight_percent_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_min_weight_percent().then(|| self.min_weight_percent())
  }
  pub fn min_weight_percent(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

}

// SAFETY:
// - `SlowStartConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SlowStartConfigView<'_> {}

// SAFETY:
// - `SlowStartConfigView` is `Send` because while its alive a `SlowStartConfigMut` cannot.
// - `SlowStartConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for SlowStartConfigView<'_> {}

impl<'msg> ::protobuf::AsView for SlowStartConfigView<'msg> {
  type Proxied = SlowStartConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, SlowStartConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SlowStartConfigView<'msg> {
  fn into_view<'shorter>(self) -> SlowStartConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SlowStartConfig> for SlowStartConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SlowStartConfig {
    let mut dst = SlowStartConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SlowStartConfig> for SlowStartConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SlowStartConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SlowStartConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SlowStartConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SlowStartConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SlowStartConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SlowStartConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SlowStartConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SlowStartConfigMut<'msg> {
  type Message = SlowStartConfig;
}

impl ::std::fmt::Debug for SlowStartConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SlowStartConfig>> for SlowStartConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SlowStartConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SlowStartConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SlowStartConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SlowStartConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // slow_start_window: optional message google.protobuf.Duration
  pub fn has_slow_start_window(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_slow_start_window(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn slow_start_window_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_slow_start_window().then(|| self.slow_start_window())
  }
  pub fn slow_start_window(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn slow_start_window_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_slow_start_window(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // aggression: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_aggression(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_aggression(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn aggression_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_>> {
    self.has_aggression().then(|| self.aggression())
  }
  pub fn aggression(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }
  pub fn aggression_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleMut<'_> {
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
  pub fn set_aggression(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // min_weight_percent: optional message envoy.type.v3.Percent
  pub fn has_min_weight_percent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_min_weight_percent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn min_weight_percent_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_min_weight_percent().then(|| self.min_weight_percent())
  }
  pub fn min_weight_percent(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn min_weight_percent_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_min_weight_percent(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

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
// - `SlowStartConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SlowStartConfigMut<'_> {}

// SAFETY:
// - `SlowStartConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SlowStartConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for SlowStartConfigMut<'msg> {
  type Proxied = SlowStartConfig;
  fn as_view(&self) -> ::protobuf::View<'_, SlowStartConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SlowStartConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SlowStartConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SlowStartConfigMut<'msg> {
  type MutProxied = SlowStartConfig;
  fn as_mut(&mut self) -> SlowStartConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SlowStartConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> SlowStartConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SlowStartConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SlowStartConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SlowStartConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SlowStartConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // slow_start_window: optional message google.protobuf.Duration
  pub fn has_slow_start_window(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_slow_start_window(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn slow_start_window_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_slow_start_window().then(|| self.slow_start_window())
  }
  pub fn slow_start_window(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn slow_start_window_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_slow_start_window(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // aggression: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_aggression(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_aggression(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn aggression_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_>> {
    self.has_aggression().then(|| self.aggression())
  }
  pub fn aggression(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }
  pub fn aggression_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleMut<'_> {
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
  pub fn set_aggression(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // min_weight_percent: optional message envoy.type.v3.Percent
  pub fn has_min_weight_percent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_min_weight_percent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn min_weight_percent_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_min_weight_percent().then(|| self.min_weight_percent())
  }
  pub fn min_weight_percent(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn min_weight_percent_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_min_weight_percent(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl SlowStartConfig

impl ::std::ops::Drop for SlowStartConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SlowStartConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SlowStartConfig {
  type Proxied = Self;
  fn as_view(&self) -> SlowStartConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SlowStartConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SlowStartConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SlowStartConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__load_0balancing_0policies__common__v3__SlowStartConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__load_0balancing_0policies__common__v3__SlowStartConfig_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__load_0balancing_0policies__common__v3__SlowStartConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SlowStartConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SlowStartConfig {
  type Msg = SlowStartConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlowStartConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SlowStartConfig {
  type Msg = SlowStartConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlowStartConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SlowStartConfigMut<'_> {
  type Msg = SlowStartConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlowStartConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SlowStartConfigMut<'_> {
  type Msg = SlowStartConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlowStartConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SlowStartConfigView<'_> {
  type Msg = SlowStartConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlowStartConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SlowStartConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__load_0balancing_0policies__common__v3__ConsistentHashingLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ConsistentHashingLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ConsistentHashingLbConfig>
}

impl ::protobuf::Message for ConsistentHashingLbConfig {
  type MessageView<'msg> = ConsistentHashingLbConfigView<'msg>;
  type MessageMut<'msg> = ConsistentHashingLbConfigMut<'msg>;
}

impl ::std::default::Default for ConsistentHashingLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ConsistentHashingLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ConsistentHashingLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ConsistentHashingLbConfigMut`.
unsafe impl ::std::marker::Sync for ConsistentHashingLbConfig {}

// SAFETY:
// - `ConsistentHashingLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ConsistentHashingLbConfig {}

impl ::protobuf::Proxied for ConsistentHashingLbConfig {
  type View<'msg> = ConsistentHashingLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ConsistentHashingLbConfig {}

impl ::protobuf::MutProxied for ConsistentHashingLbConfig {
  type Mut<'msg> = ConsistentHashingLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ConsistentHashingLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConsistentHashingLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConsistentHashingLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ConsistentHashingLbConfigView<'msg> {
  type Message = ConsistentHashingLbConfig;
}

impl ::std::fmt::Debug for ConsistentHashingLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ConsistentHashingLbConfigView<'_> {
  fn default() -> ConsistentHashingLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ConsistentHashingLbConfig>> for ConsistentHashingLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConsistentHashingLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConsistentHashingLbConfigView<'msg> {

  pub fn to_owned(&self) -> ConsistentHashingLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
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
        0, (false).into()
      ).try_into().unwrap()
    }
  }

  // hash_balance_factor: optional message google.protobuf.UInt32Value
  pub fn has_hash_balance_factor(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn hash_balance_factor_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_hash_balance_factor().then(|| self.hash_balance_factor())
  }
  pub fn hash_balance_factor(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // hash_policy: repeated message envoy.config.route.v3.RouteAction.HashPolicy
  pub fn hash_policy(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::route::v3::route_components::route_action::HashPolicy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::route_action::HashPolicy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ConsistentHashingLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ConsistentHashingLbConfigView<'_> {}

// SAFETY:
// - `ConsistentHashingLbConfigView` is `Send` because while its alive a `ConsistentHashingLbConfigMut` cannot.
// - `ConsistentHashingLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ConsistentHashingLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ConsistentHashingLbConfigView<'msg> {
  type Proxied = ConsistentHashingLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ConsistentHashingLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConsistentHashingLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> ConsistentHashingLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ConsistentHashingLbConfig> for ConsistentHashingLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConsistentHashingLbConfig {
    let mut dst = ConsistentHashingLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ConsistentHashingLbConfig> for ConsistentHashingLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConsistentHashingLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ConsistentHashingLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConsistentHashingLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConsistentHashingLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ConsistentHashingLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConsistentHashingLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConsistentHashingLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ConsistentHashingLbConfigMut<'msg> {
  type Message = ConsistentHashingLbConfig;
}

impl ::std::fmt::Debug for ConsistentHashingLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ConsistentHashingLbConfig>> for ConsistentHashingLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConsistentHashingLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConsistentHashingLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ConsistentHashingLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ConsistentHashingLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
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
        0, (false).into()
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
        0, val.into()
      )
    }
  }

  // hash_balance_factor: optional message google.protobuf.UInt32Value
  pub fn has_hash_balance_factor(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_hash_balance_factor(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn hash_balance_factor_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_hash_balance_factor().then(|| self.hash_balance_factor())
  }
  pub fn hash_balance_factor(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn hash_balance_factor_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_hash_balance_factor(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // hash_policy: repeated message envoy.config.route.v3.RouteAction.HashPolicy
  pub fn hash_policy(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::route_action::HashPolicy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::route_action::HashPolicy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn hash_policy_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::route_action::HashPolicy> {
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
  pub fn set_hash_policy(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::route_action::HashPolicy>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `ConsistentHashingLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ConsistentHashingLbConfigMut<'_> {}

// SAFETY:
// - `ConsistentHashingLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ConsistentHashingLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ConsistentHashingLbConfigMut<'msg> {
  type Proxied = ConsistentHashingLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ConsistentHashingLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConsistentHashingLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ConsistentHashingLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ConsistentHashingLbConfigMut<'msg> {
  type MutProxied = ConsistentHashingLbConfig;
  fn as_mut(&mut self) -> ConsistentHashingLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ConsistentHashingLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ConsistentHashingLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ConsistentHashingLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ConsistentHashingLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ConsistentHashingLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ConsistentHashingLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
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
        0, (false).into()
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
        0, val.into()
      )
    }
  }

  // hash_balance_factor: optional message google.protobuf.UInt32Value
  pub fn has_hash_balance_factor(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_hash_balance_factor(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn hash_balance_factor_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_hash_balance_factor().then(|| self.hash_balance_factor())
  }
  pub fn hash_balance_factor(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn hash_balance_factor_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_hash_balance_factor(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // hash_policy: repeated message envoy.config.route.v3.RouteAction.HashPolicy
  pub fn hash_policy(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::route_action::HashPolicy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::route_action::HashPolicy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn hash_policy_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::route_action::HashPolicy> {
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
  pub fn set_hash_policy(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::route_action::HashPolicy>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl ConsistentHashingLbConfig

impl ::std::ops::Drop for ConsistentHashingLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ConsistentHashingLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ConsistentHashingLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> ConsistentHashingLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ConsistentHashingLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ConsistentHashingLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ConsistentHashingLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__load_0balancing_0policies__common__v3__ConsistentHashingLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__load_0balancing_0policies__common__v3__ConsistentHashingLbConfig_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::route::v3::route_components::route_action::HashPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__load_0balancing_0policies__common__v3__ConsistentHashingLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConsistentHashingLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConsistentHashingLbConfig {
  type Msg = ConsistentHashingLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConsistentHashingLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConsistentHashingLbConfig {
  type Msg = ConsistentHashingLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConsistentHashingLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConsistentHashingLbConfigMut<'_> {
  type Msg = ConsistentHashingLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConsistentHashingLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConsistentHashingLbConfigMut<'_> {
  type Msg = ConsistentHashingLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConsistentHashingLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConsistentHashingLbConfigView<'_> {
  type Msg = ConsistentHashingLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConsistentHashingLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConsistentHashingLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



