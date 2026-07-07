const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__ResourceLocator_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResourceLocator {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResourceLocator>
}

impl ::protobuf::Message for ResourceLocator {
  type MessageView<'msg> = ResourceLocatorView<'msg>;
  type MessageMut<'msg> = ResourceLocatorMut<'msg>;
}

impl ::std::default::Default for ResourceLocator {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResourceLocator {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResourceLocator` is `Sync` because it does not implement interior mutability.
//    Neither does `ResourceLocatorMut`.
unsafe impl ::std::marker::Sync for ResourceLocator {}

// SAFETY:
// - `ResourceLocator` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ResourceLocator {}

impl ::protobuf::Proxied for ResourceLocator {
  type View<'msg> = ResourceLocatorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResourceLocator {}

impl ::protobuf::MutProxied for ResourceLocator {
  type Mut<'msg> = ResourceLocatorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResourceLocatorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceLocator>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceLocatorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResourceLocatorView<'msg> {
  type Message = ResourceLocator;
}

impl ::std::fmt::Debug for ResourceLocatorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResourceLocatorView<'_> {
  fn default() -> ResourceLocatorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceLocator>> for ResourceLocatorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceLocator>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceLocatorView<'msg> {

  pub fn to_owned(&self) -> ResourceLocator {
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

  // dynamic_parameters: repeated message envoy.service.discovery.v3.ResourceLocator.DynamicParametersEntry
  pub fn dynamic_parameters(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `ResourceLocatorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ResourceLocatorView<'_> {}

// SAFETY:
// - `ResourceLocatorView` is `Send` because while its alive a `ResourceLocatorMut` cannot.
// - `ResourceLocatorView` does not use thread-local data.
unsafe impl ::std::marker::Send for ResourceLocatorView<'_> {}

impl<'msg> ::protobuf::AsView for ResourceLocatorView<'msg> {
  type Proxied = ResourceLocator;
  fn as_view(&self) -> ::protobuf::View<'msg, ResourceLocator> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceLocatorView<'msg> {
  fn into_view<'shorter>(self) -> ResourceLocatorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceLocator> for ResourceLocatorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceLocator {
    let mut dst = ResourceLocator::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceLocator> for ResourceLocatorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceLocator {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ResourceLocator {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceLocatorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceLocatorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResourceLocatorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceLocator>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceLocatorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResourceLocatorMut<'msg> {
  type Message = ResourceLocator;
}

impl ::std::fmt::Debug for ResourceLocatorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceLocator>> for ResourceLocatorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceLocator>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceLocatorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceLocator> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ResourceLocator {
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

  // dynamic_parameters: repeated message envoy.service.discovery.v3.ResourceLocator.DynamicParametersEntry
  pub fn dynamic_parameters(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn dynamic_parameters_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_dynamic_parameters(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `ResourceLocatorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ResourceLocatorMut<'_> {}

// SAFETY:
// - `ResourceLocatorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ResourceLocatorMut<'_> {}

impl<'msg> ::protobuf::AsView for ResourceLocatorMut<'msg> {
  type Proxied = ResourceLocator;
  fn as_view(&self) -> ::protobuf::View<'_, ResourceLocator> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceLocatorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResourceLocator>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ResourceLocatorMut<'msg> {
  type MutProxied = ResourceLocator;
  fn as_mut(&mut self) -> ResourceLocatorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResourceLocatorMut<'msg> {
  fn into_mut<'shorter>(self) -> ResourceLocatorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResourceLocator {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResourceLocator> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResourceLocatorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResourceLocatorMut<'_> {
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

  // dynamic_parameters: repeated message envoy.service.discovery.v3.ResourceLocator.DynamicParametersEntry
  pub fn dynamic_parameters(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn dynamic_parameters_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_dynamic_parameters(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl ResourceLocator

impl ::std::ops::Drop for ResourceLocator {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResourceLocator {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResourceLocator {
  type Proxied = Self;
  fn as_view(&self) -> ResourceLocatorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResourceLocator {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResourceLocatorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResourceLocator {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__discovery__v3__ResourceLocator_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__discovery__v3__ResourceLocator_msg_init.0, &[<super::resource_locator::DynamicParametersEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__discovery__v3__ResourceLocator_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceLocator {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceLocator {
  type Msg = ResourceLocator;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceLocator> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceLocator {
  type Msg = ResourceLocator;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceLocator> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceLocatorMut<'_> {
  type Msg = ResourceLocator;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceLocator> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceLocatorMut<'_> {
  type Msg = ResourceLocator;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceLocator> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceLocatorView<'_> {
  type Msg = ResourceLocator;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceLocator> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceLocatorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod resource_locator {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__ResourceLocator__DynamicParametersEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct DynamicParametersEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicParametersEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::resource_locator::envoy__service__discovery__v3__ResourceLocator__DynamicParametersEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::resource_locator::envoy__service__discovery__v3__ResourceLocator__DynamicParametersEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::resource_locator::envoy__service__discovery__v3__ResourceLocator__DynamicParametersEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod resource_locator


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__ResourceName_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResourceName {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResourceName>
}

impl ::protobuf::Message for ResourceName {
  type MessageView<'msg> = ResourceNameView<'msg>;
  type MessageMut<'msg> = ResourceNameMut<'msg>;
}

impl ::std::default::Default for ResourceName {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResourceName {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResourceName` is `Sync` because it does not implement interior mutability.
//    Neither does `ResourceNameMut`.
unsafe impl ::std::marker::Sync for ResourceName {}

// SAFETY:
// - `ResourceName` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ResourceName {}

impl ::protobuf::Proxied for ResourceName {
  type View<'msg> = ResourceNameView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResourceName {}

impl ::protobuf::MutProxied for ResourceName {
  type Mut<'msg> = ResourceNameMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResourceNameView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceName>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceNameView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResourceNameView<'msg> {
  type Message = ResourceName;
}

impl ::std::fmt::Debug for ResourceNameView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResourceNameView<'_> {
  fn default() -> ResourceNameView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceName>> for ResourceNameView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceName>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceNameView<'msg> {

  pub fn to_owned(&self) -> ResourceName {
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

  // dynamic_parameter_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints
  pub fn has_dynamic_parameter_constraints(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn dynamic_parameter_constraints_opt(self) -> ::std::option::Option<super::DynamicParameterConstraintsView<'msg>> {
    self.has_dynamic_parameter_constraints().then(|| self.dynamic_parameter_constraints())
  }
  pub fn dynamic_parameter_constraints(self) -> super::DynamicParameterConstraintsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DynamicParameterConstraintsView::default())
  }

}

// SAFETY:
// - `ResourceNameView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ResourceNameView<'_> {}

// SAFETY:
// - `ResourceNameView` is `Send` because while its alive a `ResourceNameMut` cannot.
// - `ResourceNameView` does not use thread-local data.
unsafe impl ::std::marker::Send for ResourceNameView<'_> {}

impl<'msg> ::protobuf::AsView for ResourceNameView<'msg> {
  type Proxied = ResourceName;
  fn as_view(&self) -> ::protobuf::View<'msg, ResourceName> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceNameView<'msg> {
  fn into_view<'shorter>(self) -> ResourceNameView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceName> for ResourceNameView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceName {
    let mut dst = ResourceName::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceName> for ResourceNameMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceName {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ResourceName {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceNameView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceNameMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResourceNameMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceName>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceNameMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResourceNameMut<'msg> {
  type Message = ResourceName;
}

impl ::std::fmt::Debug for ResourceNameMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceName>> for ResourceNameMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceName>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceNameMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceName> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ResourceName {
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

  // dynamic_parameter_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints
  pub fn has_dynamic_parameter_constraints(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_dynamic_parameter_constraints(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn dynamic_parameter_constraints_opt(&self) -> ::std::option::Option<super::DynamicParameterConstraintsView<'_>> {
    self.has_dynamic_parameter_constraints().then(|| self.dynamic_parameter_constraints())
  }
  pub fn dynamic_parameter_constraints(&self) -> super::DynamicParameterConstraintsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DynamicParameterConstraintsView::default())
  }
  pub fn dynamic_parameter_constraints_mut(&mut self) -> super::DynamicParameterConstraintsMut<'_> {
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
  pub fn set_dynamic_parameter_constraints(&mut self,
    val: impl ::protobuf::IntoProxied<super::DynamicParameterConstraints>) {

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
// - `ResourceNameMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ResourceNameMut<'_> {}

// SAFETY:
// - `ResourceNameMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ResourceNameMut<'_> {}

impl<'msg> ::protobuf::AsView for ResourceNameMut<'msg> {
  type Proxied = ResourceName;
  fn as_view(&self) -> ::protobuf::View<'_, ResourceName> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceNameMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResourceName>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ResourceNameMut<'msg> {
  type MutProxied = ResourceName;
  fn as_mut(&mut self) -> ResourceNameMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResourceNameMut<'msg> {
  fn into_mut<'shorter>(self) -> ResourceNameMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResourceName {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResourceName> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResourceNameView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResourceNameMut<'_> {
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

  // dynamic_parameter_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints
  pub fn has_dynamic_parameter_constraints(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_dynamic_parameter_constraints(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn dynamic_parameter_constraints_opt(&self) -> ::std::option::Option<super::DynamicParameterConstraintsView<'_>> {
    self.has_dynamic_parameter_constraints().then(|| self.dynamic_parameter_constraints())
  }
  pub fn dynamic_parameter_constraints(&self) -> super::DynamicParameterConstraintsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DynamicParameterConstraintsView::default())
  }
  pub fn dynamic_parameter_constraints_mut(&mut self) -> super::DynamicParameterConstraintsMut<'_> {
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
  pub fn set_dynamic_parameter_constraints(&mut self,
    val: impl ::protobuf::IntoProxied<super::DynamicParameterConstraints>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ResourceName

impl ::std::ops::Drop for ResourceName {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResourceName {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResourceName {
  type Proxied = Self;
  fn as_view(&self) -> ResourceNameView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResourceName {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResourceNameMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResourceName {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__discovery__v3__ResourceName_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__discovery__v3__ResourceName_msg_init.0, &[<super::DynamicParameterConstraints as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__discovery__v3__ResourceName_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceName {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceName {
  type Msg = ResourceName;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceName> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceName {
  type Msg = ResourceName;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceName> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceNameMut<'_> {
  type Msg = ResourceName;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceName> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceNameMut<'_> {
  type Msg = ResourceName;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceName> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceNameView<'_> {
  type Msg = ResourceName;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceName> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceNameMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__ResourceError_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResourceError {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResourceError>
}

impl ::protobuf::Message for ResourceError {
  type MessageView<'msg> = ResourceErrorView<'msg>;
  type MessageMut<'msg> = ResourceErrorMut<'msg>;
}

impl ::std::default::Default for ResourceError {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResourceError {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResourceError` is `Sync` because it does not implement interior mutability.
//    Neither does `ResourceErrorMut`.
unsafe impl ::std::marker::Sync for ResourceError {}

// SAFETY:
// - `ResourceError` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ResourceError {}

impl ::protobuf::Proxied for ResourceError {
  type View<'msg> = ResourceErrorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResourceError {}

impl ::protobuf::MutProxied for ResourceError {
  type Mut<'msg> = ResourceErrorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResourceErrorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceError>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceErrorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResourceErrorView<'msg> {
  type Message = ResourceError;
}

impl ::std::fmt::Debug for ResourceErrorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResourceErrorView<'_> {
  fn default() -> ResourceErrorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceError>> for ResourceErrorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceError>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceErrorView<'msg> {

  pub fn to_owned(&self) -> ResourceError {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // resource_name: optional message envoy.service.discovery.v3.ResourceName
  pub fn has_resource_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn resource_name_opt(self) -> ::std::option::Option<super::ResourceNameView<'msg>> {
    self.has_resource_name().then(|| self.resource_name())
  }
  pub fn resource_name(self) -> super::ResourceNameView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResourceNameView::default())
  }

  // error_detail: optional message google.rpc.Status
  pub fn has_error_detail(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn error_detail_opt(self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'msg>> {
    self.has_error_detail().then(|| self.error_detail())
  }
  pub fn error_detail(self) -> crate::xds::generated::google::rpc::status::StatusView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }

}

// SAFETY:
// - `ResourceErrorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ResourceErrorView<'_> {}

// SAFETY:
// - `ResourceErrorView` is `Send` because while its alive a `ResourceErrorMut` cannot.
// - `ResourceErrorView` does not use thread-local data.
unsafe impl ::std::marker::Send for ResourceErrorView<'_> {}

impl<'msg> ::protobuf::AsView for ResourceErrorView<'msg> {
  type Proxied = ResourceError;
  fn as_view(&self) -> ::protobuf::View<'msg, ResourceError> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceErrorView<'msg> {
  fn into_view<'shorter>(self) -> ResourceErrorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceError> for ResourceErrorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceError {
    let mut dst = ResourceError::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceError> for ResourceErrorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceError {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ResourceError {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceErrorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceErrorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResourceErrorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceError>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceErrorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResourceErrorMut<'msg> {
  type Message = ResourceError;
}

impl ::std::fmt::Debug for ResourceErrorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceError>> for ResourceErrorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceError>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceErrorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceError> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ResourceError {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // resource_name: optional message envoy.service.discovery.v3.ResourceName
  pub fn has_resource_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_resource_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn resource_name_opt(&self) -> ::std::option::Option<super::ResourceNameView<'_>> {
    self.has_resource_name().then(|| self.resource_name())
  }
  pub fn resource_name(&self) -> super::ResourceNameView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResourceNameView::default())
  }
  pub fn resource_name_mut(&mut self) -> super::ResourceNameMut<'_> {
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
  pub fn set_resource_name(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResourceName>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // error_detail: optional message google.rpc.Status
  pub fn has_error_detail(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_error_detail(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn error_detail_opt(&self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'_>> {
    self.has_error_detail().then(|| self.error_detail())
  }
  pub fn error_detail(&self) -> crate::xds::generated::google::rpc::status::StatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }
  pub fn error_detail_mut(&mut self) -> crate::xds::generated::google::rpc::status::StatusMut<'_> {
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
  pub fn set_error_detail(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::rpc::status::Status>) {

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
// - `ResourceErrorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ResourceErrorMut<'_> {}

// SAFETY:
// - `ResourceErrorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ResourceErrorMut<'_> {}

impl<'msg> ::protobuf::AsView for ResourceErrorMut<'msg> {
  type Proxied = ResourceError;
  fn as_view(&self) -> ::protobuf::View<'_, ResourceError> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceErrorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResourceError>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ResourceErrorMut<'msg> {
  type MutProxied = ResourceError;
  fn as_mut(&mut self) -> ResourceErrorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResourceErrorMut<'msg> {
  fn into_mut<'shorter>(self) -> ResourceErrorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResourceError {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResourceError> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResourceErrorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResourceErrorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // resource_name: optional message envoy.service.discovery.v3.ResourceName
  pub fn has_resource_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_resource_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn resource_name_opt(&self) -> ::std::option::Option<super::ResourceNameView<'_>> {
    self.has_resource_name().then(|| self.resource_name())
  }
  pub fn resource_name(&self) -> super::ResourceNameView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResourceNameView::default())
  }
  pub fn resource_name_mut(&mut self) -> super::ResourceNameMut<'_> {
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
  pub fn set_resource_name(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResourceName>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // error_detail: optional message google.rpc.Status
  pub fn has_error_detail(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_error_detail(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn error_detail_opt(&self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'_>> {
    self.has_error_detail().then(|| self.error_detail())
  }
  pub fn error_detail(&self) -> crate::xds::generated::google::rpc::status::StatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }
  pub fn error_detail_mut(&mut self) -> crate::xds::generated::google::rpc::status::StatusMut<'_> {
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
  pub fn set_error_detail(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::rpc::status::Status>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ResourceError

impl ::std::ops::Drop for ResourceError {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResourceError {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResourceError {
  type Proxied = Self;
  fn as_view(&self) -> ResourceErrorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResourceError {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResourceErrorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResourceError {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__discovery__v3__ResourceError_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__discovery__v3__ResourceError_msg_init.0, &[<super::ResourceName as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::google::rpc::status::Status as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__discovery__v3__ResourceError_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceError {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceError {
  type Msg = ResourceError;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceError> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceError {
  type Msg = ResourceError;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceError> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceErrorMut<'_> {
  type Msg = ResourceError;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceError> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceErrorMut<'_> {
  type Msg = ResourceError;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceError> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceErrorView<'_> {
  type Msg = ResourceError;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceError> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceErrorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__DiscoveryRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DiscoveryRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DiscoveryRequest>
}

impl ::protobuf::Message for DiscoveryRequest {
  type MessageView<'msg> = DiscoveryRequestView<'msg>;
  type MessageMut<'msg> = DiscoveryRequestMut<'msg>;
}

impl ::std::default::Default for DiscoveryRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DiscoveryRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DiscoveryRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `DiscoveryRequestMut`.
unsafe impl ::std::marker::Sync for DiscoveryRequest {}

// SAFETY:
// - `DiscoveryRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DiscoveryRequest {}

impl ::protobuf::Proxied for DiscoveryRequest {
  type View<'msg> = DiscoveryRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DiscoveryRequest {}

impl ::protobuf::MutProxied for DiscoveryRequest {
  type Mut<'msg> = DiscoveryRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DiscoveryRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoveryRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DiscoveryRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DiscoveryRequestView<'msg> {
  type Message = DiscoveryRequest;
}

impl ::std::fmt::Debug for DiscoveryRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DiscoveryRequestView<'_> {
  fn default() -> DiscoveryRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoveryRequest>> for DiscoveryRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoveryRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DiscoveryRequestView<'msg> {

  pub fn to_owned(&self) -> DiscoveryRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn node_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'msg>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }

  // resource_names: repeated string
  pub fn resource_names(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // resource_locators: repeated message envoy.service.discovery.v3.ResourceLocator
  pub fn resource_locators(self) -> ::protobuf::RepeatedView<'msg, super::ResourceLocator> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceLocator>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // type_url: optional string
  pub fn type_url(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // response_nonce: optional string
  pub fn response_nonce(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // error_detail: optional message google.rpc.Status
  pub fn has_error_detail(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn error_detail_opt(self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'msg>> {
    self.has_error_detail().then(|| self.error_detail())
  }
  pub fn error_detail(self) -> crate::xds::generated::google::rpc::status::StatusView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }

}

// SAFETY:
// - `DiscoveryRequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DiscoveryRequestView<'_> {}

// SAFETY:
// - `DiscoveryRequestView` is `Send` because while its alive a `DiscoveryRequestMut` cannot.
// - `DiscoveryRequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for DiscoveryRequestView<'_> {}

impl<'msg> ::protobuf::AsView for DiscoveryRequestView<'msg> {
  type Proxied = DiscoveryRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, DiscoveryRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DiscoveryRequestView<'msg> {
  fn into_view<'shorter>(self) -> DiscoveryRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DiscoveryRequest> for DiscoveryRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DiscoveryRequest {
    let mut dst = DiscoveryRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DiscoveryRequest> for DiscoveryRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DiscoveryRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DiscoveryRequest {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DiscoveryRequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DiscoveryRequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DiscoveryRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoveryRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DiscoveryRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DiscoveryRequestMut<'msg> {
  type Message = DiscoveryRequest;
}

impl ::std::fmt::Debug for DiscoveryRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoveryRequest>> for DiscoveryRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoveryRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DiscoveryRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoveryRequest> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DiscoveryRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_node(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn node_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'_>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(&self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }
  pub fn node_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::NodeMut<'_> {
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
  pub fn set_node(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Node>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // resource_names: repeated string
  pub fn resource_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_resource_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // resource_locators: repeated message envoy.service.discovery.v3.ResourceLocator
  pub fn resource_locators(&self) -> ::protobuf::RepeatedView<'_, super::ResourceLocator> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceLocator>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_locators_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceLocator> {
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
  pub fn set_resource_locators(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceLocator>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // response_nonce: optional string
  pub fn response_nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_response_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // error_detail: optional message google.rpc.Status
  pub fn has_error_detail(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_error_detail(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn error_detail_opt(&self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'_>> {
    self.has_error_detail().then(|| self.error_detail())
  }
  pub fn error_detail(&self) -> crate::xds::generated::google::rpc::status::StatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }
  pub fn error_detail_mut(&mut self) -> crate::xds::generated::google::rpc::status::StatusMut<'_> {
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
  pub fn set_error_detail(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::rpc::status::Status>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}

// SAFETY:
// - `DiscoveryRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DiscoveryRequestMut<'_> {}

// SAFETY:
// - `DiscoveryRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DiscoveryRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for DiscoveryRequestMut<'msg> {
  type Proxied = DiscoveryRequest;
  fn as_view(&self) -> ::protobuf::View<'_, DiscoveryRequest> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DiscoveryRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DiscoveryRequest>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DiscoveryRequestMut<'msg> {
  type MutProxied = DiscoveryRequest;
  fn as_mut(&mut self) -> DiscoveryRequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DiscoveryRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> DiscoveryRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DiscoveryRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DiscoveryRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DiscoveryRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DiscoveryRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_node(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn node_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'_>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(&self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }
  pub fn node_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::NodeMut<'_> {
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
  pub fn set_node(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Node>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // resource_names: repeated string
  pub fn resource_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_resource_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // resource_locators: repeated message envoy.service.discovery.v3.ResourceLocator
  pub fn resource_locators(&self) -> ::protobuf::RepeatedView<'_, super::ResourceLocator> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceLocator>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_locators_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceLocator> {
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
  pub fn set_resource_locators(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceLocator>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // response_nonce: optional string
  pub fn response_nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_response_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // error_detail: optional message google.rpc.Status
  pub fn has_error_detail(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_error_detail(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn error_detail_opt(&self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'_>> {
    self.has_error_detail().then(|| self.error_detail())
  }
  pub fn error_detail(&self) -> crate::xds::generated::google::rpc::status::StatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }
  pub fn error_detail_mut(&mut self) -> crate::xds::generated::google::rpc::status::StatusMut<'_> {
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
  pub fn set_error_detail(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::rpc::status::Status>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}  // impl DiscoveryRequest

impl ::std::ops::Drop for DiscoveryRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DiscoveryRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DiscoveryRequest {
  type Proxied = Self;
  fn as_view(&self) -> DiscoveryRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DiscoveryRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DiscoveryRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DiscoveryRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__discovery__v3__DiscoveryRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3ET1X1X3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__discovery__v3__DiscoveryRequest_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::Node as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::google::rpc::status::Status as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ResourceLocator as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__discovery__v3__DiscoveryRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DiscoveryRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DiscoveryRequest {
  type Msg = DiscoveryRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoveryRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoveryRequest {
  type Msg = DiscoveryRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoveryRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DiscoveryRequestMut<'_> {
  type Msg = DiscoveryRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoveryRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoveryRequestMut<'_> {
  type Msg = DiscoveryRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoveryRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoveryRequestView<'_> {
  type Msg = DiscoveryRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoveryRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DiscoveryRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__DiscoveryResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DiscoveryResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DiscoveryResponse>
}

impl ::protobuf::Message for DiscoveryResponse {
  type MessageView<'msg> = DiscoveryResponseView<'msg>;
  type MessageMut<'msg> = DiscoveryResponseMut<'msg>;
}

impl ::std::default::Default for DiscoveryResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DiscoveryResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DiscoveryResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `DiscoveryResponseMut`.
unsafe impl ::std::marker::Sync for DiscoveryResponse {}

// SAFETY:
// - `DiscoveryResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DiscoveryResponse {}

impl ::protobuf::Proxied for DiscoveryResponse {
  type View<'msg> = DiscoveryResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DiscoveryResponse {}

impl ::protobuf::MutProxied for DiscoveryResponse {
  type Mut<'msg> = DiscoveryResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DiscoveryResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoveryResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DiscoveryResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DiscoveryResponseView<'msg> {
  type Message = DiscoveryResponse;
}

impl ::std::fmt::Debug for DiscoveryResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DiscoveryResponseView<'_> {
  fn default() -> DiscoveryResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoveryResponse>> for DiscoveryResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DiscoveryResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DiscoveryResponseView<'msg> {

  pub fn to_owned(&self) -> DiscoveryResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // resources: repeated message google.protobuf.Any
  pub fn resources(self) -> ::protobuf::RepeatedView<'msg, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // canary: optional bool
  pub fn canary(self) -> bool {
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

  // type_url: optional string
  pub fn type_url(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // nonce: optional string
  pub fn nonce(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // control_plane: optional message envoy.config.core.v3.ControlPlane
  pub fn has_control_plane(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn control_plane_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'msg>> {
    self.has_control_plane().then(|| self.control_plane())
  }
  pub fn control_plane(self) -> crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView::default())
  }

  // resource_errors: repeated message envoy.service.discovery.v3.ResourceError
  pub fn resource_errors(self) -> ::protobuf::RepeatedView<'msg, super::ResourceError> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceError>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `DiscoveryResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DiscoveryResponseView<'_> {}

// SAFETY:
// - `DiscoveryResponseView` is `Send` because while its alive a `DiscoveryResponseMut` cannot.
// - `DiscoveryResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for DiscoveryResponseView<'_> {}

impl<'msg> ::protobuf::AsView for DiscoveryResponseView<'msg> {
  type Proxied = DiscoveryResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, DiscoveryResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DiscoveryResponseView<'msg> {
  fn into_view<'shorter>(self) -> DiscoveryResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DiscoveryResponse> for DiscoveryResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DiscoveryResponse {
    let mut dst = DiscoveryResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DiscoveryResponse> for DiscoveryResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DiscoveryResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DiscoveryResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DiscoveryResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DiscoveryResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DiscoveryResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoveryResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DiscoveryResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DiscoveryResponseMut<'msg> {
  type Message = DiscoveryResponse;
}

impl ::std::fmt::Debug for DiscoveryResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoveryResponse>> for DiscoveryResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoveryResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DiscoveryResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DiscoveryResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DiscoveryResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // resources: repeated message google.protobuf.Any
  pub fn resources(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resources_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
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
  pub fn set_resources(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // canary: optional bool
  pub fn canary(&self) -> bool {
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
  pub fn set_canary(&mut self, val: bool) {
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

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // nonce: optional string
  pub fn nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // control_plane: optional message envoy.config.core.v3.ControlPlane
  pub fn has_control_plane(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_control_plane(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn control_plane_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'_>> {
    self.has_control_plane().then(|| self.control_plane())
  }
  pub fn control_plane(&self) -> crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView::default())
  }
  pub fn control_plane_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::ControlPlaneMut<'_> {
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
  pub fn set_control_plane(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::ControlPlane>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // resource_errors: repeated message envoy.service.discovery.v3.ResourceError
  pub fn resource_errors(&self) -> ::protobuf::RepeatedView<'_, super::ResourceError> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceError>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_errors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceError> {
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
  pub fn set_resource_errors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceError>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

}

// SAFETY:
// - `DiscoveryResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DiscoveryResponseMut<'_> {}

// SAFETY:
// - `DiscoveryResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DiscoveryResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for DiscoveryResponseMut<'msg> {
  type Proxied = DiscoveryResponse;
  fn as_view(&self) -> ::protobuf::View<'_, DiscoveryResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DiscoveryResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DiscoveryResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DiscoveryResponseMut<'msg> {
  type MutProxied = DiscoveryResponse;
  fn as_mut(&mut self) -> DiscoveryResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DiscoveryResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> DiscoveryResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DiscoveryResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DiscoveryResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DiscoveryResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DiscoveryResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // resources: repeated message google.protobuf.Any
  pub fn resources(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resources_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
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
  pub fn set_resources(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // canary: optional bool
  pub fn canary(&self) -> bool {
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
  pub fn set_canary(&mut self, val: bool) {
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

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // nonce: optional string
  pub fn nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // control_plane: optional message envoy.config.core.v3.ControlPlane
  pub fn has_control_plane(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_control_plane(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn control_plane_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'_>> {
    self.has_control_plane().then(|| self.control_plane())
  }
  pub fn control_plane(&self) -> crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView::default())
  }
  pub fn control_plane_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::ControlPlaneMut<'_> {
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
  pub fn set_control_plane(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::ControlPlane>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // resource_errors: repeated message envoy.service.discovery.v3.ResourceError
  pub fn resource_errors(&self) -> ::protobuf::RepeatedView<'_, super::ResourceError> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceError>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_errors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceError> {
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
  pub fn set_resource_errors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceError>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

}  // impl DiscoveryResponse

impl ::std::ops::Drop for DiscoveryResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DiscoveryResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DiscoveryResponse {
  type Proxied = Self;
  fn as_view(&self) -> DiscoveryResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DiscoveryResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DiscoveryResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DiscoveryResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__discovery__v3__DiscoveryResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG/P1X1X3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__discovery__v3__DiscoveryResponse_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::ControlPlane as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ResourceError as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__discovery__v3__DiscoveryResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DiscoveryResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DiscoveryResponse {
  type Msg = DiscoveryResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoveryResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoveryResponse {
  type Msg = DiscoveryResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoveryResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DiscoveryResponseMut<'_> {
  type Msg = DiscoveryResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoveryResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoveryResponseMut<'_> {
  type Msg = DiscoveryResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoveryResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiscoveryResponseView<'_> {
  type Msg = DiscoveryResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiscoveryResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DiscoveryResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__DeltaDiscoveryRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DeltaDiscoveryRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeltaDiscoveryRequest>
}

impl ::protobuf::Message for DeltaDiscoveryRequest {
  type MessageView<'msg> = DeltaDiscoveryRequestView<'msg>;
  type MessageMut<'msg> = DeltaDiscoveryRequestMut<'msg>;
}

impl ::std::default::Default for DeltaDiscoveryRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DeltaDiscoveryRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DeltaDiscoveryRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `DeltaDiscoveryRequestMut`.
unsafe impl ::std::marker::Sync for DeltaDiscoveryRequest {}

// SAFETY:
// - `DeltaDiscoveryRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DeltaDiscoveryRequest {}

impl ::protobuf::Proxied for DeltaDiscoveryRequest {
  type View<'msg> = DeltaDiscoveryRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeltaDiscoveryRequest {}

impl ::protobuf::MutProxied for DeltaDiscoveryRequest {
  type Mut<'msg> = DeltaDiscoveryRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeltaDiscoveryRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeltaDiscoveryRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeltaDiscoveryRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeltaDiscoveryRequestView<'msg> {
  type Message = DeltaDiscoveryRequest;
}

impl ::std::fmt::Debug for DeltaDiscoveryRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DeltaDiscoveryRequestView<'_> {
  fn default() -> DeltaDiscoveryRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DeltaDiscoveryRequest>> for DeltaDiscoveryRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeltaDiscoveryRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeltaDiscoveryRequestView<'msg> {

  pub fn to_owned(&self) -> DeltaDiscoveryRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn node_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'msg>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }

  // type_url: optional string
  pub fn type_url(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // resource_names_subscribe: repeated string
  pub fn resource_names_subscribe(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // resource_names_unsubscribe: repeated string
  pub fn resource_names_unsubscribe(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // resource_locators_subscribe: repeated message envoy.service.discovery.v3.ResourceLocator
  pub fn resource_locators_subscribe(self) -> ::protobuf::RepeatedView<'msg, super::ResourceLocator> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceLocator>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // resource_locators_unsubscribe: repeated message envoy.service.discovery.v3.ResourceLocator
  pub fn resource_locators_unsubscribe(self) -> ::protobuf::RepeatedView<'msg, super::ResourceLocator> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceLocator>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // initial_resource_versions: repeated message envoy.service.discovery.v3.DeltaDiscoveryRequest.InitialResourceVersionsEntry
  pub fn initial_resource_versions(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // response_nonce: optional string
  pub fn response_nonce(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // error_detail: optional message google.rpc.Status
  pub fn has_error_detail(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn error_detail_opt(self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'msg>> {
    self.has_error_detail().then(|| self.error_detail())
  }
  pub fn error_detail(self) -> crate::xds::generated::google::rpc::status::StatusView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }

}

// SAFETY:
// - `DeltaDiscoveryRequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DeltaDiscoveryRequestView<'_> {}

// SAFETY:
// - `DeltaDiscoveryRequestView` is `Send` because while its alive a `DeltaDiscoveryRequestMut` cannot.
// - `DeltaDiscoveryRequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for DeltaDiscoveryRequestView<'_> {}

impl<'msg> ::protobuf::AsView for DeltaDiscoveryRequestView<'msg> {
  type Proxied = DeltaDiscoveryRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, DeltaDiscoveryRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeltaDiscoveryRequestView<'msg> {
  fn into_view<'shorter>(self) -> DeltaDiscoveryRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeltaDiscoveryRequest> for DeltaDiscoveryRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeltaDiscoveryRequest {
    let mut dst = DeltaDiscoveryRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeltaDiscoveryRequest> for DeltaDiscoveryRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeltaDiscoveryRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DeltaDiscoveryRequest {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeltaDiscoveryRequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeltaDiscoveryRequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeltaDiscoveryRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeltaDiscoveryRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeltaDiscoveryRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeltaDiscoveryRequestMut<'msg> {
  type Message = DeltaDiscoveryRequest;
}

impl ::std::fmt::Debug for DeltaDiscoveryRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DeltaDiscoveryRequest>> for DeltaDiscoveryRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeltaDiscoveryRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeltaDiscoveryRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeltaDiscoveryRequest> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DeltaDiscoveryRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_node(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn node_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'_>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(&self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }
  pub fn node_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::NodeMut<'_> {
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
  pub fn set_node(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Node>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // resource_names_subscribe: repeated string
  pub fn resource_names_subscribe(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_names_subscribe_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_resource_names_subscribe(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // resource_names_unsubscribe: repeated string
  pub fn resource_names_unsubscribe(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_names_unsubscribe_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_resource_names_unsubscribe(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // resource_locators_subscribe: repeated message envoy.service.discovery.v3.ResourceLocator
  pub fn resource_locators_subscribe(&self) -> ::protobuf::RepeatedView<'_, super::ResourceLocator> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceLocator>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_locators_subscribe_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceLocator> {
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
  pub fn set_resource_locators_subscribe(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceLocator>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // resource_locators_unsubscribe: repeated message envoy.service.discovery.v3.ResourceLocator
  pub fn resource_locators_unsubscribe(&self) -> ::protobuf::RepeatedView<'_, super::ResourceLocator> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceLocator>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_locators_unsubscribe_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceLocator> {
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
  pub fn set_resource_locators_unsubscribe(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceLocator>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // initial_resource_versions: repeated message envoy.service.discovery.v3.DeltaDiscoveryRequest.InitialResourceVersionsEntry
  pub fn initial_resource_versions(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn initial_resource_versions_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          4, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_initial_resource_versions(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // response_nonce: optional string
  pub fn response_nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_response_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // error_detail: optional message google.rpc.Status
  pub fn has_error_detail(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_error_detail(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn error_detail_opt(&self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'_>> {
    self.has_error_detail().then(|| self.error_detail())
  }
  pub fn error_detail(&self) -> crate::xds::generated::google::rpc::status::StatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }
  pub fn error_detail_mut(&mut self) -> crate::xds::generated::google::rpc::status::StatusMut<'_> {
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
  pub fn set_error_detail(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::rpc::status::Status>) {

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
// - `DeltaDiscoveryRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DeltaDiscoveryRequestMut<'_> {}

// SAFETY:
// - `DeltaDiscoveryRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DeltaDiscoveryRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for DeltaDiscoveryRequestMut<'msg> {
  type Proxied = DeltaDiscoveryRequest;
  fn as_view(&self) -> ::protobuf::View<'_, DeltaDiscoveryRequest> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeltaDiscoveryRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeltaDiscoveryRequest>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DeltaDiscoveryRequestMut<'msg> {
  type MutProxied = DeltaDiscoveryRequest;
  fn as_mut(&mut self) -> DeltaDiscoveryRequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeltaDiscoveryRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> DeltaDiscoveryRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeltaDiscoveryRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeltaDiscoveryRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DeltaDiscoveryRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DeltaDiscoveryRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_node(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn node_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'_>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(&self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }
  pub fn node_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::NodeMut<'_> {
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
  pub fn set_node(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Node>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // resource_names_subscribe: repeated string
  pub fn resource_names_subscribe(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_names_subscribe_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_resource_names_subscribe(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // resource_names_unsubscribe: repeated string
  pub fn resource_names_unsubscribe(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_names_unsubscribe_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_resource_names_unsubscribe(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // resource_locators_subscribe: repeated message envoy.service.discovery.v3.ResourceLocator
  pub fn resource_locators_subscribe(&self) -> ::protobuf::RepeatedView<'_, super::ResourceLocator> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceLocator>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_locators_subscribe_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceLocator> {
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
  pub fn set_resource_locators_subscribe(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceLocator>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // resource_locators_unsubscribe: repeated message envoy.service.discovery.v3.ResourceLocator
  pub fn resource_locators_unsubscribe(&self) -> ::protobuf::RepeatedView<'_, super::ResourceLocator> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceLocator>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_locators_unsubscribe_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceLocator> {
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
  pub fn set_resource_locators_unsubscribe(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceLocator>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // initial_resource_versions: repeated message envoy.service.discovery.v3.DeltaDiscoveryRequest.InitialResourceVersionsEntry
  pub fn initial_resource_versions(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn initial_resource_versions_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          4, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_initial_resource_versions(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // response_nonce: optional string
  pub fn response_nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_response_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // error_detail: optional message google.rpc.Status
  pub fn has_error_detail(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_error_detail(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn error_detail_opt(&self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'_>> {
    self.has_error_detail().then(|| self.error_detail())
  }
  pub fn error_detail(&self) -> crate::xds::generated::google::rpc::status::StatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }
  pub fn error_detail_mut(&mut self) -> crate::xds::generated::google::rpc::status::StatusMut<'_> {
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
  pub fn set_error_detail(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::rpc::status::Status>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

}  // impl DeltaDiscoveryRequest

impl ::std::ops::Drop for DeltaDiscoveryRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DeltaDiscoveryRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeltaDiscoveryRequest {
  type Proxied = Self;
  fn as_view(&self) -> DeltaDiscoveryRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeltaDiscoveryRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeltaDiscoveryRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DeltaDiscoveryRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__discovery__v3__DeltaDiscoveryRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31XETETG1X3GG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__discovery__v3__DeltaDiscoveryRequest_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::Node as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::delta_discovery_request::InitialResourceVersionsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::google::rpc::status::Status as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ResourceLocator as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ResourceLocator as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__discovery__v3__DeltaDiscoveryRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeltaDiscoveryRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeltaDiscoveryRequest {
  type Msg = DeltaDiscoveryRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeltaDiscoveryRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeltaDiscoveryRequest {
  type Msg = DeltaDiscoveryRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeltaDiscoveryRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeltaDiscoveryRequestMut<'_> {
  type Msg = DeltaDiscoveryRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeltaDiscoveryRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeltaDiscoveryRequestMut<'_> {
  type Msg = DeltaDiscoveryRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeltaDiscoveryRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeltaDiscoveryRequestView<'_> {
  type Msg = DeltaDiscoveryRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeltaDiscoveryRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeltaDiscoveryRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod delta_discovery_request {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__DeltaDiscoveryRequest__InitialResourceVersionsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct InitialResourceVersionsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for InitialResourceVersionsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::delta_discovery_request::envoy__service__discovery__v3__DeltaDiscoveryRequest__InitialResourceVersionsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::delta_discovery_request::envoy__service__discovery__v3__DeltaDiscoveryRequest__InitialResourceVersionsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::delta_discovery_request::envoy__service__discovery__v3__DeltaDiscoveryRequest__InitialResourceVersionsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod delta_discovery_request


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__DeltaDiscoveryResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DeltaDiscoveryResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeltaDiscoveryResponse>
}

impl ::protobuf::Message for DeltaDiscoveryResponse {
  type MessageView<'msg> = DeltaDiscoveryResponseView<'msg>;
  type MessageMut<'msg> = DeltaDiscoveryResponseMut<'msg>;
}

impl ::std::default::Default for DeltaDiscoveryResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DeltaDiscoveryResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DeltaDiscoveryResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `DeltaDiscoveryResponseMut`.
unsafe impl ::std::marker::Sync for DeltaDiscoveryResponse {}

// SAFETY:
// - `DeltaDiscoveryResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DeltaDiscoveryResponse {}

impl ::protobuf::Proxied for DeltaDiscoveryResponse {
  type View<'msg> = DeltaDiscoveryResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeltaDiscoveryResponse {}

impl ::protobuf::MutProxied for DeltaDiscoveryResponse {
  type Mut<'msg> = DeltaDiscoveryResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeltaDiscoveryResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeltaDiscoveryResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeltaDiscoveryResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeltaDiscoveryResponseView<'msg> {
  type Message = DeltaDiscoveryResponse;
}

impl ::std::fmt::Debug for DeltaDiscoveryResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DeltaDiscoveryResponseView<'_> {
  fn default() -> DeltaDiscoveryResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DeltaDiscoveryResponse>> for DeltaDiscoveryResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeltaDiscoveryResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeltaDiscoveryResponseView<'msg> {

  pub fn to_owned(&self) -> DeltaDiscoveryResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // system_version_info: optional string
  pub fn system_version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // resources: repeated message envoy.service.discovery.v3.Resource
  pub fn resources(self) -> ::protobuf::RepeatedView<'msg, super::Resource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Resource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // type_url: optional string
  pub fn type_url(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // removed_resources: repeated string
  pub fn removed_resources(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // removed_resource_names: repeated message envoy.service.discovery.v3.ResourceName
  pub fn removed_resource_names(self) -> ::protobuf::RepeatedView<'msg, super::ResourceName> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceName>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // nonce: optional string
  pub fn nonce(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // control_plane: optional message envoy.config.core.v3.ControlPlane
  pub fn has_control_plane(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn control_plane_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'msg>> {
    self.has_control_plane().then(|| self.control_plane())
  }
  pub fn control_plane(self) -> crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView::default())
  }

  // resource_errors: repeated message envoy.service.discovery.v3.ResourceError
  pub fn resource_errors(self) -> ::protobuf::RepeatedView<'msg, super::ResourceError> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceError>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `DeltaDiscoveryResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DeltaDiscoveryResponseView<'_> {}

// SAFETY:
// - `DeltaDiscoveryResponseView` is `Send` because while its alive a `DeltaDiscoveryResponseMut` cannot.
// - `DeltaDiscoveryResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for DeltaDiscoveryResponseView<'_> {}

impl<'msg> ::protobuf::AsView for DeltaDiscoveryResponseView<'msg> {
  type Proxied = DeltaDiscoveryResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, DeltaDiscoveryResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeltaDiscoveryResponseView<'msg> {
  fn into_view<'shorter>(self) -> DeltaDiscoveryResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeltaDiscoveryResponse> for DeltaDiscoveryResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeltaDiscoveryResponse {
    let mut dst = DeltaDiscoveryResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeltaDiscoveryResponse> for DeltaDiscoveryResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeltaDiscoveryResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DeltaDiscoveryResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeltaDiscoveryResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeltaDiscoveryResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeltaDiscoveryResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeltaDiscoveryResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeltaDiscoveryResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeltaDiscoveryResponseMut<'msg> {
  type Message = DeltaDiscoveryResponse;
}

impl ::std::fmt::Debug for DeltaDiscoveryResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DeltaDiscoveryResponse>> for DeltaDiscoveryResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeltaDiscoveryResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeltaDiscoveryResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeltaDiscoveryResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DeltaDiscoveryResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // system_version_info: optional string
  pub fn system_version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_system_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // resources: repeated message envoy.service.discovery.v3.Resource
  pub fn resources(&self) -> ::protobuf::RepeatedView<'_, super::Resource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Resource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resources_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Resource> {
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
  pub fn set_resources(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Resource>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // removed_resources: repeated string
  pub fn removed_resources(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn removed_resources_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_removed_resources(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // removed_resource_names: repeated message envoy.service.discovery.v3.ResourceName
  pub fn removed_resource_names(&self) -> ::protobuf::RepeatedView<'_, super::ResourceName> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceName>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn removed_resource_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceName> {
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
  pub fn set_removed_resource_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceName>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // nonce: optional string
  pub fn nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // control_plane: optional message envoy.config.core.v3.ControlPlane
  pub fn has_control_plane(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_control_plane(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn control_plane_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'_>> {
    self.has_control_plane().then(|| self.control_plane())
  }
  pub fn control_plane(&self) -> crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView::default())
  }
  pub fn control_plane_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::ControlPlaneMut<'_> {
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
  pub fn set_control_plane(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::ControlPlane>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // resource_errors: repeated message envoy.service.discovery.v3.ResourceError
  pub fn resource_errors(&self) -> ::protobuf::RepeatedView<'_, super::ResourceError> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceError>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_errors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceError> {
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
  pub fn set_resource_errors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceError>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

}

// SAFETY:
// - `DeltaDiscoveryResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DeltaDiscoveryResponseMut<'_> {}

// SAFETY:
// - `DeltaDiscoveryResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DeltaDiscoveryResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for DeltaDiscoveryResponseMut<'msg> {
  type Proxied = DeltaDiscoveryResponse;
  fn as_view(&self) -> ::protobuf::View<'_, DeltaDiscoveryResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeltaDiscoveryResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeltaDiscoveryResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DeltaDiscoveryResponseMut<'msg> {
  type MutProxied = DeltaDiscoveryResponse;
  fn as_mut(&mut self) -> DeltaDiscoveryResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeltaDiscoveryResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> DeltaDiscoveryResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeltaDiscoveryResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeltaDiscoveryResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DeltaDiscoveryResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DeltaDiscoveryResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // system_version_info: optional string
  pub fn system_version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_system_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // resources: repeated message envoy.service.discovery.v3.Resource
  pub fn resources(&self) -> ::protobuf::RepeatedView<'_, super::Resource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Resource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resources_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Resource> {
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
  pub fn set_resources(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Resource>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // removed_resources: repeated string
  pub fn removed_resources(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn removed_resources_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_removed_resources(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // removed_resource_names: repeated message envoy.service.discovery.v3.ResourceName
  pub fn removed_resource_names(&self) -> ::protobuf::RepeatedView<'_, super::ResourceName> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceName>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn removed_resource_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceName> {
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
  pub fn set_removed_resource_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceName>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // nonce: optional string
  pub fn nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // control_plane: optional message envoy.config.core.v3.ControlPlane
  pub fn has_control_plane(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_control_plane(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn control_plane_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'_>> {
    self.has_control_plane().then(|| self.control_plane())
  }
  pub fn control_plane(&self) -> crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::ControlPlaneView::default())
  }
  pub fn control_plane_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::ControlPlaneMut<'_> {
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
  pub fn set_control_plane(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::ControlPlane>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // resource_errors: repeated message envoy.service.discovery.v3.ResourceError
  pub fn resource_errors(&self) -> ::protobuf::RepeatedView<'_, super::ResourceError> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceError>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_errors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceError> {
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
  pub fn set_resource_errors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceError>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

}  // impl DeltaDiscoveryResponse

impl ::std::ops::Drop for DeltaDiscoveryResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DeltaDiscoveryResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeltaDiscoveryResponse {
  type Proxied = Self;
  fn as_view(&self) -> DeltaDiscoveryResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeltaDiscoveryResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeltaDiscoveryResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DeltaDiscoveryResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__discovery__v3__DeltaDiscoveryResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XGa1X1XET3GG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__discovery__v3__DeltaDiscoveryResponse_msg_init.0, &[<super::Resource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::ControlPlane as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ResourceName as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ResourceError as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__discovery__v3__DeltaDiscoveryResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeltaDiscoveryResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeltaDiscoveryResponse {
  type Msg = DeltaDiscoveryResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeltaDiscoveryResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeltaDiscoveryResponse {
  type Msg = DeltaDiscoveryResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeltaDiscoveryResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeltaDiscoveryResponseMut<'_> {
  type Msg = DeltaDiscoveryResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeltaDiscoveryResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeltaDiscoveryResponseMut<'_> {
  type Msg = DeltaDiscoveryResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeltaDiscoveryResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeltaDiscoveryResponseView<'_> {
  type Msg = DeltaDiscoveryResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeltaDiscoveryResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeltaDiscoveryResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__DynamicParameterConstraints_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DynamicParameterConstraints {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DynamicParameterConstraints>
}

impl ::protobuf::Message for DynamicParameterConstraints {
  type MessageView<'msg> = DynamicParameterConstraintsView<'msg>;
  type MessageMut<'msg> = DynamicParameterConstraintsMut<'msg>;
}

impl ::std::default::Default for DynamicParameterConstraints {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DynamicParameterConstraints {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DynamicParameterConstraints` is `Sync` because it does not implement interior mutability.
//    Neither does `DynamicParameterConstraintsMut`.
unsafe impl ::std::marker::Sync for DynamicParameterConstraints {}

// SAFETY:
// - `DynamicParameterConstraints` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DynamicParameterConstraints {}

impl ::protobuf::Proxied for DynamicParameterConstraints {
  type View<'msg> = DynamicParameterConstraintsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DynamicParameterConstraints {}

impl ::protobuf::MutProxied for DynamicParameterConstraints {
  type Mut<'msg> = DynamicParameterConstraintsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DynamicParameterConstraintsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicParameterConstraints>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicParameterConstraintsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DynamicParameterConstraintsView<'msg> {
  type Message = DynamicParameterConstraints;
}

impl ::std::fmt::Debug for DynamicParameterConstraintsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DynamicParameterConstraintsView<'_> {
  fn default() -> DynamicParameterConstraintsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicParameterConstraints>> for DynamicParameterConstraintsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicParameterConstraints>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicParameterConstraintsView<'msg> {

  pub fn to_owned(&self) -> DynamicParameterConstraints {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // constraint: optional message envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint
  pub fn has_constraint(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn constraint_opt(self) -> ::std::option::Option<super::dynamic_parameter_constraints::SingleConstraintView<'msg>> {
    self.has_constraint().then(|| self.constraint())
  }
  pub fn constraint(self) -> super::dynamic_parameter_constraints::SingleConstraintView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::dynamic_parameter_constraints::SingleConstraintView::default())
  }

  // or_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints.ConstraintList
  pub fn has_or_constraints(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn or_constraints_opt(self) -> ::std::option::Option<super::dynamic_parameter_constraints::ConstraintListView<'msg>> {
    self.has_or_constraints().then(|| self.or_constraints())
  }
  pub fn or_constraints(self) -> super::dynamic_parameter_constraints::ConstraintListView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::dynamic_parameter_constraints::ConstraintListView::default())
  }

  // and_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints.ConstraintList
  pub fn has_and_constraints(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn and_constraints_opt(self) -> ::std::option::Option<super::dynamic_parameter_constraints::ConstraintListView<'msg>> {
    self.has_and_constraints().then(|| self.and_constraints())
  }
  pub fn and_constraints(self) -> super::dynamic_parameter_constraints::ConstraintListView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::dynamic_parameter_constraints::ConstraintListView::default())
  }

  // not_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints
  pub fn has_not_constraints(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn not_constraints_opt(self) -> ::std::option::Option<super::DynamicParameterConstraintsView<'msg>> {
    self.has_not_constraints().then(|| self.not_constraints())
  }
  pub fn not_constraints(self) -> super::DynamicParameterConstraintsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DynamicParameterConstraintsView::default())
  }

  pub fn r#type(self) -> super::dynamic_parameter_constraints::TypeOneof<'msg> {
    match self.r#type_case() {
      super::dynamic_parameter_constraints::TypeCase::Constraint =>
          super::dynamic_parameter_constraints::TypeOneof::Constraint(self.constraint()),
      super::dynamic_parameter_constraints::TypeCase::OrConstraints =>
          super::dynamic_parameter_constraints::TypeOneof::OrConstraints(self.or_constraints()),
      super::dynamic_parameter_constraints::TypeCase::AndConstraints =>
          super::dynamic_parameter_constraints::TypeOneof::AndConstraints(self.and_constraints()),
      super::dynamic_parameter_constraints::TypeCase::NotConstraints =>
          super::dynamic_parameter_constraints::TypeOneof::NotConstraints(self.not_constraints()),
      _ => super::dynamic_parameter_constraints::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(self) -> super::dynamic_parameter_constraints::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::dynamic_parameter_constraints::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DynamicParameterConstraintsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DynamicParameterConstraintsView<'_> {}

// SAFETY:
// - `DynamicParameterConstraintsView` is `Send` because while its alive a `DynamicParameterConstraintsMut` cannot.
// - `DynamicParameterConstraintsView` does not use thread-local data.
unsafe impl ::std::marker::Send for DynamicParameterConstraintsView<'_> {}

impl<'msg> ::protobuf::AsView for DynamicParameterConstraintsView<'msg> {
  type Proxied = DynamicParameterConstraints;
  fn as_view(&self) -> ::protobuf::View<'msg, DynamicParameterConstraints> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicParameterConstraintsView<'msg> {
  fn into_view<'shorter>(self) -> DynamicParameterConstraintsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicParameterConstraints> for DynamicParameterConstraintsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicParameterConstraints {
    let mut dst = DynamicParameterConstraints::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicParameterConstraints> for DynamicParameterConstraintsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicParameterConstraints {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DynamicParameterConstraints {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicParameterConstraintsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicParameterConstraintsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DynamicParameterConstraintsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicParameterConstraints>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicParameterConstraintsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DynamicParameterConstraintsMut<'msg> {
  type Message = DynamicParameterConstraints;
}

impl ::std::fmt::Debug for DynamicParameterConstraintsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicParameterConstraints>> for DynamicParameterConstraintsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicParameterConstraints>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicParameterConstraintsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicParameterConstraints> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DynamicParameterConstraints {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // constraint: optional message envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint
  pub fn has_constraint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_constraint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn constraint_opt(&self) -> ::std::option::Option<super::dynamic_parameter_constraints::SingleConstraintView<'_>> {
    self.has_constraint().then(|| self.constraint())
  }
  pub fn constraint(&self) -> super::dynamic_parameter_constraints::SingleConstraintView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::dynamic_parameter_constraints::SingleConstraintView::default())
  }
  pub fn constraint_mut(&mut self) -> super::dynamic_parameter_constraints::SingleConstraintMut<'_> {
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
  pub fn set_constraint(&mut self,
    val: impl ::protobuf::IntoProxied<super::dynamic_parameter_constraints::SingleConstraint>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // or_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints.ConstraintList
  pub fn has_or_constraints(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_or_constraints(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn or_constraints_opt(&self) -> ::std::option::Option<super::dynamic_parameter_constraints::ConstraintListView<'_>> {
    self.has_or_constraints().then(|| self.or_constraints())
  }
  pub fn or_constraints(&self) -> super::dynamic_parameter_constraints::ConstraintListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::dynamic_parameter_constraints::ConstraintListView::default())
  }
  pub fn or_constraints_mut(&mut self) -> super::dynamic_parameter_constraints::ConstraintListMut<'_> {
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
  pub fn set_or_constraints(&mut self,
    val: impl ::protobuf::IntoProxied<super::dynamic_parameter_constraints::ConstraintList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // and_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints.ConstraintList
  pub fn has_and_constraints(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_and_constraints(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn and_constraints_opt(&self) -> ::std::option::Option<super::dynamic_parameter_constraints::ConstraintListView<'_>> {
    self.has_and_constraints().then(|| self.and_constraints())
  }
  pub fn and_constraints(&self) -> super::dynamic_parameter_constraints::ConstraintListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::dynamic_parameter_constraints::ConstraintListView::default())
  }
  pub fn and_constraints_mut(&mut self) -> super::dynamic_parameter_constraints::ConstraintListMut<'_> {
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
  pub fn set_and_constraints(&mut self,
    val: impl ::protobuf::IntoProxied<super::dynamic_parameter_constraints::ConstraintList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // not_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints
  pub fn has_not_constraints(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_not_constraints(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn not_constraints_opt(&self) -> ::std::option::Option<super::DynamicParameterConstraintsView<'_>> {
    self.has_not_constraints().then(|| self.not_constraints())
  }
  pub fn not_constraints(&self) -> super::DynamicParameterConstraintsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DynamicParameterConstraintsView::default())
  }
  pub fn not_constraints_mut(&mut self) -> super::DynamicParameterConstraintsMut<'_> {
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
  pub fn set_not_constraints(&mut self,
    val: impl ::protobuf::IntoProxied<super::DynamicParameterConstraints>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn r#type(&self) -> super::dynamic_parameter_constraints::TypeOneof<'_> {
    match &self.r#type_case() {
      super::dynamic_parameter_constraints::TypeCase::Constraint =>
          super::dynamic_parameter_constraints::TypeOneof::Constraint(self.constraint()),
      super::dynamic_parameter_constraints::TypeCase::OrConstraints =>
          super::dynamic_parameter_constraints::TypeOneof::OrConstraints(self.or_constraints()),
      super::dynamic_parameter_constraints::TypeCase::AndConstraints =>
          super::dynamic_parameter_constraints::TypeOneof::AndConstraints(self.and_constraints()),
      super::dynamic_parameter_constraints::TypeCase::NotConstraints =>
          super::dynamic_parameter_constraints::TypeOneof::NotConstraints(self.not_constraints()),
      _ => super::dynamic_parameter_constraints::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::dynamic_parameter_constraints::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::dynamic_parameter_constraints::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DynamicParameterConstraintsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DynamicParameterConstraintsMut<'_> {}

// SAFETY:
// - `DynamicParameterConstraintsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DynamicParameterConstraintsMut<'_> {}

impl<'msg> ::protobuf::AsView for DynamicParameterConstraintsMut<'msg> {
  type Proxied = DynamicParameterConstraints;
  fn as_view(&self) -> ::protobuf::View<'_, DynamicParameterConstraints> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicParameterConstraintsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DynamicParameterConstraints>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DynamicParameterConstraintsMut<'msg> {
  type MutProxied = DynamicParameterConstraints;
  fn as_mut(&mut self) -> DynamicParameterConstraintsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DynamicParameterConstraintsMut<'msg> {
  fn into_mut<'shorter>(self) -> DynamicParameterConstraintsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DynamicParameterConstraints {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DynamicParameterConstraints> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DynamicParameterConstraintsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DynamicParameterConstraintsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // constraint: optional message envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint
  pub fn has_constraint(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_constraint(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn constraint_opt(&self) -> ::std::option::Option<super::dynamic_parameter_constraints::SingleConstraintView<'_>> {
    self.has_constraint().then(|| self.constraint())
  }
  pub fn constraint(&self) -> super::dynamic_parameter_constraints::SingleConstraintView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::dynamic_parameter_constraints::SingleConstraintView::default())
  }
  pub fn constraint_mut(&mut self) -> super::dynamic_parameter_constraints::SingleConstraintMut<'_> {
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
  pub fn set_constraint(&mut self,
    val: impl ::protobuf::IntoProxied<super::dynamic_parameter_constraints::SingleConstraint>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // or_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints.ConstraintList
  pub fn has_or_constraints(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_or_constraints(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn or_constraints_opt(&self) -> ::std::option::Option<super::dynamic_parameter_constraints::ConstraintListView<'_>> {
    self.has_or_constraints().then(|| self.or_constraints())
  }
  pub fn or_constraints(&self) -> super::dynamic_parameter_constraints::ConstraintListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::dynamic_parameter_constraints::ConstraintListView::default())
  }
  pub fn or_constraints_mut(&mut self) -> super::dynamic_parameter_constraints::ConstraintListMut<'_> {
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
  pub fn set_or_constraints(&mut self,
    val: impl ::protobuf::IntoProxied<super::dynamic_parameter_constraints::ConstraintList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // and_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints.ConstraintList
  pub fn has_and_constraints(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_and_constraints(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn and_constraints_opt(&self) -> ::std::option::Option<super::dynamic_parameter_constraints::ConstraintListView<'_>> {
    self.has_and_constraints().then(|| self.and_constraints())
  }
  pub fn and_constraints(&self) -> super::dynamic_parameter_constraints::ConstraintListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::dynamic_parameter_constraints::ConstraintListView::default())
  }
  pub fn and_constraints_mut(&mut self) -> super::dynamic_parameter_constraints::ConstraintListMut<'_> {
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
  pub fn set_and_constraints(&mut self,
    val: impl ::protobuf::IntoProxied<super::dynamic_parameter_constraints::ConstraintList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // not_constraints: optional message envoy.service.discovery.v3.DynamicParameterConstraints
  pub fn has_not_constraints(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_not_constraints(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn not_constraints_opt(&self) -> ::std::option::Option<super::DynamicParameterConstraintsView<'_>> {
    self.has_not_constraints().then(|| self.not_constraints())
  }
  pub fn not_constraints(&self) -> super::DynamicParameterConstraintsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DynamicParameterConstraintsView::default())
  }
  pub fn not_constraints_mut(&mut self) -> super::DynamicParameterConstraintsMut<'_> {
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
  pub fn set_not_constraints(&mut self,
    val: impl ::protobuf::IntoProxied<super::DynamicParameterConstraints>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn r#type(&self) -> super::dynamic_parameter_constraints::TypeOneof<'_> {
    match &self.r#type_case() {
      super::dynamic_parameter_constraints::TypeCase::Constraint =>
          super::dynamic_parameter_constraints::TypeOneof::Constraint(self.constraint()),
      super::dynamic_parameter_constraints::TypeCase::OrConstraints =>
          super::dynamic_parameter_constraints::TypeOneof::OrConstraints(self.or_constraints()),
      super::dynamic_parameter_constraints::TypeCase::AndConstraints =>
          super::dynamic_parameter_constraints::TypeOneof::AndConstraints(self.and_constraints()),
      super::dynamic_parameter_constraints::TypeCase::NotConstraints =>
          super::dynamic_parameter_constraints::TypeOneof::NotConstraints(self.not_constraints()),
      _ => super::dynamic_parameter_constraints::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::dynamic_parameter_constraints::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::dynamic_parameter_constraints::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl DynamicParameterConstraints

impl ::std::ops::Drop for DynamicParameterConstraints {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DynamicParameterConstraints {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DynamicParameterConstraints {
  type Proxied = Self;
  fn as_view(&self) -> DynamicParameterConstraintsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DynamicParameterConstraints {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DynamicParameterConstraintsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicParameterConstraints {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__discovery__v3__DynamicParameterConstraints_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333^!|#|$|%");
        super::dynamic_parameter_constraints::envoy__service__discovery__v3__DynamicParameterConstraints__ConstraintList_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__discovery__v3__DynamicParameterConstraints_msg_init.0, &[<super::dynamic_parameter_constraints::SingleConstraint as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::dynamic_parameter_constraints::envoy__service__discovery__v3__DynamicParameterConstraints__ConstraintList_msg_init.0,
            super::dynamic_parameter_constraints::envoy__service__discovery__v3__DynamicParameterConstraints__ConstraintList_msg_init.0,
            super::envoy__service__discovery__v3__DynamicParameterConstraints_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::dynamic_parameter_constraints::envoy__service__discovery__v3__DynamicParameterConstraints__ConstraintList_msg_init.0, &[super::envoy__service__discovery__v3__DynamicParameterConstraints_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__discovery__v3__DynamicParameterConstraints_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicParameterConstraints {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicParameterConstraints {
  type Msg = DynamicParameterConstraints;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicParameterConstraints> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicParameterConstraints {
  type Msg = DynamicParameterConstraints;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicParameterConstraints> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicParameterConstraintsMut<'_> {
  type Msg = DynamicParameterConstraints;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicParameterConstraints> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicParameterConstraintsMut<'_> {
  type Msg = DynamicParameterConstraints;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicParameterConstraints> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicParameterConstraintsView<'_> {
  type Msg = DynamicParameterConstraints;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicParameterConstraints> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicParameterConstraintsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod dynamic_parameter_constraints {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__DynamicParameterConstraints__SingleConstraint_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SingleConstraint {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SingleConstraint>
}

impl ::protobuf::Message for SingleConstraint {
  type MessageView<'msg> = SingleConstraintView<'msg>;
  type MessageMut<'msg> = SingleConstraintMut<'msg>;
}

impl ::std::default::Default for SingleConstraint {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SingleConstraint {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SingleConstraint` is `Sync` because it does not implement interior mutability.
//    Neither does `SingleConstraintMut`.
unsafe impl ::std::marker::Sync for SingleConstraint {}

// SAFETY:
// - `SingleConstraint` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SingleConstraint {}

impl ::protobuf::Proxied for SingleConstraint {
  type View<'msg> = SingleConstraintView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SingleConstraint {}

impl ::protobuf::MutProxied for SingleConstraint {
  type Mut<'msg> = SingleConstraintMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SingleConstraintView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SingleConstraint>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SingleConstraintView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SingleConstraintView<'msg> {
  type Message = SingleConstraint;
}

impl ::std::fmt::Debug for SingleConstraintView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SingleConstraintView<'_> {
  fn default() -> SingleConstraintView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SingleConstraint>> for SingleConstraintView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SingleConstraint>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SingleConstraintView<'msg> {

  pub fn to_owned(&self) -> SingleConstraint {
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

  // value: optional string
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn value_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_value().then(|| self.value())
  }
  pub fn value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // exists: optional message envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint.Exists
  pub fn has_exists(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn exists_opt(self) -> ::std::option::Option<super::super::dynamic_parameter_constraints::single_constraint::ExistsView<'msg>> {
    self.has_exists().then(|| self.exists())
  }
  pub fn exists(self) -> super::super::dynamic_parameter_constraints::single_constraint::ExistsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::dynamic_parameter_constraints::single_constraint::ExistsView::default())
  }

  pub fn constraint_type(self) -> super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof<'msg> {
    match self.constraint_type_case() {
      super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase::Value =>
          super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof::Value(self.value()),
      super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase::Exists =>
          super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof::Exists(self.exists()),
      _ => super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn constraint_type_case(self) -> super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SingleConstraintView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SingleConstraintView<'_> {}

// SAFETY:
// - `SingleConstraintView` is `Send` because while its alive a `SingleConstraintMut` cannot.
// - `SingleConstraintView` does not use thread-local data.
unsafe impl ::std::marker::Send for SingleConstraintView<'_> {}

impl<'msg> ::protobuf::AsView for SingleConstraintView<'msg> {
  type Proxied = SingleConstraint;
  fn as_view(&self) -> ::protobuf::View<'msg, SingleConstraint> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SingleConstraintView<'msg> {
  fn into_view<'shorter>(self) -> SingleConstraintView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SingleConstraint> for SingleConstraintView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SingleConstraint {
    let mut dst = SingleConstraint::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SingleConstraint> for SingleConstraintMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SingleConstraint {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SingleConstraint {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SingleConstraintView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SingleConstraintMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SingleConstraintMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SingleConstraint>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SingleConstraintMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SingleConstraintMut<'msg> {
  type Message = SingleConstraint;
}

impl ::std::fmt::Debug for SingleConstraintMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SingleConstraint>> for SingleConstraintMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SingleConstraint>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SingleConstraintMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SingleConstraint> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SingleConstraint {
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

  // value: optional string
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // exists: optional message envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint.Exists
  pub fn has_exists(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_exists(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn exists_opt(&self) -> ::std::option::Option<super::super::dynamic_parameter_constraints::single_constraint::ExistsView<'_>> {
    self.has_exists().then(|| self.exists())
  }
  pub fn exists(&self) -> super::super::dynamic_parameter_constraints::single_constraint::ExistsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::dynamic_parameter_constraints::single_constraint::ExistsView::default())
  }
  pub fn exists_mut(&mut self) -> super::super::dynamic_parameter_constraints::single_constraint::ExistsMut<'_> {
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
  pub fn set_exists(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::dynamic_parameter_constraints::single_constraint::Exists>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn constraint_type(&self) -> super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof<'_> {
    match &self.constraint_type_case() {
      super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase::Value =>
          super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof::Value(self.value()),
      super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase::Exists =>
          super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof::Exists(self.exists()),
      _ => super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn constraint_type_case(&self) -> super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SingleConstraintMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SingleConstraintMut<'_> {}

// SAFETY:
// - `SingleConstraintMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SingleConstraintMut<'_> {}

impl<'msg> ::protobuf::AsView for SingleConstraintMut<'msg> {
  type Proxied = SingleConstraint;
  fn as_view(&self) -> ::protobuf::View<'_, SingleConstraint> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SingleConstraintMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SingleConstraint>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SingleConstraintMut<'msg> {
  type MutProxied = SingleConstraint;
  fn as_mut(&mut self) -> SingleConstraintMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SingleConstraintMut<'msg> {
  fn into_mut<'shorter>(self) -> SingleConstraintMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SingleConstraint {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SingleConstraint> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SingleConstraintView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SingleConstraintMut<'_> {
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

  // value: optional string
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // exists: optional message envoy.service.discovery.v3.DynamicParameterConstraints.SingleConstraint.Exists
  pub fn has_exists(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_exists(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn exists_opt(&self) -> ::std::option::Option<super::super::dynamic_parameter_constraints::single_constraint::ExistsView<'_>> {
    self.has_exists().then(|| self.exists())
  }
  pub fn exists(&self) -> super::super::dynamic_parameter_constraints::single_constraint::ExistsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::dynamic_parameter_constraints::single_constraint::ExistsView::default())
  }
  pub fn exists_mut(&mut self) -> super::super::dynamic_parameter_constraints::single_constraint::ExistsMut<'_> {
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
  pub fn set_exists(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::dynamic_parameter_constraints::single_constraint::Exists>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn constraint_type(&self) -> super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof<'_> {
    match &self.constraint_type_case() {
      super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase::Value =>
          super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof::Value(self.value()),
      super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase::Exists =>
          super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof::Exists(self.exists()),
      _ => super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn constraint_type_case(&self) -> super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::dynamic_parameter_constraints::single_constraint::ConstraintTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl SingleConstraint

impl ::std::ops::Drop for SingleConstraint {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SingleConstraint {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SingleConstraint {
  type Proxied = Self;
  fn as_view(&self) -> SingleConstraintView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SingleConstraint {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SingleConstraintMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SingleConstraint {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::dynamic_parameter_constraints::envoy__service__discovery__v3__DynamicParameterConstraints__SingleConstraint_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1T3^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::dynamic_parameter_constraints::envoy__service__discovery__v3__DynamicParameterConstraints__SingleConstraint_msg_init.0, &[<super::super::dynamic_parameter_constraints::single_constraint::Exists as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::dynamic_parameter_constraints::envoy__service__discovery__v3__DynamicParameterConstraints__SingleConstraint_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SingleConstraint {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SingleConstraint {
  type Msg = SingleConstraint;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SingleConstraint> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SingleConstraint {
  type Msg = SingleConstraint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SingleConstraint> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SingleConstraintMut<'_> {
  type Msg = SingleConstraint;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SingleConstraint> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SingleConstraintMut<'_> {
  type Msg = SingleConstraint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SingleConstraint> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SingleConstraintView<'_> {
  type Msg = SingleConstraint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SingleConstraint> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SingleConstraintMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod single_constraint {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__DynamicParameterConstraints__SingleConstraint__Exists_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Exists {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Exists>
}

impl ::protobuf::Message for Exists {
  type MessageView<'msg> = ExistsView<'msg>;
  type MessageMut<'msg> = ExistsMut<'msg>;
}

impl ::std::default::Default for Exists {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Exists {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Exists` is `Sync` because it does not implement interior mutability.
//    Neither does `ExistsMut`.
unsafe impl ::std::marker::Sync for Exists {}

// SAFETY:
// - `Exists` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Exists {}

impl ::protobuf::Proxied for Exists {
  type View<'msg> = ExistsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Exists {}

impl ::protobuf::MutProxied for Exists {
  type Mut<'msg> = ExistsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExistsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Exists>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExistsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExistsView<'msg> {
  type Message = Exists;
}

impl ::std::fmt::Debug for ExistsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExistsView<'_> {
  fn default() -> ExistsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Exists>> for ExistsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Exists>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExistsView<'msg> {

  pub fn to_owned(&self) -> Exists {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ExistsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExistsView<'_> {}

// SAFETY:
// - `ExistsView` is `Send` because while its alive a `ExistsMut` cannot.
// - `ExistsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExistsView<'_> {}

impl<'msg> ::protobuf::AsView for ExistsView<'msg> {
  type Proxied = Exists;
  fn as_view(&self) -> ::protobuf::View<'msg, Exists> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExistsView<'msg> {
  fn into_view<'shorter>(self) -> ExistsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Exists> for ExistsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Exists {
    let mut dst = Exists::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Exists> for ExistsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Exists {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Exists {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExistsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExistsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExistsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Exists>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExistsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExistsMut<'msg> {
  type Message = Exists;
}

impl ::std::fmt::Debug for ExistsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Exists>> for ExistsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Exists>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExistsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Exists> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Exists {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ExistsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExistsMut<'_> {}

// SAFETY:
// - `ExistsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExistsMut<'_> {}

impl<'msg> ::protobuf::AsView for ExistsMut<'msg> {
  type Proxied = Exists;
  fn as_view(&self) -> ::protobuf::View<'_, Exists> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExistsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Exists>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExistsMut<'msg> {
  type MutProxied = Exists;
  fn as_mut(&mut self) -> ExistsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExistsMut<'msg> {
  fn into_mut<'shorter>(self) -> ExistsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Exists {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Exists> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExistsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExistsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Exists

impl ::std::ops::Drop for Exists {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Exists {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Exists {
  type Proxied = Self;
  fn as_view(&self) -> ExistsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Exists {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExistsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Exists {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::dynamic_parameter_constraints::single_constraint::envoy__service__discovery__v3__DynamicParameterConstraints__SingleConstraint__Exists_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::dynamic_parameter_constraints::single_constraint::envoy__service__discovery__v3__DynamicParameterConstraints__SingleConstraint__Exists_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::dynamic_parameter_constraints::single_constraint::envoy__service__discovery__v3__DynamicParameterConstraints__SingleConstraint__Exists_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Exists {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Exists {
  type Msg = Exists;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Exists> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Exists {
  type Msg = Exists;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Exists> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExistsMut<'_> {
  type Msg = Exists;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Exists> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExistsMut<'_> {
  type Msg = Exists;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Exists> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExistsView<'_> {
  type Msg = Exists;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Exists> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExistsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConstraintTypeOneof<'msg> {
  Value(&'msg ::protobuf::ProtoStr) = 2,
  Exists(::protobuf::View<'msg, super::super::super::dynamic_parameter_constraints::single_constraint::Exists>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConstraintTypeCase {
  Value = 2,
  Exists = 3,

  not_set = 0
}

impl ConstraintTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConstraintTypeCase> {
    match v {
      0 => Some(ConstraintTypeCase::not_set),
      2 => Some(ConstraintTypeCase::Value),
      3 => Some(ConstraintTypeCase::Exists),
      _ => None
    }
  }
}
}  // pub mod single_constraint

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__DynamicParameterConstraints__ConstraintList_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ConstraintList {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ConstraintList>
}

impl ::protobuf::Message for ConstraintList {
  type MessageView<'msg> = ConstraintListView<'msg>;
  type MessageMut<'msg> = ConstraintListMut<'msg>;
}

impl ::std::default::Default for ConstraintList {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ConstraintList {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ConstraintList` is `Sync` because it does not implement interior mutability.
//    Neither does `ConstraintListMut`.
unsafe impl ::std::marker::Sync for ConstraintList {}

// SAFETY:
// - `ConstraintList` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ConstraintList {}

impl ::protobuf::Proxied for ConstraintList {
  type View<'msg> = ConstraintListView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ConstraintList {}

impl ::protobuf::MutProxied for ConstraintList {
  type Mut<'msg> = ConstraintListMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ConstraintListView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConstraintList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConstraintListView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ConstraintListView<'msg> {
  type Message = ConstraintList;
}

impl ::std::fmt::Debug for ConstraintListView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ConstraintListView<'_> {
  fn default() -> ConstraintListView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ConstraintList>> for ConstraintListView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConstraintList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConstraintListView<'msg> {

  pub fn to_owned(&self) -> ConstraintList {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // constraints: repeated message envoy.service.discovery.v3.DynamicParameterConstraints
  pub fn constraints(self) -> ::protobuf::RepeatedView<'msg, super::super::DynamicParameterConstraints> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::DynamicParameterConstraints>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ConstraintListView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ConstraintListView<'_> {}

// SAFETY:
// - `ConstraintListView` is `Send` because while its alive a `ConstraintListMut` cannot.
// - `ConstraintListView` does not use thread-local data.
unsafe impl ::std::marker::Send for ConstraintListView<'_> {}

impl<'msg> ::protobuf::AsView for ConstraintListView<'msg> {
  type Proxied = ConstraintList;
  fn as_view(&self) -> ::protobuf::View<'msg, ConstraintList> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConstraintListView<'msg> {
  fn into_view<'shorter>(self) -> ConstraintListView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ConstraintList> for ConstraintListView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConstraintList {
    let mut dst = ConstraintList::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ConstraintList> for ConstraintListMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConstraintList {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ConstraintList {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConstraintListView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConstraintListMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ConstraintListMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConstraintList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConstraintListMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ConstraintListMut<'msg> {
  type Message = ConstraintList;
}

impl ::std::fmt::Debug for ConstraintListMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ConstraintList>> for ConstraintListMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConstraintList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConstraintListMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ConstraintList> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ConstraintList {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // constraints: repeated message envoy.service.discovery.v3.DynamicParameterConstraints
  pub fn constraints(&self) -> ::protobuf::RepeatedView<'_, super::super::DynamicParameterConstraints> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::DynamicParameterConstraints>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn constraints_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::DynamicParameterConstraints> {
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
  pub fn set_constraints(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::DynamicParameterConstraints>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ConstraintListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ConstraintListMut<'_> {}

// SAFETY:
// - `ConstraintListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ConstraintListMut<'_> {}

impl<'msg> ::protobuf::AsView for ConstraintListMut<'msg> {
  type Proxied = ConstraintList;
  fn as_view(&self) -> ::protobuf::View<'_, ConstraintList> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConstraintListMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ConstraintList>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ConstraintListMut<'msg> {
  type MutProxied = ConstraintList;
  fn as_mut(&mut self) -> ConstraintListMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ConstraintListMut<'msg> {
  fn into_mut<'shorter>(self) -> ConstraintListMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ConstraintList {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ConstraintList> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ConstraintListView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ConstraintListMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // constraints: repeated message envoy.service.discovery.v3.DynamicParameterConstraints
  pub fn constraints(&self) -> ::protobuf::RepeatedView<'_, super::super::DynamicParameterConstraints> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::DynamicParameterConstraints>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn constraints_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::DynamicParameterConstraints> {
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
  pub fn set_constraints(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::DynamicParameterConstraints>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ConstraintList

impl ::std::ops::Drop for ConstraintList {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ConstraintList {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ConstraintList {
  type Proxied = Self;
  fn as_view(&self) -> ConstraintListView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ConstraintList {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ConstraintListMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ConstraintList {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::DynamicParameterConstraints as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::dynamic_parameter_constraints::envoy__service__discovery__v3__DynamicParameterConstraints__ConstraintList_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConstraintList {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConstraintList {
  type Msg = ConstraintList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConstraintList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConstraintList {
  type Msg = ConstraintList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConstraintList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConstraintListMut<'_> {
  type Msg = ConstraintList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConstraintList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConstraintListMut<'_> {
  type Msg = ConstraintList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConstraintList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConstraintListView<'_> {
  type Msg = ConstraintList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConstraintList> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConstraintListMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TypeOneof<'msg> {
  Constraint(::protobuf::View<'msg, super::super::dynamic_parameter_constraints::SingleConstraint>) = 1,
  OrConstraints(::protobuf::View<'msg, super::super::dynamic_parameter_constraints::ConstraintList>) = 2,
  AndConstraints(::protobuf::View<'msg, super::super::dynamic_parameter_constraints::ConstraintList>) = 3,
  NotConstraints(::protobuf::View<'msg, super::super::DynamicParameterConstraints>) = 4,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TypeCase {
  Constraint = 1,
  OrConstraints = 2,
  AndConstraints = 3,
  NotConstraints = 4,

  not_set = 0
}

impl TypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TypeCase> {
    match v {
      0 => Some(TypeCase::not_set),
      1 => Some(TypeCase::Constraint),
      2 => Some(TypeCase::OrConstraints),
      3 => Some(TypeCase::AndConstraints),
      4 => Some(TypeCase::NotConstraints),
      _ => None
    }
  }
}
}  // pub mod dynamic_parameter_constraints


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__Resource_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Resource {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Resource>
}

impl ::protobuf::Message for Resource {
  type MessageView<'msg> = ResourceView<'msg>;
  type MessageMut<'msg> = ResourceMut<'msg>;
}

impl ::std::default::Default for Resource {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Resource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Resource` is `Sync` because it does not implement interior mutability.
//    Neither does `ResourceMut`.
unsafe impl ::std::marker::Sync for Resource {}

// SAFETY:
// - `Resource` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Resource {}

impl ::protobuf::Proxied for Resource {
  type View<'msg> = ResourceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Resource {}

impl ::protobuf::MutProxied for Resource {
  type Mut<'msg> = ResourceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResourceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Resource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResourceView<'msg> {
  type Message = Resource;
}

impl ::std::fmt::Debug for ResourceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResourceView<'_> {
  fn default() -> ResourceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Resource>> for ResourceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Resource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceView<'msg> {

  pub fn to_owned(&self) -> Resource {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // resource_name: optional message envoy.service.discovery.v3.ResourceName
  pub fn has_resource_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn resource_name_opt(self) -> ::std::option::Option<super::ResourceNameView<'msg>> {
    self.has_resource_name().then(|| self.resource_name())
  }
  pub fn resource_name(self) -> super::ResourceNameView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResourceNameView::default())
  }

  // aliases: repeated string
  pub fn aliases(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // version: optional string
  pub fn version(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // resource: optional message google.protobuf.Any
  pub fn has_resource(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn resource_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_resource().then(|| self.resource())
  }
  pub fn resource(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // ttl: optional message google.protobuf.Duration
  pub fn has_ttl(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn ttl_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_ttl().then(|| self.ttl())
  }
  pub fn ttl(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // cache_control: optional message envoy.service.discovery.v3.Resource.CacheControl
  pub fn has_cache_control(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn cache_control_opt(self) -> ::std::option::Option<super::resource::CacheControlView<'msg>> {
    self.has_cache_control().then(|| self.cache_control())
  }
  pub fn cache_control(self) -> super::resource::CacheControlView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::resource::CacheControlView::default())
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }

}

// SAFETY:
// - `ResourceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ResourceView<'_> {}

// SAFETY:
// - `ResourceView` is `Send` because while its alive a `ResourceMut` cannot.
// - `ResourceView` does not use thread-local data.
unsafe impl ::std::marker::Send for ResourceView<'_> {}

impl<'msg> ::protobuf::AsView for ResourceView<'msg> {
  type Proxied = Resource;
  fn as_view(&self) -> ::protobuf::View<'msg, Resource> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceView<'msg> {
  fn into_view<'shorter>(self) -> ResourceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Resource> for ResourceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Resource {
    let mut dst = Resource::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Resource> for ResourceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Resource {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Resource {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResourceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Resource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResourceMut<'msg> {
  type Message = Resource;
}

impl ::std::fmt::Debug for ResourceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Resource>> for ResourceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Resource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Resource> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Resource {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // resource_name: optional message envoy.service.discovery.v3.ResourceName
  pub fn has_resource_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_resource_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn resource_name_opt(&self) -> ::std::option::Option<super::ResourceNameView<'_>> {
    self.has_resource_name().then(|| self.resource_name())
  }
  pub fn resource_name(&self) -> super::ResourceNameView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResourceNameView::default())
  }
  pub fn resource_name_mut(&mut self) -> super::ResourceNameMut<'_> {
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
  pub fn set_resource_name(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResourceName>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // aliases: repeated string
  pub fn aliases(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn aliases_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_aliases(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // version: optional string
  pub fn version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // resource: optional message google.protobuf.Any
  pub fn has_resource(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_resource(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn resource_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_resource().then(|| self.resource())
  }
  pub fn resource(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn resource_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_resource(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // ttl: optional message google.protobuf.Duration
  pub fn has_ttl(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_ttl(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn ttl_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_ttl().then(|| self.ttl())
  }
  pub fn ttl(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn ttl_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_ttl(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // cache_control: optional message envoy.service.discovery.v3.Resource.CacheControl
  pub fn has_cache_control(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_cache_control(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn cache_control_opt(&self) -> ::std::option::Option<super::resource::CacheControlView<'_>> {
    self.has_cache_control().then(|| self.cache_control())
  }
  pub fn cache_control(&self) -> super::resource::CacheControlView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::resource::CacheControlView::default())
  }
  pub fn cache_control_mut(&mut self) -> super::resource::CacheControlMut<'_> {
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
  pub fn set_cache_control(&mut self,
    val: impl ::protobuf::IntoProxied<super::resource::CacheControl>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

}

// SAFETY:
// - `ResourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ResourceMut<'_> {}

// SAFETY:
// - `ResourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ResourceMut<'_> {}

impl<'msg> ::protobuf::AsView for ResourceMut<'msg> {
  type Proxied = Resource;
  fn as_view(&self) -> ::protobuf::View<'_, Resource> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Resource>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ResourceMut<'msg> {
  type MutProxied = Resource;
  fn as_mut(&mut self) -> ResourceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResourceMut<'msg> {
  fn into_mut<'shorter>(self) -> ResourceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Resource {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Resource> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResourceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResourceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // resource_name: optional message envoy.service.discovery.v3.ResourceName
  pub fn has_resource_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_resource_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn resource_name_opt(&self) -> ::std::option::Option<super::ResourceNameView<'_>> {
    self.has_resource_name().then(|| self.resource_name())
  }
  pub fn resource_name(&self) -> super::ResourceNameView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResourceNameView::default())
  }
  pub fn resource_name_mut(&mut self) -> super::ResourceNameMut<'_> {
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
  pub fn set_resource_name(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResourceName>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // aliases: repeated string
  pub fn aliases(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn aliases_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_aliases(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // version: optional string
  pub fn version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // resource: optional message google.protobuf.Any
  pub fn has_resource(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_resource(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn resource_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_resource().then(|| self.resource())
  }
  pub fn resource(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn resource_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_resource(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // ttl: optional message google.protobuf.Duration
  pub fn has_ttl(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_ttl(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn ttl_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_ttl().then(|| self.ttl())
  }
  pub fn ttl(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn ttl_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_ttl(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // cache_control: optional message envoy.service.discovery.v3.Resource.CacheControl
  pub fn has_cache_control(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_cache_control(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn cache_control_opt(&self) -> ::std::option::Option<super::resource::CacheControlView<'_>> {
    self.has_cache_control().then(|| self.cache_control())
  }
  pub fn cache_control(&self) -> super::resource::CacheControlView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::resource::CacheControlView::default())
  }
  pub fn cache_control_mut(&mut self) -> super::resource::CacheControlMut<'_> {
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
  pub fn set_cache_control(&mut self,
    val: impl ::protobuf::IntoProxied<super::resource::CacheControl>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

}  // impl Resource

impl ::std::ops::Drop for Resource {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Resource {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Resource {
  type Proxied = Self;
  fn as_view(&self) -> ResourceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Resource {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResourceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Resource {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__discovery__v3__Resource_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X31XETa3333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__discovery__v3__Resource_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::resource::CacheControl as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ResourceName as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__discovery__v3__Resource_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Resource {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Resource {
  type Msg = Resource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Resource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Resource {
  type Msg = Resource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Resource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceMut<'_> {
  type Msg = Resource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Resource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceMut<'_> {
  type Msg = Resource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Resource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceView<'_> {
  type Msg = Resource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Resource> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod resource {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__discovery__v3__Resource__CacheControl_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CacheControl {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CacheControl>
}

impl ::protobuf::Message for CacheControl {
  type MessageView<'msg> = CacheControlView<'msg>;
  type MessageMut<'msg> = CacheControlMut<'msg>;
}

impl ::std::default::Default for CacheControl {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CacheControl {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CacheControl` is `Sync` because it does not implement interior mutability.
//    Neither does `CacheControlMut`.
unsafe impl ::std::marker::Sync for CacheControl {}

// SAFETY:
// - `CacheControl` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CacheControl {}

impl ::protobuf::Proxied for CacheControl {
  type View<'msg> = CacheControlView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CacheControl {}

impl ::protobuf::MutProxied for CacheControl {
  type Mut<'msg> = CacheControlMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CacheControlView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CacheControl>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CacheControlView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CacheControlView<'msg> {
  type Message = CacheControl;
}

impl ::std::fmt::Debug for CacheControlView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CacheControlView<'_> {
  fn default() -> CacheControlView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CacheControl>> for CacheControlView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CacheControl>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CacheControlView<'msg> {

  pub fn to_owned(&self) -> CacheControl {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // do_not_cache: optional bool
  pub fn do_not_cache(self) -> bool {
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

}

// SAFETY:
// - `CacheControlView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CacheControlView<'_> {}

// SAFETY:
// - `CacheControlView` is `Send` because while its alive a `CacheControlMut` cannot.
// - `CacheControlView` does not use thread-local data.
unsafe impl ::std::marker::Send for CacheControlView<'_> {}

impl<'msg> ::protobuf::AsView for CacheControlView<'msg> {
  type Proxied = CacheControl;
  fn as_view(&self) -> ::protobuf::View<'msg, CacheControl> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CacheControlView<'msg> {
  fn into_view<'shorter>(self) -> CacheControlView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CacheControl> for CacheControlView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CacheControl {
    let mut dst = CacheControl::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CacheControl> for CacheControlMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CacheControl {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CacheControl {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CacheControlView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CacheControlMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CacheControlMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CacheControl>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CacheControlMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CacheControlMut<'msg> {
  type Message = CacheControl;
}

impl ::std::fmt::Debug for CacheControlMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CacheControl>> for CacheControlMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CacheControl>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CacheControlMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CacheControl> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CacheControl {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // do_not_cache: optional bool
  pub fn do_not_cache(&self) -> bool {
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
  pub fn set_do_not_cache(&mut self, val: bool) {
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

}

// SAFETY:
// - `CacheControlMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CacheControlMut<'_> {}

// SAFETY:
// - `CacheControlMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CacheControlMut<'_> {}

impl<'msg> ::protobuf::AsView for CacheControlMut<'msg> {
  type Proxied = CacheControl;
  fn as_view(&self) -> ::protobuf::View<'_, CacheControl> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CacheControlMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CacheControl>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CacheControlMut<'msg> {
  type MutProxied = CacheControl;
  fn as_mut(&mut self) -> CacheControlMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CacheControlMut<'msg> {
  fn into_mut<'shorter>(self) -> CacheControlMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CacheControl {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CacheControl> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CacheControlView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CacheControlMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // do_not_cache: optional bool
  pub fn do_not_cache(&self) -> bool {
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
  pub fn set_do_not_cache(&mut self, val: bool) {
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

}  // impl CacheControl

impl ::std::ops::Drop for CacheControl {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CacheControl {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CacheControl {
  type Proxied = Self;
  fn as_view(&self) -> CacheControlView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CacheControl {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CacheControlMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CacheControl {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::resource::envoy__service__discovery__v3__Resource__CacheControl_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::resource::envoy__service__discovery__v3__Resource__CacheControl_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::resource::envoy__service__discovery__v3__Resource__CacheControl_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CacheControl {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CacheControl {
  type Msg = CacheControl;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CacheControl> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CacheControl {
  type Msg = CacheControl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CacheControl> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CacheControlMut<'_> {
  type Msg = CacheControl;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CacheControl> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CacheControlMut<'_> {
  type Msg = CacheControl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CacheControl> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CacheControlView<'_> {
  type Msg = CacheControl;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CacheControl> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CacheControlMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod resource


