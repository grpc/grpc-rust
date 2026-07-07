const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__FilterStateMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FilterStateMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FilterStateMatcher>
}

impl ::protobuf::Message for FilterStateMatcher {
  type MessageView<'msg> = FilterStateMatcherView<'msg>;
  type MessageMut<'msg> = FilterStateMatcherMut<'msg>;
}

impl ::std::default::Default for FilterStateMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FilterStateMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FilterStateMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `FilterStateMatcherMut`.
unsafe impl ::std::marker::Sync for FilterStateMatcher {}

// SAFETY:
// - `FilterStateMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FilterStateMatcher {}

impl ::protobuf::Proxied for FilterStateMatcher {
  type View<'msg> = FilterStateMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FilterStateMatcher {}

impl ::protobuf::MutProxied for FilterStateMatcher {
  type Mut<'msg> = FilterStateMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FilterStateMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FilterStateMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilterStateMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FilterStateMatcherView<'msg> {
  type Message = FilterStateMatcher;
}

impl ::std::fmt::Debug for FilterStateMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FilterStateMatcherView<'_> {
  fn default() -> FilterStateMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FilterStateMatcher>> for FilterStateMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FilterStateMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilterStateMatcherView<'msg> {

  pub fn to_owned(&self) -> FilterStateMatcher {
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

  // string_match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_string_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn string_match_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_string_match().then(|| self.string_match())
  }
  pub fn string_match(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }

  // address_match: optional message envoy.type.matcher.v3.AddressMatcher
  pub fn has_address_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn address_match_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcherView<'msg>> {
    self.has_address_match().then(|| self.address_match())
  }
  pub fn address_match(self) -> crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcherView::default())
  }

  pub fn matcher(self) -> super::filter_state_matcher::MatcherOneof<'msg> {
    match self.matcher_case() {
      super::filter_state_matcher::MatcherCase::StringMatch =>
          super::filter_state_matcher::MatcherOneof::StringMatch(self.string_match()),
      super::filter_state_matcher::MatcherCase::AddressMatch =>
          super::filter_state_matcher::MatcherOneof::AddressMatch(self.address_match()),
      _ => super::filter_state_matcher::MatcherOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn matcher_case(self) -> super::filter_state_matcher::MatcherCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::filter_state_matcher::MatcherCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FilterStateMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FilterStateMatcherView<'_> {}

// SAFETY:
// - `FilterStateMatcherView` is `Send` because while its alive a `FilterStateMatcherMut` cannot.
// - `FilterStateMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for FilterStateMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for FilterStateMatcherView<'msg> {
  type Proxied = FilterStateMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, FilterStateMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilterStateMatcherView<'msg> {
  fn into_view<'shorter>(self) -> FilterStateMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FilterStateMatcher> for FilterStateMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FilterStateMatcher {
    let mut dst = FilterStateMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FilterStateMatcher> for FilterStateMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FilterStateMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FilterStateMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FilterStateMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FilterStateMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FilterStateMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterStateMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FilterStateMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FilterStateMatcherMut<'msg> {
  type Message = FilterStateMatcher;
}

impl ::std::fmt::Debug for FilterStateMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FilterStateMatcher>> for FilterStateMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterStateMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FilterStateMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FilterStateMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FilterStateMatcher {
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

  // string_match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_string_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_string_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn string_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_string_match().then(|| self.string_match())
  }
  pub fn string_match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn string_match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_string_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // address_match: optional message envoy.type.matcher.v3.AddressMatcher
  pub fn has_address_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_address_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn address_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcherView<'_>> {
    self.has_address_match().then(|| self.address_match())
  }
  pub fn address_match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcherView::default())
  }
  pub fn address_match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcherMut<'_> {
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
  pub fn set_address_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn matcher(&self) -> super::filter_state_matcher::MatcherOneof<'_> {
    match &self.matcher_case() {
      super::filter_state_matcher::MatcherCase::StringMatch =>
          super::filter_state_matcher::MatcherOneof::StringMatch(self.string_match()),
      super::filter_state_matcher::MatcherCase::AddressMatch =>
          super::filter_state_matcher::MatcherOneof::AddressMatch(self.address_match()),
      _ => super::filter_state_matcher::MatcherOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn matcher_case(&self) -> super::filter_state_matcher::MatcherCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::filter_state_matcher::MatcherCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FilterStateMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FilterStateMatcherMut<'_> {}

// SAFETY:
// - `FilterStateMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FilterStateMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for FilterStateMatcherMut<'msg> {
  type Proxied = FilterStateMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, FilterStateMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FilterStateMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FilterStateMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FilterStateMatcherMut<'msg> {
  type MutProxied = FilterStateMatcher;
  fn as_mut(&mut self) -> FilterStateMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FilterStateMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> FilterStateMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FilterStateMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FilterStateMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FilterStateMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FilterStateMatcherMut<'_> {
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

  // string_match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_string_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_string_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn string_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_string_match().then(|| self.string_match())
  }
  pub fn string_match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn string_match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_string_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // address_match: optional message envoy.type.matcher.v3.AddressMatcher
  pub fn has_address_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_address_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn address_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcherView<'_>> {
    self.has_address_match().then(|| self.address_match())
  }
  pub fn address_match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcherView::default())
  }
  pub fn address_match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcherMut<'_> {
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
  pub fn set_address_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn matcher(&self) -> super::filter_state_matcher::MatcherOneof<'_> {
    match &self.matcher_case() {
      super::filter_state_matcher::MatcherCase::StringMatch =>
          super::filter_state_matcher::MatcherOneof::StringMatch(self.string_match()),
      super::filter_state_matcher::MatcherCase::AddressMatch =>
          super::filter_state_matcher::MatcherOneof::AddressMatch(self.address_match()),
      _ => super::filter_state_matcher::MatcherOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn matcher_case(&self) -> super::filter_state_matcher::MatcherCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::filter_state_matcher::MatcherCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl FilterStateMatcher

impl ::std::ops::Drop for FilterStateMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FilterStateMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FilterStateMatcher {
  type Proxied = Self;
  fn as_view(&self) -> FilterStateMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FilterStateMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FilterStateMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FilterStateMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__FilterStateMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X33^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__FilterStateMatcher_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__FilterStateMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FilterStateMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FilterStateMatcher {
  type Msg = FilterStateMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterStateMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterStateMatcher {
  type Msg = FilterStateMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterStateMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FilterStateMatcherMut<'_> {
  type Msg = FilterStateMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterStateMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterStateMatcherMut<'_> {
  type Msg = FilterStateMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterStateMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FilterStateMatcherView<'_> {
  type Msg = FilterStateMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FilterStateMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FilterStateMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod filter_state_matcher {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MatcherOneof<'msg> {
  StringMatch(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) = 2,
  AddressMatch(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::address::AddressMatcher>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MatcherCase {
  StringMatch = 2,
  AddressMatch = 3,

  not_set = 0
}

impl MatcherCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MatcherCase> {
    match v {
      0 => Some(MatcherCase::not_set),
      2 => Some(MatcherCase::StringMatch),
      3 => Some(MatcherCase::AddressMatch),
      _ => None
    }
  }
}
}  // pub mod filter_state_matcher


