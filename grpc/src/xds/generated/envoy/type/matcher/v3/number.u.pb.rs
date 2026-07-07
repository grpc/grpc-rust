const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__DoubleMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DoubleMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DoubleMatcher>
}

impl ::protobuf::Message for DoubleMatcher {
  type MessageView<'msg> = DoubleMatcherView<'msg>;
  type MessageMut<'msg> = DoubleMatcherMut<'msg>;
}

impl ::std::default::Default for DoubleMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DoubleMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DoubleMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `DoubleMatcherMut`.
unsafe impl ::std::marker::Sync for DoubleMatcher {}

// SAFETY:
// - `DoubleMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DoubleMatcher {}

impl ::protobuf::Proxied for DoubleMatcher {
  type View<'msg> = DoubleMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DoubleMatcher {}

impl ::protobuf::MutProxied for DoubleMatcher {
  type Mut<'msg> = DoubleMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DoubleMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DoubleMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DoubleMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DoubleMatcherView<'msg> {
  type Message = DoubleMatcher;
}

impl ::std::fmt::Debug for DoubleMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DoubleMatcherView<'_> {
  fn default() -> DoubleMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DoubleMatcher>> for DoubleMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DoubleMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DoubleMatcherView<'msg> {

  pub fn to_owned(&self) -> DoubleMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // range: optional message envoy.type.v3.DoubleRange
  pub fn has_range(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn range_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::range::DoubleRangeView<'msg>> {
    self.has_range().then(|| self.range())
  }
  pub fn range(self) -> crate::xds::generated::envoy::r#type::v3::range::DoubleRangeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::range::DoubleRangeView::default())
  }

  // exact: optional double
  pub fn has_exact(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn exact_opt(self) -> ::std::option::Option<f64> {
    self.has_exact().then(|| self.exact())
  }
  pub fn exact(self) -> f64 {
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

  pub fn match_pattern(self) -> super::double_matcher::MatchPatternOneof<'msg> {
    match self.match_pattern_case() {
      super::double_matcher::MatchPatternCase::Range =>
          super::double_matcher::MatchPatternOneof::Range(self.range()),
      super::double_matcher::MatchPatternCase::Exact =>
          super::double_matcher::MatchPatternOneof::Exact(self.exact()),
      _ => super::double_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(self) -> super::double_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::double_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DoubleMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DoubleMatcherView<'_> {}

// SAFETY:
// - `DoubleMatcherView` is `Send` because while its alive a `DoubleMatcherMut` cannot.
// - `DoubleMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for DoubleMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for DoubleMatcherView<'msg> {
  type Proxied = DoubleMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, DoubleMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DoubleMatcherView<'msg> {
  fn into_view<'shorter>(self) -> DoubleMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DoubleMatcher> for DoubleMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DoubleMatcher {
    let mut dst = DoubleMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DoubleMatcher> for DoubleMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DoubleMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DoubleMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DoubleMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DoubleMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DoubleMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DoubleMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DoubleMatcherMut<'msg> {
  type Message = DoubleMatcher;
}

impl ::std::fmt::Debug for DoubleMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleMatcher>> for DoubleMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DoubleMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DoubleMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DoubleMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // range: optional message envoy.type.v3.DoubleRange
  pub fn has_range(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_range(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn range_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::range::DoubleRangeView<'_>> {
    self.has_range().then(|| self.range())
  }
  pub fn range(&self) -> crate::xds::generated::envoy::r#type::v3::range::DoubleRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::range::DoubleRangeView::default())
  }
  pub fn range_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::range::DoubleRangeMut<'_> {
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
  pub fn set_range(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::range::DoubleRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // exact: optional double
  pub fn has_exact(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_exact(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn exact_opt(&self) -> ::std::option::Option<f64> {
    self.has_exact().then(|| self.exact())
  }
  pub fn exact(&self) -> f64 {
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
  pub fn set_exact(&mut self, val: f64) {
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

  pub fn match_pattern(&self) -> super::double_matcher::MatchPatternOneof<'_> {
    match &self.match_pattern_case() {
      super::double_matcher::MatchPatternCase::Range =>
          super::double_matcher::MatchPatternOneof::Range(self.range()),
      super::double_matcher::MatchPatternCase::Exact =>
          super::double_matcher::MatchPatternOneof::Exact(self.exact()),
      _ => super::double_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(&self) -> super::double_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::double_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DoubleMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DoubleMatcherMut<'_> {}

// SAFETY:
// - `DoubleMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DoubleMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for DoubleMatcherMut<'msg> {
  type Proxied = DoubleMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, DoubleMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DoubleMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DoubleMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DoubleMatcherMut<'msg> {
  type MutProxied = DoubleMatcher;
  fn as_mut(&mut self) -> DoubleMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DoubleMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> DoubleMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DoubleMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DoubleMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DoubleMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DoubleMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // range: optional message envoy.type.v3.DoubleRange
  pub fn has_range(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_range(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn range_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::range::DoubleRangeView<'_>> {
    self.has_range().then(|| self.range())
  }
  pub fn range(&self) -> crate::xds::generated::envoy::r#type::v3::range::DoubleRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::range::DoubleRangeView::default())
  }
  pub fn range_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::range::DoubleRangeMut<'_> {
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
  pub fn set_range(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::range::DoubleRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // exact: optional double
  pub fn has_exact(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_exact(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn exact_opt(&self) -> ::std::option::Option<f64> {
    self.has_exact().then(|| self.exact())
  }
  pub fn exact(&self) -> f64 {
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
  pub fn set_exact(&mut self, val: f64) {
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

  pub fn match_pattern(&self) -> super::double_matcher::MatchPatternOneof<'_> {
    match &self.match_pattern_case() {
      super::double_matcher::MatchPatternCase::Range =>
          super::double_matcher::MatchPatternOneof::Range(self.range()),
      super::double_matcher::MatchPatternCase::Exact =>
          super::double_matcher::MatchPatternOneof::Exact(self.exact()),
      _ => super::double_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(&self) -> super::double_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::double_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl DoubleMatcher

impl ::std::ops::Drop for DoubleMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DoubleMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DoubleMatcher {
  type Proxied = Self;
  fn as_view(&self) -> DoubleMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DoubleMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DoubleMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DoubleMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__DoubleMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3 ^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__DoubleMatcher_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::range::DoubleRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__DoubleMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DoubleMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DoubleMatcher {
  type Msg = DoubleMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DoubleMatcher {
  type Msg = DoubleMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DoubleMatcherMut<'_> {
  type Msg = DoubleMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DoubleMatcherMut<'_> {
  type Msg = DoubleMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DoubleMatcherView<'_> {
  type Msg = DoubleMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DoubleMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DoubleMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod double_matcher {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MatchPatternOneof<'msg> {
  Range(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::v3::range::DoubleRange>) = 1,
  Exact(f64) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MatchPatternCase {
  Range = 1,
  Exact = 2,

  not_set = 0
}

impl MatchPatternCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MatchPatternCase> {
    match v {
      0 => Some(MatchPatternCase::not_set),
      1 => Some(MatchPatternCase::Range),
      2 => Some(MatchPatternCase::Exact),
      _ => None
    }
  }
}
}  // pub mod double_matcher


