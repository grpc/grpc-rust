const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__CelExpressionConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CelExpressionConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CelExpressionConfig>
}

impl ::protobuf::Message for CelExpressionConfig {
  type MessageView<'msg> = CelExpressionConfigView<'msg>;
  type MessageMut<'msg> = CelExpressionConfigMut<'msg>;
}

impl ::std::default::Default for CelExpressionConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CelExpressionConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CelExpressionConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `CelExpressionConfigMut`.
unsafe impl ::std::marker::Sync for CelExpressionConfig {}

// SAFETY:
// - `CelExpressionConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CelExpressionConfig {}

impl ::protobuf::Proxied for CelExpressionConfig {
  type View<'msg> = CelExpressionConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CelExpressionConfig {}

impl ::protobuf::MutProxied for CelExpressionConfig {
  type Mut<'msg> = CelExpressionConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CelExpressionConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CelExpressionConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CelExpressionConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CelExpressionConfigView<'msg> {
  type Message = CelExpressionConfig;
}

impl ::std::fmt::Debug for CelExpressionConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CelExpressionConfigView<'_> {
  fn default() -> CelExpressionConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CelExpressionConfig>> for CelExpressionConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CelExpressionConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CelExpressionConfigView<'msg> {

  pub fn to_owned(&self) -> CelExpressionConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // enable_string_conversion: optional bool
  pub fn enable_string_conversion(self) -> bool {
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

  // enable_string_concat: optional bool
  pub fn enable_string_concat(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

  // enable_string_functions: optional bool
  pub fn enable_string_functions(self) -> bool {
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

}

// SAFETY:
// - `CelExpressionConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CelExpressionConfigView<'_> {}

// SAFETY:
// - `CelExpressionConfigView` is `Send` because while its alive a `CelExpressionConfigMut` cannot.
// - `CelExpressionConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for CelExpressionConfigView<'_> {}

impl<'msg> ::protobuf::AsView for CelExpressionConfigView<'msg> {
  type Proxied = CelExpressionConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, CelExpressionConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CelExpressionConfigView<'msg> {
  fn into_view<'shorter>(self) -> CelExpressionConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CelExpressionConfig> for CelExpressionConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CelExpressionConfig {
    let mut dst = CelExpressionConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CelExpressionConfig> for CelExpressionConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CelExpressionConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CelExpressionConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CelExpressionConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CelExpressionConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CelExpressionConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CelExpressionConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CelExpressionConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CelExpressionConfigMut<'msg> {
  type Message = CelExpressionConfig;
}

impl ::std::fmt::Debug for CelExpressionConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CelExpressionConfig>> for CelExpressionConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CelExpressionConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CelExpressionConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CelExpressionConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CelExpressionConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // enable_string_conversion: optional bool
  pub fn enable_string_conversion(&self) -> bool {
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
  pub fn set_enable_string_conversion(&mut self, val: bool) {
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

  // enable_string_concat: optional bool
  pub fn enable_string_concat(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enable_string_concat(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // enable_string_functions: optional bool
  pub fn enable_string_functions(&self) -> bool {
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
  pub fn set_enable_string_functions(&mut self, val: bool) {
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

}

// SAFETY:
// - `CelExpressionConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CelExpressionConfigMut<'_> {}

// SAFETY:
// - `CelExpressionConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CelExpressionConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for CelExpressionConfigMut<'msg> {
  type Proxied = CelExpressionConfig;
  fn as_view(&self) -> ::protobuf::View<'_, CelExpressionConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CelExpressionConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CelExpressionConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CelExpressionConfigMut<'msg> {
  type MutProxied = CelExpressionConfig;
  fn as_mut(&mut self) -> CelExpressionConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CelExpressionConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> CelExpressionConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CelExpressionConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CelExpressionConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CelExpressionConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CelExpressionConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // enable_string_conversion: optional bool
  pub fn enable_string_conversion(&self) -> bool {
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
  pub fn set_enable_string_conversion(&mut self, val: bool) {
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

  // enable_string_concat: optional bool
  pub fn enable_string_concat(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enable_string_concat(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // enable_string_functions: optional bool
  pub fn enable_string_functions(&self) -> bool {
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
  pub fn set_enable_string_functions(&mut self, val: bool) {
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

}  // impl CelExpressionConfig

impl ::std::ops::Drop for CelExpressionConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CelExpressionConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CelExpressionConfig {
  type Proxied = Self;
  fn as_view(&self) -> CelExpressionConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CelExpressionConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CelExpressionConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CelExpressionConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__CelExpressionConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__CelExpressionConfig_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__CelExpressionConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CelExpressionConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CelExpressionConfig {
  type Msg = CelExpressionConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExpressionConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelExpressionConfig {
  type Msg = CelExpressionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExpressionConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CelExpressionConfigMut<'_> {
  type Msg = CelExpressionConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExpressionConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelExpressionConfigMut<'_> {
  type Msg = CelExpressionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExpressionConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelExpressionConfigView<'_> {
  type Msg = CelExpressionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelExpressionConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CelExpressionConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



