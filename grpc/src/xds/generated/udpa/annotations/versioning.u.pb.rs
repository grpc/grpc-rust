const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut udpa__annotations__VersioningAnnotation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct VersioningAnnotation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<VersioningAnnotation>
}

impl ::protobuf::Message for VersioningAnnotation {
  type MessageView<'msg> = VersioningAnnotationView<'msg>;
  type MessageMut<'msg> = VersioningAnnotationMut<'msg>;
}

impl ::std::default::Default for VersioningAnnotation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for VersioningAnnotation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `VersioningAnnotation` is `Sync` because it does not implement interior mutability.
//    Neither does `VersioningAnnotationMut`.
unsafe impl ::std::marker::Sync for VersioningAnnotation {}

// SAFETY:
// - `VersioningAnnotation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for VersioningAnnotation {}

impl ::protobuf::Proxied for VersioningAnnotation {
  type View<'msg> = VersioningAnnotationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for VersioningAnnotation {}

impl ::protobuf::MutProxied for VersioningAnnotation {
  type Mut<'msg> = VersioningAnnotationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct VersioningAnnotationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, VersioningAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VersioningAnnotationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for VersioningAnnotationView<'msg> {
  type Message = VersioningAnnotation;
}

impl ::std::fmt::Debug for VersioningAnnotationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for VersioningAnnotationView<'_> {
  fn default() -> VersioningAnnotationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, VersioningAnnotation>> for VersioningAnnotationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, VersioningAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VersioningAnnotationView<'msg> {

  pub fn to_owned(&self) -> VersioningAnnotation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // previous_message_type: optional string
  pub fn previous_message_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `VersioningAnnotationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for VersioningAnnotationView<'_> {}

// SAFETY:
// - `VersioningAnnotationView` is `Send` because while its alive a `VersioningAnnotationMut` cannot.
// - `VersioningAnnotationView` does not use thread-local data.
unsafe impl ::std::marker::Send for VersioningAnnotationView<'_> {}

impl<'msg> ::protobuf::AsView for VersioningAnnotationView<'msg> {
  type Proxied = VersioningAnnotation;
  fn as_view(&self) -> ::protobuf::View<'msg, VersioningAnnotation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VersioningAnnotationView<'msg> {
  fn into_view<'shorter>(self) -> VersioningAnnotationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<VersioningAnnotation> for VersioningAnnotationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> VersioningAnnotation {
    let mut dst = VersioningAnnotation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<VersioningAnnotation> for VersioningAnnotationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> VersioningAnnotation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for VersioningAnnotation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VersioningAnnotationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VersioningAnnotationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct VersioningAnnotationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, VersioningAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VersioningAnnotationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for VersioningAnnotationMut<'msg> {
  type Message = VersioningAnnotation;
}

impl ::std::fmt::Debug for VersioningAnnotationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, VersioningAnnotation>> for VersioningAnnotationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, VersioningAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VersioningAnnotationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, VersioningAnnotation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> VersioningAnnotation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // previous_message_type: optional string
  pub fn previous_message_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_previous_message_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `VersioningAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for VersioningAnnotationMut<'_> {}

// SAFETY:
// - `VersioningAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for VersioningAnnotationMut<'_> {}

impl<'msg> ::protobuf::AsView for VersioningAnnotationMut<'msg> {
  type Proxied = VersioningAnnotation;
  fn as_view(&self) -> ::protobuf::View<'_, VersioningAnnotation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VersioningAnnotationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, VersioningAnnotation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for VersioningAnnotationMut<'msg> {
  type MutProxied = VersioningAnnotation;
  fn as_mut(&mut self) -> VersioningAnnotationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for VersioningAnnotationMut<'msg> {
  fn into_mut<'shorter>(self) -> VersioningAnnotationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl VersioningAnnotation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, VersioningAnnotation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> VersioningAnnotationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> VersioningAnnotationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // previous_message_type: optional string
  pub fn previous_message_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_previous_message_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl VersioningAnnotation

impl ::std::ops::Drop for VersioningAnnotation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for VersioningAnnotation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for VersioningAnnotation {
  type Proxied = Self;
  fn as_view(&self) -> VersioningAnnotationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for VersioningAnnotation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> VersioningAnnotationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for VersioningAnnotation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::udpa__annotations__VersioningAnnotation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::udpa__annotations__VersioningAnnotation_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::udpa__annotations__VersioningAnnotation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for VersioningAnnotation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for VersioningAnnotation {
  type Msg = VersioningAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VersioningAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VersioningAnnotation {
  type Msg = VersioningAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VersioningAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for VersioningAnnotationMut<'_> {
  type Msg = VersioningAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VersioningAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VersioningAnnotationMut<'_> {
  type Msg = VersioningAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VersioningAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VersioningAnnotationView<'_> {
  type Msg = VersioningAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VersioningAnnotation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for VersioningAnnotationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}




