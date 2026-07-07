const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__v3__Int64Range_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Int64Range {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Int64Range>
}

impl ::protobuf::Message for Int64Range {
  type MessageView<'msg> = Int64RangeView<'msg>;
  type MessageMut<'msg> = Int64RangeMut<'msg>;
}

impl ::std::default::Default for Int64Range {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Int64Range {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Int64Range` is `Sync` because it does not implement interior mutability.
//    Neither does `Int64RangeMut`.
unsafe impl ::std::marker::Sync for Int64Range {}

// SAFETY:
// - `Int64Range` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Int64Range {}

impl ::protobuf::Proxied for Int64Range {
  type View<'msg> = Int64RangeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Int64Range {}

impl ::protobuf::MutProxied for Int64Range {
  type Mut<'msg> = Int64RangeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Int64RangeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Int64Range>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Int64RangeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Int64RangeView<'msg> {
  type Message = Int64Range;
}

impl ::std::fmt::Debug for Int64RangeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Int64RangeView<'_> {
  fn default() -> Int64RangeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Int64Range>> for Int64RangeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Int64Range>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Int64RangeView<'msg> {

  pub fn to_owned(&self) -> Int64Range {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // start: optional int64
  pub fn start(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // end: optional int64
  pub fn end(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `Int64RangeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Int64RangeView<'_> {}

// SAFETY:
// - `Int64RangeView` is `Send` because while its alive a `Int64RangeMut` cannot.
// - `Int64RangeView` does not use thread-local data.
unsafe impl ::std::marker::Send for Int64RangeView<'_> {}

impl<'msg> ::protobuf::AsView for Int64RangeView<'msg> {
  type Proxied = Int64Range;
  fn as_view(&self) -> ::protobuf::View<'msg, Int64Range> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Int64RangeView<'msg> {
  fn into_view<'shorter>(self) -> Int64RangeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Int64Range> for Int64RangeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Int64Range {
    let mut dst = Int64Range::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Int64Range> for Int64RangeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Int64Range {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Int64Range {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Int64RangeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Int64RangeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Int64RangeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Int64Range>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Int64RangeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Int64RangeMut<'msg> {
  type Message = Int64Range;
}

impl ::std::fmt::Debug for Int64RangeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Int64Range>> for Int64RangeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Int64Range>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Int64RangeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Int64Range> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Int64Range {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // start: optional int64
  pub fn start(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // end: optional int64
  pub fn end(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `Int64RangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Int64RangeMut<'_> {}

// SAFETY:
// - `Int64RangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Int64RangeMut<'_> {}

impl<'msg> ::protobuf::AsView for Int64RangeMut<'msg> {
  type Proxied = Int64Range;
  fn as_view(&self) -> ::protobuf::View<'_, Int64Range> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Int64RangeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Int64Range>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Int64RangeMut<'msg> {
  type MutProxied = Int64Range;
  fn as_mut(&mut self) -> Int64RangeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Int64RangeMut<'msg> {
  fn into_mut<'shorter>(self) -> Int64RangeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Int64Range {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Int64Range> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Int64RangeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Int64RangeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // start: optional int64
  pub fn start(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

  // end: optional int64
  pub fn end(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

}  // impl Int64Range

impl ::std::ops::Drop for Int64Range {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Int64Range {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Int64Range {
  type Proxied = Self;
  fn as_view(&self) -> Int64RangeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Int64Range {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Int64RangeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Int64Range {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__v3__Int64Range_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$+P+P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__v3__Int64Range_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__v3__Int64Range_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Int64Range {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Int64Range {
  type Msg = Int64Range;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int64Range> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int64Range {
  type Msg = Int64Range;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int64Range> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Int64RangeMut<'_> {
  type Msg = Int64Range;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int64Range> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int64RangeMut<'_> {
  type Msg = Int64Range;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int64Range> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int64RangeView<'_> {
  type Msg = Int64Range;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int64Range> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Int64RangeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__v3__Int32Range_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Int32Range {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Int32Range>
}

impl ::protobuf::Message for Int32Range {
  type MessageView<'msg> = Int32RangeView<'msg>;
  type MessageMut<'msg> = Int32RangeMut<'msg>;
}

impl ::std::default::Default for Int32Range {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Int32Range {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Int32Range` is `Sync` because it does not implement interior mutability.
//    Neither does `Int32RangeMut`.
unsafe impl ::std::marker::Sync for Int32Range {}

// SAFETY:
// - `Int32Range` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Int32Range {}

impl ::protobuf::Proxied for Int32Range {
  type View<'msg> = Int32RangeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Int32Range {}

impl ::protobuf::MutProxied for Int32Range {
  type Mut<'msg> = Int32RangeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Int32RangeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Int32Range>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Int32RangeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Int32RangeView<'msg> {
  type Message = Int32Range;
}

impl ::std::fmt::Debug for Int32RangeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Int32RangeView<'_> {
  fn default() -> Int32RangeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Int32Range>> for Int32RangeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Int32Range>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Int32RangeView<'msg> {

  pub fn to_owned(&self) -> Int32Range {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // start: optional int32
  pub fn start(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // end: optional int32
  pub fn end(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `Int32RangeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Int32RangeView<'_> {}

// SAFETY:
// - `Int32RangeView` is `Send` because while its alive a `Int32RangeMut` cannot.
// - `Int32RangeView` does not use thread-local data.
unsafe impl ::std::marker::Send for Int32RangeView<'_> {}

impl<'msg> ::protobuf::AsView for Int32RangeView<'msg> {
  type Proxied = Int32Range;
  fn as_view(&self) -> ::protobuf::View<'msg, Int32Range> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Int32RangeView<'msg> {
  fn into_view<'shorter>(self) -> Int32RangeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Int32Range> for Int32RangeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Int32Range {
    let mut dst = Int32Range::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Int32Range> for Int32RangeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Int32Range {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Int32Range {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Int32RangeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Int32RangeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Int32RangeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Int32Range>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Int32RangeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Int32RangeMut<'msg> {
  type Message = Int32Range;
}

impl ::std::fmt::Debug for Int32RangeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Int32Range>> for Int32RangeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Int32Range>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Int32RangeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Int32Range> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Int32Range {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // start: optional int32
  pub fn start(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // end: optional int32
  pub fn end(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `Int32RangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Int32RangeMut<'_> {}

// SAFETY:
// - `Int32RangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Int32RangeMut<'_> {}

impl<'msg> ::protobuf::AsView for Int32RangeMut<'msg> {
  type Proxied = Int32Range;
  fn as_view(&self) -> ::protobuf::View<'_, Int32Range> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Int32RangeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Int32Range>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Int32RangeMut<'msg> {
  type MutProxied = Int32Range;
  fn as_mut(&mut self) -> Int32RangeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Int32RangeMut<'msg> {
  fn into_mut<'shorter>(self) -> Int32RangeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Int32Range {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Int32Range> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Int32RangeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Int32RangeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // start: optional int32
  pub fn start(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // end: optional int32
  pub fn end(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}  // impl Int32Range

impl ::std::ops::Drop for Int32Range {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Int32Range {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Int32Range {
  type Proxied = Self;
  fn as_view(&self) -> Int32RangeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Int32Range {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Int32RangeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Int32Range {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__v3__Int32Range_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__v3__Int32Range_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__v3__Int32Range_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Int32Range {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Int32Range {
  type Msg = Int32Range;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int32Range> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int32Range {
  type Msg = Int32Range;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int32Range> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Int32RangeMut<'_> {
  type Msg = Int32Range;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int32Range> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int32RangeMut<'_> {
  type Msg = Int32Range;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int32Range> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Int32RangeView<'_> {
  type Msg = Int32Range;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Int32Range> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Int32RangeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__v3__DoubleRange_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DoubleRange {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DoubleRange>
}

impl ::protobuf::Message for DoubleRange {
  type MessageView<'msg> = DoubleRangeView<'msg>;
  type MessageMut<'msg> = DoubleRangeMut<'msg>;
}

impl ::std::default::Default for DoubleRange {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DoubleRange {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DoubleRange` is `Sync` because it does not implement interior mutability.
//    Neither does `DoubleRangeMut`.
unsafe impl ::std::marker::Sync for DoubleRange {}

// SAFETY:
// - `DoubleRange` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DoubleRange {}

impl ::protobuf::Proxied for DoubleRange {
  type View<'msg> = DoubleRangeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DoubleRange {}

impl ::protobuf::MutProxied for DoubleRange {
  type Mut<'msg> = DoubleRangeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DoubleRangeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DoubleRange>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DoubleRangeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DoubleRangeView<'msg> {
  type Message = DoubleRange;
}

impl ::std::fmt::Debug for DoubleRangeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DoubleRangeView<'_> {
  fn default() -> DoubleRangeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DoubleRange>> for DoubleRangeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DoubleRange>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DoubleRangeView<'msg> {

  pub fn to_owned(&self) -> DoubleRange {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // start: optional double
  pub fn start(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // end: optional double
  pub fn end(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DoubleRangeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DoubleRangeView<'_> {}

// SAFETY:
// - `DoubleRangeView` is `Send` because while its alive a `DoubleRangeMut` cannot.
// - `DoubleRangeView` does not use thread-local data.
unsafe impl ::std::marker::Send for DoubleRangeView<'_> {}

impl<'msg> ::protobuf::AsView for DoubleRangeView<'msg> {
  type Proxied = DoubleRange;
  fn as_view(&self) -> ::protobuf::View<'msg, DoubleRange> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DoubleRangeView<'msg> {
  fn into_view<'shorter>(self) -> DoubleRangeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DoubleRange> for DoubleRangeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DoubleRange {
    let mut dst = DoubleRange::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DoubleRange> for DoubleRangeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DoubleRange {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DoubleRange {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DoubleRangeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DoubleRangeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DoubleRangeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleRange>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DoubleRangeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DoubleRangeMut<'msg> {
  type Message = DoubleRange;
}

impl ::std::fmt::Debug for DoubleRangeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleRange>> for DoubleRangeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleRange>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DoubleRangeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleRange> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DoubleRange {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // start: optional double
  pub fn start(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        0, val.into()
      )
    }
  }

  // end: optional double
  pub fn end(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `DoubleRangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DoubleRangeMut<'_> {}

// SAFETY:
// - `DoubleRangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DoubleRangeMut<'_> {}

impl<'msg> ::protobuf::AsView for DoubleRangeMut<'msg> {
  type Proxied = DoubleRange;
  fn as_view(&self) -> ::protobuf::View<'_, DoubleRange> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DoubleRangeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DoubleRange>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DoubleRangeMut<'msg> {
  type MutProxied = DoubleRange;
  fn as_mut(&mut self) -> DoubleRangeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DoubleRangeMut<'msg> {
  fn into_mut<'shorter>(self) -> DoubleRangeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DoubleRange {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DoubleRange> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DoubleRangeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DoubleRangeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // start: optional double
  pub fn start(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        0, val.into()
      )
    }
  }

  // end: optional double
  pub fn end(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

}  // impl DoubleRange

impl ::std::ops::Drop for DoubleRange {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DoubleRange {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DoubleRange {
  type Proxied = Self;
  fn as_view(&self) -> DoubleRangeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DoubleRange {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DoubleRangeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DoubleRange {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__v3__DoubleRange_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ P P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__v3__DoubleRange_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__v3__DoubleRange_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DoubleRange {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DoubleRange {
  type Msg = DoubleRange;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleRange> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DoubleRange {
  type Msg = DoubleRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleRange> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DoubleRangeMut<'_> {
  type Msg = DoubleRange;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleRange> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DoubleRangeMut<'_> {
  type Msg = DoubleRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleRange> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DoubleRangeView<'_> {
  type Msg = DoubleRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleRange> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DoubleRangeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



