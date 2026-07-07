const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__type__matcher__v3__CelMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CelMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CelMatcher>
}

impl ::protobuf::Message for CelMatcher {
  type MessageView<'msg> = CelMatcherView<'msg>;
  type MessageMut<'msg> = CelMatcherMut<'msg>;
}

impl ::std::default::Default for CelMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CelMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CelMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `CelMatcherMut`.
unsafe impl ::std::marker::Sync for CelMatcher {}

// SAFETY:
// - `CelMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CelMatcher {}

impl ::protobuf::Proxied for CelMatcher {
  type View<'msg> = CelMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CelMatcher {}

impl ::protobuf::MutProxied for CelMatcher {
  type Mut<'msg> = CelMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CelMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CelMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CelMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CelMatcherView<'msg> {
  type Message = CelMatcher;
}

impl ::std::fmt::Debug for CelMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CelMatcherView<'_> {
  fn default() -> CelMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CelMatcher>> for CelMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CelMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CelMatcherView<'msg> {

  pub fn to_owned(&self) -> CelMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // expr_match: optional message xds.type.v3.CelExpression
  pub fn has_expr_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn expr_match_opt(self) -> ::std::option::Option<crate::xds::generated::xds::r#type::v3::cel::CelExpressionView<'msg>> {
    self.has_expr_match().then(|| self.expr_match())
  }
  pub fn expr_match(self) -> crate::xds::generated::xds::r#type::v3::cel::CelExpressionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::v3::cel::CelExpressionView::default())
  }

  // description: optional string
  pub fn description(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `CelMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CelMatcherView<'_> {}

// SAFETY:
// - `CelMatcherView` is `Send` because while its alive a `CelMatcherMut` cannot.
// - `CelMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for CelMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for CelMatcherView<'msg> {
  type Proxied = CelMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, CelMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CelMatcherView<'msg> {
  fn into_view<'shorter>(self) -> CelMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CelMatcher> for CelMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CelMatcher {
    let mut dst = CelMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CelMatcher> for CelMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CelMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CelMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CelMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CelMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CelMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CelMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CelMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CelMatcherMut<'msg> {
  type Message = CelMatcher;
}

impl ::std::fmt::Debug for CelMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CelMatcher>> for CelMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CelMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CelMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CelMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CelMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // expr_match: optional message xds.type.v3.CelExpression
  pub fn has_expr_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_expr_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn expr_match_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::v3::cel::CelExpressionView<'_>> {
    self.has_expr_match().then(|| self.expr_match())
  }
  pub fn expr_match(&self) -> crate::xds::generated::xds::r#type::v3::cel::CelExpressionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::v3::cel::CelExpressionView::default())
  }
  pub fn expr_match_mut(&mut self) -> crate::xds::generated::xds::r#type::v3::cel::CelExpressionMut<'_> {
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
  pub fn set_expr_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::v3::cel::CelExpression>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // description: optional string
  pub fn description(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_description(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `CelMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CelMatcherMut<'_> {}

// SAFETY:
// - `CelMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CelMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for CelMatcherMut<'msg> {
  type Proxied = CelMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, CelMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CelMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CelMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CelMatcherMut<'msg> {
  type MutProxied = CelMatcher;
  fn as_mut(&mut self) -> CelMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CelMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> CelMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CelMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CelMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CelMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CelMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // expr_match: optional message xds.type.v3.CelExpression
  pub fn has_expr_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_expr_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn expr_match_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::v3::cel::CelExpressionView<'_>> {
    self.has_expr_match().then(|| self.expr_match())
  }
  pub fn expr_match(&self) -> crate::xds::generated::xds::r#type::v3::cel::CelExpressionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::v3::cel::CelExpressionView::default())
  }
  pub fn expr_match_mut(&mut self) -> crate::xds::generated::xds::r#type::v3::cel::CelExpressionMut<'_> {
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
  pub fn set_expr_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::v3::cel::CelExpression>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // description: optional string
  pub fn description(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_description(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl CelMatcher

impl ::std::ops::Drop for CelMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CelMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CelMatcher {
  type Proxied = Self;
  fn as_view(&self) -> CelMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CelMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CelMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CelMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__type__matcher__v3__CelMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__type__matcher__v3__CelMatcher_msg_init.0, &[<crate::xds::generated::xds::r#type::v3::cel::CelExpression as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__type__matcher__v3__CelMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CelMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CelMatcher {
  type Msg = CelMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelMatcher {
  type Msg = CelMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CelMatcherMut<'_> {
  type Msg = CelMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelMatcherMut<'_> {
  type Msg = CelMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CelMatcherView<'_> {
  type Msg = CelMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CelMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CelMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



