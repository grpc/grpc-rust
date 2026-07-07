const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__api__Http_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Http {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Http>
}

impl ::protobuf::Message for Http {
  type MessageView<'msg> = HttpView<'msg>;
  type MessageMut<'msg> = HttpMut<'msg>;
}

impl ::std::default::Default for Http {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Http {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Http` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpMut`.
unsafe impl ::std::marker::Sync for Http {}

// SAFETY:
// - `Http` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Http {}

impl ::protobuf::Proxied for Http {
  type View<'msg> = HttpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Http {}

impl ::protobuf::MutProxied for Http {
  type Mut<'msg> = HttpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpView<'msg> {
  type Message = Http;
}

impl ::std::fmt::Debug for HttpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpView<'_> {
  fn default() -> HttpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Http>> for HttpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpView<'msg> {

  pub fn to_owned(&self) -> Http {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // rules: repeated message google.api.HttpRule
  pub fn rules(self) -> ::protobuf::RepeatedView<'msg, super::HttpRule> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HttpRule>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // fully_decode_reserved_expansion: optional bool
  pub fn fully_decode_reserved_expansion(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `HttpView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpView<'_> {}

// SAFETY:
// - `HttpView` is `Send` because while its alive a `HttpMut` cannot.
// - `HttpView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpView<'_> {}

impl<'msg> ::protobuf::AsView for HttpView<'msg> {
  type Proxied = Http;
  fn as_view(&self) -> ::protobuf::View<'msg, Http> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpView<'msg> {
  fn into_view<'shorter>(self) -> HttpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Http> for HttpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http {
    let mut dst = Http::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Http> for HttpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Http {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpMut<'msg> {
  type Message = Http;
}

impl ::std::fmt::Debug for HttpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Http>> for HttpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Http> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Http {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // rules: repeated message google.api.HttpRule
  pub fn rules(&self) -> ::protobuf::RepeatedView<'_, super::HttpRule> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HttpRule>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn rules_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HttpRule> {
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
  pub fn set_rules(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HttpRule>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // fully_decode_reserved_expansion: optional bool
  pub fn fully_decode_reserved_expansion(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fully_decode_reserved_expansion(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `HttpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpMut<'_> {}

// SAFETY:
// - `HttpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpMut<'msg> {
  type Proxied = Http;
  fn as_view(&self) -> ::protobuf::View<'_, Http> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Http>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpMut<'msg> {
  type MutProxied = Http;
  fn as_mut(&mut self) -> HttpMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Http {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Http> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // rules: repeated message google.api.HttpRule
  pub fn rules(&self) -> ::protobuf::RepeatedView<'_, super::HttpRule> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HttpRule>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn rules_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HttpRule> {
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
  pub fn set_rules(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HttpRule>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // fully_decode_reserved_expansion: optional bool
  pub fn fully_decode_reserved_expansion(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fully_decode_reserved_expansion(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

}  // impl Http

impl ::std::ops::Drop for Http {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Http {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Http {
  type Proxied = Self;
  fn as_view(&self) -> HttpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Http {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Http {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__api__Http_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__api__Http_msg_init.0, &[<super::HttpRule as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__api__Http_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Http {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Http {
  type Msg = Http;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http {
  type Msg = Http;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpMut<'_> {
  type Msg = Http;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpMut<'_> {
  type Msg = Http;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpView<'_> {
  type Msg = Http;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__api__HttpRule_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpRule {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpRule>
}

impl ::protobuf::Message for HttpRule {
  type MessageView<'msg> = HttpRuleView<'msg>;
  type MessageMut<'msg> = HttpRuleMut<'msg>;
}

impl ::std::default::Default for HttpRule {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpRule {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpRule` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpRuleMut`.
unsafe impl ::std::marker::Sync for HttpRule {}

// SAFETY:
// - `HttpRule` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpRule {}

impl ::protobuf::Proxied for HttpRule {
  type View<'msg> = HttpRuleView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpRule {}

impl ::protobuf::MutProxied for HttpRule {
  type Mut<'msg> = HttpRuleMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpRuleView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRule>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRuleView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpRuleView<'msg> {
  type Message = HttpRule;
}

impl ::std::fmt::Debug for HttpRuleView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpRuleView<'_> {
  fn default() -> HttpRuleView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRule>> for HttpRuleView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRule>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRuleView<'msg> {

  pub fn to_owned(&self) -> HttpRule {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // selector: optional string
  pub fn selector(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // get: optional string
  pub fn has_get(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn get_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_get().then(|| self.get())
  }
  pub fn get(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // put: optional string
  pub fn has_put(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn put_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_put().then(|| self.put())
  }
  pub fn put(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // post: optional string
  pub fn has_post(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn post_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_post().then(|| self.post())
  }
  pub fn post(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // delete: optional string
  pub fn has_delete(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn delete_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_delete().then(|| self.delete())
  }
  pub fn delete(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // patch: optional string
  pub fn has_patch(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn patch_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_patch().then(|| self.patch())
  }
  pub fn patch(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // custom: optional message google.api.CustomHttpPattern
  pub fn has_custom(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn custom_opt(self) -> ::std::option::Option<super::CustomHttpPatternView<'msg>> {
    self.has_custom().then(|| self.custom())
  }
  pub fn custom(self) -> super::CustomHttpPatternView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CustomHttpPatternView::default())
  }

  // body: optional string
  pub fn body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // response_body: optional string
  pub fn response_body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // additional_bindings: repeated message google.api.HttpRule
  pub fn additional_bindings(self) -> ::protobuf::RepeatedView<'msg, super::HttpRule> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HttpRule>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  pub fn pattern(self) -> super::http_rule::PatternOneof<'msg> {
    match self.pattern_case() {
      super::http_rule::PatternCase::Get =>
          super::http_rule::PatternOneof::Get(self.get()),
      super::http_rule::PatternCase::Put =>
          super::http_rule::PatternOneof::Put(self.put()),
      super::http_rule::PatternCase::Post =>
          super::http_rule::PatternOneof::Post(self.post()),
      super::http_rule::PatternCase::Delete =>
          super::http_rule::PatternOneof::Delete(self.delete()),
      super::http_rule::PatternCase::Patch =>
          super::http_rule::PatternOneof::Patch(self.patch()),
      super::http_rule::PatternCase::Custom =>
          super::http_rule::PatternOneof::Custom(self.custom()),
      _ => super::http_rule::PatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn pattern_case(self) -> super::http_rule::PatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::http_rule::PatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HttpRuleView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpRuleView<'_> {}

// SAFETY:
// - `HttpRuleView` is `Send` because while its alive a `HttpRuleMut` cannot.
// - `HttpRuleView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpRuleView<'_> {}

impl<'msg> ::protobuf::AsView for HttpRuleView<'msg> {
  type Proxied = HttpRule;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpRule> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRuleView<'msg> {
  fn into_view<'shorter>(self) -> HttpRuleView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRule> for HttpRuleView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRule {
    let mut dst = HttpRule::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRule> for HttpRuleMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRule {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpRule {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpRuleView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpRuleMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpRuleMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRule>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRuleMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpRuleMut<'msg> {
  type Message = HttpRule;
}

impl ::std::fmt::Debug for HttpRuleMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRule>> for HttpRuleMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRule>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRuleMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRule> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpRule {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // selector: optional string
  pub fn selector(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_selector(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // get: optional string
  pub fn has_get(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_get(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn get_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_get().then(|| self.get())
  }
  pub fn get(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_get(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // put: optional string
  pub fn has_put(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_put(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn put_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_put().then(|| self.put())
  }
  pub fn put(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_put(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // post: optional string
  pub fn has_post(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_post(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn post_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_post().then(|| self.post())
  }
  pub fn post(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_post(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // delete: optional string
  pub fn has_delete(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_delete(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn delete_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_delete().then(|| self.delete())
  }
  pub fn delete(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_delete(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // patch: optional string
  pub fn has_patch(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_patch(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn patch_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_patch().then(|| self.patch())
  }
  pub fn patch(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_patch(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // custom: optional message google.api.CustomHttpPattern
  pub fn has_custom(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_custom(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn custom_opt(&self) -> ::std::option::Option<super::CustomHttpPatternView<'_>> {
    self.has_custom().then(|| self.custom())
  }
  pub fn custom(&self) -> super::CustomHttpPatternView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CustomHttpPatternView::default())
  }
  pub fn custom_mut(&mut self) -> super::CustomHttpPatternMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_custom(&mut self,
    val: impl ::protobuf::IntoProxied<super::CustomHttpPattern>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // body: optional string
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // response_body: optional string
  pub fn response_body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_response_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // additional_bindings: repeated message google.api.HttpRule
  pub fn additional_bindings(&self) -> ::protobuf::RepeatedView<'_, super::HttpRule> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HttpRule>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn additional_bindings_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HttpRule> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        8,
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
  pub fn set_additional_bindings(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HttpRule>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  pub fn pattern(&self) -> super::http_rule::PatternOneof<'_> {
    match &self.pattern_case() {
      super::http_rule::PatternCase::Get =>
          super::http_rule::PatternOneof::Get(self.get()),
      super::http_rule::PatternCase::Put =>
          super::http_rule::PatternOneof::Put(self.put()),
      super::http_rule::PatternCase::Post =>
          super::http_rule::PatternOneof::Post(self.post()),
      super::http_rule::PatternCase::Delete =>
          super::http_rule::PatternOneof::Delete(self.delete()),
      super::http_rule::PatternCase::Patch =>
          super::http_rule::PatternOneof::Patch(self.patch()),
      super::http_rule::PatternCase::Custom =>
          super::http_rule::PatternOneof::Custom(self.custom()),
      _ => super::http_rule::PatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn pattern_case(&self) -> super::http_rule::PatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::http_rule::PatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HttpRuleMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpRuleMut<'_> {}

// SAFETY:
// - `HttpRuleMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpRuleMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpRuleMut<'msg> {
  type Proxied = HttpRule;
  fn as_view(&self) -> ::protobuf::View<'_, HttpRule> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRuleMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpRule>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpRuleMut<'msg> {
  type MutProxied = HttpRule;
  fn as_mut(&mut self) -> HttpRuleMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpRuleMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpRuleMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpRule {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpRule> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpRuleView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpRuleMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // selector: optional string
  pub fn selector(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_selector(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // get: optional string
  pub fn has_get(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_get(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn get_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_get().then(|| self.get())
  }
  pub fn get(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_get(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // put: optional string
  pub fn has_put(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_put(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn put_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_put().then(|| self.put())
  }
  pub fn put(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_put(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // post: optional string
  pub fn has_post(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_post(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn post_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_post().then(|| self.post())
  }
  pub fn post(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_post(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // delete: optional string
  pub fn has_delete(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_delete(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn delete_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_delete().then(|| self.delete())
  }
  pub fn delete(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_delete(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // patch: optional string
  pub fn has_patch(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_patch(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn patch_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_patch().then(|| self.patch())
  }
  pub fn patch(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_patch(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // custom: optional message google.api.CustomHttpPattern
  pub fn has_custom(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_custom(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn custom_opt(&self) -> ::std::option::Option<super::CustomHttpPatternView<'_>> {
    self.has_custom().then(|| self.custom())
  }
  pub fn custom(&self) -> super::CustomHttpPatternView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CustomHttpPatternView::default())
  }
  pub fn custom_mut(&mut self) -> super::CustomHttpPatternMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_custom(&mut self,
    val: impl ::protobuf::IntoProxied<super::CustomHttpPattern>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // body: optional string
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // response_body: optional string
  pub fn response_body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_response_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // additional_bindings: repeated message google.api.HttpRule
  pub fn additional_bindings(&self) -> ::protobuf::RepeatedView<'_, super::HttpRule> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HttpRule>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn additional_bindings_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HttpRule> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        8,
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
  pub fn set_additional_bindings(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HttpRule>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  pub fn pattern(&self) -> super::http_rule::PatternOneof<'_> {
    match &self.pattern_case() {
      super::http_rule::PatternCase::Get =>
          super::http_rule::PatternOneof::Get(self.get()),
      super::http_rule::PatternCase::Put =>
          super::http_rule::PatternOneof::Put(self.put()),
      super::http_rule::PatternCase::Post =>
          super::http_rule::PatternOneof::Post(self.post()),
      super::http_rule::PatternCase::Delete =>
          super::http_rule::PatternOneof::Delete(self.delete()),
      super::http_rule::PatternCase::Patch =>
          super::http_rule::PatternOneof::Patch(self.patch()),
      super::http_rule::PatternCase::Custom =>
          super::http_rule::PatternOneof::Custom(self.custom()),
      _ => super::http_rule::PatternOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn pattern_case(&self) -> super::http_rule::PatternCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::http_rule::PatternCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl HttpRule

impl ::std::ops::Drop for HttpRule {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpRule {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpRule {
  type Proxied = Self;
  fn as_view(&self) -> HttpRuleView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpRule {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpRuleMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpRule {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__api__HttpRule_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1T1T1T1T1T1X3bG1X^#|$|%|&|(|*");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__api__HttpRule_msg_init.0, &[<super::CustomHttpPattern as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::google__api__HttpRule_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__api__HttpRule_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRule {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRule {
  type Msg = HttpRule;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRule> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRule {
  type Msg = HttpRule;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRule> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRuleMut<'_> {
  type Msg = HttpRule;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRule> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRuleMut<'_> {
  type Msg = HttpRule;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRule> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRuleView<'_> {
  type Msg = HttpRule;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRule> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRuleMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod http_rule {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum PatternOneof<'msg> {
  Get(&'msg ::protobuf::ProtoStr) = 2,
  Put(&'msg ::protobuf::ProtoStr) = 3,
  Post(&'msg ::protobuf::ProtoStr) = 4,
  Delete(&'msg ::protobuf::ProtoStr) = 5,
  Patch(&'msg ::protobuf::ProtoStr) = 6,
  Custom(::protobuf::View<'msg, super::super::CustomHttpPattern>) = 8,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum PatternCase {
  Get = 2,
  Put = 3,
  Post = 4,
  Delete = 5,
  Patch = 6,
  Custom = 8,

  not_set = 0
}

impl PatternCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<PatternCase> {
    match v {
      0 => Some(PatternCase::not_set),
      2 => Some(PatternCase::Get),
      3 => Some(PatternCase::Put),
      4 => Some(PatternCase::Post),
      5 => Some(PatternCase::Delete),
      6 => Some(PatternCase::Patch),
      8 => Some(PatternCase::Custom),
      _ => None
    }
  }
}
}  // pub mod http_rule


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__api__CustomHttpPattern_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CustomHttpPattern {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CustomHttpPattern>
}

impl ::protobuf::Message for CustomHttpPattern {
  type MessageView<'msg> = CustomHttpPatternView<'msg>;
  type MessageMut<'msg> = CustomHttpPatternMut<'msg>;
}

impl ::std::default::Default for CustomHttpPattern {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CustomHttpPattern {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CustomHttpPattern` is `Sync` because it does not implement interior mutability.
//    Neither does `CustomHttpPatternMut`.
unsafe impl ::std::marker::Sync for CustomHttpPattern {}

// SAFETY:
// - `CustomHttpPattern` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CustomHttpPattern {}

impl ::protobuf::Proxied for CustomHttpPattern {
  type View<'msg> = CustomHttpPatternView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CustomHttpPattern {}

impl ::protobuf::MutProxied for CustomHttpPattern {
  type Mut<'msg> = CustomHttpPatternMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CustomHttpPatternView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CustomHttpPattern>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CustomHttpPatternView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CustomHttpPatternView<'msg> {
  type Message = CustomHttpPattern;
}

impl ::std::fmt::Debug for CustomHttpPatternView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CustomHttpPatternView<'_> {
  fn default() -> CustomHttpPatternView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CustomHttpPattern>> for CustomHttpPatternView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CustomHttpPattern>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CustomHttpPatternView<'msg> {

  pub fn to_owned(&self) -> CustomHttpPattern {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // kind: optional string
  pub fn kind(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // path: optional string
  pub fn path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `CustomHttpPatternView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CustomHttpPatternView<'_> {}

// SAFETY:
// - `CustomHttpPatternView` is `Send` because while its alive a `CustomHttpPatternMut` cannot.
// - `CustomHttpPatternView` does not use thread-local data.
unsafe impl ::std::marker::Send for CustomHttpPatternView<'_> {}

impl<'msg> ::protobuf::AsView for CustomHttpPatternView<'msg> {
  type Proxied = CustomHttpPattern;
  fn as_view(&self) -> ::protobuf::View<'msg, CustomHttpPattern> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CustomHttpPatternView<'msg> {
  fn into_view<'shorter>(self) -> CustomHttpPatternView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CustomHttpPattern> for CustomHttpPatternView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CustomHttpPattern {
    let mut dst = CustomHttpPattern::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CustomHttpPattern> for CustomHttpPatternMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CustomHttpPattern {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CustomHttpPattern {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CustomHttpPatternView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CustomHttpPatternMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CustomHttpPatternMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomHttpPattern>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CustomHttpPatternMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CustomHttpPatternMut<'msg> {
  type Message = CustomHttpPattern;
}

impl ::std::fmt::Debug for CustomHttpPatternMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CustomHttpPattern>> for CustomHttpPatternMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomHttpPattern>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CustomHttpPatternMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomHttpPattern> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CustomHttpPattern {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // kind: optional string
  pub fn kind(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_kind(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `CustomHttpPatternMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CustomHttpPatternMut<'_> {}

// SAFETY:
// - `CustomHttpPatternMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CustomHttpPatternMut<'_> {}

impl<'msg> ::protobuf::AsView for CustomHttpPatternMut<'msg> {
  type Proxied = CustomHttpPattern;
  fn as_view(&self) -> ::protobuf::View<'_, CustomHttpPattern> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CustomHttpPatternMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CustomHttpPattern>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CustomHttpPatternMut<'msg> {
  type MutProxied = CustomHttpPattern;
  fn as_mut(&mut self) -> CustomHttpPatternMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CustomHttpPatternMut<'msg> {
  fn into_mut<'shorter>(self) -> CustomHttpPatternMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CustomHttpPattern {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CustomHttpPattern> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CustomHttpPatternView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CustomHttpPatternMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // kind: optional string
  pub fn kind(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_kind(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl CustomHttpPattern

impl ::std::ops::Drop for CustomHttpPattern {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CustomHttpPattern {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CustomHttpPattern {
  type Proxied = Self;
  fn as_view(&self) -> CustomHttpPatternView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CustomHttpPattern {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CustomHttpPatternMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CustomHttpPattern {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__api__CustomHttpPattern_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__api__CustomHttpPattern_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__api__CustomHttpPattern_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CustomHttpPattern {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CustomHttpPattern {
  type Msg = CustomHttpPattern;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomHttpPattern> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomHttpPattern {
  type Msg = CustomHttpPattern;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomHttpPattern> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CustomHttpPatternMut<'_> {
  type Msg = CustomHttpPattern;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomHttpPattern> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomHttpPatternMut<'_> {
  type Msg = CustomHttpPattern;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomHttpPattern> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomHttpPatternView<'_> {
  type Msg = CustomHttpPattern;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomHttpPattern> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CustomHttpPatternMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



