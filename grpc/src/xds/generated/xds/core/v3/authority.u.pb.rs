const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__core__v3__Authority_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Authority {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Authority>
}

impl ::protobuf::Message for Authority {
  type MessageView<'msg> = AuthorityView<'msg>;
  type MessageMut<'msg> = AuthorityMut<'msg>;
}

impl ::std::default::Default for Authority {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Authority {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Authority` is `Sync` because it does not implement interior mutability.
//    Neither does `AuthorityMut`.
unsafe impl ::std::marker::Sync for Authority {}

// SAFETY:
// - `Authority` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Authority {}

impl ::protobuf::Proxied for Authority {
  type View<'msg> = AuthorityView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Authority {}

impl ::protobuf::MutProxied for Authority {
  type Mut<'msg> = AuthorityMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AuthorityView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Authority>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuthorityView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AuthorityView<'msg> {
  type Message = Authority;
}

impl ::std::fmt::Debug for AuthorityView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AuthorityView<'_> {
  fn default() -> AuthorityView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Authority>> for AuthorityView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Authority>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuthorityView<'msg> {

  pub fn to_owned(&self) -> Authority {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `AuthorityView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AuthorityView<'_> {}

// SAFETY:
// - `AuthorityView` is `Send` because while its alive a `AuthorityMut` cannot.
// - `AuthorityView` does not use thread-local data.
unsafe impl ::std::marker::Send for AuthorityView<'_> {}

impl<'msg> ::protobuf::AsView for AuthorityView<'msg> {
  type Proxied = Authority;
  fn as_view(&self) -> ::protobuf::View<'msg, Authority> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuthorityView<'msg> {
  fn into_view<'shorter>(self) -> AuthorityView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Authority> for AuthorityView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Authority {
    let mut dst = Authority::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Authority> for AuthorityMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Authority {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Authority {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuthorityView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AuthorityMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AuthorityMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Authority>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AuthorityMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AuthorityMut<'msg> {
  type Message = Authority;
}

impl ::std::fmt::Debug for AuthorityMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Authority>> for AuthorityMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Authority>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AuthorityMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Authority> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Authority {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `AuthorityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AuthorityMut<'_> {}

// SAFETY:
// - `AuthorityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AuthorityMut<'_> {}

impl<'msg> ::protobuf::AsView for AuthorityMut<'msg> {
  type Proxied = Authority;
  fn as_view(&self) -> ::protobuf::View<'_, Authority> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AuthorityMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Authority>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AuthorityMut<'msg> {
  type MutProxied = Authority;
  fn as_mut(&mut self) -> AuthorityMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AuthorityMut<'msg> {
  fn into_mut<'shorter>(self) -> AuthorityMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Authority {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Authority> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AuthorityView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AuthorityMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl Authority

impl ::std::ops::Drop for Authority {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Authority {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Authority {
  type Proxied = Self;
  fn as_view(&self) -> AuthorityView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Authority {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AuthorityMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Authority {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__core__v3__Authority_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__core__v3__Authority_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__core__v3__Authority_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Authority {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Authority {
  type Msg = Authority;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Authority> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Authority {
  type Msg = Authority;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Authority> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AuthorityMut<'_> {
  type Msg = Authority;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Authority> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthorityMut<'_> {
  type Msg = Authority;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Authority> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AuthorityView<'_> {
  type Msg = Authority;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Authority> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AuthorityMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



