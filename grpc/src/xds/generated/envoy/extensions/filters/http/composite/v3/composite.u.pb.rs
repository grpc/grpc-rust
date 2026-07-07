const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__composite__v3__Composite_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Composite {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Composite>
}

impl ::protobuf::Message for Composite {
  type MessageView<'msg> = CompositeView<'msg>;
  type MessageMut<'msg> = CompositeMut<'msg>;
}

impl ::std::default::Default for Composite {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Composite {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Composite` is `Sync` because it does not implement interior mutability.
//    Neither does `CompositeMut`.
unsafe impl ::std::marker::Sync for Composite {}

// SAFETY:
// - `Composite` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Composite {}

impl ::protobuf::Proxied for Composite {
  type View<'msg> = CompositeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Composite {}

impl ::protobuf::MutProxied for Composite {
  type Mut<'msg> = CompositeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CompositeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Composite>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CompositeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CompositeView<'msg> {
  type Message = Composite;
}

impl ::std::fmt::Debug for CompositeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CompositeView<'_> {
  fn default() -> CompositeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Composite>> for CompositeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Composite>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CompositeView<'msg> {

  pub fn to_owned(&self) -> Composite {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // named_filter_chains: repeated message envoy.extensions.filters.http.composite.v3.Composite.NamedFilterChainsEntry
  pub fn named_filter_chains(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, super::FilterChainConfiguration> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::FilterChainConfiguration>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `CompositeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CompositeView<'_> {}

// SAFETY:
// - `CompositeView` is `Send` because while its alive a `CompositeMut` cannot.
// - `CompositeView` does not use thread-local data.
unsafe impl ::std::marker::Send for CompositeView<'_> {}

impl<'msg> ::protobuf::AsView for CompositeView<'msg> {
  type Proxied = Composite;
  fn as_view(&self) -> ::protobuf::View<'msg, Composite> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CompositeView<'msg> {
  fn into_view<'shorter>(self) -> CompositeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Composite> for CompositeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Composite {
    let mut dst = Composite::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Composite> for CompositeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Composite {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Composite {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CompositeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CompositeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CompositeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Composite>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CompositeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CompositeMut<'msg> {
  type Message = Composite;
}

impl ::std::fmt::Debug for CompositeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Composite>> for CompositeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Composite>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CompositeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Composite> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Composite {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // named_filter_chains: repeated message envoy.extensions.filters.http.composite.v3.Composite.NamedFilterChainsEntry
  pub fn named_filter_chains(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::FilterChainConfiguration> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::FilterChainConfiguration>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn named_filter_chains_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::FilterChainConfiguration> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_named_filter_chains(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::FilterChainConfiguration>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `CompositeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CompositeMut<'_> {}

// SAFETY:
// - `CompositeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CompositeMut<'_> {}

impl<'msg> ::protobuf::AsView for CompositeMut<'msg> {
  type Proxied = Composite;
  fn as_view(&self) -> ::protobuf::View<'_, Composite> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CompositeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Composite>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CompositeMut<'msg> {
  type MutProxied = Composite;
  fn as_mut(&mut self) -> CompositeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CompositeMut<'msg> {
  fn into_mut<'shorter>(self) -> CompositeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Composite {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Composite> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CompositeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CompositeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // named_filter_chains: repeated message envoy.extensions.filters.http.composite.v3.Composite.NamedFilterChainsEntry
  pub fn named_filter_chains(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::FilterChainConfiguration> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::FilterChainConfiguration>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn named_filter_chains_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::FilterChainConfiguration> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_named_filter_chains(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::FilterChainConfiguration>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Composite

impl ::std::ops::Drop for Composite {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Composite {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Composite {
  type Proxied = Self;
  fn as_view(&self) -> CompositeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Composite {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CompositeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Composite {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__composite__v3__Composite_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__composite__v3__Composite_msg_init.0, &[<super::composite::NamedFilterChainsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__composite__v3__Composite_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Composite {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Composite {
  type Msg = Composite;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Composite> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Composite {
  type Msg = Composite;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Composite> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CompositeMut<'_> {
  type Msg = Composite;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Composite> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CompositeMut<'_> {
  type Msg = Composite;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Composite> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CompositeView<'_> {
  type Msg = Composite;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Composite> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CompositeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod composite {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__composite__v3__Composite__NamedFilterChainsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct NamedFilterChainsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NamedFilterChainsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::composite::envoy__extensions__filters__http__composite__v3__Composite__NamedFilterChainsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::composite::envoy__extensions__filters__http__composite__v3__Composite__NamedFilterChainsEntry_msg_init.0, &[<super::super::FilterChainConfiguration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::composite::envoy__extensions__filters__http__composite__v3__Composite__NamedFilterChainsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod composite


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__composite__v3__FilterChainConfiguration_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FilterChainConfiguration {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FilterChainConfiguration>
}

impl ::protobuf::Message for FilterChainConfiguration {
  type MessageView<'msg> = FilterChainConfigurationView<'msg>;
  type MessageMut<'msg> = FilterChainConfigurationMut<'msg>;
}

impl ::std::default::Default for FilterChainConfiguration {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FilterChainConfiguration {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FilterChainConfiguration` is `Sync` because it does not implement interior mutability.
//    Neither does `FilterChainConfigurationMut`.
unsafe impl ::std::marker::Sync for FilterChainConfiguration {}

// SAFETY:
// - `FilterChainConfiguration` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FilterChainConfiguration {}

impl ::protobuf::Proxied for FilterChainConfiguration {
  type View<'msg> = FilterChainConfigurationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FilterChainConfiguration {}

impl ::protobuf::MutProxied for FilterChainConfiguration {
  type Mut<'msg> = FilterChainConfigurationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FilterChainConfigurationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FilterChainConfiguration>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilterChainConfigurationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FilterChainConfigurationView<'msg> {
  type Message = FilterChainConfiguration;
}

impl ::std::fmt::Debug for FilterChainConfigurationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FilterChainConfigurationView<'_> {
  fn default() -> FilterChainConfigurationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FilterChainConfiguration>> for FilterChainConfigurationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FilterChainConfiguration>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilterChainConfigurationView<'msg> {

  pub fn to_owned(&self) -> FilterChainConfiguration {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // typed_config: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn typed_config(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `FilterChainConfigurationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FilterChainConfigurationView<'_> {}

// SAFETY:
// - `FilterChainConfigurationView` is `Send` because while its alive a `FilterChainConfigurationMut` cannot.
// - `FilterChainConfigurationView` does not use thread-local data.
unsafe impl ::std::marker::Send for FilterChainConfigurationView<'_> {}

impl<'msg> ::protobuf::AsView for FilterChainConfigurationView<'msg> {
  type Proxied = FilterChainConfiguration;
  fn as_view(&self) -> ::protobuf::View<'msg, FilterChainConfiguration> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilterChainConfigurationView<'msg> {
  fn into_view<'shorter>(self) -> FilterChainConfigurationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FilterChainConfiguration> for FilterChainConfigurationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FilterChainConfiguration {
    let mut dst = FilterChainConfiguration::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FilterChainConfiguration> for FilterChainConfigurationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FilterChainConfiguration {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FilterChainConfiguration {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FilterChainConfigurationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FilterChainConfigurationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FilterChainConfigurationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChainConfiguration>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilterChainConfigurationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FilterChainConfigurationMut<'msg> {
  type Message = FilterChainConfiguration;
}

impl ::std::fmt::Debug for FilterChainConfigurationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChainConfiguration>> for FilterChainConfigurationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChainConfiguration>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilterChainConfigurationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChainConfiguration> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FilterChainConfiguration {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // typed_config: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn typed_config(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
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
  pub fn set_typed_config(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `FilterChainConfigurationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FilterChainConfigurationMut<'_> {}

// SAFETY:
// - `FilterChainConfigurationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FilterChainConfigurationMut<'_> {}

impl<'msg> ::protobuf::AsView for FilterChainConfigurationMut<'msg> {
  type Proxied = FilterChainConfiguration;
  fn as_view(&self) -> ::protobuf::View<'_, FilterChainConfiguration> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilterChainConfigurationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FilterChainConfiguration>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FilterChainConfigurationMut<'msg> {
  type MutProxied = FilterChainConfiguration;
  fn as_mut(&mut self) -> FilterChainConfigurationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FilterChainConfigurationMut<'msg> {
  fn into_mut<'shorter>(self) -> FilterChainConfigurationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FilterChainConfiguration {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FilterChainConfiguration> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FilterChainConfigurationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FilterChainConfigurationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // typed_config: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn typed_config(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
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
  pub fn set_typed_config(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl FilterChainConfiguration

impl ::std::ops::Drop for FilterChainConfiguration {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FilterChainConfiguration {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FilterChainConfiguration {
  type Proxied = Self;
  fn as_view(&self) -> FilterChainConfigurationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FilterChainConfiguration {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FilterChainConfigurationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FilterChainConfiguration {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__composite__v3__FilterChainConfiguration_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__composite__v3__FilterChainConfiguration_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__composite__v3__FilterChainConfiguration_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FilterChainConfiguration {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FilterChainConfiguration {
  type Msg = FilterChainConfiguration;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChainConfiguration> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterChainConfiguration {
  type Msg = FilterChainConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChainConfiguration> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FilterChainConfigurationMut<'_> {
  type Msg = FilterChainConfiguration;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChainConfiguration> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterChainConfigurationMut<'_> {
  type Msg = FilterChainConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChainConfiguration> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterChainConfigurationView<'_> {
  type Msg = FilterChainConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChainConfiguration> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FilterChainConfigurationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__composite__v3__DynamicConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DynamicConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DynamicConfig>
}

impl ::protobuf::Message for DynamicConfig {
  type MessageView<'msg> = DynamicConfigView<'msg>;
  type MessageMut<'msg> = DynamicConfigMut<'msg>;
}

impl ::std::default::Default for DynamicConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DynamicConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DynamicConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `DynamicConfigMut`.
unsafe impl ::std::marker::Sync for DynamicConfig {}

// SAFETY:
// - `DynamicConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DynamicConfig {}

impl ::protobuf::Proxied for DynamicConfig {
  type View<'msg> = DynamicConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DynamicConfig {}

impl ::protobuf::MutProxied for DynamicConfig {
  type Mut<'msg> = DynamicConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DynamicConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DynamicConfigView<'msg> {
  type Message = DynamicConfig;
}

impl ::std::fmt::Debug for DynamicConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DynamicConfigView<'_> {
  fn default() -> DynamicConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicConfig>> for DynamicConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicConfigView<'msg> {

  pub fn to_owned(&self) -> DynamicConfig {
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

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn config_discovery_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'msg>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }

}

// SAFETY:
// - `DynamicConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DynamicConfigView<'_> {}

// SAFETY:
// - `DynamicConfigView` is `Send` because while its alive a `DynamicConfigMut` cannot.
// - `DynamicConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for DynamicConfigView<'_> {}

impl<'msg> ::protobuf::AsView for DynamicConfigView<'msg> {
  type Proxied = DynamicConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, DynamicConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicConfigView<'msg> {
  fn into_view<'shorter>(self) -> DynamicConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicConfig> for DynamicConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicConfig {
    let mut dst = DynamicConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicConfig> for DynamicConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DynamicConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DynamicConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DynamicConfigMut<'msg> {
  type Message = DynamicConfig;
}

impl ::std::fmt::Debug for DynamicConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicConfig>> for DynamicConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DynamicConfig {
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

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_config_discovery(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn config_discovery_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }
  pub fn config_discovery_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceMut<'_> {
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
  pub fn set_config_discovery(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource>) {

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
// - `DynamicConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DynamicConfigMut<'_> {}

// SAFETY:
// - `DynamicConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DynamicConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for DynamicConfigMut<'msg> {
  type Proxied = DynamicConfig;
  fn as_view(&self) -> ::protobuf::View<'_, DynamicConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DynamicConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DynamicConfigMut<'msg> {
  type MutProxied = DynamicConfig;
  fn as_mut(&mut self) -> DynamicConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DynamicConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> DynamicConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DynamicConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DynamicConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DynamicConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DynamicConfigMut<'_> {
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

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_config_discovery(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn config_discovery_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }
  pub fn config_discovery_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceMut<'_> {
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
  pub fn set_config_discovery(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl DynamicConfig

impl ::std::ops::Drop for DynamicConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DynamicConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DynamicConfig {
  type Proxied = Self;
  fn as_view(&self) -> DynamicConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DynamicConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DynamicConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__composite__v3__DynamicConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__composite__v3__DynamicConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__composite__v3__DynamicConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicConfig {
  type Msg = DynamicConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicConfig {
  type Msg = DynamicConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicConfigMut<'_> {
  type Msg = DynamicConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicConfigMut<'_> {
  type Msg = DynamicConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicConfigView<'_> {
  type Msg = DynamicConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__composite__v3__ExecuteFilterAction_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExecuteFilterAction {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExecuteFilterAction>
}

impl ::protobuf::Message for ExecuteFilterAction {
  type MessageView<'msg> = ExecuteFilterActionView<'msg>;
  type MessageMut<'msg> = ExecuteFilterActionMut<'msg>;
}

impl ::std::default::Default for ExecuteFilterAction {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExecuteFilterAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExecuteFilterAction` is `Sync` because it does not implement interior mutability.
//    Neither does `ExecuteFilterActionMut`.
unsafe impl ::std::marker::Sync for ExecuteFilterAction {}

// SAFETY:
// - `ExecuteFilterAction` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExecuteFilterAction {}

impl ::protobuf::Proxied for ExecuteFilterAction {
  type View<'msg> = ExecuteFilterActionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExecuteFilterAction {}

impl ::protobuf::MutProxied for ExecuteFilterAction {
  type Mut<'msg> = ExecuteFilterActionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExecuteFilterActionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExecuteFilterAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExecuteFilterActionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExecuteFilterActionView<'msg> {
  type Message = ExecuteFilterAction;
}

impl ::std::fmt::Debug for ExecuteFilterActionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExecuteFilterActionView<'_> {
  fn default() -> ExecuteFilterActionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExecuteFilterAction>> for ExecuteFilterActionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExecuteFilterAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExecuteFilterActionView<'msg> {

  pub fn to_owned(&self) -> ExecuteFilterAction {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // typed_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn typed_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // dynamic_config: optional message envoy.extensions.filters.http.composite.v3.DynamicConfig
  pub fn has_dynamic_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn dynamic_config_opt(self) -> ::std::option::Option<super::DynamicConfigView<'msg>> {
    self.has_dynamic_config().then(|| self.dynamic_config())
  }
  pub fn dynamic_config(self) -> super::DynamicConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DynamicConfigView::default())
  }

  // filter_chain: optional message envoy.extensions.filters.http.composite.v3.FilterChainConfiguration
  pub fn has_filter_chain(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn filter_chain_opt(self) -> ::std::option::Option<super::FilterChainConfigurationView<'msg>> {
    self.has_filter_chain().then(|| self.filter_chain())
  }
  pub fn filter_chain(self) -> super::FilterChainConfigurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FilterChainConfigurationView::default())
  }

  // filter_chain_name: optional string
  pub fn filter_chain_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // sample_percent: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_sample_percent(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn sample_percent_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'msg>> {
    self.has_sample_percent().then(|| self.sample_percent())
  }
  pub fn sample_percent(self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }

}

// SAFETY:
// - `ExecuteFilterActionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExecuteFilterActionView<'_> {}

// SAFETY:
// - `ExecuteFilterActionView` is `Send` because while its alive a `ExecuteFilterActionMut` cannot.
// - `ExecuteFilterActionView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExecuteFilterActionView<'_> {}

impl<'msg> ::protobuf::AsView for ExecuteFilterActionView<'msg> {
  type Proxied = ExecuteFilterAction;
  fn as_view(&self) -> ::protobuf::View<'msg, ExecuteFilterAction> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExecuteFilterActionView<'msg> {
  fn into_view<'shorter>(self) -> ExecuteFilterActionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExecuteFilterAction> for ExecuteFilterActionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExecuteFilterAction {
    let mut dst = ExecuteFilterAction::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExecuteFilterAction> for ExecuteFilterActionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExecuteFilterAction {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExecuteFilterAction {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExecuteFilterActionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExecuteFilterActionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExecuteFilterActionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExecuteFilterAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExecuteFilterActionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExecuteFilterActionMut<'msg> {
  type Message = ExecuteFilterAction;
}

impl ::std::fmt::Debug for ExecuteFilterActionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExecuteFilterAction>> for ExecuteFilterActionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExecuteFilterAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExecuteFilterActionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExecuteFilterAction> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExecuteFilterAction {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // typed_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_typed_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // dynamic_config: optional message envoy.extensions.filters.http.composite.v3.DynamicConfig
  pub fn has_dynamic_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_dynamic_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn dynamic_config_opt(&self) -> ::std::option::Option<super::DynamicConfigView<'_>> {
    self.has_dynamic_config().then(|| self.dynamic_config())
  }
  pub fn dynamic_config(&self) -> super::DynamicConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DynamicConfigView::default())
  }
  pub fn dynamic_config_mut(&mut self) -> super::DynamicConfigMut<'_> {
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
  pub fn set_dynamic_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::DynamicConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // filter_chain: optional message envoy.extensions.filters.http.composite.v3.FilterChainConfiguration
  pub fn has_filter_chain(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_filter_chain(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn filter_chain_opt(&self) -> ::std::option::Option<super::FilterChainConfigurationView<'_>> {
    self.has_filter_chain().then(|| self.filter_chain())
  }
  pub fn filter_chain(&self) -> super::FilterChainConfigurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FilterChainConfigurationView::default())
  }
  pub fn filter_chain_mut(&mut self) -> super::FilterChainConfigurationMut<'_> {
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
  pub fn set_filter_chain(&mut self,
    val: impl ::protobuf::IntoProxied<super::FilterChainConfiguration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // filter_chain_name: optional string
  pub fn filter_chain_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_filter_chain_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // sample_percent: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_sample_percent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_sample_percent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn sample_percent_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_>> {
    self.has_sample_percent().then(|| self.sample_percent())
  }
  pub fn sample_percent(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }
  pub fn sample_percent_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentMut<'_> {
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
  pub fn set_sample_percent(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercent>) {

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
// - `ExecuteFilterActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExecuteFilterActionMut<'_> {}

// SAFETY:
// - `ExecuteFilterActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExecuteFilterActionMut<'_> {}

impl<'msg> ::protobuf::AsView for ExecuteFilterActionMut<'msg> {
  type Proxied = ExecuteFilterAction;
  fn as_view(&self) -> ::protobuf::View<'_, ExecuteFilterAction> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExecuteFilterActionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExecuteFilterAction>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExecuteFilterActionMut<'msg> {
  type MutProxied = ExecuteFilterAction;
  fn as_mut(&mut self) -> ExecuteFilterActionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExecuteFilterActionMut<'msg> {
  fn into_mut<'shorter>(self) -> ExecuteFilterActionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExecuteFilterAction {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExecuteFilterAction> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExecuteFilterActionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExecuteFilterActionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // typed_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_typed_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // dynamic_config: optional message envoy.extensions.filters.http.composite.v3.DynamicConfig
  pub fn has_dynamic_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_dynamic_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn dynamic_config_opt(&self) -> ::std::option::Option<super::DynamicConfigView<'_>> {
    self.has_dynamic_config().then(|| self.dynamic_config())
  }
  pub fn dynamic_config(&self) -> super::DynamicConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DynamicConfigView::default())
  }
  pub fn dynamic_config_mut(&mut self) -> super::DynamicConfigMut<'_> {
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
  pub fn set_dynamic_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::DynamicConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // filter_chain: optional message envoy.extensions.filters.http.composite.v3.FilterChainConfiguration
  pub fn has_filter_chain(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_filter_chain(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn filter_chain_opt(&self) -> ::std::option::Option<super::FilterChainConfigurationView<'_>> {
    self.has_filter_chain().then(|| self.filter_chain())
  }
  pub fn filter_chain(&self) -> super::FilterChainConfigurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FilterChainConfigurationView::default())
  }
  pub fn filter_chain_mut(&mut self) -> super::FilterChainConfigurationMut<'_> {
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
  pub fn set_filter_chain(&mut self,
    val: impl ::protobuf::IntoProxied<super::FilterChainConfiguration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // filter_chain_name: optional string
  pub fn filter_chain_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_filter_chain_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // sample_percent: optional message envoy.config.core.v3.RuntimeFractionalPercent
  pub fn has_sample_percent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_sample_percent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn sample_percent_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_>> {
    self.has_sample_percent().then(|| self.sample_percent())
  }
  pub fn sample_percent(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentView::default())
  }
  pub fn sample_percent_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercentMut<'_> {
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
  pub fn set_sample_percent(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl ExecuteFilterAction

impl ::std::ops::Drop for ExecuteFilterAction {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExecuteFilterAction {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExecuteFilterAction {
  type Proxied = Self;
  fn as_view(&self) -> ExecuteFilterActionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExecuteFilterAction {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExecuteFilterActionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExecuteFilterAction {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__composite__v3__ExecuteFilterAction_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33331X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__composite__v3__ExecuteFilterAction_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::DynamicConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RuntimeFractionalPercent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::FilterChainConfiguration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__composite__v3__ExecuteFilterAction_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExecuteFilterAction {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExecuteFilterAction {
  type Msg = ExecuteFilterAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExecuteFilterAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExecuteFilterAction {
  type Msg = ExecuteFilterAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExecuteFilterAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExecuteFilterActionMut<'_> {
  type Msg = ExecuteFilterAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExecuteFilterAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExecuteFilterActionMut<'_> {
  type Msg = ExecuteFilterAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExecuteFilterAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExecuteFilterActionView<'_> {
  type Msg = ExecuteFilterAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExecuteFilterAction> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExecuteFilterActionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



