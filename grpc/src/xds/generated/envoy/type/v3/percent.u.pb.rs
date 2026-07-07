const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__v3__Percent_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Percent {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Percent>
}

impl ::protobuf::Message for Percent {
  type MessageView<'msg> = PercentView<'msg>;
  type MessageMut<'msg> = PercentMut<'msg>;
}

impl ::std::default::Default for Percent {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Percent {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Percent` is `Sync` because it does not implement interior mutability.
//    Neither does `PercentMut`.
unsafe impl ::std::marker::Sync for Percent {}

// SAFETY:
// - `Percent` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Percent {}

impl ::protobuf::Proxied for Percent {
  type View<'msg> = PercentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Percent {}

impl ::protobuf::MutProxied for Percent {
  type Mut<'msg> = PercentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PercentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Percent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PercentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PercentView<'msg> {
  type Message = Percent;
}

impl ::std::fmt::Debug for PercentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PercentView<'_> {
  fn default() -> PercentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Percent>> for PercentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Percent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PercentView<'msg> {

  pub fn to_owned(&self) -> Percent {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // value: optional double
  pub fn value(self) -> f64 {
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

}

// SAFETY:
// - `PercentView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PercentView<'_> {}

// SAFETY:
// - `PercentView` is `Send` because while its alive a `PercentMut` cannot.
// - `PercentView` does not use thread-local data.
unsafe impl ::std::marker::Send for PercentView<'_> {}

impl<'msg> ::protobuf::AsView for PercentView<'msg> {
  type Proxied = Percent;
  fn as_view(&self) -> ::protobuf::View<'msg, Percent> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PercentView<'msg> {
  fn into_view<'shorter>(self) -> PercentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Percent> for PercentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Percent {
    let mut dst = Percent::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Percent> for PercentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Percent {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Percent {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PercentView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PercentMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PercentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Percent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PercentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PercentMut<'msg> {
  type Message = Percent;
}

impl ::std::fmt::Debug for PercentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Percent>> for PercentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Percent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PercentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Percent> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Percent {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // value: optional double
  pub fn value(&self) -> f64 {
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
  pub fn set_value(&mut self, val: f64) {
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

}

// SAFETY:
// - `PercentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PercentMut<'_> {}

// SAFETY:
// - `PercentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PercentMut<'_> {}

impl<'msg> ::protobuf::AsView for PercentMut<'msg> {
  type Proxied = Percent;
  fn as_view(&self) -> ::protobuf::View<'_, Percent> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PercentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Percent>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PercentMut<'msg> {
  type MutProxied = Percent;
  fn as_mut(&mut self) -> PercentMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PercentMut<'msg> {
  fn into_mut<'shorter>(self) -> PercentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Percent {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Percent> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PercentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PercentMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // value: optional double
  pub fn value(&self) -> f64 {
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
  pub fn set_value(&mut self, val: f64) {
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

}  // impl Percent

impl ::std::ops::Drop for Percent {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Percent {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Percent {
  type Proxied = Self;
  fn as_view(&self) -> PercentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Percent {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PercentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Percent {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__v3__Percent_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__v3__Percent_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__v3__Percent_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Percent {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Percent {
  type Msg = Percent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Percent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Percent {
  type Msg = Percent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Percent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PercentMut<'_> {
  type Msg = Percent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Percent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PercentMut<'_> {
  type Msg = Percent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Percent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PercentView<'_> {
  type Msg = Percent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Percent> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PercentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__v3__FractionalPercent_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FractionalPercent {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FractionalPercent>
}

impl ::protobuf::Message for FractionalPercent {
  type MessageView<'msg> = FractionalPercentView<'msg>;
  type MessageMut<'msg> = FractionalPercentMut<'msg>;
}

impl ::std::default::Default for FractionalPercent {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FractionalPercent {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FractionalPercent` is `Sync` because it does not implement interior mutability.
//    Neither does `FractionalPercentMut`.
unsafe impl ::std::marker::Sync for FractionalPercent {}

// SAFETY:
// - `FractionalPercent` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FractionalPercent {}

impl ::protobuf::Proxied for FractionalPercent {
  type View<'msg> = FractionalPercentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FractionalPercent {}

impl ::protobuf::MutProxied for FractionalPercent {
  type Mut<'msg> = FractionalPercentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FractionalPercentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FractionalPercent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FractionalPercentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FractionalPercentView<'msg> {
  type Message = FractionalPercent;
}

impl ::std::fmt::Debug for FractionalPercentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FractionalPercentView<'_> {
  fn default() -> FractionalPercentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FractionalPercent>> for FractionalPercentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FractionalPercent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FractionalPercentView<'msg> {

  pub fn to_owned(&self) -> FractionalPercent {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // numerator: optional uint32
  pub fn numerator(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // denominator: optional enum envoy.type.v3.FractionalPercent.DenominatorType
  pub fn denominator(self) -> super::fractional_percent::DenominatorType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::fractional_percent::DenominatorType::Hundred).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `FractionalPercentView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FractionalPercentView<'_> {}

// SAFETY:
// - `FractionalPercentView` is `Send` because while its alive a `FractionalPercentMut` cannot.
// - `FractionalPercentView` does not use thread-local data.
unsafe impl ::std::marker::Send for FractionalPercentView<'_> {}

impl<'msg> ::protobuf::AsView for FractionalPercentView<'msg> {
  type Proxied = FractionalPercent;
  fn as_view(&self) -> ::protobuf::View<'msg, FractionalPercent> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FractionalPercentView<'msg> {
  fn into_view<'shorter>(self) -> FractionalPercentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FractionalPercent> for FractionalPercentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FractionalPercent {
    let mut dst = FractionalPercent::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FractionalPercent> for FractionalPercentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FractionalPercent {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FractionalPercent {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FractionalPercentView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FractionalPercentMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FractionalPercentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FractionalPercent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FractionalPercentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FractionalPercentMut<'msg> {
  type Message = FractionalPercent;
}

impl ::std::fmt::Debug for FractionalPercentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FractionalPercent>> for FractionalPercentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FractionalPercent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FractionalPercentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FractionalPercent> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FractionalPercent {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // numerator: optional uint32
  pub fn numerator(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_numerator(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // denominator: optional enum envoy.type.v3.FractionalPercent.DenominatorType
  pub fn denominator(&self) -> super::fractional_percent::DenominatorType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::fractional_percent::DenominatorType::Hundred).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_denominator(&mut self, val: super::fractional_percent::DenominatorType) {
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
// - `FractionalPercentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FractionalPercentMut<'_> {}

// SAFETY:
// - `FractionalPercentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FractionalPercentMut<'_> {}

impl<'msg> ::protobuf::AsView for FractionalPercentMut<'msg> {
  type Proxied = FractionalPercent;
  fn as_view(&self) -> ::protobuf::View<'_, FractionalPercent> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FractionalPercentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FractionalPercent>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FractionalPercentMut<'msg> {
  type MutProxied = FractionalPercent;
  fn as_mut(&mut self) -> FractionalPercentMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FractionalPercentMut<'msg> {
  fn into_mut<'shorter>(self) -> FractionalPercentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FractionalPercent {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FractionalPercent> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FractionalPercentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FractionalPercentMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // numerator: optional uint32
  pub fn numerator(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_numerator(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // denominator: optional enum envoy.type.v3.FractionalPercent.DenominatorType
  pub fn denominator(&self) -> super::fractional_percent::DenominatorType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::fractional_percent::DenominatorType::Hundred).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_denominator(&mut self, val: super::fractional_percent::DenominatorType) {
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

}  // impl FractionalPercent

impl ::std::ops::Drop for FractionalPercent {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FractionalPercent {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FractionalPercent {
  type Proxied = Self;
  fn as_view(&self) -> FractionalPercentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FractionalPercent {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FractionalPercentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FractionalPercent {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__v3__FractionalPercent_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__v3__FractionalPercent_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__v3__FractionalPercent_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FractionalPercent {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FractionalPercent {
  type Msg = FractionalPercent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FractionalPercent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FractionalPercent {
  type Msg = FractionalPercent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FractionalPercent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FractionalPercentMut<'_> {
  type Msg = FractionalPercent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FractionalPercent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FractionalPercentMut<'_> {
  type Msg = FractionalPercent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FractionalPercent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FractionalPercentView<'_> {
  type Msg = FractionalPercent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FractionalPercent> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FractionalPercentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod fractional_percent {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DenominatorType(i32);

#[allow(non_upper_case_globals)]
impl DenominatorType {
  pub const Hundred: DenominatorType = DenominatorType(0);
  pub const TenThousand: DenominatorType = DenominatorType(1);
  pub const Million: DenominatorType = DenominatorType(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Hundred",
      1 => "TenThousand",
      2 => "Million",
      _ => return None
    })
  }
}

impl ::std::convert::From<DenominatorType> for i32 {
  fn from(val: DenominatorType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for DenominatorType {
  fn from(val: i32) -> DenominatorType {
    Self(val)
  }
}

impl ::std::default::Default for DenominatorType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for DenominatorType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "DenominatorType::{}", constant_name)
    } else {
      write!(f, "DenominatorType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for DenominatorType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for DenominatorType {}

impl ::protobuf::Proxied for DenominatorType {
  type View<'a> = DenominatorType;
}

impl ::protobuf::AsView for DenominatorType {
  type Proxied = DenominatorType;

  fn as_view(&self) -> DenominatorType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DenominatorType {
  fn into_view<'shorter>(self) -> DenominatorType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for DenominatorType {
  const NAME: &'static str = "DenominatorType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for DenominatorType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod fractional_percent


