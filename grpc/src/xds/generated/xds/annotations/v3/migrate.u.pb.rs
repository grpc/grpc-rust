const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__annotations__v3__MigrateAnnotation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MigrateAnnotation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MigrateAnnotation>
}

impl ::protobuf::Message for MigrateAnnotation {
  type MessageView<'msg> = MigrateAnnotationView<'msg>;
  type MessageMut<'msg> = MigrateAnnotationMut<'msg>;
}

impl ::std::default::Default for MigrateAnnotation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MigrateAnnotation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MigrateAnnotation` is `Sync` because it does not implement interior mutability.
//    Neither does `MigrateAnnotationMut`.
unsafe impl ::std::marker::Sync for MigrateAnnotation {}

// SAFETY:
// - `MigrateAnnotation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MigrateAnnotation {}

impl ::protobuf::Proxied for MigrateAnnotation {
  type View<'msg> = MigrateAnnotationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MigrateAnnotation {}

impl ::protobuf::MutProxied for MigrateAnnotation {
  type Mut<'msg> = MigrateAnnotationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MigrateAnnotationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MigrateAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MigrateAnnotationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MigrateAnnotationView<'msg> {
  type Message = MigrateAnnotation;
}

impl ::std::fmt::Debug for MigrateAnnotationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MigrateAnnotationView<'_> {
  fn default() -> MigrateAnnotationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MigrateAnnotation>> for MigrateAnnotationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MigrateAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MigrateAnnotationView<'msg> {

  pub fn to_owned(&self) -> MigrateAnnotation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // rename: optional string
  pub fn rename(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `MigrateAnnotationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MigrateAnnotationView<'_> {}

// SAFETY:
// - `MigrateAnnotationView` is `Send` because while its alive a `MigrateAnnotationMut` cannot.
// - `MigrateAnnotationView` does not use thread-local data.
unsafe impl ::std::marker::Send for MigrateAnnotationView<'_> {}

impl<'msg> ::protobuf::AsView for MigrateAnnotationView<'msg> {
  type Proxied = MigrateAnnotation;
  fn as_view(&self) -> ::protobuf::View<'msg, MigrateAnnotation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MigrateAnnotationView<'msg> {
  fn into_view<'shorter>(self) -> MigrateAnnotationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MigrateAnnotation> for MigrateAnnotationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MigrateAnnotation {
    let mut dst = MigrateAnnotation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MigrateAnnotation> for MigrateAnnotationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MigrateAnnotation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MigrateAnnotation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MigrateAnnotationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MigrateAnnotationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MigrateAnnotationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MigrateAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MigrateAnnotationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MigrateAnnotationMut<'msg> {
  type Message = MigrateAnnotation;
}

impl ::std::fmt::Debug for MigrateAnnotationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MigrateAnnotation>> for MigrateAnnotationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MigrateAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MigrateAnnotationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MigrateAnnotation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MigrateAnnotation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // rename: optional string
  pub fn rename(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_rename(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `MigrateAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MigrateAnnotationMut<'_> {}

// SAFETY:
// - `MigrateAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MigrateAnnotationMut<'_> {}

impl<'msg> ::protobuf::AsView for MigrateAnnotationMut<'msg> {
  type Proxied = MigrateAnnotation;
  fn as_view(&self) -> ::protobuf::View<'_, MigrateAnnotation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MigrateAnnotationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MigrateAnnotation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MigrateAnnotationMut<'msg> {
  type MutProxied = MigrateAnnotation;
  fn as_mut(&mut self) -> MigrateAnnotationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MigrateAnnotationMut<'msg> {
  fn into_mut<'shorter>(self) -> MigrateAnnotationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MigrateAnnotation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MigrateAnnotation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MigrateAnnotationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MigrateAnnotationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // rename: optional string
  pub fn rename(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_rename(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl MigrateAnnotation

impl ::std::ops::Drop for MigrateAnnotation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MigrateAnnotation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MigrateAnnotation {
  type Proxied = Self;
  fn as_view(&self) -> MigrateAnnotationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MigrateAnnotation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MigrateAnnotationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MigrateAnnotation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__annotations__v3__MigrateAnnotation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__annotations__v3__MigrateAnnotation_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__annotations__v3__MigrateAnnotation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MigrateAnnotation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MigrateAnnotation {
  type Msg = MigrateAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MigrateAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MigrateAnnotation {
  type Msg = MigrateAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MigrateAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MigrateAnnotationMut<'_> {
  type Msg = MigrateAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MigrateAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MigrateAnnotationMut<'_> {
  type Msg = MigrateAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MigrateAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MigrateAnnotationView<'_> {
  type Msg = MigrateAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MigrateAnnotation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MigrateAnnotationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__annotations__v3__FieldMigrateAnnotation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FieldMigrateAnnotation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FieldMigrateAnnotation>
}

impl ::protobuf::Message for FieldMigrateAnnotation {
  type MessageView<'msg> = FieldMigrateAnnotationView<'msg>;
  type MessageMut<'msg> = FieldMigrateAnnotationMut<'msg>;
}

impl ::std::default::Default for FieldMigrateAnnotation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FieldMigrateAnnotation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FieldMigrateAnnotation` is `Sync` because it does not implement interior mutability.
//    Neither does `FieldMigrateAnnotationMut`.
unsafe impl ::std::marker::Sync for FieldMigrateAnnotation {}

// SAFETY:
// - `FieldMigrateAnnotation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FieldMigrateAnnotation {}

impl ::protobuf::Proxied for FieldMigrateAnnotation {
  type View<'msg> = FieldMigrateAnnotationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FieldMigrateAnnotation {}

impl ::protobuf::MutProxied for FieldMigrateAnnotation {
  type Mut<'msg> = FieldMigrateAnnotationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FieldMigrateAnnotationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldMigrateAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldMigrateAnnotationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FieldMigrateAnnotationView<'msg> {
  type Message = FieldMigrateAnnotation;
}

impl ::std::fmt::Debug for FieldMigrateAnnotationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FieldMigrateAnnotationView<'_> {
  fn default() -> FieldMigrateAnnotationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FieldMigrateAnnotation>> for FieldMigrateAnnotationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldMigrateAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldMigrateAnnotationView<'msg> {

  pub fn to_owned(&self) -> FieldMigrateAnnotation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // rename: optional string
  pub fn rename(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // oneof_promotion: optional string
  pub fn oneof_promotion(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `FieldMigrateAnnotationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FieldMigrateAnnotationView<'_> {}

// SAFETY:
// - `FieldMigrateAnnotationView` is `Send` because while its alive a `FieldMigrateAnnotationMut` cannot.
// - `FieldMigrateAnnotationView` does not use thread-local data.
unsafe impl ::std::marker::Send for FieldMigrateAnnotationView<'_> {}

impl<'msg> ::protobuf::AsView for FieldMigrateAnnotationView<'msg> {
  type Proxied = FieldMigrateAnnotation;
  fn as_view(&self) -> ::protobuf::View<'msg, FieldMigrateAnnotation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldMigrateAnnotationView<'msg> {
  fn into_view<'shorter>(self) -> FieldMigrateAnnotationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldMigrateAnnotation> for FieldMigrateAnnotationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldMigrateAnnotation {
    let mut dst = FieldMigrateAnnotation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldMigrateAnnotation> for FieldMigrateAnnotationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldMigrateAnnotation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FieldMigrateAnnotation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FieldMigrateAnnotationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FieldMigrateAnnotationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FieldMigrateAnnotationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldMigrateAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldMigrateAnnotationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FieldMigrateAnnotationMut<'msg> {
  type Message = FieldMigrateAnnotation;
}

impl ::std::fmt::Debug for FieldMigrateAnnotationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FieldMigrateAnnotation>> for FieldMigrateAnnotationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldMigrateAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldMigrateAnnotationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldMigrateAnnotation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FieldMigrateAnnotation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // rename: optional string
  pub fn rename(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_rename(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // oneof_promotion: optional string
  pub fn oneof_promotion(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_oneof_promotion(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `FieldMigrateAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FieldMigrateAnnotationMut<'_> {}

// SAFETY:
// - `FieldMigrateAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FieldMigrateAnnotationMut<'_> {}

impl<'msg> ::protobuf::AsView for FieldMigrateAnnotationMut<'msg> {
  type Proxied = FieldMigrateAnnotation;
  fn as_view(&self) -> ::protobuf::View<'_, FieldMigrateAnnotation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldMigrateAnnotationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FieldMigrateAnnotation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FieldMigrateAnnotationMut<'msg> {
  type MutProxied = FieldMigrateAnnotation;
  fn as_mut(&mut self) -> FieldMigrateAnnotationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FieldMigrateAnnotationMut<'msg> {
  fn into_mut<'shorter>(self) -> FieldMigrateAnnotationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FieldMigrateAnnotation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FieldMigrateAnnotation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FieldMigrateAnnotationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FieldMigrateAnnotationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // rename: optional string
  pub fn rename(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_rename(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // oneof_promotion: optional string
  pub fn oneof_promotion(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_oneof_promotion(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl FieldMigrateAnnotation

impl ::std::ops::Drop for FieldMigrateAnnotation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FieldMigrateAnnotation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FieldMigrateAnnotation {
  type Proxied = Self;
  fn as_view(&self) -> FieldMigrateAnnotationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FieldMigrateAnnotation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FieldMigrateAnnotationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FieldMigrateAnnotation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__annotations__v3__FieldMigrateAnnotation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__annotations__v3__FieldMigrateAnnotation_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__annotations__v3__FieldMigrateAnnotation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldMigrateAnnotation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldMigrateAnnotation {
  type Msg = FieldMigrateAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldMigrateAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldMigrateAnnotation {
  type Msg = FieldMigrateAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldMigrateAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldMigrateAnnotationMut<'_> {
  type Msg = FieldMigrateAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldMigrateAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldMigrateAnnotationMut<'_> {
  type Msg = FieldMigrateAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldMigrateAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldMigrateAnnotationView<'_> {
  type Msg = FieldMigrateAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldMigrateAnnotation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldMigrateAnnotationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__annotations__v3__FileMigrateAnnotation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FileMigrateAnnotation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FileMigrateAnnotation>
}

impl ::protobuf::Message for FileMigrateAnnotation {
  type MessageView<'msg> = FileMigrateAnnotationView<'msg>;
  type MessageMut<'msg> = FileMigrateAnnotationMut<'msg>;
}

impl ::std::default::Default for FileMigrateAnnotation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FileMigrateAnnotation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FileMigrateAnnotation` is `Sync` because it does not implement interior mutability.
//    Neither does `FileMigrateAnnotationMut`.
unsafe impl ::std::marker::Sync for FileMigrateAnnotation {}

// SAFETY:
// - `FileMigrateAnnotation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FileMigrateAnnotation {}

impl ::protobuf::Proxied for FileMigrateAnnotation {
  type View<'msg> = FileMigrateAnnotationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FileMigrateAnnotation {}

impl ::protobuf::MutProxied for FileMigrateAnnotation {
  type Mut<'msg> = FileMigrateAnnotationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FileMigrateAnnotationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FileMigrateAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FileMigrateAnnotationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FileMigrateAnnotationView<'msg> {
  type Message = FileMigrateAnnotation;
}

impl ::std::fmt::Debug for FileMigrateAnnotationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FileMigrateAnnotationView<'_> {
  fn default() -> FileMigrateAnnotationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FileMigrateAnnotation>> for FileMigrateAnnotationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FileMigrateAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FileMigrateAnnotationView<'msg> {

  pub fn to_owned(&self) -> FileMigrateAnnotation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // move_to_package: optional string
  pub fn move_to_package(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `FileMigrateAnnotationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FileMigrateAnnotationView<'_> {}

// SAFETY:
// - `FileMigrateAnnotationView` is `Send` because while its alive a `FileMigrateAnnotationMut` cannot.
// - `FileMigrateAnnotationView` does not use thread-local data.
unsafe impl ::std::marker::Send for FileMigrateAnnotationView<'_> {}

impl<'msg> ::protobuf::AsView for FileMigrateAnnotationView<'msg> {
  type Proxied = FileMigrateAnnotation;
  fn as_view(&self) -> ::protobuf::View<'msg, FileMigrateAnnotation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FileMigrateAnnotationView<'msg> {
  fn into_view<'shorter>(self) -> FileMigrateAnnotationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FileMigrateAnnotation> for FileMigrateAnnotationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FileMigrateAnnotation {
    let mut dst = FileMigrateAnnotation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FileMigrateAnnotation> for FileMigrateAnnotationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FileMigrateAnnotation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FileMigrateAnnotation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FileMigrateAnnotationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FileMigrateAnnotationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FileMigrateAnnotationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FileMigrateAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FileMigrateAnnotationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FileMigrateAnnotationMut<'msg> {
  type Message = FileMigrateAnnotation;
}

impl ::std::fmt::Debug for FileMigrateAnnotationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FileMigrateAnnotation>> for FileMigrateAnnotationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FileMigrateAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FileMigrateAnnotationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FileMigrateAnnotation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FileMigrateAnnotation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // move_to_package: optional string
  pub fn move_to_package(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_move_to_package(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `FileMigrateAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FileMigrateAnnotationMut<'_> {}

// SAFETY:
// - `FileMigrateAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FileMigrateAnnotationMut<'_> {}

impl<'msg> ::protobuf::AsView for FileMigrateAnnotationMut<'msg> {
  type Proxied = FileMigrateAnnotation;
  fn as_view(&self) -> ::protobuf::View<'_, FileMigrateAnnotation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FileMigrateAnnotationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FileMigrateAnnotation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FileMigrateAnnotationMut<'msg> {
  type MutProxied = FileMigrateAnnotation;
  fn as_mut(&mut self) -> FileMigrateAnnotationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FileMigrateAnnotationMut<'msg> {
  fn into_mut<'shorter>(self) -> FileMigrateAnnotationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FileMigrateAnnotation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FileMigrateAnnotation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FileMigrateAnnotationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FileMigrateAnnotationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // move_to_package: optional string
  pub fn move_to_package(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_move_to_package(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl FileMigrateAnnotation

impl ::std::ops::Drop for FileMigrateAnnotation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FileMigrateAnnotation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FileMigrateAnnotation {
  type Proxied = Self;
  fn as_view(&self) -> FileMigrateAnnotationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FileMigrateAnnotation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FileMigrateAnnotationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FileMigrateAnnotation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__annotations__v3__FileMigrateAnnotation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$Ma1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__annotations__v3__FileMigrateAnnotation_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__annotations__v3__FileMigrateAnnotation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FileMigrateAnnotation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FileMigrateAnnotation {
  type Msg = FileMigrateAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FileMigrateAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FileMigrateAnnotation {
  type Msg = FileMigrateAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FileMigrateAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FileMigrateAnnotationMut<'_> {
  type Msg = FileMigrateAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FileMigrateAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FileMigrateAnnotationMut<'_> {
  type Msg = FileMigrateAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FileMigrateAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FileMigrateAnnotationView<'_> {
  type Msg = FileMigrateAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FileMigrateAnnotation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FileMigrateAnnotationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}








