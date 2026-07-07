const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__trace__v3__DynamicOtConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DynamicOtConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DynamicOtConfig>
}

impl ::protobuf::Message for DynamicOtConfig {
  type MessageView<'msg> = DynamicOtConfigView<'msg>;
  type MessageMut<'msg> = DynamicOtConfigMut<'msg>;
}

impl ::std::default::Default for DynamicOtConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DynamicOtConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DynamicOtConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `DynamicOtConfigMut`.
unsafe impl ::std::marker::Sync for DynamicOtConfig {}

// SAFETY:
// - `DynamicOtConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DynamicOtConfig {}

impl ::protobuf::Proxied for DynamicOtConfig {
  type View<'msg> = DynamicOtConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DynamicOtConfig {}

impl ::protobuf::MutProxied for DynamicOtConfig {
  type Mut<'msg> = DynamicOtConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DynamicOtConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicOtConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicOtConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DynamicOtConfigView<'msg> {
  type Message = DynamicOtConfig;
}

impl ::std::fmt::Debug for DynamicOtConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DynamicOtConfigView<'_> {
  fn default() -> DynamicOtConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicOtConfig>> for DynamicOtConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicOtConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicOtConfigView<'msg> {

  pub fn to_owned(&self) -> DynamicOtConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // library: optional string
  pub fn library(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // config: optional message google.protobuf.Struct
  pub fn has_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

}

// SAFETY:
// - `DynamicOtConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DynamicOtConfigView<'_> {}

// SAFETY:
// - `DynamicOtConfigView` is `Send` because while its alive a `DynamicOtConfigMut` cannot.
// - `DynamicOtConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for DynamicOtConfigView<'_> {}

impl<'msg> ::protobuf::AsView for DynamicOtConfigView<'msg> {
  type Proxied = DynamicOtConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, DynamicOtConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicOtConfigView<'msg> {
  fn into_view<'shorter>(self) -> DynamicOtConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicOtConfig> for DynamicOtConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicOtConfig {
    let mut dst = DynamicOtConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicOtConfig> for DynamicOtConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicOtConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DynamicOtConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicOtConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicOtConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DynamicOtConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicOtConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicOtConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DynamicOtConfigMut<'msg> {
  type Message = DynamicOtConfig;
}

impl ::std::fmt::Debug for DynamicOtConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicOtConfig>> for DynamicOtConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicOtConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicOtConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicOtConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DynamicOtConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // library: optional string
  pub fn library(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_library(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // config: optional message google.protobuf.Struct
  pub fn has_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn config_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

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
// - `DynamicOtConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DynamicOtConfigMut<'_> {}

// SAFETY:
// - `DynamicOtConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DynamicOtConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for DynamicOtConfigMut<'msg> {
  type Proxied = DynamicOtConfig;
  fn as_view(&self) -> ::protobuf::View<'_, DynamicOtConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicOtConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DynamicOtConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DynamicOtConfigMut<'msg> {
  type MutProxied = DynamicOtConfig;
  fn as_mut(&mut self) -> DynamicOtConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DynamicOtConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> DynamicOtConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DynamicOtConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DynamicOtConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DynamicOtConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DynamicOtConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // library: optional string
  pub fn library(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_library(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // config: optional message google.protobuf.Struct
  pub fn has_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn config_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl DynamicOtConfig

impl ::std::ops::Drop for DynamicOtConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DynamicOtConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DynamicOtConfig {
  type Proxied = Self;
  fn as_view(&self) -> DynamicOtConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DynamicOtConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DynamicOtConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicOtConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__trace__v3__DynamicOtConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__trace__v3__DynamicOtConfig_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__trace__v3__DynamicOtConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicOtConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicOtConfig {
  type Msg = DynamicOtConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicOtConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicOtConfig {
  type Msg = DynamicOtConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicOtConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicOtConfigMut<'_> {
  type Msg = DynamicOtConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicOtConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicOtConfigMut<'_> {
  type Msg = DynamicOtConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicOtConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicOtConfigView<'_> {
  type Msg = DynamicOtConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicOtConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicOtConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



