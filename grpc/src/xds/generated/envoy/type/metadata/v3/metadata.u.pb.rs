const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__metadata__v3__MetadataKey_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MetadataKey {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MetadataKey>
}

impl ::protobuf::Message for MetadataKey {
  type MessageView<'msg> = MetadataKeyView<'msg>;
  type MessageMut<'msg> = MetadataKeyMut<'msg>;
}

impl ::std::default::Default for MetadataKey {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MetadataKey {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MetadataKey` is `Sync` because it does not implement interior mutability.
//    Neither does `MetadataKeyMut`.
unsafe impl ::std::marker::Sync for MetadataKey {}

// SAFETY:
// - `MetadataKey` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MetadataKey {}

impl ::protobuf::Proxied for MetadataKey {
  type View<'msg> = MetadataKeyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MetadataKey {}

impl ::protobuf::MutProxied for MetadataKey {
  type Mut<'msg> = MetadataKeyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MetadataKeyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataKey>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataKeyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MetadataKeyView<'msg> {
  type Message = MetadataKey;
}

impl ::std::fmt::Debug for MetadataKeyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MetadataKeyView<'_> {
  fn default() -> MetadataKeyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataKey>> for MetadataKeyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataKey>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataKeyView<'msg> {

  pub fn to_owned(&self) -> MetadataKey {
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

  // path: repeated message envoy.type.metadata.v3.MetadataKey.PathSegment
  pub fn path(self) -> ::protobuf::RepeatedView<'msg, super::metadata_key::PathSegment> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::metadata_key::PathSegment>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `MetadataKeyView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MetadataKeyView<'_> {}

// SAFETY:
// - `MetadataKeyView` is `Send` because while its alive a `MetadataKeyMut` cannot.
// - `MetadataKeyView` does not use thread-local data.
unsafe impl ::std::marker::Send for MetadataKeyView<'_> {}

impl<'msg> ::protobuf::AsView for MetadataKeyView<'msg> {
  type Proxied = MetadataKey;
  fn as_view(&self) -> ::protobuf::View<'msg, MetadataKey> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataKeyView<'msg> {
  fn into_view<'shorter>(self) -> MetadataKeyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataKey> for MetadataKeyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataKey {
    let mut dst = MetadataKey::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataKey> for MetadataKeyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataKey {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MetadataKey {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataKeyView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataKeyMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MetadataKeyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataKey>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataKeyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MetadataKeyMut<'msg> {
  type Message = MetadataKey;
}

impl ::std::fmt::Debug for MetadataKeyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataKey>> for MetadataKeyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataKey>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataKeyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataKey> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MetadataKey {
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

  // path: repeated message envoy.type.metadata.v3.MetadataKey.PathSegment
  pub fn path(&self) -> ::protobuf::RepeatedView<'_, super::metadata_key::PathSegment> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::metadata_key::PathSegment>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn path_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::metadata_key::PathSegment> {
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
  pub fn set_path(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::metadata_key::PathSegment>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `MetadataKeyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MetadataKeyMut<'_> {}

// SAFETY:
// - `MetadataKeyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MetadataKeyMut<'_> {}

impl<'msg> ::protobuf::AsView for MetadataKeyMut<'msg> {
  type Proxied = MetadataKey;
  fn as_view(&self) -> ::protobuf::View<'_, MetadataKey> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataKeyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MetadataKey>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MetadataKeyMut<'msg> {
  type MutProxied = MetadataKey;
  fn as_mut(&mut self) -> MetadataKeyMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MetadataKeyMut<'msg> {
  fn into_mut<'shorter>(self) -> MetadataKeyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MetadataKey {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MetadataKey> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MetadataKeyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MetadataKeyMut<'_> {
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

  // path: repeated message envoy.type.metadata.v3.MetadataKey.PathSegment
  pub fn path(&self) -> ::protobuf::RepeatedView<'_, super::metadata_key::PathSegment> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::metadata_key::PathSegment>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn path_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::metadata_key::PathSegment> {
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
  pub fn set_path(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::metadata_key::PathSegment>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl MetadataKey

impl ::std::ops::Drop for MetadataKey {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MetadataKey {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MetadataKey {
  type Proxied = Self;
  fn as_view(&self) -> MetadataKeyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MetadataKey {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MetadataKeyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MetadataKey {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__metadata__v3__MetadataKey_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__metadata__v3__MetadataKey_msg_init.0, &[<super::metadata_key::PathSegment as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__metadata__v3__MetadataKey_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataKey {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataKey {
  type Msg = MetadataKey;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataKey> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataKey {
  type Msg = MetadataKey;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataKey> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataKeyMut<'_> {
  type Msg = MetadataKey;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataKey> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataKeyMut<'_> {
  type Msg = MetadataKey;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataKey> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataKeyView<'_> {
  type Msg = MetadataKey;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataKey> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataKeyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod metadata_key {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__metadata__v3__MetadataKey__PathSegment_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PathSegment {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PathSegment>
}

impl ::protobuf::Message for PathSegment {
  type MessageView<'msg> = PathSegmentView<'msg>;
  type MessageMut<'msg> = PathSegmentMut<'msg>;
}

impl ::std::default::Default for PathSegment {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PathSegment {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PathSegment` is `Sync` because it does not implement interior mutability.
//    Neither does `PathSegmentMut`.
unsafe impl ::std::marker::Sync for PathSegment {}

// SAFETY:
// - `PathSegment` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PathSegment {}

impl ::protobuf::Proxied for PathSegment {
  type View<'msg> = PathSegmentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PathSegment {}

impl ::protobuf::MutProxied for PathSegment {
  type Mut<'msg> = PathSegmentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PathSegmentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathSegment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathSegmentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PathSegmentView<'msg> {
  type Message = PathSegment;
}

impl ::std::fmt::Debug for PathSegmentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PathSegmentView<'_> {
  fn default() -> PathSegmentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PathSegment>> for PathSegmentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathSegment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathSegmentView<'msg> {

  pub fn to_owned(&self) -> PathSegment {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key: optional string
  pub fn has_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn key_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_key().then(|| self.key())
  }
  pub fn key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn segment(self) -> super::super::metadata_key::path_segment::SegmentOneof<'msg> {
    match self.segment_case() {
      super::super::metadata_key::path_segment::SegmentCase::Key =>
          super::super::metadata_key::path_segment::SegmentOneof::Key(self.key()),
      _ => super::super::metadata_key::path_segment::SegmentOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn segment_case(self) -> super::super::metadata_key::path_segment::SegmentCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::metadata_key::path_segment::SegmentCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PathSegmentView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PathSegmentView<'_> {}

// SAFETY:
// - `PathSegmentView` is `Send` because while its alive a `PathSegmentMut` cannot.
// - `PathSegmentView` does not use thread-local data.
unsafe impl ::std::marker::Send for PathSegmentView<'_> {}

impl<'msg> ::protobuf::AsView for PathSegmentView<'msg> {
  type Proxied = PathSegment;
  fn as_view(&self) -> ::protobuf::View<'msg, PathSegment> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathSegmentView<'msg> {
  fn into_view<'shorter>(self) -> PathSegmentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PathSegment> for PathSegmentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathSegment {
    let mut dst = PathSegment::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PathSegment> for PathSegmentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathSegment {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PathSegment {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathSegmentView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathSegmentMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PathSegmentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathSegment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathSegmentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PathSegmentMut<'msg> {
  type Message = PathSegment;
}

impl ::std::fmt::Debug for PathSegmentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PathSegment>> for PathSegmentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathSegment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathSegmentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PathSegment> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PathSegment {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key: optional string
  pub fn has_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_key().then(|| self.key())
  }
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

  pub fn segment(&self) -> super::super::metadata_key::path_segment::SegmentOneof<'_> {
    match &self.segment_case() {
      super::super::metadata_key::path_segment::SegmentCase::Key =>
          super::super::metadata_key::path_segment::SegmentOneof::Key(self.key()),
      _ => super::super::metadata_key::path_segment::SegmentOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn segment_case(&self) -> super::super::metadata_key::path_segment::SegmentCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::metadata_key::path_segment::SegmentCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PathSegmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PathSegmentMut<'_> {}

// SAFETY:
// - `PathSegmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PathSegmentMut<'_> {}

impl<'msg> ::protobuf::AsView for PathSegmentMut<'msg> {
  type Proxied = PathSegment;
  fn as_view(&self) -> ::protobuf::View<'_, PathSegment> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathSegmentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PathSegment>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PathSegmentMut<'msg> {
  type MutProxied = PathSegment;
  fn as_mut(&mut self) -> PathSegmentMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PathSegmentMut<'msg> {
  fn into_mut<'shorter>(self) -> PathSegmentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PathSegment {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PathSegment> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PathSegmentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PathSegmentMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key: optional string
  pub fn has_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_key().then(|| self.key())
  }
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

  pub fn segment(&self) -> super::super::metadata_key::path_segment::SegmentOneof<'_> {
    match &self.segment_case() {
      super::super::metadata_key::path_segment::SegmentCase::Key =>
          super::super::metadata_key::path_segment::SegmentOneof::Key(self.key()),
      _ => super::super::metadata_key::path_segment::SegmentOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn segment_case(&self) -> super::super::metadata_key::path_segment::SegmentCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::metadata_key::path_segment::SegmentCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl PathSegment

impl ::std::ops::Drop for PathSegment {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PathSegment {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PathSegment {
  type Proxied = Self;
  fn as_view(&self) -> PathSegmentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PathSegment {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PathSegmentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PathSegment {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::metadata_key::envoy__type__metadata__v3__MetadataKey__PathSegment_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1^!");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::metadata_key::envoy__type__metadata__v3__MetadataKey__PathSegment_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::metadata_key::envoy__type__metadata__v3__MetadataKey__PathSegment_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathSegment {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathSegment {
  type Msg = PathSegment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathSegment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathSegment {
  type Msg = PathSegment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathSegment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathSegmentMut<'_> {
  type Msg = PathSegment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathSegment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathSegmentMut<'_> {
  type Msg = PathSegment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathSegment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathSegmentView<'_> {
  type Msg = PathSegment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathSegment> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathSegmentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod path_segment {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum SegmentOneof<'msg> {
  Key(&'msg ::protobuf::ProtoStr) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum SegmentCase {
  Key = 1,

  not_set = 0
}

impl SegmentCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<SegmentCase> {
    match v {
      0 => Some(SegmentCase::not_set),
      1 => Some(SegmentCase::Key),
      _ => None
    }
  }
}
}  // pub mod path_segment


}  // pub mod metadata_key


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__metadata__v3__MetadataKind_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MetadataKind {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MetadataKind>
}

impl ::protobuf::Message for MetadataKind {
  type MessageView<'msg> = MetadataKindView<'msg>;
  type MessageMut<'msg> = MetadataKindMut<'msg>;
}

impl ::std::default::Default for MetadataKind {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MetadataKind {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MetadataKind` is `Sync` because it does not implement interior mutability.
//    Neither does `MetadataKindMut`.
unsafe impl ::std::marker::Sync for MetadataKind {}

// SAFETY:
// - `MetadataKind` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MetadataKind {}

impl ::protobuf::Proxied for MetadataKind {
  type View<'msg> = MetadataKindView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MetadataKind {}

impl ::protobuf::MutProxied for MetadataKind {
  type Mut<'msg> = MetadataKindMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MetadataKindView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataKind>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataKindView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MetadataKindView<'msg> {
  type Message = MetadataKind;
}

impl ::std::fmt::Debug for MetadataKindView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MetadataKindView<'_> {
  fn default() -> MetadataKindView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataKind>> for MetadataKindView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataKind>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataKindView<'msg> {

  pub fn to_owned(&self) -> MetadataKind {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // request: optional message envoy.type.metadata.v3.MetadataKind.Request
  pub fn has_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn request_opt(self) -> ::std::option::Option<super::metadata_kind::RequestView<'msg>> {
    self.has_request().then(|| self.request())
  }
  pub fn request(self) -> super::metadata_kind::RequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::RequestView::default())
  }

  // route: optional message envoy.type.metadata.v3.MetadataKind.Route
  pub fn has_route(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn route_opt(self) -> ::std::option::Option<super::metadata_kind::RouteView<'msg>> {
    self.has_route().then(|| self.route())
  }
  pub fn route(self) -> super::metadata_kind::RouteView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::RouteView::default())
  }

  // cluster: optional message envoy.type.metadata.v3.MetadataKind.Cluster
  pub fn has_cluster(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn cluster_opt(self) -> ::std::option::Option<super::metadata_kind::ClusterView<'msg>> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(self) -> super::metadata_kind::ClusterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::ClusterView::default())
  }

  // host: optional message envoy.type.metadata.v3.MetadataKind.Host
  pub fn has_host(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn host_opt(self) -> ::std::option::Option<super::metadata_kind::HostView<'msg>> {
    self.has_host().then(|| self.host())
  }
  pub fn host(self) -> super::metadata_kind::HostView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::HostView::default())
  }

  pub fn kind(self) -> super::metadata_kind::KindOneof<'msg> {
    match self.kind_case() {
      super::metadata_kind::KindCase::Request =>
          super::metadata_kind::KindOneof::Request(self.request()),
      super::metadata_kind::KindCase::Route =>
          super::metadata_kind::KindOneof::Route(self.route()),
      super::metadata_kind::KindCase::Cluster =>
          super::metadata_kind::KindOneof::Cluster(self.cluster()),
      super::metadata_kind::KindCase::Host =>
          super::metadata_kind::KindOneof::Host(self.host()),
      _ => super::metadata_kind::KindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn kind_case(self) -> super::metadata_kind::KindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::metadata_kind::KindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `MetadataKindView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MetadataKindView<'_> {}

// SAFETY:
// - `MetadataKindView` is `Send` because while its alive a `MetadataKindMut` cannot.
// - `MetadataKindView` does not use thread-local data.
unsafe impl ::std::marker::Send for MetadataKindView<'_> {}

impl<'msg> ::protobuf::AsView for MetadataKindView<'msg> {
  type Proxied = MetadataKind;
  fn as_view(&self) -> ::protobuf::View<'msg, MetadataKind> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataKindView<'msg> {
  fn into_view<'shorter>(self) -> MetadataKindView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataKind> for MetadataKindView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataKind {
    let mut dst = MetadataKind::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataKind> for MetadataKindMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataKind {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MetadataKind {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataKindView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataKindMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MetadataKindMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataKind>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataKindMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MetadataKindMut<'msg> {
  type Message = MetadataKind;
}

impl ::std::fmt::Debug for MetadataKindMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataKind>> for MetadataKindMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataKind>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataKindMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataKind> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MetadataKind {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // request: optional message envoy.type.metadata.v3.MetadataKind.Request
  pub fn has_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn request_opt(&self) -> ::std::option::Option<super::metadata_kind::RequestView<'_>> {
    self.has_request().then(|| self.request())
  }
  pub fn request(&self) -> super::metadata_kind::RequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::RequestView::default())
  }
  pub fn request_mut(&mut self) -> super::metadata_kind::RequestMut<'_> {
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
  pub fn set_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_kind::Request>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // route: optional message envoy.type.metadata.v3.MetadataKind.Route
  pub fn has_route(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_route(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn route_opt(&self) -> ::std::option::Option<super::metadata_kind::RouteView<'_>> {
    self.has_route().then(|| self.route())
  }
  pub fn route(&self) -> super::metadata_kind::RouteView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::RouteView::default())
  }
  pub fn route_mut(&mut self) -> super::metadata_kind::RouteMut<'_> {
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
  pub fn set_route(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_kind::Route>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cluster: optional message envoy.type.metadata.v3.MetadataKind.Cluster
  pub fn has_cluster(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cluster(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cluster_opt(&self) -> ::std::option::Option<super::metadata_kind::ClusterView<'_>> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(&self) -> super::metadata_kind::ClusterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::ClusterView::default())
  }
  pub fn cluster_mut(&mut self) -> super::metadata_kind::ClusterMut<'_> {
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
  pub fn set_cluster(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_kind::Cluster>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // host: optional message envoy.type.metadata.v3.MetadataKind.Host
  pub fn has_host(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_host(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn host_opt(&self) -> ::std::option::Option<super::metadata_kind::HostView<'_>> {
    self.has_host().then(|| self.host())
  }
  pub fn host(&self) -> super::metadata_kind::HostView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::HostView::default())
  }
  pub fn host_mut(&mut self) -> super::metadata_kind::HostMut<'_> {
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
  pub fn set_host(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_kind::Host>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn kind(&self) -> super::metadata_kind::KindOneof<'_> {
    match &self.kind_case() {
      super::metadata_kind::KindCase::Request =>
          super::metadata_kind::KindOneof::Request(self.request()),
      super::metadata_kind::KindCase::Route =>
          super::metadata_kind::KindOneof::Route(self.route()),
      super::metadata_kind::KindCase::Cluster =>
          super::metadata_kind::KindOneof::Cluster(self.cluster()),
      super::metadata_kind::KindCase::Host =>
          super::metadata_kind::KindOneof::Host(self.host()),
      _ => super::metadata_kind::KindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn kind_case(&self) -> super::metadata_kind::KindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::metadata_kind::KindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `MetadataKindMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MetadataKindMut<'_> {}

// SAFETY:
// - `MetadataKindMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MetadataKindMut<'_> {}

impl<'msg> ::protobuf::AsView for MetadataKindMut<'msg> {
  type Proxied = MetadataKind;
  fn as_view(&self) -> ::protobuf::View<'_, MetadataKind> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataKindMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MetadataKind>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MetadataKindMut<'msg> {
  type MutProxied = MetadataKind;
  fn as_mut(&mut self) -> MetadataKindMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MetadataKindMut<'msg> {
  fn into_mut<'shorter>(self) -> MetadataKindMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MetadataKind {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MetadataKind> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MetadataKindView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MetadataKindMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // request: optional message envoy.type.metadata.v3.MetadataKind.Request
  pub fn has_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn request_opt(&self) -> ::std::option::Option<super::metadata_kind::RequestView<'_>> {
    self.has_request().then(|| self.request())
  }
  pub fn request(&self) -> super::metadata_kind::RequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::RequestView::default())
  }
  pub fn request_mut(&mut self) -> super::metadata_kind::RequestMut<'_> {
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
  pub fn set_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_kind::Request>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // route: optional message envoy.type.metadata.v3.MetadataKind.Route
  pub fn has_route(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_route(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn route_opt(&self) -> ::std::option::Option<super::metadata_kind::RouteView<'_>> {
    self.has_route().then(|| self.route())
  }
  pub fn route(&self) -> super::metadata_kind::RouteView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::RouteView::default())
  }
  pub fn route_mut(&mut self) -> super::metadata_kind::RouteMut<'_> {
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
  pub fn set_route(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_kind::Route>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cluster: optional message envoy.type.metadata.v3.MetadataKind.Cluster
  pub fn has_cluster(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cluster(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cluster_opt(&self) -> ::std::option::Option<super::metadata_kind::ClusterView<'_>> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(&self) -> super::metadata_kind::ClusterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::ClusterView::default())
  }
  pub fn cluster_mut(&mut self) -> super::metadata_kind::ClusterMut<'_> {
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
  pub fn set_cluster(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_kind::Cluster>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // host: optional message envoy.type.metadata.v3.MetadataKind.Host
  pub fn has_host(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_host(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn host_opt(&self) -> ::std::option::Option<super::metadata_kind::HostView<'_>> {
    self.has_host().then(|| self.host())
  }
  pub fn host(&self) -> super::metadata_kind::HostView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::metadata_kind::HostView::default())
  }
  pub fn host_mut(&mut self) -> super::metadata_kind::HostMut<'_> {
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
  pub fn set_host(&mut self,
    val: impl ::protobuf::IntoProxied<super::metadata_kind::Host>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn kind(&self) -> super::metadata_kind::KindOneof<'_> {
    match &self.kind_case() {
      super::metadata_kind::KindCase::Request =>
          super::metadata_kind::KindOneof::Request(self.request()),
      super::metadata_kind::KindCase::Route =>
          super::metadata_kind::KindOneof::Route(self.route()),
      super::metadata_kind::KindCase::Cluster =>
          super::metadata_kind::KindOneof::Cluster(self.cluster()),
      super::metadata_kind::KindCase::Host =>
          super::metadata_kind::KindOneof::Host(self.host()),
      _ => super::metadata_kind::KindOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn kind_case(&self) -> super::metadata_kind::KindCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::metadata_kind::KindCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl MetadataKind

impl ::std::ops::Drop for MetadataKind {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MetadataKind {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MetadataKind {
  type Proxied = Self;
  fn as_view(&self) -> MetadataKindView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MetadataKind {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MetadataKindMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MetadataKind {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__metadata__v3__MetadataKind_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333^!|#|$|%");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__metadata__v3__MetadataKind_msg_init.0, &[<super::metadata_kind::Request as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::metadata_kind::Route as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::metadata_kind::Cluster as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::metadata_kind::Host as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__metadata__v3__MetadataKind_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataKind {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataKind {
  type Msg = MetadataKind;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataKind> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataKind {
  type Msg = MetadataKind;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataKind> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataKindMut<'_> {
  type Msg = MetadataKind;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataKind> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataKindMut<'_> {
  type Msg = MetadataKind;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataKind> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataKindView<'_> {
  type Msg = MetadataKind;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataKind> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataKindMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod metadata_kind {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__metadata__v3__MetadataKind__Request_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Request {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Request>
}

impl ::protobuf::Message for Request {
  type MessageView<'msg> = RequestView<'msg>;
  type MessageMut<'msg> = RequestMut<'msg>;
}

impl ::std::default::Default for Request {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Request {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Request` is `Sync` because it does not implement interior mutability.
//    Neither does `RequestMut`.
unsafe impl ::std::marker::Sync for Request {}

// SAFETY:
// - `Request` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Request {}

impl ::protobuf::Proxied for Request {
  type View<'msg> = RequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Request {}

impl ::protobuf::MutProxied for Request {
  type Mut<'msg> = RequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Request>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RequestView<'msg> {
  type Message = Request;
}

impl ::std::fmt::Debug for RequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RequestView<'_> {
  fn default() -> RequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Request>> for RequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Request>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RequestView<'msg> {

  pub fn to_owned(&self) -> Request {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `RequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RequestView<'_> {}

// SAFETY:
// - `RequestView` is `Send` because while its alive a `RequestMut` cannot.
// - `RequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for RequestView<'_> {}

impl<'msg> ::protobuf::AsView for RequestView<'msg> {
  type Proxied = Request;
  fn as_view(&self) -> ::protobuf::View<'msg, Request> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RequestView<'msg> {
  fn into_view<'shorter>(self) -> RequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Request> for RequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Request {
    let mut dst = Request::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Request> for RequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Request {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Request {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Request>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RequestMut<'msg> {
  type Message = Request;
}

impl ::std::fmt::Debug for RequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Request>> for RequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Request>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Request> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Request {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `RequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RequestMut<'_> {}

// SAFETY:
// - `RequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RequestMut<'_> {}

impl<'msg> ::protobuf::AsView for RequestMut<'msg> {
  type Proxied = Request;
  fn as_view(&self) -> ::protobuf::View<'_, Request> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Request>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RequestMut<'msg> {
  type MutProxied = Request;
  fn as_mut(&mut self) -> RequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RequestMut<'msg> {
  fn into_mut<'shorter>(self) -> RequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Request {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Request> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Request

impl ::std::ops::Drop for Request {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Request {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Request {
  type Proxied = Self;
  fn as_view(&self) -> RequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Request {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Request {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Request_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Request_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Request_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Request {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Request {
  type Msg = Request;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Request> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Request {
  type Msg = Request;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Request> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RequestMut<'_> {
  type Msg = Request;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Request> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RequestMut<'_> {
  type Msg = Request;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Request> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RequestView<'_> {
  type Msg = Request;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Request> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__metadata__v3__MetadataKind__Route_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Route {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Route>
}

impl ::protobuf::Message for Route {
  type MessageView<'msg> = RouteView<'msg>;
  type MessageMut<'msg> = RouteMut<'msg>;
}

impl ::std::default::Default for Route {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Route {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Route` is `Sync` because it does not implement interior mutability.
//    Neither does `RouteMut`.
unsafe impl ::std::marker::Sync for Route {}

// SAFETY:
// - `Route` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Route {}

impl ::protobuf::Proxied for Route {
  type View<'msg> = RouteView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Route {}

impl ::protobuf::MutProxied for Route {
  type Mut<'msg> = RouteMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RouteView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Route>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RouteView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RouteView<'msg> {
  type Message = Route;
}

impl ::std::fmt::Debug for RouteView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RouteView<'_> {
  fn default() -> RouteView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Route>> for RouteView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Route>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RouteView<'msg> {

  pub fn to_owned(&self) -> Route {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `RouteView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RouteView<'_> {}

// SAFETY:
// - `RouteView` is `Send` because while its alive a `RouteMut` cannot.
// - `RouteView` does not use thread-local data.
unsafe impl ::std::marker::Send for RouteView<'_> {}

impl<'msg> ::protobuf::AsView for RouteView<'msg> {
  type Proxied = Route;
  fn as_view(&self) -> ::protobuf::View<'msg, Route> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RouteView<'msg> {
  fn into_view<'shorter>(self) -> RouteView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Route> for RouteView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Route {
    let mut dst = Route::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Route> for RouteMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Route {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Route {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RouteView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RouteMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RouteMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Route>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RouteMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RouteMut<'msg> {
  type Message = Route;
}

impl ::std::fmt::Debug for RouteMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Route>> for RouteMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Route>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RouteMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Route> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Route {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `RouteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RouteMut<'_> {}

// SAFETY:
// - `RouteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RouteMut<'_> {}

impl<'msg> ::protobuf::AsView for RouteMut<'msg> {
  type Proxied = Route;
  fn as_view(&self) -> ::protobuf::View<'_, Route> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RouteMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Route>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RouteMut<'msg> {
  type MutProxied = Route;
  fn as_mut(&mut self) -> RouteMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RouteMut<'msg> {
  fn into_mut<'shorter>(self) -> RouteMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Route {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Route> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RouteView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RouteMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Route

impl ::std::ops::Drop for Route {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Route {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Route {
  type Proxied = Self;
  fn as_view(&self) -> RouteView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Route {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RouteMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Route {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Route_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Route_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Route_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Route {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Route {
  type Msg = Route;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Route> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Route {
  type Msg = Route;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Route> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RouteMut<'_> {
  type Msg = Route;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Route> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouteMut<'_> {
  type Msg = Route;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Route> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouteView<'_> {
  type Msg = Route;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Route> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RouteMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__metadata__v3__MetadataKind__Cluster_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Cluster {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Cluster>
}

impl ::protobuf::Message for Cluster {
  type MessageView<'msg> = ClusterView<'msg>;
  type MessageMut<'msg> = ClusterMut<'msg>;
}

impl ::std::default::Default for Cluster {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Cluster {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Cluster` is `Sync` because it does not implement interior mutability.
//    Neither does `ClusterMut`.
unsafe impl ::std::marker::Sync for Cluster {}

// SAFETY:
// - `Cluster` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Cluster {}

impl ::protobuf::Proxied for Cluster {
  type View<'msg> = ClusterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Cluster {}

impl ::protobuf::MutProxied for Cluster {
  type Mut<'msg> = ClusterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClusterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Cluster>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClusterView<'msg> {
  type Message = Cluster;
}

impl ::std::fmt::Debug for ClusterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClusterView<'_> {
  fn default() -> ClusterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Cluster>> for ClusterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Cluster>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterView<'msg> {

  pub fn to_owned(&self) -> Cluster {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ClusterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClusterView<'_> {}

// SAFETY:
// - `ClusterView` is `Send` because while its alive a `ClusterMut` cannot.
// - `ClusterView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClusterView<'_> {}

impl<'msg> ::protobuf::AsView for ClusterView<'msg> {
  type Proxied = Cluster;
  fn as_view(&self) -> ::protobuf::View<'msg, Cluster> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterView<'msg> {
  fn into_view<'shorter>(self) -> ClusterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Cluster> for ClusterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Cluster {
    let mut dst = Cluster::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Cluster> for ClusterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Cluster {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Cluster {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClusterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Cluster>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClusterMut<'msg> {
  type Message = Cluster;
}

impl ::std::fmt::Debug for ClusterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Cluster>> for ClusterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Cluster>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Cluster> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Cluster {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ClusterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClusterMut<'_> {}

// SAFETY:
// - `ClusterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClusterMut<'_> {}

impl<'msg> ::protobuf::AsView for ClusterMut<'msg> {
  type Proxied = Cluster;
  fn as_view(&self) -> ::protobuf::View<'_, Cluster> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Cluster>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClusterMut<'msg> {
  type MutProxied = Cluster;
  fn as_mut(&mut self) -> ClusterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClusterMut<'msg> {
  fn into_mut<'shorter>(self) -> ClusterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Cluster {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Cluster> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClusterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClusterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Cluster

impl ::std::ops::Drop for Cluster {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Cluster {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Cluster {
  type Proxied = Self;
  fn as_view(&self) -> ClusterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Cluster {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClusterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Cluster {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Cluster_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Cluster_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Cluster_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Cluster {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Cluster {
  type Msg = Cluster;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Cluster> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Cluster {
  type Msg = Cluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Cluster> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterMut<'_> {
  type Msg = Cluster;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Cluster> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterMut<'_> {
  type Msg = Cluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Cluster> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterView<'_> {
  type Msg = Cluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Cluster> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__metadata__v3__MetadataKind__Host_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Host {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Host>
}

impl ::protobuf::Message for Host {
  type MessageView<'msg> = HostView<'msg>;
  type MessageMut<'msg> = HostMut<'msg>;
}

impl ::std::default::Default for Host {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Host {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Host` is `Sync` because it does not implement interior mutability.
//    Neither does `HostMut`.
unsafe impl ::std::marker::Sync for Host {}

// SAFETY:
// - `Host` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Host {}

impl ::protobuf::Proxied for Host {
  type View<'msg> = HostView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Host {}

impl ::protobuf::MutProxied for Host {
  type Mut<'msg> = HostMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HostView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Host>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HostView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HostView<'msg> {
  type Message = Host;
}

impl ::std::fmt::Debug for HostView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HostView<'_> {
  fn default() -> HostView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Host>> for HostView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Host>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HostView<'msg> {

  pub fn to_owned(&self) -> Host {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `HostView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HostView<'_> {}

// SAFETY:
// - `HostView` is `Send` because while its alive a `HostMut` cannot.
// - `HostView` does not use thread-local data.
unsafe impl ::std::marker::Send for HostView<'_> {}

impl<'msg> ::protobuf::AsView for HostView<'msg> {
  type Proxied = Host;
  fn as_view(&self) -> ::protobuf::View<'msg, Host> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HostView<'msg> {
  fn into_view<'shorter>(self) -> HostView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Host> for HostView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Host {
    let mut dst = Host::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Host> for HostMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Host {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Host {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HostView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HostMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HostMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Host>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HostMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HostMut<'msg> {
  type Message = Host;
}

impl ::std::fmt::Debug for HostMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Host>> for HostMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Host>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HostMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Host> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Host {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `HostMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HostMut<'_> {}

// SAFETY:
// - `HostMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HostMut<'_> {}

impl<'msg> ::protobuf::AsView for HostMut<'msg> {
  type Proxied = Host;
  fn as_view(&self) -> ::protobuf::View<'_, Host> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HostMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Host>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HostMut<'msg> {
  type MutProxied = Host;
  fn as_mut(&mut self) -> HostMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HostMut<'msg> {
  fn into_mut<'shorter>(self) -> HostMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Host {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Host> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HostView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HostMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl Host

impl ::std::ops::Drop for Host {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Host {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Host {
  type Proxied = Self;
  fn as_view(&self) -> HostView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Host {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HostMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Host {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Host_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Host_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::metadata_kind::envoy__type__metadata__v3__MetadataKind__Host_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Host {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Host {
  type Msg = Host;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Host> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Host {
  type Msg = Host;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Host> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HostMut<'_> {
  type Msg = Host;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Host> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HostMut<'_> {
  type Msg = Host;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Host> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HostView<'_> {
  type Msg = Host;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Host> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HostMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum KindOneof<'msg> {
  Request(::protobuf::View<'msg, super::super::metadata_kind::Request>) = 1,
  Route(::protobuf::View<'msg, super::super::metadata_kind::Route>) = 2,
  Cluster(::protobuf::View<'msg, super::super::metadata_kind::Cluster>) = 3,
  Host(::protobuf::View<'msg, super::super::metadata_kind::Host>) = 4,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum KindCase {
  Request = 1,
  Route = 2,
  Cluster = 3,
  Host = 4,

  not_set = 0
}

impl KindCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<KindCase> {
    match v {
      0 => Some(KindCase::not_set),
      1 => Some(KindCase::Request),
      2 => Some(KindCase::Route),
      3 => Some(KindCase::Cluster),
      4 => Some(KindCase::Host),
      _ => None
    }
  }
}
}  // pub mod metadata_kind


