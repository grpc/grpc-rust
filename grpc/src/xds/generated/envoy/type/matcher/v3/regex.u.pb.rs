const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__RegexMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RegexMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RegexMatcher>
}

impl ::protobuf::Message for RegexMatcher {
  type MessageView<'msg> = RegexMatcherView<'msg>;
  type MessageMut<'msg> = RegexMatcherMut<'msg>;
}

impl ::std::default::Default for RegexMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RegexMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RegexMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `RegexMatcherMut`.
unsafe impl ::std::marker::Sync for RegexMatcher {}

// SAFETY:
// - `RegexMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RegexMatcher {}

impl ::protobuf::Proxied for RegexMatcher {
  type View<'msg> = RegexMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RegexMatcher {}

impl ::protobuf::MutProxied for RegexMatcher {
  type Mut<'msg> = RegexMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RegexMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RegexMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegexMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RegexMatcherView<'msg> {
  type Message = RegexMatcher;
}

impl ::std::fmt::Debug for RegexMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RegexMatcherView<'_> {
  fn default() -> RegexMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RegexMatcher>> for RegexMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RegexMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RegexMatcherView<'msg> {

  pub fn to_owned(&self) -> RegexMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // google_re2: optional message envoy.type.matcher.v3.RegexMatcher.GoogleRE2
  pub fn has_google_re2(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn google_re2_opt(self) -> ::std::option::Option<super::regex_matcher::GoogleRE2View<'msg>> {
    self.has_google_re2().then(|| self.google_re2())
  }
  pub fn google_re2(self) -> super::regex_matcher::GoogleRE2View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::regex_matcher::GoogleRE2View::default())
  }

  // regex: optional string
  pub fn regex(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn engine_type(self) -> super::regex_matcher::EngineTypeOneof<'msg> {
    match self.engine_type_case() {
      super::regex_matcher::EngineTypeCase::GoogleRe2 =>
          super::regex_matcher::EngineTypeOneof::GoogleRe2(self.google_re2()),
      _ => super::regex_matcher::EngineTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn engine_type_case(self) -> super::regex_matcher::EngineTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::regex_matcher::EngineTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RegexMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RegexMatcherView<'_> {}

// SAFETY:
// - `RegexMatcherView` is `Send` because while its alive a `RegexMatcherMut` cannot.
// - `RegexMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for RegexMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for RegexMatcherView<'msg> {
  type Proxied = RegexMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, RegexMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegexMatcherView<'msg> {
  fn into_view<'shorter>(self) -> RegexMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RegexMatcher> for RegexMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegexMatcher {
    let mut dst = RegexMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RegexMatcher> for RegexMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegexMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RegexMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RegexMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RegexMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RegexMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RegexMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegexMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RegexMatcherMut<'msg> {
  type Message = RegexMatcher;
}

impl ::std::fmt::Debug for RegexMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RegexMatcher>> for RegexMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RegexMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RegexMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RegexMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RegexMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // google_re2: optional message envoy.type.matcher.v3.RegexMatcher.GoogleRE2
  pub fn has_google_re2(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_google_re2(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn google_re2_opt(&self) -> ::std::option::Option<super::regex_matcher::GoogleRE2View<'_>> {
    self.has_google_re2().then(|| self.google_re2())
  }
  pub fn google_re2(&self) -> super::regex_matcher::GoogleRE2View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::regex_matcher::GoogleRE2View::default())
  }
  pub fn google_re2_mut(&mut self) -> super::regex_matcher::GoogleRE2Mut<'_> {
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
  pub fn set_google_re2(&mut self,
    val: impl ::protobuf::IntoProxied<super::regex_matcher::GoogleRE2>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // regex: optional string
  pub fn regex(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_regex(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn engine_type(&self) -> super::regex_matcher::EngineTypeOneof<'_> {
    match &self.engine_type_case() {
      super::regex_matcher::EngineTypeCase::GoogleRe2 =>
          super::regex_matcher::EngineTypeOneof::GoogleRe2(self.google_re2()),
      _ => super::regex_matcher::EngineTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn engine_type_case(&self) -> super::regex_matcher::EngineTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::regex_matcher::EngineTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RegexMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RegexMatcherMut<'_> {}

// SAFETY:
// - `RegexMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RegexMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for RegexMatcherMut<'msg> {
  type Proxied = RegexMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, RegexMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegexMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RegexMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RegexMatcherMut<'msg> {
  type MutProxied = RegexMatcher;
  fn as_mut(&mut self) -> RegexMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RegexMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> RegexMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RegexMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RegexMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RegexMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RegexMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // google_re2: optional message envoy.type.matcher.v3.RegexMatcher.GoogleRE2
  pub fn has_google_re2(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_google_re2(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn google_re2_opt(&self) -> ::std::option::Option<super::regex_matcher::GoogleRE2View<'_>> {
    self.has_google_re2().then(|| self.google_re2())
  }
  pub fn google_re2(&self) -> super::regex_matcher::GoogleRE2View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::regex_matcher::GoogleRE2View::default())
  }
  pub fn google_re2_mut(&mut self) -> super::regex_matcher::GoogleRE2Mut<'_> {
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
  pub fn set_google_re2(&mut self,
    val: impl ::protobuf::IntoProxied<super::regex_matcher::GoogleRE2>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // regex: optional string
  pub fn regex(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_regex(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn engine_type(&self) -> super::regex_matcher::EngineTypeOneof<'_> {
    match &self.engine_type_case() {
      super::regex_matcher::EngineTypeCase::GoogleRe2 =>
          super::regex_matcher::EngineTypeOneof::GoogleRe2(self.google_re2()),
      _ => super::regex_matcher::EngineTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn engine_type_case(&self) -> super::regex_matcher::EngineTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::regex_matcher::EngineTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl RegexMatcher

impl ::std::ops::Drop for RegexMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RegexMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RegexMatcher {
  type Proxied = Self;
  fn as_view(&self) -> RegexMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RegexMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RegexMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RegexMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__RegexMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X^!");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__RegexMatcher_msg_init.0, &[<super::regex_matcher::GoogleRE2 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__RegexMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RegexMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RegexMatcher {
  type Msg = RegexMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RegexMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RegexMatcher {
  type Msg = RegexMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RegexMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RegexMatcherMut<'_> {
  type Msg = RegexMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RegexMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RegexMatcherMut<'_> {
  type Msg = RegexMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RegexMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RegexMatcherView<'_> {
  type Msg = RegexMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RegexMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RegexMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod regex_matcher {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__RegexMatcher__GoogleRE2_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GoogleRE2 {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GoogleRE2>
}

impl ::protobuf::Message for GoogleRE2 {
  type MessageView<'msg> = GoogleRE2View<'msg>;
  type MessageMut<'msg> = GoogleRE2Mut<'msg>;
}

impl ::std::default::Default for GoogleRE2 {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GoogleRE2 {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GoogleRE2` is `Sync` because it does not implement interior mutability.
//    Neither does `GoogleRE2Mut`.
unsafe impl ::std::marker::Sync for GoogleRE2 {}

// SAFETY:
// - `GoogleRE2` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GoogleRE2 {}

impl ::protobuf::Proxied for GoogleRE2 {
  type View<'msg> = GoogleRE2View<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GoogleRE2 {}

impl ::protobuf::MutProxied for GoogleRE2 {
  type Mut<'msg> = GoogleRE2Mut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GoogleRE2View<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleRE2>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleRE2View<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GoogleRE2View<'msg> {
  type Message = GoogleRE2;
}

impl ::std::fmt::Debug for GoogleRE2View<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GoogleRE2View<'_> {
  fn default() -> GoogleRE2View<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleRE2>> for GoogleRE2View<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GoogleRE2>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleRE2View<'msg> {

  pub fn to_owned(&self) -> GoogleRE2 {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // max_program_size: optional message google.protobuf.UInt32Value
  pub fn has_max_program_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn max_program_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_program_size().then(|| self.max_program_size())
  }
  pub fn max_program_size(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `GoogleRE2View` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GoogleRE2View<'_> {}

// SAFETY:
// - `GoogleRE2View` is `Send` because while its alive a `GoogleRE2Mut` cannot.
// - `GoogleRE2View` does not use thread-local data.
unsafe impl ::std::marker::Send for GoogleRE2View<'_> {}

impl<'msg> ::protobuf::AsView for GoogleRE2View<'msg> {
  type Proxied = GoogleRE2;
  fn as_view(&self) -> ::protobuf::View<'msg, GoogleRE2> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleRE2View<'msg> {
  fn into_view<'shorter>(self) -> GoogleRE2View<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleRE2> for GoogleRE2View<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleRE2 {
    let mut dst = GoogleRE2::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GoogleRE2> for GoogleRE2Mut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GoogleRE2 {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GoogleRE2 {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GoogleRE2View<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GoogleRE2Mut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GoogleRE2Mut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleRE2>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GoogleRE2Mut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GoogleRE2Mut<'msg> {
  type Message = GoogleRE2;
}

impl ::std::fmt::Debug for GoogleRE2Mut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleRE2>> for GoogleRE2Mut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleRE2>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GoogleRE2Mut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GoogleRE2> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GoogleRE2 {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // max_program_size: optional message google.protobuf.UInt32Value
  pub fn has_max_program_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_program_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_program_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_program_size().then(|| self.max_program_size())
  }
  pub fn max_program_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_program_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_program_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `GoogleRE2Mut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GoogleRE2Mut<'_> {}

// SAFETY:
// - `GoogleRE2Mut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GoogleRE2Mut<'_> {}

impl<'msg> ::protobuf::AsView for GoogleRE2Mut<'msg> {
  type Proxied = GoogleRE2;
  fn as_view(&self) -> ::protobuf::View<'_, GoogleRE2> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GoogleRE2Mut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GoogleRE2>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GoogleRE2Mut<'msg> {
  type MutProxied = GoogleRE2;
  fn as_mut(&mut self) -> GoogleRE2Mut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GoogleRE2Mut<'msg> {
  fn into_mut<'shorter>(self) -> GoogleRE2Mut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GoogleRE2 {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GoogleRE2> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GoogleRE2View<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GoogleRE2Mut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // max_program_size: optional message google.protobuf.UInt32Value
  pub fn has_max_program_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_program_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_program_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_program_size().then(|| self.max_program_size())
  }
  pub fn max_program_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_program_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_program_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl GoogleRE2

impl ::std::ops::Drop for GoogleRE2 {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GoogleRE2 {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GoogleRE2 {
  type Proxied = Self;
  fn as_view(&self) -> GoogleRE2View<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GoogleRE2 {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GoogleRE2Mut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GoogleRE2 {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::regex_matcher::envoy__type__matcher__v3__RegexMatcher__GoogleRE2_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::regex_matcher::envoy__type__matcher__v3__RegexMatcher__GoogleRE2_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::regex_matcher::envoy__type__matcher__v3__RegexMatcher__GoogleRE2_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleRE2 {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleRE2 {
  type Msg = GoogleRE2;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleRE2> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleRE2 {
  type Msg = GoogleRE2;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleRE2> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GoogleRE2Mut<'_> {
  type Msg = GoogleRE2;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleRE2> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleRE2Mut<'_> {
  type Msg = GoogleRE2;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleRE2> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GoogleRE2View<'_> {
  type Msg = GoogleRE2;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GoogleRE2> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GoogleRE2Mut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum EngineTypeOneof<'msg> {
  GoogleRe2(::protobuf::View<'msg, super::super::regex_matcher::GoogleRE2>) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum EngineTypeCase {
  GoogleRe2 = 1,

  not_set = 0
}

impl EngineTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<EngineTypeCase> {
    match v {
      0 => Some(EngineTypeCase::not_set),
      1 => Some(EngineTypeCase::GoogleRe2),
      _ => None
    }
  }
}
}  // pub mod regex_matcher


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__RegexMatchAndSubstitute_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RegexMatchAndSubstitute {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RegexMatchAndSubstitute>
}

impl ::protobuf::Message for RegexMatchAndSubstitute {
  type MessageView<'msg> = RegexMatchAndSubstituteView<'msg>;
  type MessageMut<'msg> = RegexMatchAndSubstituteMut<'msg>;
}

impl ::std::default::Default for RegexMatchAndSubstitute {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RegexMatchAndSubstitute {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RegexMatchAndSubstitute` is `Sync` because it does not implement interior mutability.
//    Neither does `RegexMatchAndSubstituteMut`.
unsafe impl ::std::marker::Sync for RegexMatchAndSubstitute {}

// SAFETY:
// - `RegexMatchAndSubstitute` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RegexMatchAndSubstitute {}

impl ::protobuf::Proxied for RegexMatchAndSubstitute {
  type View<'msg> = RegexMatchAndSubstituteView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RegexMatchAndSubstitute {}

impl ::protobuf::MutProxied for RegexMatchAndSubstitute {
  type Mut<'msg> = RegexMatchAndSubstituteMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RegexMatchAndSubstituteView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RegexMatchAndSubstitute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegexMatchAndSubstituteView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RegexMatchAndSubstituteView<'msg> {
  type Message = RegexMatchAndSubstitute;
}

impl ::std::fmt::Debug for RegexMatchAndSubstituteView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RegexMatchAndSubstituteView<'_> {
  fn default() -> RegexMatchAndSubstituteView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RegexMatchAndSubstitute>> for RegexMatchAndSubstituteView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RegexMatchAndSubstitute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RegexMatchAndSubstituteView<'msg> {

  pub fn to_owned(&self) -> RegexMatchAndSubstitute {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // pattern: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_pattern(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn pattern_opt(self) -> ::std::option::Option<super::RegexMatcherView<'msg>> {
    self.has_pattern().then(|| self.pattern())
  }
  pub fn pattern(self) -> super::RegexMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RegexMatcherView::default())
  }

  // substitution: optional string
  pub fn substitution(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `RegexMatchAndSubstituteView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RegexMatchAndSubstituteView<'_> {}

// SAFETY:
// - `RegexMatchAndSubstituteView` is `Send` because while its alive a `RegexMatchAndSubstituteMut` cannot.
// - `RegexMatchAndSubstituteView` does not use thread-local data.
unsafe impl ::std::marker::Send for RegexMatchAndSubstituteView<'_> {}

impl<'msg> ::protobuf::AsView for RegexMatchAndSubstituteView<'msg> {
  type Proxied = RegexMatchAndSubstitute;
  fn as_view(&self) -> ::protobuf::View<'msg, RegexMatchAndSubstitute> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegexMatchAndSubstituteView<'msg> {
  fn into_view<'shorter>(self) -> RegexMatchAndSubstituteView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RegexMatchAndSubstitute> for RegexMatchAndSubstituteView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegexMatchAndSubstitute {
    let mut dst = RegexMatchAndSubstitute::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RegexMatchAndSubstitute> for RegexMatchAndSubstituteMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RegexMatchAndSubstitute {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RegexMatchAndSubstitute {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RegexMatchAndSubstituteView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RegexMatchAndSubstituteMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RegexMatchAndSubstituteMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RegexMatchAndSubstitute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RegexMatchAndSubstituteMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RegexMatchAndSubstituteMut<'msg> {
  type Message = RegexMatchAndSubstitute;
}

impl ::std::fmt::Debug for RegexMatchAndSubstituteMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RegexMatchAndSubstitute>> for RegexMatchAndSubstituteMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RegexMatchAndSubstitute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RegexMatchAndSubstituteMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RegexMatchAndSubstitute> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RegexMatchAndSubstitute {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // pattern: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_pattern(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_pattern(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn pattern_opt(&self) -> ::std::option::Option<super::RegexMatcherView<'_>> {
    self.has_pattern().then(|| self.pattern())
  }
  pub fn pattern(&self) -> super::RegexMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RegexMatcherView::default())
  }
  pub fn pattern_mut(&mut self) -> super::RegexMatcherMut<'_> {
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
  pub fn set_pattern(&mut self,
    val: impl ::protobuf::IntoProxied<super::RegexMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // substitution: optional string
  pub fn substitution(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_substitution(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `RegexMatchAndSubstituteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RegexMatchAndSubstituteMut<'_> {}

// SAFETY:
// - `RegexMatchAndSubstituteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RegexMatchAndSubstituteMut<'_> {}

impl<'msg> ::protobuf::AsView for RegexMatchAndSubstituteMut<'msg> {
  type Proxied = RegexMatchAndSubstitute;
  fn as_view(&self) -> ::protobuf::View<'_, RegexMatchAndSubstitute> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RegexMatchAndSubstituteMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RegexMatchAndSubstitute>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RegexMatchAndSubstituteMut<'msg> {
  type MutProxied = RegexMatchAndSubstitute;
  fn as_mut(&mut self) -> RegexMatchAndSubstituteMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RegexMatchAndSubstituteMut<'msg> {
  fn into_mut<'shorter>(self) -> RegexMatchAndSubstituteMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RegexMatchAndSubstitute {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RegexMatchAndSubstitute> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RegexMatchAndSubstituteView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RegexMatchAndSubstituteMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // pattern: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_pattern(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_pattern(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn pattern_opt(&self) -> ::std::option::Option<super::RegexMatcherView<'_>> {
    self.has_pattern().then(|| self.pattern())
  }
  pub fn pattern(&self) -> super::RegexMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RegexMatcherView::default())
  }
  pub fn pattern_mut(&mut self) -> super::RegexMatcherMut<'_> {
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
  pub fn set_pattern(&mut self,
    val: impl ::protobuf::IntoProxied<super::RegexMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // substitution: optional string
  pub fn substitution(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_substitution(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl RegexMatchAndSubstitute

impl ::std::ops::Drop for RegexMatchAndSubstitute {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RegexMatchAndSubstitute {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RegexMatchAndSubstitute {
  type Proxied = Self;
  fn as_view(&self) -> RegexMatchAndSubstituteView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RegexMatchAndSubstitute {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RegexMatchAndSubstituteMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RegexMatchAndSubstitute {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__RegexMatchAndSubstitute_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__RegexMatchAndSubstitute_msg_init.0, &[<super::RegexMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__RegexMatchAndSubstitute_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RegexMatchAndSubstitute {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RegexMatchAndSubstitute {
  type Msg = RegexMatchAndSubstitute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RegexMatchAndSubstitute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RegexMatchAndSubstitute {
  type Msg = RegexMatchAndSubstitute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RegexMatchAndSubstitute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RegexMatchAndSubstituteMut<'_> {
  type Msg = RegexMatchAndSubstitute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RegexMatchAndSubstitute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RegexMatchAndSubstituteMut<'_> {
  type Msg = RegexMatchAndSubstitute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RegexMatchAndSubstitute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RegexMatchAndSubstituteView<'_> {
  type Msg = RegexMatchAndSubstitute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RegexMatchAndSubstitute> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RegexMatchAndSubstituteMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



