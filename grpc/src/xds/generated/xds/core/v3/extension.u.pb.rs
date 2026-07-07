const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__core__v3__TypedExtensionConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TypedExtensionConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TypedExtensionConfig>
}

impl ::protobuf::Message for TypedExtensionConfig {
  type MessageView<'msg> = TypedExtensionConfigView<'msg>;
  type MessageMut<'msg> = TypedExtensionConfigMut<'msg>;
}

impl ::std::default::Default for TypedExtensionConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TypedExtensionConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TypedExtensionConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `TypedExtensionConfigMut`.
unsafe impl ::std::marker::Sync for TypedExtensionConfig {}

// SAFETY:
// - `TypedExtensionConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TypedExtensionConfig {}

impl ::protobuf::Proxied for TypedExtensionConfig {
  type View<'msg> = TypedExtensionConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TypedExtensionConfig {}

impl ::protobuf::MutProxied for TypedExtensionConfig {
  type Mut<'msg> = TypedExtensionConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TypedExtensionConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TypedExtensionConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TypedExtensionConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TypedExtensionConfigView<'msg> {
  type Message = TypedExtensionConfig;
}

impl ::std::fmt::Debug for TypedExtensionConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TypedExtensionConfigView<'_> {
  fn default() -> TypedExtensionConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TypedExtensionConfig>> for TypedExtensionConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TypedExtensionConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TypedExtensionConfigView<'msg> {

  pub fn to_owned(&self) -> TypedExtensionConfig {
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

}

// SAFETY:
// - `TypedExtensionConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TypedExtensionConfigView<'_> {}

// SAFETY:
// - `TypedExtensionConfigView` is `Send` because while its alive a `TypedExtensionConfigMut` cannot.
// - `TypedExtensionConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for TypedExtensionConfigView<'_> {}

impl<'msg> ::protobuf::AsView for TypedExtensionConfigView<'msg> {
  type Proxied = TypedExtensionConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, TypedExtensionConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TypedExtensionConfigView<'msg> {
  fn into_view<'shorter>(self) -> TypedExtensionConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TypedExtensionConfig> for TypedExtensionConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TypedExtensionConfig {
    let mut dst = TypedExtensionConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TypedExtensionConfig> for TypedExtensionConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TypedExtensionConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TypedExtensionConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TypedExtensionConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TypedExtensionConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TypedExtensionConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TypedExtensionConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TypedExtensionConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TypedExtensionConfigMut<'msg> {
  type Message = TypedExtensionConfig;
}

impl ::std::fmt::Debug for TypedExtensionConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TypedExtensionConfig>> for TypedExtensionConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TypedExtensionConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TypedExtensionConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TypedExtensionConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TypedExtensionConfig {
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

}

// SAFETY:
// - `TypedExtensionConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TypedExtensionConfigMut<'_> {}

// SAFETY:
// - `TypedExtensionConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TypedExtensionConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for TypedExtensionConfigMut<'msg> {
  type Proxied = TypedExtensionConfig;
  fn as_view(&self) -> ::protobuf::View<'_, TypedExtensionConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TypedExtensionConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TypedExtensionConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TypedExtensionConfigMut<'msg> {
  type MutProxied = TypedExtensionConfig;
  fn as_mut(&mut self) -> TypedExtensionConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TypedExtensionConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> TypedExtensionConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TypedExtensionConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TypedExtensionConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TypedExtensionConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TypedExtensionConfigMut<'_> {
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

}  // impl TypedExtensionConfig

impl ::std::ops::Drop for TypedExtensionConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TypedExtensionConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TypedExtensionConfig {
  type Proxied = Self;
  fn as_view(&self) -> TypedExtensionConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TypedExtensionConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TypedExtensionConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TypedExtensionConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__core__v3__TypedExtensionConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__core__v3__TypedExtensionConfig_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__core__v3__TypedExtensionConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TypedExtensionConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TypedExtensionConfig {
  type Msg = TypedExtensionConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TypedExtensionConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TypedExtensionConfig {
  type Msg = TypedExtensionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TypedExtensionConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TypedExtensionConfigMut<'_> {
  type Msg = TypedExtensionConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TypedExtensionConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TypedExtensionConfigMut<'_> {
  type Msg = TypedExtensionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TypedExtensionConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TypedExtensionConfigView<'_> {
  type Msg = TypedExtensionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TypedExtensionConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TypedExtensionConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



