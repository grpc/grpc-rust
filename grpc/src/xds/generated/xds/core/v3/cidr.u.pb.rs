const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__core__v3__CidrRange_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CidrRange {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CidrRange>
}

impl ::protobuf::Message for CidrRange {
  type MessageView<'msg> = CidrRangeView<'msg>;
  type MessageMut<'msg> = CidrRangeMut<'msg>;
}

impl ::std::default::Default for CidrRange {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CidrRange {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CidrRange` is `Sync` because it does not implement interior mutability.
//    Neither does `CidrRangeMut`.
unsafe impl ::std::marker::Sync for CidrRange {}

// SAFETY:
// - `CidrRange` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CidrRange {}

impl ::protobuf::Proxied for CidrRange {
  type View<'msg> = CidrRangeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CidrRange {}

impl ::protobuf::MutProxied for CidrRange {
  type Mut<'msg> = CidrRangeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CidrRangeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CidrRange>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CidrRangeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CidrRangeView<'msg> {
  type Message = CidrRange;
}

impl ::std::fmt::Debug for CidrRangeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CidrRangeView<'_> {
  fn default() -> CidrRangeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CidrRange>> for CidrRangeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CidrRange>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CidrRangeView<'msg> {

  pub fn to_owned(&self) -> CidrRange {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // address_prefix: optional string
  pub fn address_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // prefix_len: optional message google.protobuf.UInt32Value
  pub fn has_prefix_len(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn prefix_len_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_prefix_len().then(|| self.prefix_len())
  }
  pub fn prefix_len(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `CidrRangeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CidrRangeView<'_> {}

// SAFETY:
// - `CidrRangeView` is `Send` because while its alive a `CidrRangeMut` cannot.
// - `CidrRangeView` does not use thread-local data.
unsafe impl ::std::marker::Send for CidrRangeView<'_> {}

impl<'msg> ::protobuf::AsView for CidrRangeView<'msg> {
  type Proxied = CidrRange;
  fn as_view(&self) -> ::protobuf::View<'msg, CidrRange> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CidrRangeView<'msg> {
  fn into_view<'shorter>(self) -> CidrRangeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CidrRange> for CidrRangeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CidrRange {
    let mut dst = CidrRange::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CidrRange> for CidrRangeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CidrRange {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CidrRange {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CidrRangeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CidrRangeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CidrRangeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CidrRange>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CidrRangeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CidrRangeMut<'msg> {
  type Message = CidrRange;
}

impl ::std::fmt::Debug for CidrRangeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CidrRange>> for CidrRangeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CidrRange>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CidrRangeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CidrRange> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CidrRange {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // address_prefix: optional string
  pub fn address_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_address_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // prefix_len: optional message google.protobuf.UInt32Value
  pub fn has_prefix_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_prefix_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn prefix_len_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_prefix_len().then(|| self.prefix_len())
  }
  pub fn prefix_len(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn prefix_len_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_prefix_len(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

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
// - `CidrRangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CidrRangeMut<'_> {}

// SAFETY:
// - `CidrRangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CidrRangeMut<'_> {}

impl<'msg> ::protobuf::AsView for CidrRangeMut<'msg> {
  type Proxied = CidrRange;
  fn as_view(&self) -> ::protobuf::View<'_, CidrRange> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CidrRangeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CidrRange>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CidrRangeMut<'msg> {
  type MutProxied = CidrRange;
  fn as_mut(&mut self) -> CidrRangeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CidrRangeMut<'msg> {
  fn into_mut<'shorter>(self) -> CidrRangeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CidrRange {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CidrRange> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CidrRangeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CidrRangeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // address_prefix: optional string
  pub fn address_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_address_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // prefix_len: optional message google.protobuf.UInt32Value
  pub fn has_prefix_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_prefix_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn prefix_len_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_prefix_len().then(|| self.prefix_len())
  }
  pub fn prefix_len(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn prefix_len_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_prefix_len(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl CidrRange

impl ::std::ops::Drop for CidrRange {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CidrRange {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CidrRange {
  type Proxied = Self;
  fn as_view(&self) -> CidrRangeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CidrRange {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CidrRangeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CidrRange {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__core__v3__CidrRange_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__core__v3__CidrRange_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__core__v3__CidrRange_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CidrRange {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CidrRange {
  type Msg = CidrRange;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CidrRange> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CidrRange {
  type Msg = CidrRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CidrRange> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CidrRangeMut<'_> {
  type Msg = CidrRange;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CidrRange> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CidrRangeMut<'_> {
  type Msg = CidrRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CidrRange> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CidrRangeView<'_> {
  type Msg = CidrRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CidrRange> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CidrRangeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



