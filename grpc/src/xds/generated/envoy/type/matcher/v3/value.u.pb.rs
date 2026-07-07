const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__ValueMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ValueMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ValueMatcher>
}

impl ::protobuf::Message for ValueMatcher {
  type MessageView<'msg> = ValueMatcherView<'msg>;
  type MessageMut<'msg> = ValueMatcherMut<'msg>;
}

impl ::std::default::Default for ValueMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ValueMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ValueMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `ValueMatcherMut`.
unsafe impl ::std::marker::Sync for ValueMatcher {}

// SAFETY:
// - `ValueMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ValueMatcher {}

impl ::protobuf::Proxied for ValueMatcher {
  type View<'msg> = ValueMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ValueMatcher {}

impl ::protobuf::MutProxied for ValueMatcher {
  type Mut<'msg> = ValueMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ValueMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValueMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValueMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ValueMatcherView<'msg> {
  type Message = ValueMatcher;
}

impl ::std::fmt::Debug for ValueMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ValueMatcherView<'_> {
  fn default() -> ValueMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ValueMatcher>> for ValueMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ValueMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValueMatcherView<'msg> {

  pub fn to_owned(&self) -> ValueMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // null_match: optional message envoy.type.matcher.v3.ValueMatcher.NullMatch
  pub fn has_null_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn null_match_opt(self) -> ::std::option::Option<super::value_matcher::NullMatchView<'msg>> {
    self.has_null_match().then(|| self.null_match())
  }
  pub fn null_match(self) -> super::value_matcher::NullMatchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::value_matcher::NullMatchView::default())
  }

  // double_match: optional message envoy.type.matcher.v3.DoubleMatcher
  pub fn has_double_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn double_match_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcherView<'msg>> {
    self.has_double_match().then(|| self.double_match())
  }
  pub fn double_match(self) -> crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcherView::default())
  }

  // string_match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_string_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn string_match_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_string_match().then(|| self.string_match())
  }
  pub fn string_match(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }

  // bool_match: optional bool
  pub fn has_bool_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn bool_match_opt(self) -> ::std::option::Option<bool> {
    self.has_bool_match().then(|| self.bool_match())
  }
  pub fn bool_match(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

  // present_match: optional bool
  pub fn has_present_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn present_match_opt(self) -> ::std::option::Option<bool> {
    self.has_present_match().then(|| self.present_match())
  }
  pub fn present_match(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }

  // list_match: optional message envoy.type.matcher.v3.ListMatcher
  pub fn has_list_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn list_match_opt(self) -> ::std::option::Option<super::ListMatcherView<'msg>> {
    self.has_list_match().then(|| self.list_match())
  }
  pub fn list_match(self) -> super::ListMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ListMatcherView::default())
  }

  // or_match: optional message envoy.type.matcher.v3.OrMatcher
  pub fn has_or_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn or_match_opt(self) -> ::std::option::Option<super::OrMatcherView<'msg>> {
    self.has_or_match().then(|| self.or_match())
  }
  pub fn or_match(self) -> super::OrMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::OrMatcherView::default())
  }

  pub fn match_pattern(self) -> super::value_matcher::MatchPatternOneof<'msg> {
    match self.match_pattern_case() {
      super::value_matcher::MatchPatternCase::NullMatch =>
          super::value_matcher::MatchPatternOneof::NullMatch(self.null_match()),
      super::value_matcher::MatchPatternCase::DoubleMatch =>
          super::value_matcher::MatchPatternOneof::DoubleMatch(self.double_match()),
      super::value_matcher::MatchPatternCase::StringMatch =>
          super::value_matcher::MatchPatternOneof::StringMatch(self.string_match()),
      super::value_matcher::MatchPatternCase::BoolMatch =>
          super::value_matcher::MatchPatternOneof::BoolMatch(self.bool_match()),
      super::value_matcher::MatchPatternCase::PresentMatch =>
          super::value_matcher::MatchPatternOneof::PresentMatch(self.present_match()),
      super::value_matcher::MatchPatternCase::ListMatch =>
          super::value_matcher::MatchPatternOneof::ListMatch(self.list_match()),
      super::value_matcher::MatchPatternCase::OrMatch =>
          super::value_matcher::MatchPatternOneof::OrMatch(self.or_match()),
      _ => super::value_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(self) -> super::value_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::value_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ValueMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ValueMatcherView<'_> {}

// SAFETY:
// - `ValueMatcherView` is `Send` because while its alive a `ValueMatcherMut` cannot.
// - `ValueMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for ValueMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for ValueMatcherView<'msg> {
  type Proxied = ValueMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, ValueMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValueMatcherView<'msg> {
  fn into_view<'shorter>(self) -> ValueMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ValueMatcher> for ValueMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValueMatcher {
    let mut dst = ValueMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ValueMatcher> for ValueMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ValueMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ValueMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ValueMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ValueMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ValueMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValueMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ValueMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ValueMatcherMut<'msg> {
  type Message = ValueMatcher;
}

impl ::std::fmt::Debug for ValueMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ValueMatcher>> for ValueMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ValueMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ValueMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ValueMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ValueMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // null_match: optional message envoy.type.matcher.v3.ValueMatcher.NullMatch
  pub fn has_null_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_null_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn null_match_opt(&self) -> ::std::option::Option<super::value_matcher::NullMatchView<'_>> {
    self.has_null_match().then(|| self.null_match())
  }
  pub fn null_match(&self) -> super::value_matcher::NullMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::value_matcher::NullMatchView::default())
  }
  pub fn null_match_mut(&mut self) -> super::value_matcher::NullMatchMut<'_> {
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
  pub fn set_null_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::value_matcher::NullMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // double_match: optional message envoy.type.matcher.v3.DoubleMatcher
  pub fn has_double_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_double_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn double_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcherView<'_>> {
    self.has_double_match().then(|| self.double_match())
  }
  pub fn double_match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcherView::default())
  }
  pub fn double_match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcherMut<'_> {
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
  pub fn set_double_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // string_match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_string_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_string_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn string_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_string_match().then(|| self.string_match())
  }
  pub fn string_match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn string_match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_string_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // bool_match: optional bool
  pub fn has_bool_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_bool_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn bool_match_opt(&self) -> ::std::option::Option<bool> {
    self.has_bool_match().then(|| self.bool_match())
  }
  pub fn bool_match(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bool_match(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // present_match: optional bool
  pub fn has_present_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_present_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn present_match_opt(&self) -> ::std::option::Option<bool> {
    self.has_present_match().then(|| self.present_match())
  }
  pub fn present_match(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_present_match(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

  // list_match: optional message envoy.type.matcher.v3.ListMatcher
  pub fn has_list_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_list_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn list_match_opt(&self) -> ::std::option::Option<super::ListMatcherView<'_>> {
    self.has_list_match().then(|| self.list_match())
  }
  pub fn list_match(&self) -> super::ListMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ListMatcherView::default())
  }
  pub fn list_match_mut(&mut self) -> super::ListMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_list_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::ListMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // or_match: optional message envoy.type.matcher.v3.OrMatcher
  pub fn has_or_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_or_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn or_match_opt(&self) -> ::std::option::Option<super::OrMatcherView<'_>> {
    self.has_or_match().then(|| self.or_match())
  }
  pub fn or_match(&self) -> super::OrMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::OrMatcherView::default())
  }
  pub fn or_match_mut(&mut self) -> super::OrMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_or_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::OrMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn match_pattern(&self) -> super::value_matcher::MatchPatternOneof<'_> {
    match &self.match_pattern_case() {
      super::value_matcher::MatchPatternCase::NullMatch =>
          super::value_matcher::MatchPatternOneof::NullMatch(self.null_match()),
      super::value_matcher::MatchPatternCase::DoubleMatch =>
          super::value_matcher::MatchPatternOneof::DoubleMatch(self.double_match()),
      super::value_matcher::MatchPatternCase::StringMatch =>
          super::value_matcher::MatchPatternOneof::StringMatch(self.string_match()),
      super::value_matcher::MatchPatternCase::BoolMatch =>
          super::value_matcher::MatchPatternOneof::BoolMatch(self.bool_match()),
      super::value_matcher::MatchPatternCase::PresentMatch =>
          super::value_matcher::MatchPatternOneof::PresentMatch(self.present_match()),
      super::value_matcher::MatchPatternCase::ListMatch =>
          super::value_matcher::MatchPatternOneof::ListMatch(self.list_match()),
      super::value_matcher::MatchPatternCase::OrMatch =>
          super::value_matcher::MatchPatternOneof::OrMatch(self.or_match()),
      _ => super::value_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(&self) -> super::value_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::value_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ValueMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ValueMatcherMut<'_> {}

// SAFETY:
// - `ValueMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ValueMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for ValueMatcherMut<'msg> {
  type Proxied = ValueMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, ValueMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ValueMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ValueMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ValueMatcherMut<'msg> {
  type MutProxied = ValueMatcher;
  fn as_mut(&mut self) -> ValueMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ValueMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> ValueMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ValueMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ValueMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ValueMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ValueMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // null_match: optional message envoy.type.matcher.v3.ValueMatcher.NullMatch
  pub fn has_null_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_null_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn null_match_opt(&self) -> ::std::option::Option<super::value_matcher::NullMatchView<'_>> {
    self.has_null_match().then(|| self.null_match())
  }
  pub fn null_match(&self) -> super::value_matcher::NullMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::value_matcher::NullMatchView::default())
  }
  pub fn null_match_mut(&mut self) -> super::value_matcher::NullMatchMut<'_> {
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
  pub fn set_null_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::value_matcher::NullMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // double_match: optional message envoy.type.matcher.v3.DoubleMatcher
  pub fn has_double_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_double_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn double_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcherView<'_>> {
    self.has_double_match().then(|| self.double_match())
  }
  pub fn double_match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcherView::default())
  }
  pub fn double_match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcherMut<'_> {
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
  pub fn set_double_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // string_match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_string_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_string_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn string_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_string_match().then(|| self.string_match())
  }
  pub fn string_match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn string_match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_string_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // bool_match: optional bool
  pub fn has_bool_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_bool_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn bool_match_opt(&self) -> ::std::option::Option<bool> {
    self.has_bool_match().then(|| self.bool_match())
  }
  pub fn bool_match(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bool_match(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // present_match: optional bool
  pub fn has_present_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_present_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn present_match_opt(&self) -> ::std::option::Option<bool> {
    self.has_present_match().then(|| self.present_match())
  }
  pub fn present_match(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_present_match(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

  // list_match: optional message envoy.type.matcher.v3.ListMatcher
  pub fn has_list_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_list_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn list_match_opt(&self) -> ::std::option::Option<super::ListMatcherView<'_>> {
    self.has_list_match().then(|| self.list_match())
  }
  pub fn list_match(&self) -> super::ListMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ListMatcherView::default())
  }
  pub fn list_match_mut(&mut self) -> super::ListMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_list_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::ListMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // or_match: optional message envoy.type.matcher.v3.OrMatcher
  pub fn has_or_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_or_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn or_match_opt(&self) -> ::std::option::Option<super::OrMatcherView<'_>> {
    self.has_or_match().then(|| self.or_match())
  }
  pub fn or_match(&self) -> super::OrMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::OrMatcherView::default())
  }
  pub fn or_match_mut(&mut self) -> super::OrMatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_or_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::OrMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  pub fn match_pattern(&self) -> super::value_matcher::MatchPatternOneof<'_> {
    match &self.match_pattern_case() {
      super::value_matcher::MatchPatternCase::NullMatch =>
          super::value_matcher::MatchPatternOneof::NullMatch(self.null_match()),
      super::value_matcher::MatchPatternCase::DoubleMatch =>
          super::value_matcher::MatchPatternOneof::DoubleMatch(self.double_match()),
      super::value_matcher::MatchPatternCase::StringMatch =>
          super::value_matcher::MatchPatternOneof::StringMatch(self.string_match()),
      super::value_matcher::MatchPatternCase::BoolMatch =>
          super::value_matcher::MatchPatternOneof::BoolMatch(self.bool_match()),
      super::value_matcher::MatchPatternCase::PresentMatch =>
          super::value_matcher::MatchPatternOneof::PresentMatch(self.present_match()),
      super::value_matcher::MatchPatternCase::ListMatch =>
          super::value_matcher::MatchPatternOneof::ListMatch(self.list_match()),
      super::value_matcher::MatchPatternCase::OrMatch =>
          super::value_matcher::MatchPatternOneof::OrMatch(self.or_match()),
      _ => super::value_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(&self) -> super::value_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::value_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ValueMatcher

impl ::std::ops::Drop for ValueMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ValueMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ValueMatcher {
  type Proxied = Self;
  fn as_view(&self) -> ValueMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ValueMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ValueMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ValueMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ListMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__ValueMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValueMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValueMatcher {
  type Msg = ValueMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValueMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValueMatcher {
  type Msg = ValueMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValueMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ValueMatcherMut<'_> {
  type Msg = ValueMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValueMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValueMatcherMut<'_> {
  type Msg = ValueMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValueMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ValueMatcherView<'_> {
  type Msg = ValueMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ValueMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ValueMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod value_matcher {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__ValueMatcher__NullMatch_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NullMatch {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NullMatch>
}

impl ::protobuf::Message for NullMatch {
  type MessageView<'msg> = NullMatchView<'msg>;
  type MessageMut<'msg> = NullMatchMut<'msg>;
}

impl ::std::default::Default for NullMatch {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NullMatch {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NullMatch` is `Sync` because it does not implement interior mutability.
//    Neither does `NullMatchMut`.
unsafe impl ::std::marker::Sync for NullMatch {}

// SAFETY:
// - `NullMatch` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for NullMatch {}

impl ::protobuf::Proxied for NullMatch {
  type View<'msg> = NullMatchView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NullMatch {}

impl ::protobuf::MutProxied for NullMatch {
  type Mut<'msg> = NullMatchMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NullMatchView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NullMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NullMatchView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NullMatchView<'msg> {
  type Message = NullMatch;
}

impl ::std::fmt::Debug for NullMatchView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NullMatchView<'_> {
  fn default() -> NullMatchView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NullMatch>> for NullMatchView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NullMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NullMatchView<'msg> {

  pub fn to_owned(&self) -> NullMatch {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `NullMatchView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for NullMatchView<'_> {}

// SAFETY:
// - `NullMatchView` is `Send` because while its alive a `NullMatchMut` cannot.
// - `NullMatchView` does not use thread-local data.
unsafe impl ::std::marker::Send for NullMatchView<'_> {}

impl<'msg> ::protobuf::AsView for NullMatchView<'msg> {
  type Proxied = NullMatch;
  fn as_view(&self) -> ::protobuf::View<'msg, NullMatch> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NullMatchView<'msg> {
  fn into_view<'shorter>(self) -> NullMatchView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NullMatch> for NullMatchView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NullMatch {
    let mut dst = NullMatch::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NullMatch> for NullMatchMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NullMatch {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for NullMatch {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NullMatchView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NullMatchMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NullMatchMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NullMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NullMatchMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NullMatchMut<'msg> {
  type Message = NullMatch;
}

impl ::std::fmt::Debug for NullMatchMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NullMatch>> for NullMatchMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NullMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NullMatchMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NullMatch> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> NullMatch {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `NullMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for NullMatchMut<'_> {}

// SAFETY:
// - `NullMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for NullMatchMut<'_> {}

impl<'msg> ::protobuf::AsView for NullMatchMut<'msg> {
  type Proxied = NullMatch;
  fn as_view(&self) -> ::protobuf::View<'_, NullMatch> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NullMatchMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NullMatch>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for NullMatchMut<'msg> {
  type MutProxied = NullMatch;
  fn as_mut(&mut self) -> NullMatchMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NullMatchMut<'msg> {
  fn into_mut<'shorter>(self) -> NullMatchMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NullMatch {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NullMatch> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NullMatchView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NullMatchMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl NullMatch

impl ::std::ops::Drop for NullMatch {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NullMatch {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NullMatch {
  type Proxied = Self;
  fn as_view(&self) -> NullMatchView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NullMatch {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NullMatchMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NullMatch {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::value_matcher::envoy__type__matcher__v3__ValueMatcher__NullMatch_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::value_matcher::envoy__type__matcher__v3__ValueMatcher__NullMatch_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::value_matcher::envoy__type__matcher__v3__ValueMatcher__NullMatch_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NullMatch {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NullMatch {
  type Msg = NullMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NullMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NullMatch {
  type Msg = NullMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NullMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NullMatchMut<'_> {
  type Msg = NullMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NullMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NullMatchMut<'_> {
  type Msg = NullMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NullMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NullMatchView<'_> {
  type Msg = NullMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NullMatch> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NullMatchMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MatchPatternOneof<'msg> {
  NullMatch(::protobuf::View<'msg, super::super::value_matcher::NullMatch>) = 1,
  DoubleMatch(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcher>) = 2,
  StringMatch(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) = 3,
  BoolMatch(bool) = 4,
  PresentMatch(bool) = 5,
  ListMatch(::protobuf::View<'msg, super::super::ListMatcher>) = 6,
  OrMatch(::protobuf::View<'msg, super::super::OrMatcher>) = 7,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MatchPatternCase {
  NullMatch = 1,
  DoubleMatch = 2,
  StringMatch = 3,
  BoolMatch = 4,
  PresentMatch = 5,
  ListMatch = 6,
  OrMatch = 7,

  not_set = 0
}

impl MatchPatternCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MatchPatternCase> {
    match v {
      0 => Some(MatchPatternCase::not_set),
      1 => Some(MatchPatternCase::NullMatch),
      2 => Some(MatchPatternCase::DoubleMatch),
      3 => Some(MatchPatternCase::StringMatch),
      4 => Some(MatchPatternCase::BoolMatch),
      5 => Some(MatchPatternCase::PresentMatch),
      6 => Some(MatchPatternCase::ListMatch),
      7 => Some(MatchPatternCase::OrMatch),
      _ => None
    }
  }
}
}  // pub mod value_matcher


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__ListMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListMatcher>
}

impl ::protobuf::Message for ListMatcher {
  type MessageView<'msg> = ListMatcherView<'msg>;
  type MessageMut<'msg> = ListMatcherMut<'msg>;
}

impl ::std::default::Default for ListMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `ListMatcherMut`.
unsafe impl ::std::marker::Sync for ListMatcher {}

// SAFETY:
// - `ListMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ListMatcher {}

impl ::protobuf::Proxied for ListMatcher {
  type View<'msg> = ListMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListMatcher {}

impl ::protobuf::MutProxied for ListMatcher {
  type Mut<'msg> = ListMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListMatcherView<'msg> {
  type Message = ListMatcher;
}

impl ::std::fmt::Debug for ListMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListMatcherView<'_> {
  fn default() -> ListMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListMatcher>> for ListMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListMatcherView<'msg> {

  pub fn to_owned(&self) -> ListMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // one_of: optional message envoy.type.matcher.v3.ValueMatcher
  pub fn has_one_of(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn one_of_opt(self) -> ::std::option::Option<super::ValueMatcherView<'msg>> {
    self.has_one_of().then(|| self.one_of())
  }
  pub fn one_of(self) -> super::ValueMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ValueMatcherView::default())
  }

  pub fn match_pattern(self) -> super::list_matcher::MatchPatternOneof<'msg> {
    match self.match_pattern_case() {
      super::list_matcher::MatchPatternCase::OneOf =>
          super::list_matcher::MatchPatternOneof::OneOf(self.one_of()),
      _ => super::list_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(self) -> super::list_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::list_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ListMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ListMatcherView<'_> {}

// SAFETY:
// - `ListMatcherView` is `Send` because while its alive a `ListMatcherMut` cannot.
// - `ListMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for ListMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for ListMatcherView<'msg> {
  type Proxied = ListMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, ListMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListMatcherView<'msg> {
  fn into_view<'shorter>(self) -> ListMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListMatcher> for ListMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListMatcher {
    let mut dst = ListMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListMatcher> for ListMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ListMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListMatcherMut<'msg> {
  type Message = ListMatcher;
}

impl ::std::fmt::Debug for ListMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListMatcher>> for ListMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ListMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // one_of: optional message envoy.type.matcher.v3.ValueMatcher
  pub fn has_one_of(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_one_of(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn one_of_opt(&self) -> ::std::option::Option<super::ValueMatcherView<'_>> {
    self.has_one_of().then(|| self.one_of())
  }
  pub fn one_of(&self) -> super::ValueMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ValueMatcherView::default())
  }
  pub fn one_of_mut(&mut self) -> super::ValueMatcherMut<'_> {
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
  pub fn set_one_of(&mut self,
    val: impl ::protobuf::IntoProxied<super::ValueMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  pub fn match_pattern(&self) -> super::list_matcher::MatchPatternOneof<'_> {
    match &self.match_pattern_case() {
      super::list_matcher::MatchPatternCase::OneOf =>
          super::list_matcher::MatchPatternOneof::OneOf(self.one_of()),
      _ => super::list_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(&self) -> super::list_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::list_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ListMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ListMatcherMut<'_> {}

// SAFETY:
// - `ListMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ListMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for ListMatcherMut<'msg> {
  type Proxied = ListMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, ListMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ListMatcherMut<'msg> {
  type MutProxied = ListMatcher;
  fn as_mut(&mut self) -> ListMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> ListMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // one_of: optional message envoy.type.matcher.v3.ValueMatcher
  pub fn has_one_of(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_one_of(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn one_of_opt(&self) -> ::std::option::Option<super::ValueMatcherView<'_>> {
    self.has_one_of().then(|| self.one_of())
  }
  pub fn one_of(&self) -> super::ValueMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ValueMatcherView::default())
  }
  pub fn one_of_mut(&mut self) -> super::ValueMatcherMut<'_> {
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
  pub fn set_one_of(&mut self,
    val: impl ::protobuf::IntoProxied<super::ValueMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  pub fn match_pattern(&self) -> super::list_matcher::MatchPatternOneof<'_> {
    match &self.match_pattern_case() {
      super::list_matcher::MatchPatternCase::OneOf =>
          super::list_matcher::MatchPatternOneof::OneOf(self.one_of()),
      _ => super::list_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(&self) -> super::list_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::list_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ListMatcher

impl ::std::ops::Drop for ListMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListMatcher {
  type Proxied = Self;
  fn as_view(&self) -> ListMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__ListMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3^!");
        super::envoy__type__matcher__v3__OrMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        super::envoy__type__matcher__v3__ValueMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333//33^!|#|$|%|&|(|)");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__ListMatcher_msg_init.0, &[super::envoy__type__matcher__v3__ValueMatcher_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__OrMatcher_msg_init.0, &[super::envoy__type__matcher__v3__ValueMatcher_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__ValueMatcher_msg_init.0, &[<super::value_matcher::NullMatch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::number::DoubleMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::envoy__type__matcher__v3__ListMatcher_msg_init.0,
            super::envoy__type__matcher__v3__OrMatcher_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__ListMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListMatcher {
  type Msg = ListMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListMatcher {
  type Msg = ListMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListMatcherMut<'_> {
  type Msg = ListMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListMatcherMut<'_> {
  type Msg = ListMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListMatcherView<'_> {
  type Msg = ListMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod list_matcher {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MatchPatternOneof<'msg> {
  OneOf(::protobuf::View<'msg, super::super::ValueMatcher>) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MatchPatternCase {
  OneOf = 1,

  not_set = 0
}

impl MatchPatternCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MatchPatternCase> {
    match v {
      0 => Some(MatchPatternCase::not_set),
      1 => Some(MatchPatternCase::OneOf),
      _ => None
    }
  }
}
}  // pub mod list_matcher


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__OrMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OrMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OrMatcher>
}

impl ::protobuf::Message for OrMatcher {
  type MessageView<'msg> = OrMatcherView<'msg>;
  type MessageMut<'msg> = OrMatcherMut<'msg>;
}

impl ::std::default::Default for OrMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OrMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OrMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `OrMatcherMut`.
unsafe impl ::std::marker::Sync for OrMatcher {}

// SAFETY:
// - `OrMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for OrMatcher {}

impl ::protobuf::Proxied for OrMatcher {
  type View<'msg> = OrMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OrMatcher {}

impl ::protobuf::MutProxied for OrMatcher {
  type Mut<'msg> = OrMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OrMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OrMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OrMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OrMatcherView<'msg> {
  type Message = OrMatcher;
}

impl ::std::fmt::Debug for OrMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OrMatcherView<'_> {
  fn default() -> OrMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OrMatcher>> for OrMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OrMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OrMatcherView<'msg> {

  pub fn to_owned(&self) -> OrMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // value_matchers: repeated message envoy.type.matcher.v3.ValueMatcher
  pub fn value_matchers(self) -> ::protobuf::RepeatedView<'msg, super::ValueMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ValueMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `OrMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OrMatcherView<'_> {}

// SAFETY:
// - `OrMatcherView` is `Send` because while its alive a `OrMatcherMut` cannot.
// - `OrMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for OrMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for OrMatcherView<'msg> {
  type Proxied = OrMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, OrMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OrMatcherView<'msg> {
  fn into_view<'shorter>(self) -> OrMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OrMatcher> for OrMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OrMatcher {
    let mut dst = OrMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OrMatcher> for OrMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OrMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for OrMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OrMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OrMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OrMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OrMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OrMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OrMatcherMut<'msg> {
  type Message = OrMatcher;
}

impl ::std::fmt::Debug for OrMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OrMatcher>> for OrMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OrMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OrMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OrMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> OrMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // value_matchers: repeated message envoy.type.matcher.v3.ValueMatcher
  pub fn value_matchers(&self) -> ::protobuf::RepeatedView<'_, super::ValueMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ValueMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn value_matchers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ValueMatcher> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_value_matchers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ValueMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `OrMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OrMatcherMut<'_> {}

// SAFETY:
// - `OrMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OrMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for OrMatcherMut<'msg> {
  type Proxied = OrMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, OrMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OrMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OrMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OrMatcherMut<'msg> {
  type MutProxied = OrMatcher;
  fn as_mut(&mut self) -> OrMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OrMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> OrMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OrMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OrMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OrMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OrMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // value_matchers: repeated message envoy.type.matcher.v3.ValueMatcher
  pub fn value_matchers(&self) -> ::protobuf::RepeatedView<'_, super::ValueMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ValueMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn value_matchers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ValueMatcher> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_value_matchers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ValueMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl OrMatcher

impl ::std::ops::Drop for OrMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OrMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OrMatcher {
  type Proxied = Self;
  fn as_view(&self) -> OrMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OrMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OrMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OrMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::ListMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__OrMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OrMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OrMatcher {
  type Msg = OrMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrMatcher {
  type Msg = OrMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OrMatcherMut<'_> {
  type Msg = OrMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrMatcherMut<'_> {
  type Msg = OrMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrMatcherView<'_> {
  type Msg = OrMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OrMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



