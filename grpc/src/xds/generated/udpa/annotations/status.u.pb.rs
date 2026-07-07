const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut udpa__annotations__StatusAnnotation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StatusAnnotation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StatusAnnotation>
}

impl ::protobuf::Message for StatusAnnotation {
  type MessageView<'msg> = StatusAnnotationView<'msg>;
  type MessageMut<'msg> = StatusAnnotationMut<'msg>;
}

impl ::std::default::Default for StatusAnnotation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StatusAnnotation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StatusAnnotation` is `Sync` because it does not implement interior mutability.
//    Neither does `StatusAnnotationMut`.
unsafe impl ::std::marker::Sync for StatusAnnotation {}

// SAFETY:
// - `StatusAnnotation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StatusAnnotation {}

impl ::protobuf::Proxied for StatusAnnotation {
  type View<'msg> = StatusAnnotationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StatusAnnotation {}

impl ::protobuf::MutProxied for StatusAnnotation {
  type Mut<'msg> = StatusAnnotationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StatusAnnotationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatusAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatusAnnotationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StatusAnnotationView<'msg> {
  type Message = StatusAnnotation;
}

impl ::std::fmt::Debug for StatusAnnotationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StatusAnnotationView<'_> {
  fn default() -> StatusAnnotationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StatusAnnotation>> for StatusAnnotationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatusAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatusAnnotationView<'msg> {

  pub fn to_owned(&self) -> StatusAnnotation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // work_in_progress: optional bool
  pub fn work_in_progress(self) -> bool {
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

  // package_version_status: optional enum udpa.annotations.PackageVersionStatus
  pub fn package_version_status(self) -> super::PackageVersionStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::PackageVersionStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `StatusAnnotationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StatusAnnotationView<'_> {}

// SAFETY:
// - `StatusAnnotationView` is `Send` because while its alive a `StatusAnnotationMut` cannot.
// - `StatusAnnotationView` does not use thread-local data.
unsafe impl ::std::marker::Send for StatusAnnotationView<'_> {}

impl<'msg> ::protobuf::AsView for StatusAnnotationView<'msg> {
  type Proxied = StatusAnnotation;
  fn as_view(&self) -> ::protobuf::View<'msg, StatusAnnotation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatusAnnotationView<'msg> {
  fn into_view<'shorter>(self) -> StatusAnnotationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StatusAnnotation> for StatusAnnotationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatusAnnotation {
    let mut dst = StatusAnnotation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StatusAnnotation> for StatusAnnotationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatusAnnotation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StatusAnnotation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatusAnnotationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatusAnnotationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StatusAnnotationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatusAnnotation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatusAnnotationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StatusAnnotationMut<'msg> {
  type Message = StatusAnnotation;
}

impl ::std::fmt::Debug for StatusAnnotationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StatusAnnotation>> for StatusAnnotationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatusAnnotation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatusAnnotationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StatusAnnotation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StatusAnnotation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // work_in_progress: optional bool
  pub fn work_in_progress(&self) -> bool {
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
  pub fn set_work_in_progress(&mut self, val: bool) {
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

  // package_version_status: optional enum udpa.annotations.PackageVersionStatus
  pub fn package_version_status(&self) -> super::PackageVersionStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::PackageVersionStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_package_version_status(&mut self, val: super::PackageVersionStatus) {
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
// - `StatusAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StatusAnnotationMut<'_> {}

// SAFETY:
// - `StatusAnnotationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StatusAnnotationMut<'_> {}

impl<'msg> ::protobuf::AsView for StatusAnnotationMut<'msg> {
  type Proxied = StatusAnnotation;
  fn as_view(&self) -> ::protobuf::View<'_, StatusAnnotation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatusAnnotationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StatusAnnotation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StatusAnnotationMut<'msg> {
  type MutProxied = StatusAnnotation;
  fn as_mut(&mut self) -> StatusAnnotationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StatusAnnotationMut<'msg> {
  fn into_mut<'shorter>(self) -> StatusAnnotationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StatusAnnotation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StatusAnnotation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StatusAnnotationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StatusAnnotationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // work_in_progress: optional bool
  pub fn work_in_progress(&self) -> bool {
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
  pub fn set_work_in_progress(&mut self, val: bool) {
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

  // package_version_status: optional enum udpa.annotations.PackageVersionStatus
  pub fn package_version_status(&self) -> super::PackageVersionStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::PackageVersionStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_package_version_status(&mut self, val: super::PackageVersionStatus) {
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

}  // impl StatusAnnotation

impl ::std::ops::Drop for StatusAnnotation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StatusAnnotation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StatusAnnotation {
  type Proxied = Self;
  fn as_view(&self) -> StatusAnnotationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StatusAnnotation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StatusAnnotationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StatusAnnotation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::udpa__annotations__StatusAnnotation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::udpa__annotations__StatusAnnotation_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::udpa__annotations__StatusAnnotation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatusAnnotation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatusAnnotation {
  type Msg = StatusAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatusAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatusAnnotation {
  type Msg = StatusAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatusAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatusAnnotationMut<'_> {
  type Msg = StatusAnnotation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatusAnnotation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatusAnnotationMut<'_> {
  type Msg = StatusAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatusAnnotation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatusAnnotationView<'_> {
  type Msg = StatusAnnotation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatusAnnotation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatusAnnotationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PackageVersionStatus(i32);

#[allow(non_upper_case_globals)]
impl PackageVersionStatus {
  pub const Unknown: PackageVersionStatus = PackageVersionStatus(0);
  pub const Frozen: PackageVersionStatus = PackageVersionStatus(1);
  pub const Active: PackageVersionStatus = PackageVersionStatus(2);
  pub const NextMajorVersionCandidate: PackageVersionStatus = PackageVersionStatus(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unknown",
      1 => "Frozen",
      2 => "Active",
      3 => "NextMajorVersionCandidate",
      _ => return None
    })
  }
}

impl ::std::convert::From<PackageVersionStatus> for i32 {
  fn from(val: PackageVersionStatus) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for PackageVersionStatus {
  fn from(val: i32) -> PackageVersionStatus {
    Self(val)
  }
}

impl ::std::default::Default for PackageVersionStatus {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for PackageVersionStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "PackageVersionStatus::{}", constant_name)
    } else {
      write!(f, "PackageVersionStatus::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for PackageVersionStatus {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for PackageVersionStatus {}

impl ::protobuf::Proxied for PackageVersionStatus {
  type View<'a> = PackageVersionStatus;
}

impl ::protobuf::AsView for PackageVersionStatus {
  type Proxied = PackageVersionStatus;

  fn as_view(&self) -> PackageVersionStatus {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PackageVersionStatus {
  fn into_view<'shorter>(self) -> PackageVersionStatus where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for PackageVersionStatus {
  const NAME: &'static str = "PackageVersionStatus";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for PackageVersionStatus {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}



