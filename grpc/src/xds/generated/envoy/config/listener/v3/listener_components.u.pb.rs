const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__Filter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Filter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Filter>
}

impl ::protobuf::Message for Filter {
  type MessageView<'msg> = FilterView<'msg>;
  type MessageMut<'msg> = FilterMut<'msg>;
}

impl ::std::default::Default for Filter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Filter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Filter` is `Sync` because it does not implement interior mutability.
//    Neither does `FilterMut`.
unsafe impl ::std::marker::Sync for Filter {}

// SAFETY:
// - `Filter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Filter {}

impl ::protobuf::Proxied for Filter {
  type View<'msg> = FilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Filter {}

impl ::protobuf::MutProxied for Filter {
  type Mut<'msg> = FilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Filter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FilterView<'msg> {
  type Message = Filter;
}

impl ::std::fmt::Debug for FilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FilterView<'_> {
  fn default() -> FilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Filter>> for FilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Filter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilterView<'msg> {

  pub fn to_owned(&self) -> Filter {
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

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn config_discovery_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'msg>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }

  pub fn config_type(self) -> super::filter::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::filter::ConfigTypeCase::TypedConfig =>
          super::filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      super::filter::ConfigTypeCase::ConfigDiscovery =>
          super::filter::ConfigTypeOneof::ConfigDiscovery(self.config_discovery()),
      _ => super::filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FilterView<'_> {}

// SAFETY:
// - `FilterView` is `Send` because while its alive a `FilterMut` cannot.
// - `FilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for FilterView<'_> {}

impl<'msg> ::protobuf::AsView for FilterView<'msg> {
  type Proxied = Filter;
  fn as_view(&self) -> ::protobuf::View<'msg, Filter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilterView<'msg> {
  fn into_view<'shorter>(self) -> FilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Filter> for FilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Filter {
    let mut dst = Filter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Filter> for FilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Filter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Filter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Filter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FilterMut<'msg> {
  type Message = Filter;
}

impl ::std::fmt::Debug for FilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Filter>> for FilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Filter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Filter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Filter {
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

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_config_discovery(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn config_discovery_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }
  pub fn config_discovery_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceMut<'_> {
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
  pub fn set_config_discovery(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn config_type(&self) -> super::filter::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::filter::ConfigTypeCase::TypedConfig =>
          super::filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      super::filter::ConfigTypeCase::ConfigDiscovery =>
          super::filter::ConfigTypeOneof::ConfigDiscovery(self.config_discovery()),
      _ => super::filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FilterMut<'_> {}

// SAFETY:
// - `FilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FilterMut<'_> {}

impl<'msg> ::protobuf::AsView for FilterMut<'msg> {
  type Proxied = Filter;
  fn as_view(&self) -> ::protobuf::View<'_, Filter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Filter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FilterMut<'msg> {
  type MutProxied = Filter;
  fn as_mut(&mut self) -> FilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FilterMut<'msg> {
  fn into_mut<'shorter>(self) -> FilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Filter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Filter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FilterMut<'_> {
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

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_config_discovery(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn config_discovery_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }
  pub fn config_discovery_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceMut<'_> {
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
  pub fn set_config_discovery(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn config_type(&self) -> super::filter::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::filter::ConfigTypeCase::TypedConfig =>
          super::filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      super::filter::ConfigTypeCase::ConfigDiscovery =>
          super::filter::ConfigTypeOneof::ConfigDiscovery(self.config_discovery()),
      _ => super::filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Filter

impl ::std::ops::Drop for Filter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Filter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Filter {
  type Proxied = Self;
  fn as_view(&self) -> FilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Filter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Filter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__Filter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xb33^%|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__Filter_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__Filter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Filter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Filter {
  type Msg = Filter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Filter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Filter {
  type Msg = Filter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Filter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FilterMut<'_> {
  type Msg = Filter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Filter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterMut<'_> {
  type Msg = Filter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Filter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterView<'_> {
  type Msg = Filter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Filter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod filter {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigTypeOneof<'msg> {
  TypedConfig(::protobuf::View<'msg, ::protobuf_well_known_types::Any>) = 4,
  ConfigDiscovery(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigTypeCase {
  TypedConfig = 4,
  ConfigDiscovery = 5,

  not_set = 0
}

impl ConfigTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigTypeCase> {
    match v {
      0 => Some(ConfigTypeCase::not_set),
      4 => Some(ConfigTypeCase::TypedConfig),
      5 => Some(ConfigTypeCase::ConfigDiscovery),
      _ => None
    }
  }
}
}  // pub mod filter


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__FilterChainMatch_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FilterChainMatch {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FilterChainMatch>
}

impl ::protobuf::Message for FilterChainMatch {
  type MessageView<'msg> = FilterChainMatchView<'msg>;
  type MessageMut<'msg> = FilterChainMatchMut<'msg>;
}

impl ::std::default::Default for FilterChainMatch {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FilterChainMatch {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FilterChainMatch` is `Sync` because it does not implement interior mutability.
//    Neither does `FilterChainMatchMut`.
unsafe impl ::std::marker::Sync for FilterChainMatch {}

// SAFETY:
// - `FilterChainMatch` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FilterChainMatch {}

impl ::protobuf::Proxied for FilterChainMatch {
  type View<'msg> = FilterChainMatchView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FilterChainMatch {}

impl ::protobuf::MutProxied for FilterChainMatch {
  type Mut<'msg> = FilterChainMatchMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FilterChainMatchView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FilterChainMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilterChainMatchView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FilterChainMatchView<'msg> {
  type Message = FilterChainMatch;
}

impl ::std::fmt::Debug for FilterChainMatchView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FilterChainMatchView<'_> {
  fn default() -> FilterChainMatchView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FilterChainMatch>> for FilterChainMatchView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FilterChainMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilterChainMatchView<'msg> {

  pub fn to_owned(&self) -> FilterChainMatch {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // destination_port: optional message google.protobuf.UInt32Value
  pub fn has_destination_port(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn destination_port_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_destination_port().then(|| self.destination_port())
  }
  pub fn destination_port(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // prefix_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn prefix_ranges(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // address_suffix: optional string
  pub fn address_suffix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // suffix_len: optional message google.protobuf.UInt32Value
  pub fn has_suffix_len(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn suffix_len_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_suffix_len().then(|| self.suffix_len())
  }
  pub fn suffix_len(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // direct_source_prefix_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn direct_source_prefix_ranges(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // source_type: optional enum envoy.config.listener.v3.FilterChainMatch.ConnectionSourceType
  pub fn source_type(self) -> super::filter_chain_match::ConnectionSourceType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        9, (super::filter_chain_match::ConnectionSourceType::Any).into()
      ).try_into().unwrap()
    }
  }

  // source_prefix_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn source_prefix_ranges(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // source_ports: repeated uint32
  pub fn source_ports(self) -> ::protobuf::RepeatedView<'msg, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // server_names: repeated string
  pub fn server_names(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // transport_protocol: optional string
  pub fn transport_protocol(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // application_protocols: repeated string
  pub fn application_protocols(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
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
// - `FilterChainMatchView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FilterChainMatchView<'_> {}

// SAFETY:
// - `FilterChainMatchView` is `Send` because while its alive a `FilterChainMatchMut` cannot.
// - `FilterChainMatchView` does not use thread-local data.
unsafe impl ::std::marker::Send for FilterChainMatchView<'_> {}

impl<'msg> ::protobuf::AsView for FilterChainMatchView<'msg> {
  type Proxied = FilterChainMatch;
  fn as_view(&self) -> ::protobuf::View<'msg, FilterChainMatch> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilterChainMatchView<'msg> {
  fn into_view<'shorter>(self) -> FilterChainMatchView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FilterChainMatch> for FilterChainMatchView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FilterChainMatch {
    let mut dst = FilterChainMatch::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FilterChainMatch> for FilterChainMatchMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FilterChainMatch {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FilterChainMatch {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FilterChainMatchView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FilterChainMatchMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FilterChainMatchMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChainMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilterChainMatchMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FilterChainMatchMut<'msg> {
  type Message = FilterChainMatch;
}

impl ::std::fmt::Debug for FilterChainMatchMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChainMatch>> for FilterChainMatchMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChainMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilterChainMatchMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChainMatch> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FilterChainMatch {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // destination_port: optional message google.protobuf.UInt32Value
  pub fn has_destination_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_destination_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn destination_port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_destination_port().then(|| self.destination_port())
  }
  pub fn destination_port(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn destination_port_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_destination_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // prefix_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn prefix_ranges(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn prefix_ranges_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
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
  pub fn set_prefix_ranges(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // address_suffix: optional string
  pub fn address_suffix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_address_suffix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // suffix_len: optional message google.protobuf.UInt32Value
  pub fn has_suffix_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_suffix_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn suffix_len_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_suffix_len().then(|| self.suffix_len())
  }
  pub fn suffix_len(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn suffix_len_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_suffix_len(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // direct_source_prefix_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn direct_source_prefix_ranges(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn direct_source_prefix_ranges_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        10,
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
  pub fn set_direct_source_prefix_ranges(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        src);
    }
  }

  // source_type: optional enum envoy.config.listener.v3.FilterChainMatch.ConnectionSourceType
  pub fn source_type(&self) -> super::filter_chain_match::ConnectionSourceType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        9, (super::filter_chain_match::ConnectionSourceType::Any).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_source_type(&mut self, val: super::filter_chain_match::ConnectionSourceType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        9, val.into()
      )
    }
  }

  // source_prefix_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn source_prefix_ranges(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn source_prefix_ranges_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
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
  pub fn set_source_prefix_ranges(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // source_ports: repeated uint32
  pub fn source_ports(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn source_ports_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
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
  pub fn set_source_ports(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // server_names: repeated string
  pub fn server_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn server_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        8,
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
  pub fn set_server_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // transport_protocol: optional string
  pub fn transport_protocol(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_transport_protocol(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // application_protocols: repeated string
  pub fn application_protocols(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn application_protocols_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_application_protocols(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

}

// SAFETY:
// - `FilterChainMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FilterChainMatchMut<'_> {}

// SAFETY:
// - `FilterChainMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FilterChainMatchMut<'_> {}

impl<'msg> ::protobuf::AsView for FilterChainMatchMut<'msg> {
  type Proxied = FilterChainMatch;
  fn as_view(&self) -> ::protobuf::View<'_, FilterChainMatch> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilterChainMatchMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FilterChainMatch>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FilterChainMatchMut<'msg> {
  type MutProxied = FilterChainMatch;
  fn as_mut(&mut self) -> FilterChainMatchMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FilterChainMatchMut<'msg> {
  fn into_mut<'shorter>(self) -> FilterChainMatchMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FilterChainMatch {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FilterChainMatch> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FilterChainMatchView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FilterChainMatchMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // destination_port: optional message google.protobuf.UInt32Value
  pub fn has_destination_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_destination_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn destination_port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_destination_port().then(|| self.destination_port())
  }
  pub fn destination_port(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn destination_port_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_destination_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // prefix_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn prefix_ranges(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn prefix_ranges_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
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
  pub fn set_prefix_ranges(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // address_suffix: optional string
  pub fn address_suffix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_address_suffix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // suffix_len: optional message google.protobuf.UInt32Value
  pub fn has_suffix_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_suffix_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn suffix_len_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_suffix_len().then(|| self.suffix_len())
  }
  pub fn suffix_len(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn suffix_len_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_suffix_len(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // direct_source_prefix_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn direct_source_prefix_ranges(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn direct_source_prefix_ranges_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        10,
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
  pub fn set_direct_source_prefix_ranges(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        src);
    }
  }

  // source_type: optional enum envoy.config.listener.v3.FilterChainMatch.ConnectionSourceType
  pub fn source_type(&self) -> super::filter_chain_match::ConnectionSourceType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        9, (super::filter_chain_match::ConnectionSourceType::Any).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_source_type(&mut self, val: super::filter_chain_match::ConnectionSourceType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        9, val.into()
      )
    }
  }

  // source_prefix_ranges: repeated message envoy.config.core.v3.CidrRange
  pub fn source_prefix_ranges(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn source_prefix_ranges_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
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
  pub fn set_source_prefix_ranges(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // source_ports: repeated uint32
  pub fn source_ports(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn source_ports_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
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
  pub fn set_source_ports(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // server_names: repeated string
  pub fn server_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn server_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        8,
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
  pub fn set_server_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // transport_protocol: optional string
  pub fn transport_protocol(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_transport_protocol(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // application_protocols: repeated string
  pub fn application_protocols(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn application_protocols_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_application_protocols(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

}  // impl FilterChainMatch

impl ::std::ops::Drop for FilterChainMatch {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FilterChainMatch {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FilterChainMatch {
  type Proxied = Self;
  fn as_view(&self) -> FilterChainMatchView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FilterChainMatch {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FilterChainMatchMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FilterChainMatch {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__FilterChainMatch_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$NbG1X3G=31XETET.PG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__FilterChainMatch_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::CidrRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::CidrRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::CidrRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__FilterChainMatch_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FilterChainMatch {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FilterChainMatch {
  type Msg = FilterChainMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChainMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterChainMatch {
  type Msg = FilterChainMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChainMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FilterChainMatchMut<'_> {
  type Msg = FilterChainMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChainMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterChainMatchMut<'_> {
  type Msg = FilterChainMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChainMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterChainMatchView<'_> {
  type Msg = FilterChainMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChainMatch> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FilterChainMatchMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod filter_chain_match {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConnectionSourceType(i32);

#[allow(non_upper_case_globals)]
impl ConnectionSourceType {
  pub const Any: ConnectionSourceType = ConnectionSourceType(0);
  pub const SameIpOrLoopback: ConnectionSourceType = ConnectionSourceType(1);
  pub const External: ConnectionSourceType = ConnectionSourceType(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Any",
      1 => "SameIpOrLoopback",
      2 => "External",
      _ => return None
    })
  }
}

impl ::std::convert::From<ConnectionSourceType> for i32 {
  fn from(val: ConnectionSourceType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ConnectionSourceType {
  fn from(val: i32) -> ConnectionSourceType {
    Self(val)
  }
}

impl ::std::default::Default for ConnectionSourceType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ConnectionSourceType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ConnectionSourceType::{}", constant_name)
    } else {
      write!(f, "ConnectionSourceType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ConnectionSourceType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ConnectionSourceType {}

impl ::protobuf::Proxied for ConnectionSourceType {
  type View<'a> = ConnectionSourceType;
}

impl ::protobuf::AsView for ConnectionSourceType {
  type Proxied = ConnectionSourceType;

  fn as_view(&self) -> ConnectionSourceType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConnectionSourceType {
  fn into_view<'shorter>(self) -> ConnectionSourceType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ConnectionSourceType {
  const NAME: &'static str = "ConnectionSourceType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for ConnectionSourceType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod filter_chain_match


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__FilterChain_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FilterChain {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FilterChain>
}

impl ::protobuf::Message for FilterChain {
  type MessageView<'msg> = FilterChainView<'msg>;
  type MessageMut<'msg> = FilterChainMut<'msg>;
}

impl ::std::default::Default for FilterChain {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FilterChain {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FilterChain` is `Sync` because it does not implement interior mutability.
//    Neither does `FilterChainMut`.
unsafe impl ::std::marker::Sync for FilterChain {}

// SAFETY:
// - `FilterChain` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FilterChain {}

impl ::protobuf::Proxied for FilterChain {
  type View<'msg> = FilterChainView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FilterChain {}

impl ::protobuf::MutProxied for FilterChain {
  type Mut<'msg> = FilterChainMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FilterChainView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FilterChain>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilterChainView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FilterChainView<'msg> {
  type Message = FilterChain;
}

impl ::std::fmt::Debug for FilterChainView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FilterChainView<'_> {
  fn default() -> FilterChainView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FilterChain>> for FilterChainView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FilterChain>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilterChainView<'msg> {

  pub fn to_owned(&self) -> FilterChain {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // filter_chain_match: optional message envoy.config.listener.v3.FilterChainMatch
  pub fn has_filter_chain_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn filter_chain_match_opt(self) -> ::std::option::Option<super::FilterChainMatchView<'msg>> {
    self.has_filter_chain_match().then(|| self.filter_chain_match())
  }
  pub fn filter_chain_match(self) -> super::FilterChainMatchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FilterChainMatchView::default())
  }

  // filters: repeated message envoy.config.listener.v3.Filter
  pub fn filters(self) -> ::protobuf::RepeatedView<'msg, super::Filter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Filter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // use_proxy_proto: optional message google.protobuf.BoolValue
  pub fn has_use_proxy_proto(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn use_proxy_proto_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_use_proxy_proto().then(|| self.use_proxy_proto())
  }
  pub fn use_proxy_proto(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn transport_socket_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'msg>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }

  // transport_socket_connect_timeout: optional message google.protobuf.Duration
  pub fn has_transport_socket_connect_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn transport_socket_connect_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_transport_socket_connect_timeout().then(|| self.transport_socket_connect_timeout())
  }
  pub fn transport_socket_connect_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `FilterChainView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FilterChainView<'_> {}

// SAFETY:
// - `FilterChainView` is `Send` because while its alive a `FilterChainMut` cannot.
// - `FilterChainView` does not use thread-local data.
unsafe impl ::std::marker::Send for FilterChainView<'_> {}

impl<'msg> ::protobuf::AsView for FilterChainView<'msg> {
  type Proxied = FilterChain;
  fn as_view(&self) -> ::protobuf::View<'msg, FilterChain> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilterChainView<'msg> {
  fn into_view<'shorter>(self) -> FilterChainView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FilterChain> for FilterChainView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FilterChain {
    let mut dst = FilterChain::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FilterChain> for FilterChainMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FilterChain {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FilterChain {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FilterChainView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FilterChainMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FilterChainMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChain>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilterChainMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FilterChainMut<'msg> {
  type Message = FilterChain;
}

impl ::std::fmt::Debug for FilterChainMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChain>> for FilterChainMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChain>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilterChainMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterChain> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FilterChain {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // filter_chain_match: optional message envoy.config.listener.v3.FilterChainMatch
  pub fn has_filter_chain_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_filter_chain_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn filter_chain_match_opt(&self) -> ::std::option::Option<super::FilterChainMatchView<'_>> {
    self.has_filter_chain_match().then(|| self.filter_chain_match())
  }
  pub fn filter_chain_match(&self) -> super::FilterChainMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FilterChainMatchView::default())
  }
  pub fn filter_chain_match_mut(&mut self) -> super::FilterChainMatchMut<'_> {
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
  pub fn set_filter_chain_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::FilterChainMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // filters: repeated message envoy.config.listener.v3.Filter
  pub fn filters(&self) -> ::protobuf::RepeatedView<'_, super::Filter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Filter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Filter> {
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
  pub fn set_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Filter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // use_proxy_proto: optional message google.protobuf.BoolValue
  pub fn has_use_proxy_proto(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_use_proxy_proto(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn use_proxy_proto_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_proxy_proto().then(|| self.use_proxy_proto())
  }
  pub fn use_proxy_proto(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_proxy_proto_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_proxy_proto(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_transport_socket(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn transport_socket_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(&self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }
  pub fn transport_socket_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketMut<'_> {
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
  pub fn set_transport_socket(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::TransportSocket>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // transport_socket_connect_timeout: optional message google.protobuf.Duration
  pub fn has_transport_socket_connect_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_transport_socket_connect_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn transport_socket_connect_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_transport_socket_connect_timeout().then(|| self.transport_socket_connect_timeout())
  }
  pub fn transport_socket_connect_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn transport_socket_connect_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_transport_socket_connect_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

}

// SAFETY:
// - `FilterChainMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FilterChainMut<'_> {}

// SAFETY:
// - `FilterChainMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FilterChainMut<'_> {}

impl<'msg> ::protobuf::AsView for FilterChainMut<'msg> {
  type Proxied = FilterChain;
  fn as_view(&self) -> ::protobuf::View<'_, FilterChain> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilterChainMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FilterChain>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FilterChainMut<'msg> {
  type MutProxied = FilterChain;
  fn as_mut(&mut self) -> FilterChainMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FilterChainMut<'msg> {
  fn into_mut<'shorter>(self) -> FilterChainMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FilterChain {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FilterChain> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FilterChainView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FilterChainMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // filter_chain_match: optional message envoy.config.listener.v3.FilterChainMatch
  pub fn has_filter_chain_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_filter_chain_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn filter_chain_match_opt(&self) -> ::std::option::Option<super::FilterChainMatchView<'_>> {
    self.has_filter_chain_match().then(|| self.filter_chain_match())
  }
  pub fn filter_chain_match(&self) -> super::FilterChainMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FilterChainMatchView::default())
  }
  pub fn filter_chain_match_mut(&mut self) -> super::FilterChainMatchMut<'_> {
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
  pub fn set_filter_chain_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::FilterChainMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // filters: repeated message envoy.config.listener.v3.Filter
  pub fn filters(&self) -> ::protobuf::RepeatedView<'_, super::Filter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Filter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Filter> {
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
  pub fn set_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Filter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // use_proxy_proto: optional message google.protobuf.BoolValue
  pub fn has_use_proxy_proto(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_use_proxy_proto(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn use_proxy_proto_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_proxy_proto().then(|| self.use_proxy_proto())
  }
  pub fn use_proxy_proto(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_proxy_proto_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_proxy_proto(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_transport_socket(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn transport_socket_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(&self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }
  pub fn transport_socket_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketMut<'_> {
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
  pub fn set_transport_socket(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::TransportSocket>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // transport_socket_connect_timeout: optional message google.protobuf.Duration
  pub fn has_transport_socket_connect_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_transport_socket_connect_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn transport_socket_connect_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_transport_socket_connect_timeout().then(|| self.transport_socket_connect_timeout())
  }
  pub fn transport_socket_connect_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn transport_socket_connect_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_transport_socket_connect_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

}  // impl FilterChain

impl ::std::ops::Drop for FilterChain {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FilterChain {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FilterChain {
  type Proxied = Self;
  fn as_view(&self) -> FilterChainView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FilterChain {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FilterChainMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FilterChain {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__FilterChain_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3aG3331Xa3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__FilterChain_msg_init.0, &[<super::FilterChainMatch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Filter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::TransportSocket as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__FilterChain_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FilterChain {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FilterChain {
  type Msg = FilterChain;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChain> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterChain {
  type Msg = FilterChain;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChain> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FilterChainMut<'_> {
  type Msg = FilterChain;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChain> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterChainMut<'_> {
  type Msg = FilterChain;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChain> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterChainView<'_> {
  type Msg = FilterChain;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterChain> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FilterChainMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__ListenerFilterChainMatchPredicate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListenerFilterChainMatchPredicate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListenerFilterChainMatchPredicate>
}

impl ::protobuf::Message for ListenerFilterChainMatchPredicate {
  type MessageView<'msg> = ListenerFilterChainMatchPredicateView<'msg>;
  type MessageMut<'msg> = ListenerFilterChainMatchPredicateMut<'msg>;
}

impl ::std::default::Default for ListenerFilterChainMatchPredicate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListenerFilterChainMatchPredicate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListenerFilterChainMatchPredicate` is `Sync` because it does not implement interior mutability.
//    Neither does `ListenerFilterChainMatchPredicateMut`.
unsafe impl ::std::marker::Sync for ListenerFilterChainMatchPredicate {}

// SAFETY:
// - `ListenerFilterChainMatchPredicate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ListenerFilterChainMatchPredicate {}

impl ::protobuf::Proxied for ListenerFilterChainMatchPredicate {
  type View<'msg> = ListenerFilterChainMatchPredicateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListenerFilterChainMatchPredicate {}

impl ::protobuf::MutProxied for ListenerFilterChainMatchPredicate {
  type Mut<'msg> = ListenerFilterChainMatchPredicateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListenerFilterChainMatchPredicateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerFilterChainMatchPredicate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenerFilterChainMatchPredicateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListenerFilterChainMatchPredicateView<'msg> {
  type Message = ListenerFilterChainMatchPredicate;
}

impl ::std::fmt::Debug for ListenerFilterChainMatchPredicateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListenerFilterChainMatchPredicateView<'_> {
  fn default() -> ListenerFilterChainMatchPredicateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerFilterChainMatchPredicate>> for ListenerFilterChainMatchPredicateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerFilterChainMatchPredicate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenerFilterChainMatchPredicateView<'msg> {

  pub fn to_owned(&self) -> ListenerFilterChainMatchPredicate {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // or_match: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate.MatchSet
  pub fn has_or_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn or_match_opt(self) -> ::std::option::Option<super::listener_filter_chain_match_predicate::MatchSetView<'msg>> {
    self.has_or_match().then(|| self.or_match())
  }
  pub fn or_match(self) -> super::listener_filter_chain_match_predicate::MatchSetView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener_filter_chain_match_predicate::MatchSetView::default())
  }

  // and_match: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate.MatchSet
  pub fn has_and_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn and_match_opt(self) -> ::std::option::Option<super::listener_filter_chain_match_predicate::MatchSetView<'msg>> {
    self.has_and_match().then(|| self.and_match())
  }
  pub fn and_match(self) -> super::listener_filter_chain_match_predicate::MatchSetView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener_filter_chain_match_predicate::MatchSetView::default())
  }

  // not_match: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate
  pub fn has_not_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn not_match_opt(self) -> ::std::option::Option<super::ListenerFilterChainMatchPredicateView<'msg>> {
    self.has_not_match().then(|| self.not_match())
  }
  pub fn not_match(self) -> super::ListenerFilterChainMatchPredicateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ListenerFilterChainMatchPredicateView::default())
  }

  // any_match: optional bool
  pub fn has_any_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn any_match_opt(self) -> ::std::option::Option<bool> {
    self.has_any_match().then(|| self.any_match())
  }
  pub fn any_match(self) -> bool {
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

  // destination_port_range: optional message envoy.type.v3.Int32Range
  pub fn has_destination_port_range(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn destination_port_range_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'msg>> {
    self.has_destination_port_range().then(|| self.destination_port_range())
  }
  pub fn destination_port_range(self) -> crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::range::Int32RangeView::default())
  }

  pub fn rule(self) -> super::listener_filter_chain_match_predicate::RuleOneof<'msg> {
    match self.rule_case() {
      super::listener_filter_chain_match_predicate::RuleCase::OrMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::OrMatch(self.or_match()),
      super::listener_filter_chain_match_predicate::RuleCase::AndMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::AndMatch(self.and_match()),
      super::listener_filter_chain_match_predicate::RuleCase::NotMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::NotMatch(self.not_match()),
      super::listener_filter_chain_match_predicate::RuleCase::AnyMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::AnyMatch(self.any_match()),
      super::listener_filter_chain_match_predicate::RuleCase::DestinationPortRange =>
          super::listener_filter_chain_match_predicate::RuleOneof::DestinationPortRange(self.destination_port_range()),
      _ => super::listener_filter_chain_match_predicate::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(self) -> super::listener_filter_chain_match_predicate::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::listener_filter_chain_match_predicate::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ListenerFilterChainMatchPredicateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ListenerFilterChainMatchPredicateView<'_> {}

// SAFETY:
// - `ListenerFilterChainMatchPredicateView` is `Send` because while its alive a `ListenerFilterChainMatchPredicateMut` cannot.
// - `ListenerFilterChainMatchPredicateView` does not use thread-local data.
unsafe impl ::std::marker::Send for ListenerFilterChainMatchPredicateView<'_> {}

impl<'msg> ::protobuf::AsView for ListenerFilterChainMatchPredicateView<'msg> {
  type Proxied = ListenerFilterChainMatchPredicate;
  fn as_view(&self) -> ::protobuf::View<'msg, ListenerFilterChainMatchPredicate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenerFilterChainMatchPredicateView<'msg> {
  fn into_view<'shorter>(self) -> ListenerFilterChainMatchPredicateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListenerFilterChainMatchPredicate> for ListenerFilterChainMatchPredicateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListenerFilterChainMatchPredicate {
    let mut dst = ListenerFilterChainMatchPredicate::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListenerFilterChainMatchPredicate> for ListenerFilterChainMatchPredicateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListenerFilterChainMatchPredicate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ListenerFilterChainMatchPredicate {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenerFilterChainMatchPredicateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenerFilterChainMatchPredicateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListenerFilterChainMatchPredicateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerFilterChainMatchPredicate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenerFilterChainMatchPredicateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListenerFilterChainMatchPredicateMut<'msg> {
  type Message = ListenerFilterChainMatchPredicate;
}

impl ::std::fmt::Debug for ListenerFilterChainMatchPredicateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerFilterChainMatchPredicate>> for ListenerFilterChainMatchPredicateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerFilterChainMatchPredicate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenerFilterChainMatchPredicateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerFilterChainMatchPredicate> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ListenerFilterChainMatchPredicate {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // or_match: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate.MatchSet
  pub fn has_or_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_or_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn or_match_opt(&self) -> ::std::option::Option<super::listener_filter_chain_match_predicate::MatchSetView<'_>> {
    self.has_or_match().then(|| self.or_match())
  }
  pub fn or_match(&self) -> super::listener_filter_chain_match_predicate::MatchSetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener_filter_chain_match_predicate::MatchSetView::default())
  }
  pub fn or_match_mut(&mut self) -> super::listener_filter_chain_match_predicate::MatchSetMut<'_> {
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
  pub fn set_or_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener_filter_chain_match_predicate::MatchSet>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // and_match: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate.MatchSet
  pub fn has_and_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_and_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn and_match_opt(&self) -> ::std::option::Option<super::listener_filter_chain_match_predicate::MatchSetView<'_>> {
    self.has_and_match().then(|| self.and_match())
  }
  pub fn and_match(&self) -> super::listener_filter_chain_match_predicate::MatchSetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener_filter_chain_match_predicate::MatchSetView::default())
  }
  pub fn and_match_mut(&mut self) -> super::listener_filter_chain_match_predicate::MatchSetMut<'_> {
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
  pub fn set_and_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener_filter_chain_match_predicate::MatchSet>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // not_match: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate
  pub fn has_not_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_not_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn not_match_opt(&self) -> ::std::option::Option<super::ListenerFilterChainMatchPredicateView<'_>> {
    self.has_not_match().then(|| self.not_match())
  }
  pub fn not_match(&self) -> super::ListenerFilterChainMatchPredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ListenerFilterChainMatchPredicateView::default())
  }
  pub fn not_match_mut(&mut self) -> super::ListenerFilterChainMatchPredicateMut<'_> {
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
  pub fn set_not_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::ListenerFilterChainMatchPredicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // any_match: optional bool
  pub fn has_any_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_any_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn any_match_opt(&self) -> ::std::option::Option<bool> {
    self.has_any_match().then(|| self.any_match())
  }
  pub fn any_match(&self) -> bool {
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
  pub fn set_any_match(&mut self, val: bool) {
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

  // destination_port_range: optional message envoy.type.v3.Int32Range
  pub fn has_destination_port_range(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_destination_port_range(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn destination_port_range_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'_>> {
    self.has_destination_port_range().then(|| self.destination_port_range())
  }
  pub fn destination_port_range(&self) -> crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::range::Int32RangeView::default())
  }
  pub fn destination_port_range_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::range::Int32RangeMut<'_> {
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
  pub fn set_destination_port_range(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::range::Int32Range>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn rule(&self) -> super::listener_filter_chain_match_predicate::RuleOneof<'_> {
    match &self.rule_case() {
      super::listener_filter_chain_match_predicate::RuleCase::OrMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::OrMatch(self.or_match()),
      super::listener_filter_chain_match_predicate::RuleCase::AndMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::AndMatch(self.and_match()),
      super::listener_filter_chain_match_predicate::RuleCase::NotMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::NotMatch(self.not_match()),
      super::listener_filter_chain_match_predicate::RuleCase::AnyMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::AnyMatch(self.any_match()),
      super::listener_filter_chain_match_predicate::RuleCase::DestinationPortRange =>
          super::listener_filter_chain_match_predicate::RuleOneof::DestinationPortRange(self.destination_port_range()),
      _ => super::listener_filter_chain_match_predicate::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(&self) -> super::listener_filter_chain_match_predicate::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::listener_filter_chain_match_predicate::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ListenerFilterChainMatchPredicateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ListenerFilterChainMatchPredicateMut<'_> {}

// SAFETY:
// - `ListenerFilterChainMatchPredicateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ListenerFilterChainMatchPredicateMut<'_> {}

impl<'msg> ::protobuf::AsView for ListenerFilterChainMatchPredicateMut<'msg> {
  type Proxied = ListenerFilterChainMatchPredicate;
  fn as_view(&self) -> ::protobuf::View<'_, ListenerFilterChainMatchPredicate> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenerFilterChainMatchPredicateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListenerFilterChainMatchPredicate>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ListenerFilterChainMatchPredicateMut<'msg> {
  type MutProxied = ListenerFilterChainMatchPredicate;
  fn as_mut(&mut self) -> ListenerFilterChainMatchPredicateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListenerFilterChainMatchPredicateMut<'msg> {
  fn into_mut<'shorter>(self) -> ListenerFilterChainMatchPredicateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListenerFilterChainMatchPredicate {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListenerFilterChainMatchPredicate> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListenerFilterChainMatchPredicateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListenerFilterChainMatchPredicateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // or_match: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate.MatchSet
  pub fn has_or_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_or_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn or_match_opt(&self) -> ::std::option::Option<super::listener_filter_chain_match_predicate::MatchSetView<'_>> {
    self.has_or_match().then(|| self.or_match())
  }
  pub fn or_match(&self) -> super::listener_filter_chain_match_predicate::MatchSetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener_filter_chain_match_predicate::MatchSetView::default())
  }
  pub fn or_match_mut(&mut self) -> super::listener_filter_chain_match_predicate::MatchSetMut<'_> {
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
  pub fn set_or_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener_filter_chain_match_predicate::MatchSet>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // and_match: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate.MatchSet
  pub fn has_and_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_and_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn and_match_opt(&self) -> ::std::option::Option<super::listener_filter_chain_match_predicate::MatchSetView<'_>> {
    self.has_and_match().then(|| self.and_match())
  }
  pub fn and_match(&self) -> super::listener_filter_chain_match_predicate::MatchSetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::listener_filter_chain_match_predicate::MatchSetView::default())
  }
  pub fn and_match_mut(&mut self) -> super::listener_filter_chain_match_predicate::MatchSetMut<'_> {
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
  pub fn set_and_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::listener_filter_chain_match_predicate::MatchSet>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // not_match: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate
  pub fn has_not_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_not_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn not_match_opt(&self) -> ::std::option::Option<super::ListenerFilterChainMatchPredicateView<'_>> {
    self.has_not_match().then(|| self.not_match())
  }
  pub fn not_match(&self) -> super::ListenerFilterChainMatchPredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ListenerFilterChainMatchPredicateView::default())
  }
  pub fn not_match_mut(&mut self) -> super::ListenerFilterChainMatchPredicateMut<'_> {
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
  pub fn set_not_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::ListenerFilterChainMatchPredicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // any_match: optional bool
  pub fn has_any_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_any_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn any_match_opt(&self) -> ::std::option::Option<bool> {
    self.has_any_match().then(|| self.any_match())
  }
  pub fn any_match(&self) -> bool {
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
  pub fn set_any_match(&mut self, val: bool) {
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

  // destination_port_range: optional message envoy.type.v3.Int32Range
  pub fn has_destination_port_range(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_destination_port_range(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn destination_port_range_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'_>> {
    self.has_destination_port_range().then(|| self.destination_port_range())
  }
  pub fn destination_port_range(&self) -> crate::xds::generated::envoy::r#type::v3::range::Int32RangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::range::Int32RangeView::default())
  }
  pub fn destination_port_range_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::range::Int32RangeMut<'_> {
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
  pub fn set_destination_port_range(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::range::Int32Range>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn rule(&self) -> super::listener_filter_chain_match_predicate::RuleOneof<'_> {
    match &self.rule_case() {
      super::listener_filter_chain_match_predicate::RuleCase::OrMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::OrMatch(self.or_match()),
      super::listener_filter_chain_match_predicate::RuleCase::AndMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::AndMatch(self.and_match()),
      super::listener_filter_chain_match_predicate::RuleCase::NotMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::NotMatch(self.not_match()),
      super::listener_filter_chain_match_predicate::RuleCase::AnyMatch =>
          super::listener_filter_chain_match_predicate::RuleOneof::AnyMatch(self.any_match()),
      super::listener_filter_chain_match_predicate::RuleCase::DestinationPortRange =>
          super::listener_filter_chain_match_predicate::RuleOneof::DestinationPortRange(self.destination_port_range()),
      _ => super::listener_filter_chain_match_predicate::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(&self) -> super::listener_filter_chain_match_predicate::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::listener_filter_chain_match_predicate::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ListenerFilterChainMatchPredicate

impl ::std::ops::Drop for ListenerFilterChainMatchPredicate {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListenerFilterChainMatchPredicate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListenerFilterChainMatchPredicate {
  type Proxied = Self;
  fn as_view(&self) -> ListenerFilterChainMatchPredicateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListenerFilterChainMatchPredicate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListenerFilterChainMatchPredicateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListenerFilterChainMatchPredicate {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__ListenerFilterChainMatchPredicate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333/3^!|#|$|%|&");
        super::listener_filter_chain_match_predicate::envoy__config__listener__v3__ListenerFilterChainMatchPredicate__MatchSet_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__ListenerFilterChainMatchPredicate_msg_init.0, &[super::listener_filter_chain_match_predicate::envoy__config__listener__v3__ListenerFilterChainMatchPredicate__MatchSet_msg_init.0,
            super::listener_filter_chain_match_predicate::envoy__config__listener__v3__ListenerFilterChainMatchPredicate__MatchSet_msg_init.0,
            super::envoy__config__listener__v3__ListenerFilterChainMatchPredicate_msg_init.0,
            <crate::xds::generated::envoy::r#type::v3::range::Int32Range as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::listener_filter_chain_match_predicate::envoy__config__listener__v3__ListenerFilterChainMatchPredicate__MatchSet_msg_init.0, &[super::envoy__config__listener__v3__ListenerFilterChainMatchPredicate_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__ListenerFilterChainMatchPredicate_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListenerFilterChainMatchPredicate {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListenerFilterChainMatchPredicate {
  type Msg = ListenerFilterChainMatchPredicate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerFilterChainMatchPredicate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerFilterChainMatchPredicate {
  type Msg = ListenerFilterChainMatchPredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerFilterChainMatchPredicate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListenerFilterChainMatchPredicateMut<'_> {
  type Msg = ListenerFilterChainMatchPredicate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerFilterChainMatchPredicate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerFilterChainMatchPredicateMut<'_> {
  type Msg = ListenerFilterChainMatchPredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerFilterChainMatchPredicate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerFilterChainMatchPredicateView<'_> {
  type Msg = ListenerFilterChainMatchPredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerFilterChainMatchPredicate> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListenerFilterChainMatchPredicateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod listener_filter_chain_match_predicate {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__ListenerFilterChainMatchPredicate__MatchSet_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MatchSet {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MatchSet>
}

impl ::protobuf::Message for MatchSet {
  type MessageView<'msg> = MatchSetView<'msg>;
  type MessageMut<'msg> = MatchSetMut<'msg>;
}

impl ::std::default::Default for MatchSet {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MatchSet {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MatchSet` is `Sync` because it does not implement interior mutability.
//    Neither does `MatchSetMut`.
unsafe impl ::std::marker::Sync for MatchSet {}

// SAFETY:
// - `MatchSet` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MatchSet {}

impl ::protobuf::Proxied for MatchSet {
  type View<'msg> = MatchSetView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MatchSet {}

impl ::protobuf::MutProxied for MatchSet {
  type Mut<'msg> = MatchSetMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MatchSetView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatchSet>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatchSetView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MatchSetView<'msg> {
  type Message = MatchSet;
}

impl ::std::fmt::Debug for MatchSetView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MatchSetView<'_> {
  fn default() -> MatchSetView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MatchSet>> for MatchSetView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatchSet>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatchSetView<'msg> {

  pub fn to_owned(&self) -> MatchSet {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // rules: repeated message envoy.config.listener.v3.ListenerFilterChainMatchPredicate
  pub fn rules(self) -> ::protobuf::RepeatedView<'msg, super::super::ListenerFilterChainMatchPredicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::ListenerFilterChainMatchPredicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `MatchSetView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MatchSetView<'_> {}

// SAFETY:
// - `MatchSetView` is `Send` because while its alive a `MatchSetMut` cannot.
// - `MatchSetView` does not use thread-local data.
unsafe impl ::std::marker::Send for MatchSetView<'_> {}

impl<'msg> ::protobuf::AsView for MatchSetView<'msg> {
  type Proxied = MatchSet;
  fn as_view(&self) -> ::protobuf::View<'msg, MatchSet> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatchSetView<'msg> {
  fn into_view<'shorter>(self) -> MatchSetView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MatchSet> for MatchSetView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatchSet {
    let mut dst = MatchSet::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MatchSet> for MatchSetMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatchSet {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MatchSet {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatchSetView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatchSetMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MatchSetMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchSet>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatchSetMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MatchSetMut<'msg> {
  type Message = MatchSet;
}

impl ::std::fmt::Debug for MatchSetMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MatchSet>> for MatchSetMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchSet>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatchSetMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchSet> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MatchSet {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // rules: repeated message envoy.config.listener.v3.ListenerFilterChainMatchPredicate
  pub fn rules(&self) -> ::protobuf::RepeatedView<'_, super::super::ListenerFilterChainMatchPredicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::ListenerFilterChainMatchPredicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn rules_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::ListenerFilterChainMatchPredicate> {
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
  pub fn set_rules(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::ListenerFilterChainMatchPredicate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `MatchSetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MatchSetMut<'_> {}

// SAFETY:
// - `MatchSetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MatchSetMut<'_> {}

impl<'msg> ::protobuf::AsView for MatchSetMut<'msg> {
  type Proxied = MatchSet;
  fn as_view(&self) -> ::protobuf::View<'_, MatchSet> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatchSetMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MatchSet>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MatchSetMut<'msg> {
  type MutProxied = MatchSet;
  fn as_mut(&mut self) -> MatchSetMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MatchSetMut<'msg> {
  fn into_mut<'shorter>(self) -> MatchSetMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MatchSet {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MatchSet> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MatchSetView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MatchSetMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // rules: repeated message envoy.config.listener.v3.ListenerFilterChainMatchPredicate
  pub fn rules(&self) -> ::protobuf::RepeatedView<'_, super::super::ListenerFilterChainMatchPredicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::ListenerFilterChainMatchPredicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn rules_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::ListenerFilterChainMatchPredicate> {
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
  pub fn set_rules(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::ListenerFilterChainMatchPredicate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl MatchSet

impl ::std::ops::Drop for MatchSet {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MatchSet {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MatchSet {
  type Proxied = Self;
  fn as_view(&self) -> MatchSetView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MatchSet {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MatchSetMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MatchSet {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::ListenerFilterChainMatchPredicate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::listener_filter_chain_match_predicate::envoy__config__listener__v3__ListenerFilterChainMatchPredicate__MatchSet_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatchSet {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatchSet {
  type Msg = MatchSet;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchSet> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchSet {
  type Msg = MatchSet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchSet> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatchSetMut<'_> {
  type Msg = MatchSet;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchSet> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchSetMut<'_> {
  type Msg = MatchSet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchSet> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchSetView<'_> {
  type Msg = MatchSet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchSet> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatchSetMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum RuleOneof<'msg> {
  OrMatch(::protobuf::View<'msg, super::super::listener_filter_chain_match_predicate::MatchSet>) = 1,
  AndMatch(::protobuf::View<'msg, super::super::listener_filter_chain_match_predicate::MatchSet>) = 2,
  NotMatch(::protobuf::View<'msg, super::super::ListenerFilterChainMatchPredicate>) = 3,
  AnyMatch(bool) = 4,
  DestinationPortRange(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::v3::range::Int32Range>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum RuleCase {
  OrMatch = 1,
  AndMatch = 2,
  NotMatch = 3,
  AnyMatch = 4,
  DestinationPortRange = 5,

  not_set = 0
}

impl RuleCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<RuleCase> {
    match v {
      0 => Some(RuleCase::not_set),
      1 => Some(RuleCase::OrMatch),
      2 => Some(RuleCase::AndMatch),
      3 => Some(RuleCase::NotMatch),
      4 => Some(RuleCase::AnyMatch),
      5 => Some(RuleCase::DestinationPortRange),
      _ => None
    }
  }
}
}  // pub mod listener_filter_chain_match_predicate


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__ListenerFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListenerFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListenerFilter>
}

impl ::protobuf::Message for ListenerFilter {
  type MessageView<'msg> = ListenerFilterView<'msg>;
  type MessageMut<'msg> = ListenerFilterMut<'msg>;
}

impl ::std::default::Default for ListenerFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListenerFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListenerFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `ListenerFilterMut`.
unsafe impl ::std::marker::Sync for ListenerFilter {}

// SAFETY:
// - `ListenerFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ListenerFilter {}

impl ::protobuf::Proxied for ListenerFilter {
  type View<'msg> = ListenerFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListenerFilter {}

impl ::protobuf::MutProxied for ListenerFilter {
  type Mut<'msg> = ListenerFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListenerFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenerFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListenerFilterView<'msg> {
  type Message = ListenerFilter;
}

impl ::std::fmt::Debug for ListenerFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListenerFilterView<'_> {
  fn default() -> ListenerFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerFilter>> for ListenerFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListenerFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenerFilterView<'msg> {

  pub fn to_owned(&self) -> ListenerFilter {
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

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn config_discovery_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'msg>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }

  // filter_disabled: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate
  pub fn has_filter_disabled(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn filter_disabled_opt(self) -> ::std::option::Option<super::ListenerFilterChainMatchPredicateView<'msg>> {
    self.has_filter_disabled().then(|| self.filter_disabled())
  }
  pub fn filter_disabled(self) -> super::ListenerFilterChainMatchPredicateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ListenerFilterChainMatchPredicateView::default())
  }

  pub fn config_type(self) -> super::listener_filter::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::listener_filter::ConfigTypeCase::TypedConfig =>
          super::listener_filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      super::listener_filter::ConfigTypeCase::ConfigDiscovery =>
          super::listener_filter::ConfigTypeOneof::ConfigDiscovery(self.config_discovery()),
      _ => super::listener_filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::listener_filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::listener_filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ListenerFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ListenerFilterView<'_> {}

// SAFETY:
// - `ListenerFilterView` is `Send` because while its alive a `ListenerFilterMut` cannot.
// - `ListenerFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for ListenerFilterView<'_> {}

impl<'msg> ::protobuf::AsView for ListenerFilterView<'msg> {
  type Proxied = ListenerFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, ListenerFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenerFilterView<'msg> {
  fn into_view<'shorter>(self) -> ListenerFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListenerFilter> for ListenerFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListenerFilter {
    let mut dst = ListenerFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListenerFilter> for ListenerFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListenerFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ListenerFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenerFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListenerFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListenerFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListenerFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListenerFilterMut<'msg> {
  type Message = ListenerFilter;
}

impl ::std::fmt::Debug for ListenerFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerFilter>> for ListenerFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListenerFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListenerFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ListenerFilter {
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

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_config_discovery(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn config_discovery_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }
  pub fn config_discovery_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceMut<'_> {
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
  pub fn set_config_discovery(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // filter_disabled: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate
  pub fn has_filter_disabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_filter_disabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn filter_disabled_opt(&self) -> ::std::option::Option<super::ListenerFilterChainMatchPredicateView<'_>> {
    self.has_filter_disabled().then(|| self.filter_disabled())
  }
  pub fn filter_disabled(&self) -> super::ListenerFilterChainMatchPredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ListenerFilterChainMatchPredicateView::default())
  }
  pub fn filter_disabled_mut(&mut self) -> super::ListenerFilterChainMatchPredicateMut<'_> {
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
  pub fn set_filter_disabled(&mut self,
    val: impl ::protobuf::IntoProxied<super::ListenerFilterChainMatchPredicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn config_type(&self) -> super::listener_filter::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::listener_filter::ConfigTypeCase::TypedConfig =>
          super::listener_filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      super::listener_filter::ConfigTypeCase::ConfigDiscovery =>
          super::listener_filter::ConfigTypeOneof::ConfigDiscovery(self.config_discovery()),
      _ => super::listener_filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::listener_filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::listener_filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ListenerFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ListenerFilterMut<'_> {}

// SAFETY:
// - `ListenerFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ListenerFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for ListenerFilterMut<'msg> {
  type Proxied = ListenerFilter;
  fn as_view(&self) -> ::protobuf::View<'_, ListenerFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListenerFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListenerFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ListenerFilterMut<'msg> {
  type MutProxied = ListenerFilter;
  fn as_mut(&mut self) -> ListenerFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListenerFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> ListenerFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListenerFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListenerFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListenerFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListenerFilterMut<'_> {
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

  // config_discovery: optional message envoy.config.core.v3.ExtensionConfigSource
  pub fn has_config_discovery(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_config_discovery(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn config_discovery_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_>> {
    self.has_config_discovery().then(|| self.config_discovery())
  }
  pub fn config_discovery(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceView::default())
  }
  pub fn config_discovery_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSourceMut<'_> {
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
  pub fn set_config_discovery(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // filter_disabled: optional message envoy.config.listener.v3.ListenerFilterChainMatchPredicate
  pub fn has_filter_disabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_filter_disabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn filter_disabled_opt(&self) -> ::std::option::Option<super::ListenerFilterChainMatchPredicateView<'_>> {
    self.has_filter_disabled().then(|| self.filter_disabled())
  }
  pub fn filter_disabled(&self) -> super::ListenerFilterChainMatchPredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ListenerFilterChainMatchPredicateView::default())
  }
  pub fn filter_disabled_mut(&mut self) -> super::ListenerFilterChainMatchPredicateMut<'_> {
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
  pub fn set_filter_disabled(&mut self,
    val: impl ::protobuf::IntoProxied<super::ListenerFilterChainMatchPredicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn config_type(&self) -> super::listener_filter::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::listener_filter::ConfigTypeCase::TypedConfig =>
          super::listener_filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      super::listener_filter::ConfigTypeCase::ConfigDiscovery =>
          super::listener_filter::ConfigTypeOneof::ConfigDiscovery(self.config_discovery()),
      _ => super::listener_filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::listener_filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::listener_filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ListenerFilter

impl ::std::ops::Drop for ListenerFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListenerFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListenerFilter {
  type Proxied = Self;
  fn as_view(&self) -> ListenerFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListenerFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListenerFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListenerFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__ListenerFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xa333^$|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__ListenerFilter_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ListenerFilterChainMatchPredicate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__ListenerFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListenerFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListenerFilter {
  type Msg = ListenerFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerFilter {
  type Msg = ListenerFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListenerFilterMut<'_> {
  type Msg = ListenerFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerFilterMut<'_> {
  type Msg = ListenerFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListenerFilterView<'_> {
  type Msg = ListenerFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListenerFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListenerFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod listener_filter {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigTypeOneof<'msg> {
  TypedConfig(::protobuf::View<'msg, ::protobuf_well_known_types::Any>) = 3,
  ConfigDiscovery(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::config_source::ExtensionConfigSource>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigTypeCase {
  TypedConfig = 3,
  ConfigDiscovery = 5,

  not_set = 0
}

impl ConfigTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigTypeCase> {
    match v {
      0 => Some(ConfigTypeCase::not_set),
      3 => Some(ConfigTypeCase::TypedConfig),
      5 => Some(ConfigTypeCase::ConfigDiscovery),
      _ => None
    }
  }
}
}  // pub mod listener_filter


