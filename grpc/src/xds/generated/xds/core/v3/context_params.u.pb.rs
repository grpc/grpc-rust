const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__core__v3__ContextParams_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ContextParams {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ContextParams>
}

impl ::protobuf::Message for ContextParams {
  type MessageView<'msg> = ContextParamsView<'msg>;
  type MessageMut<'msg> = ContextParamsMut<'msg>;
}

impl ::std::default::Default for ContextParams {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ContextParams {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ContextParams` is `Sync` because it does not implement interior mutability.
//    Neither does `ContextParamsMut`.
unsafe impl ::std::marker::Sync for ContextParams {}

// SAFETY:
// - `ContextParams` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ContextParams {}

impl ::protobuf::Proxied for ContextParams {
  type View<'msg> = ContextParamsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ContextParams {}

impl ::protobuf::MutProxied for ContextParams {
  type Mut<'msg> = ContextParamsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ContextParamsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ContextParams>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ContextParamsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ContextParamsView<'msg> {
  type Message = ContextParams;
}

impl ::std::fmt::Debug for ContextParamsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ContextParamsView<'_> {
  fn default() -> ContextParamsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ContextParams>> for ContextParamsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ContextParams>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ContextParamsView<'msg> {

  pub fn to_owned(&self) -> ContextParams {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // params: repeated message xds.core.v3.ContextParams.ParamsEntry
  pub fn params(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `ContextParamsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ContextParamsView<'_> {}

// SAFETY:
// - `ContextParamsView` is `Send` because while its alive a `ContextParamsMut` cannot.
// - `ContextParamsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ContextParamsView<'_> {}

impl<'msg> ::protobuf::AsView for ContextParamsView<'msg> {
  type Proxied = ContextParams;
  fn as_view(&self) -> ::protobuf::View<'msg, ContextParams> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ContextParamsView<'msg> {
  fn into_view<'shorter>(self) -> ContextParamsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ContextParams> for ContextParamsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ContextParams {
    let mut dst = ContextParams::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ContextParams> for ContextParamsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ContextParams {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ContextParams {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ContextParamsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ContextParamsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ContextParamsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ContextParams>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ContextParamsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ContextParamsMut<'msg> {
  type Message = ContextParams;
}

impl ::std::fmt::Debug for ContextParamsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ContextParams>> for ContextParamsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ContextParams>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ContextParamsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ContextParams> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ContextParams {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // params: repeated message xds.core.v3.ContextParams.ParamsEntry
  pub fn params(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn params_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_params(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ContextParamsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ContextParamsMut<'_> {}

// SAFETY:
// - `ContextParamsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ContextParamsMut<'_> {}

impl<'msg> ::protobuf::AsView for ContextParamsMut<'msg> {
  type Proxied = ContextParams;
  fn as_view(&self) -> ::protobuf::View<'_, ContextParams> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ContextParamsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ContextParams>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ContextParamsMut<'msg> {
  type MutProxied = ContextParams;
  fn as_mut(&mut self) -> ContextParamsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ContextParamsMut<'msg> {
  fn into_mut<'shorter>(self) -> ContextParamsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ContextParams {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ContextParams> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ContextParamsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ContextParamsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // params: repeated message xds.core.v3.ContextParams.ParamsEntry
  pub fn params(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn params_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_params(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ContextParams

impl ::std::ops::Drop for ContextParams {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ContextParams {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ContextParams {
  type Proxied = Self;
  fn as_view(&self) -> ContextParamsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ContextParams {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ContextParamsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ContextParams {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__core__v3__ContextParams_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__core__v3__ContextParams_msg_init.0, &[<super::context_params::ParamsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__core__v3__ContextParams_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ContextParams {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ContextParams {
  type Msg = ContextParams;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ContextParams> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ContextParams {
  type Msg = ContextParams;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ContextParams> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ContextParamsMut<'_> {
  type Msg = ContextParams;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ContextParams> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ContextParamsMut<'_> {
  type Msg = ContextParams;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ContextParams> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ContextParamsView<'_> {
  type Msg = ContextParams;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ContextParams> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ContextParamsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod context_params {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__core__v3__ContextParams__ParamsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct ParamsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ParamsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::context_params::xds__core__v3__ContextParams__ParamsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::context_params::xds__core__v3__ContextParams__ParamsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::context_params::xds__core__v3__ContextParams__ParamsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod context_params


