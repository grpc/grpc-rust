const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__StringMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StringMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StringMatcher>
}

impl ::protobuf::Message for StringMatcher {
  type MessageView<'msg> = StringMatcherView<'msg>;
  type MessageMut<'msg> = StringMatcherMut<'msg>;
}

impl ::std::default::Default for StringMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StringMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StringMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `StringMatcherMut`.
unsafe impl ::std::marker::Sync for StringMatcher {}

// SAFETY:
// - `StringMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StringMatcher {}

impl ::protobuf::Proxied for StringMatcher {
  type View<'msg> = StringMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StringMatcher {}

impl ::protobuf::MutProxied for StringMatcher {
  type Mut<'msg> = StringMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StringMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StringMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StringMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StringMatcherView<'msg> {
  type Message = StringMatcher;
}

impl ::std::fmt::Debug for StringMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StringMatcherView<'_> {
  fn default() -> StringMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StringMatcher>> for StringMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StringMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StringMatcherView<'msg> {

  pub fn to_owned(&self) -> StringMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // exact: optional string
  pub fn has_exact(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn exact_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_exact().then(|| self.exact())
  }
  pub fn exact(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // prefix: optional string
  pub fn has_prefix(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn prefix_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_prefix().then(|| self.prefix())
  }
  pub fn prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // suffix: optional string
  pub fn has_suffix(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn suffix_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_suffix().then(|| self.suffix())
  }
  pub fn suffix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // safe_regex: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_safe_regex(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn safe_regex_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'msg>> {
    self.has_safe_regex().then(|| self.safe_regex())
  }
  pub fn safe_regex(self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView::default())
  }

  // contains: optional string
  pub fn has_contains(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn contains_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_contains().then(|| self.contains())
  }
  pub fn contains(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // custom: optional message xds.core.v3.TypedExtensionConfig
  pub fn has_custom(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn custom_opt(self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_custom().then(|| self.custom())
  }
  pub fn custom(self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }

  // ignore_case: optional bool
  pub fn ignore_case(self) -> bool {
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

  pub fn match_pattern(self) -> super::string_matcher::MatchPatternOneof<'msg> {
    match self.match_pattern_case() {
      super::string_matcher::MatchPatternCase::Exact =>
          super::string_matcher::MatchPatternOneof::Exact(self.exact()),
      super::string_matcher::MatchPatternCase::Prefix =>
          super::string_matcher::MatchPatternOneof::Prefix(self.prefix()),
      super::string_matcher::MatchPatternCase::Suffix =>
          super::string_matcher::MatchPatternOneof::Suffix(self.suffix()),
      super::string_matcher::MatchPatternCase::SafeRegex =>
          super::string_matcher::MatchPatternOneof::SafeRegex(self.safe_regex()),
      super::string_matcher::MatchPatternCase::Contains =>
          super::string_matcher::MatchPatternOneof::Contains(self.contains()),
      super::string_matcher::MatchPatternCase::Custom =>
          super::string_matcher::MatchPatternOneof::Custom(self.custom()),
      _ => super::string_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(self) -> super::string_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::string_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StringMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StringMatcherView<'_> {}

// SAFETY:
// - `StringMatcherView` is `Send` because while its alive a `StringMatcherMut` cannot.
// - `StringMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for StringMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for StringMatcherView<'msg> {
  type Proxied = StringMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, StringMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StringMatcherView<'msg> {
  fn into_view<'shorter>(self) -> StringMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StringMatcher> for StringMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StringMatcher {
    let mut dst = StringMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StringMatcher> for StringMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StringMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StringMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StringMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StringMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StringMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StringMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StringMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StringMatcherMut<'msg> {
  type Message = StringMatcher;
}

impl ::std::fmt::Debug for StringMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StringMatcher>> for StringMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StringMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StringMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StringMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StringMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // exact: optional string
  pub fn has_exact(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_exact(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn exact_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_exact().then(|| self.exact())
  }
  pub fn exact(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_exact(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // prefix: optional string
  pub fn has_prefix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_prefix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn prefix_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_prefix().then(|| self.prefix())
  }
  pub fn prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // suffix: optional string
  pub fn has_suffix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_suffix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn suffix_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_suffix().then(|| self.suffix())
  }
  pub fn suffix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_suffix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // safe_regex: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_safe_regex(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_safe_regex(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn safe_regex_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_>> {
    self.has_safe_regex().then(|| self.safe_regex())
  }
  pub fn safe_regex(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView::default())
  }
  pub fn safe_regex_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherMut<'_> {
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
  pub fn set_safe_regex(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // contains: optional string
  pub fn has_contains(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_contains(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn contains_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_contains().then(|| self.contains())
  }
  pub fn contains(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_contains(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // custom: optional message xds.core.v3.TypedExtensionConfig
  pub fn has_custom(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_custom(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn custom_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom().then(|| self.custom())
  }
  pub fn custom(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // ignore_case: optional bool
  pub fn ignore_case(&self) -> bool {
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
  pub fn set_ignore_case(&mut self, val: bool) {
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

  pub fn match_pattern(&self) -> super::string_matcher::MatchPatternOneof<'_> {
    match &self.match_pattern_case() {
      super::string_matcher::MatchPatternCase::Exact =>
          super::string_matcher::MatchPatternOneof::Exact(self.exact()),
      super::string_matcher::MatchPatternCase::Prefix =>
          super::string_matcher::MatchPatternOneof::Prefix(self.prefix()),
      super::string_matcher::MatchPatternCase::Suffix =>
          super::string_matcher::MatchPatternOneof::Suffix(self.suffix()),
      super::string_matcher::MatchPatternCase::SafeRegex =>
          super::string_matcher::MatchPatternOneof::SafeRegex(self.safe_regex()),
      super::string_matcher::MatchPatternCase::Contains =>
          super::string_matcher::MatchPatternOneof::Contains(self.contains()),
      super::string_matcher::MatchPatternCase::Custom =>
          super::string_matcher::MatchPatternOneof::Custom(self.custom()),
      _ => super::string_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(&self) -> super::string_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::string_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StringMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StringMatcherMut<'_> {}

// SAFETY:
// - `StringMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StringMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for StringMatcherMut<'msg> {
  type Proxied = StringMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, StringMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StringMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StringMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StringMatcherMut<'msg> {
  type MutProxied = StringMatcher;
  fn as_mut(&mut self) -> StringMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StringMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> StringMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StringMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StringMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StringMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StringMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // exact: optional string
  pub fn has_exact(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_exact(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn exact_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_exact().then(|| self.exact())
  }
  pub fn exact(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_exact(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // prefix: optional string
  pub fn has_prefix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_prefix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn prefix_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_prefix().then(|| self.prefix())
  }
  pub fn prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // suffix: optional string
  pub fn has_suffix(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_suffix(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn suffix_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_suffix().then(|| self.suffix())
  }
  pub fn suffix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_suffix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // safe_regex: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_safe_regex(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_safe_regex(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn safe_regex_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_>> {
    self.has_safe_regex().then(|| self.safe_regex())
  }
  pub fn safe_regex(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView::default())
  }
  pub fn safe_regex_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherMut<'_> {
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
  pub fn set_safe_regex(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // contains: optional string
  pub fn has_contains(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_contains(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn contains_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_contains().then(|| self.contains())
  }
  pub fn contains(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_contains(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // custom: optional message xds.core.v3.TypedExtensionConfig
  pub fn has_custom(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_custom(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn custom_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom().then(|| self.custom())
  }
  pub fn custom(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // ignore_case: optional bool
  pub fn ignore_case(&self) -> bool {
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
  pub fn set_ignore_case(&mut self, val: bool) {
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

  pub fn match_pattern(&self) -> super::string_matcher::MatchPatternOneof<'_> {
    match &self.match_pattern_case() {
      super::string_matcher::MatchPatternCase::Exact =>
          super::string_matcher::MatchPatternOneof::Exact(self.exact()),
      super::string_matcher::MatchPatternCase::Prefix =>
          super::string_matcher::MatchPatternOneof::Prefix(self.prefix()),
      super::string_matcher::MatchPatternCase::Suffix =>
          super::string_matcher::MatchPatternOneof::Suffix(self.suffix()),
      super::string_matcher::MatchPatternCase::SafeRegex =>
          super::string_matcher::MatchPatternOneof::SafeRegex(self.safe_regex()),
      super::string_matcher::MatchPatternCase::Contains =>
          super::string_matcher::MatchPatternOneof::Contains(self.contains()),
      super::string_matcher::MatchPatternCase::Custom =>
          super::string_matcher::MatchPatternOneof::Custom(self.custom()),
      _ => super::string_matcher::MatchPatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_pattern_case(&self) -> super::string_matcher::MatchPatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::string_matcher::MatchPatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl StringMatcher

impl ::std::ops::Drop for StringMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StringMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StringMatcher {
  type Proxied = Self;
  fn as_view(&self) -> StringMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StringMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StringMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StringMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__StringMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T1T1Ta3/P1T3^!|#|$|&|)|*");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__StringMatcher_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__StringMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StringMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StringMatcher {
  type Msg = StringMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StringMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StringMatcher {
  type Msg = StringMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StringMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StringMatcherMut<'_> {
  type Msg = StringMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StringMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StringMatcherMut<'_> {
  type Msg = StringMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StringMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StringMatcherView<'_> {
  type Msg = StringMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StringMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StringMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod string_matcher {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MatchPatternOneof<'msg> {
  Exact(&'msg ::protobuf::ProtoStr) = 1,
  Prefix(&'msg ::protobuf::ProtoStr) = 2,
  Suffix(&'msg ::protobuf::ProtoStr) = 3,
  SafeRegex(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcher>) = 5,
  Contains(&'msg ::protobuf::ProtoStr) = 7,
  Custom(::protobuf::View<'msg, crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) = 8,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MatchPatternCase {
  Exact = 1,
  Prefix = 2,
  Suffix = 3,
  SafeRegex = 5,
  Contains = 7,
  Custom = 8,

  not_set = 0
}

impl MatchPatternCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MatchPatternCase> {
    match v {
      0 => Some(MatchPatternCase::not_set),
      1 => Some(MatchPatternCase::Exact),
      2 => Some(MatchPatternCase::Prefix),
      3 => Some(MatchPatternCase::Suffix),
      5 => Some(MatchPatternCase::SafeRegex),
      7 => Some(MatchPatternCase::Contains),
      8 => Some(MatchPatternCase::Custom),
      _ => None
    }
  }
}
}  // pub mod string_matcher


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__ListStringMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListStringMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListStringMatcher>
}

impl ::protobuf::Message for ListStringMatcher {
  type MessageView<'msg> = ListStringMatcherView<'msg>;
  type MessageMut<'msg> = ListStringMatcherMut<'msg>;
}

impl ::std::default::Default for ListStringMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListStringMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListStringMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `ListStringMatcherMut`.
unsafe impl ::std::marker::Sync for ListStringMatcher {}

// SAFETY:
// - `ListStringMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ListStringMatcher {}

impl ::protobuf::Proxied for ListStringMatcher {
  type View<'msg> = ListStringMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListStringMatcher {}

impl ::protobuf::MutProxied for ListStringMatcher {
  type Mut<'msg> = ListStringMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListStringMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListStringMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListStringMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListStringMatcherView<'msg> {
  type Message = ListStringMatcher;
}

impl ::std::fmt::Debug for ListStringMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListStringMatcherView<'_> {
  fn default() -> ListStringMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListStringMatcher>> for ListStringMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListStringMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListStringMatcherView<'msg> {

  pub fn to_owned(&self) -> ListStringMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // patterns: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn patterns(self) -> ::protobuf::RepeatedView<'msg, super::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ListStringMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ListStringMatcherView<'_> {}

// SAFETY:
// - `ListStringMatcherView` is `Send` because while its alive a `ListStringMatcherMut` cannot.
// - `ListStringMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for ListStringMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for ListStringMatcherView<'msg> {
  type Proxied = ListStringMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, ListStringMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListStringMatcherView<'msg> {
  fn into_view<'shorter>(self) -> ListStringMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListStringMatcher> for ListStringMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListStringMatcher {
    let mut dst = ListStringMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListStringMatcher> for ListStringMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListStringMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ListStringMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListStringMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ListStringMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListStringMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListStringMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListStringMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListStringMatcherMut<'msg> {
  type Message = ListStringMatcher;
}

impl ::std::fmt::Debug for ListStringMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListStringMatcher>> for ListStringMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListStringMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListStringMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListStringMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ListStringMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // patterns: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn patterns(&self) -> ::protobuf::RepeatedView<'_, super::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn patterns_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::StringMatcher> {
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
  pub fn set_patterns(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::StringMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ListStringMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ListStringMatcherMut<'_> {}

// SAFETY:
// - `ListStringMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ListStringMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for ListStringMatcherMut<'msg> {
  type Proxied = ListStringMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, ListStringMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListStringMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListStringMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ListStringMatcherMut<'msg> {
  type MutProxied = ListStringMatcher;
  fn as_mut(&mut self) -> ListStringMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListStringMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> ListStringMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListStringMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListStringMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListStringMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListStringMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // patterns: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn patterns(&self) -> ::protobuf::RepeatedView<'_, super::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn patterns_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::StringMatcher> {
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
  pub fn set_patterns(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::StringMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ListStringMatcher

impl ::std::ops::Drop for ListStringMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListStringMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListStringMatcher {
  type Proxied = Self;
  fn as_view(&self) -> ListStringMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListStringMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListStringMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListStringMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__ListStringMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__ListStringMatcher_msg_init.0, &[<super::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__ListStringMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListStringMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListStringMatcher {
  type Msg = ListStringMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListStringMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListStringMatcher {
  type Msg = ListStringMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListStringMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListStringMatcherMut<'_> {
  type Msg = ListStringMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListStringMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListStringMatcherMut<'_> {
  type Msg = ListStringMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListStringMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListStringMatcherView<'_> {
  type Msg = ListStringMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListStringMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListStringMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



