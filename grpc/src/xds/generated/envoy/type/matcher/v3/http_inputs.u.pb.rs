const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__HttpRequestHeaderMatchInput_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpRequestHeaderMatchInput {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpRequestHeaderMatchInput>
}

impl ::protobuf::Message for HttpRequestHeaderMatchInput {
  type MessageView<'msg> = HttpRequestHeaderMatchInputView<'msg>;
  type MessageMut<'msg> = HttpRequestHeaderMatchInputMut<'msg>;
}

impl ::std::default::Default for HttpRequestHeaderMatchInput {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpRequestHeaderMatchInput {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpRequestHeaderMatchInput` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpRequestHeaderMatchInputMut`.
unsafe impl ::std::marker::Sync for HttpRequestHeaderMatchInput {}

// SAFETY:
// - `HttpRequestHeaderMatchInput` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpRequestHeaderMatchInput {}

impl ::protobuf::Proxied for HttpRequestHeaderMatchInput {
  type View<'msg> = HttpRequestHeaderMatchInputView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpRequestHeaderMatchInput {}

impl ::protobuf::MutProxied for HttpRequestHeaderMatchInput {
  type Mut<'msg> = HttpRequestHeaderMatchInputMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpRequestHeaderMatchInputView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequestHeaderMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRequestHeaderMatchInputView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpRequestHeaderMatchInputView<'msg> {
  type Message = HttpRequestHeaderMatchInput;
}

impl ::std::fmt::Debug for HttpRequestHeaderMatchInputView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpRequestHeaderMatchInputView<'_> {
  fn default() -> HttpRequestHeaderMatchInputView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequestHeaderMatchInput>> for HttpRequestHeaderMatchInputView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequestHeaderMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRequestHeaderMatchInputView<'msg> {

  pub fn to_owned(&self) -> HttpRequestHeaderMatchInput {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // header_name: optional string
  pub fn header_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `HttpRequestHeaderMatchInputView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpRequestHeaderMatchInputView<'_> {}

// SAFETY:
// - `HttpRequestHeaderMatchInputView` is `Send` because while its alive a `HttpRequestHeaderMatchInputMut` cannot.
// - `HttpRequestHeaderMatchInputView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpRequestHeaderMatchInputView<'_> {}

impl<'msg> ::protobuf::AsView for HttpRequestHeaderMatchInputView<'msg> {
  type Proxied = HttpRequestHeaderMatchInput;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpRequestHeaderMatchInput> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRequestHeaderMatchInputView<'msg> {
  fn into_view<'shorter>(self) -> HttpRequestHeaderMatchInputView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRequestHeaderMatchInput> for HttpRequestHeaderMatchInputView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRequestHeaderMatchInput {
    let mut dst = HttpRequestHeaderMatchInput::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRequestHeaderMatchInput> for HttpRequestHeaderMatchInputMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRequestHeaderMatchInput {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpRequestHeaderMatchInput {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpRequestHeaderMatchInputView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpRequestHeaderMatchInputMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpRequestHeaderMatchInputMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestHeaderMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRequestHeaderMatchInputMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpRequestHeaderMatchInputMut<'msg> {
  type Message = HttpRequestHeaderMatchInput;
}

impl ::std::fmt::Debug for HttpRequestHeaderMatchInputMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestHeaderMatchInput>> for HttpRequestHeaderMatchInputMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestHeaderMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRequestHeaderMatchInputMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestHeaderMatchInput> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpRequestHeaderMatchInput {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // header_name: optional string
  pub fn header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `HttpRequestHeaderMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpRequestHeaderMatchInputMut<'_> {}

// SAFETY:
// - `HttpRequestHeaderMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpRequestHeaderMatchInputMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpRequestHeaderMatchInputMut<'msg> {
  type Proxied = HttpRequestHeaderMatchInput;
  fn as_view(&self) -> ::protobuf::View<'_, HttpRequestHeaderMatchInput> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRequestHeaderMatchInputMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpRequestHeaderMatchInput>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpRequestHeaderMatchInputMut<'msg> {
  type MutProxied = HttpRequestHeaderMatchInput;
  fn as_mut(&mut self) -> HttpRequestHeaderMatchInputMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpRequestHeaderMatchInputMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpRequestHeaderMatchInputMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpRequestHeaderMatchInput {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpRequestHeaderMatchInput> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpRequestHeaderMatchInputView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpRequestHeaderMatchInputMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // header_name: optional string
  pub fn header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl HttpRequestHeaderMatchInput

impl ::std::ops::Drop for HttpRequestHeaderMatchInput {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpRequestHeaderMatchInput {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpRequestHeaderMatchInput {
  type Proxied = Self;
  fn as_view(&self) -> HttpRequestHeaderMatchInputView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpRequestHeaderMatchInput {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpRequestHeaderMatchInputMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpRequestHeaderMatchInput {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__HttpRequestHeaderMatchInput_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__HttpRequestHeaderMatchInput_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__HttpRequestHeaderMatchInput_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRequestHeaderMatchInput {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRequestHeaderMatchInput {
  type Msg = HttpRequestHeaderMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestHeaderMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestHeaderMatchInput {
  type Msg = HttpRequestHeaderMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestHeaderMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRequestHeaderMatchInputMut<'_> {
  type Msg = HttpRequestHeaderMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestHeaderMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestHeaderMatchInputMut<'_> {
  type Msg = HttpRequestHeaderMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestHeaderMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestHeaderMatchInputView<'_> {
  type Msg = HttpRequestHeaderMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestHeaderMatchInput> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRequestHeaderMatchInputMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__HttpRequestTrailerMatchInput_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpRequestTrailerMatchInput {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpRequestTrailerMatchInput>
}

impl ::protobuf::Message for HttpRequestTrailerMatchInput {
  type MessageView<'msg> = HttpRequestTrailerMatchInputView<'msg>;
  type MessageMut<'msg> = HttpRequestTrailerMatchInputMut<'msg>;
}

impl ::std::default::Default for HttpRequestTrailerMatchInput {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpRequestTrailerMatchInput {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpRequestTrailerMatchInput` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpRequestTrailerMatchInputMut`.
unsafe impl ::std::marker::Sync for HttpRequestTrailerMatchInput {}

// SAFETY:
// - `HttpRequestTrailerMatchInput` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpRequestTrailerMatchInput {}

impl ::protobuf::Proxied for HttpRequestTrailerMatchInput {
  type View<'msg> = HttpRequestTrailerMatchInputView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpRequestTrailerMatchInput {}

impl ::protobuf::MutProxied for HttpRequestTrailerMatchInput {
  type Mut<'msg> = HttpRequestTrailerMatchInputMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpRequestTrailerMatchInputView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequestTrailerMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRequestTrailerMatchInputView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpRequestTrailerMatchInputView<'msg> {
  type Message = HttpRequestTrailerMatchInput;
}

impl ::std::fmt::Debug for HttpRequestTrailerMatchInputView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpRequestTrailerMatchInputView<'_> {
  fn default() -> HttpRequestTrailerMatchInputView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequestTrailerMatchInput>> for HttpRequestTrailerMatchInputView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequestTrailerMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRequestTrailerMatchInputView<'msg> {

  pub fn to_owned(&self) -> HttpRequestTrailerMatchInput {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // header_name: optional string
  pub fn header_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `HttpRequestTrailerMatchInputView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpRequestTrailerMatchInputView<'_> {}

// SAFETY:
// - `HttpRequestTrailerMatchInputView` is `Send` because while its alive a `HttpRequestTrailerMatchInputMut` cannot.
// - `HttpRequestTrailerMatchInputView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpRequestTrailerMatchInputView<'_> {}

impl<'msg> ::protobuf::AsView for HttpRequestTrailerMatchInputView<'msg> {
  type Proxied = HttpRequestTrailerMatchInput;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpRequestTrailerMatchInput> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRequestTrailerMatchInputView<'msg> {
  fn into_view<'shorter>(self) -> HttpRequestTrailerMatchInputView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRequestTrailerMatchInput> for HttpRequestTrailerMatchInputView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRequestTrailerMatchInput {
    let mut dst = HttpRequestTrailerMatchInput::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRequestTrailerMatchInput> for HttpRequestTrailerMatchInputMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRequestTrailerMatchInput {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpRequestTrailerMatchInput {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpRequestTrailerMatchInputView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpRequestTrailerMatchInputMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpRequestTrailerMatchInputMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestTrailerMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRequestTrailerMatchInputMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpRequestTrailerMatchInputMut<'msg> {
  type Message = HttpRequestTrailerMatchInput;
}

impl ::std::fmt::Debug for HttpRequestTrailerMatchInputMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestTrailerMatchInput>> for HttpRequestTrailerMatchInputMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestTrailerMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRequestTrailerMatchInputMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestTrailerMatchInput> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpRequestTrailerMatchInput {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // header_name: optional string
  pub fn header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `HttpRequestTrailerMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpRequestTrailerMatchInputMut<'_> {}

// SAFETY:
// - `HttpRequestTrailerMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpRequestTrailerMatchInputMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpRequestTrailerMatchInputMut<'msg> {
  type Proxied = HttpRequestTrailerMatchInput;
  fn as_view(&self) -> ::protobuf::View<'_, HttpRequestTrailerMatchInput> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRequestTrailerMatchInputMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpRequestTrailerMatchInput>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpRequestTrailerMatchInputMut<'msg> {
  type MutProxied = HttpRequestTrailerMatchInput;
  fn as_mut(&mut self) -> HttpRequestTrailerMatchInputMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpRequestTrailerMatchInputMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpRequestTrailerMatchInputMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpRequestTrailerMatchInput {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpRequestTrailerMatchInput> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpRequestTrailerMatchInputView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpRequestTrailerMatchInputMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // header_name: optional string
  pub fn header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl HttpRequestTrailerMatchInput

impl ::std::ops::Drop for HttpRequestTrailerMatchInput {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpRequestTrailerMatchInput {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpRequestTrailerMatchInput {
  type Proxied = Self;
  fn as_view(&self) -> HttpRequestTrailerMatchInputView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpRequestTrailerMatchInput {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpRequestTrailerMatchInputMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpRequestTrailerMatchInput {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__HttpRequestTrailerMatchInput_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__HttpRequestTrailerMatchInput_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__HttpRequestTrailerMatchInput_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRequestTrailerMatchInput {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRequestTrailerMatchInput {
  type Msg = HttpRequestTrailerMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestTrailerMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestTrailerMatchInput {
  type Msg = HttpRequestTrailerMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestTrailerMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRequestTrailerMatchInputMut<'_> {
  type Msg = HttpRequestTrailerMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestTrailerMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestTrailerMatchInputMut<'_> {
  type Msg = HttpRequestTrailerMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestTrailerMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestTrailerMatchInputView<'_> {
  type Msg = HttpRequestTrailerMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestTrailerMatchInput> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRequestTrailerMatchInputMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__HttpResponseHeaderMatchInput_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpResponseHeaderMatchInput {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpResponseHeaderMatchInput>
}

impl ::protobuf::Message for HttpResponseHeaderMatchInput {
  type MessageView<'msg> = HttpResponseHeaderMatchInputView<'msg>;
  type MessageMut<'msg> = HttpResponseHeaderMatchInputMut<'msg>;
}

impl ::std::default::Default for HttpResponseHeaderMatchInput {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpResponseHeaderMatchInput {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpResponseHeaderMatchInput` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpResponseHeaderMatchInputMut`.
unsafe impl ::std::marker::Sync for HttpResponseHeaderMatchInput {}

// SAFETY:
// - `HttpResponseHeaderMatchInput` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpResponseHeaderMatchInput {}

impl ::protobuf::Proxied for HttpResponseHeaderMatchInput {
  type View<'msg> = HttpResponseHeaderMatchInputView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpResponseHeaderMatchInput {}

impl ::protobuf::MutProxied for HttpResponseHeaderMatchInput {
  type Mut<'msg> = HttpResponseHeaderMatchInputMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpResponseHeaderMatchInputView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpResponseHeaderMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpResponseHeaderMatchInputView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpResponseHeaderMatchInputView<'msg> {
  type Message = HttpResponseHeaderMatchInput;
}

impl ::std::fmt::Debug for HttpResponseHeaderMatchInputView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpResponseHeaderMatchInputView<'_> {
  fn default() -> HttpResponseHeaderMatchInputView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpResponseHeaderMatchInput>> for HttpResponseHeaderMatchInputView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpResponseHeaderMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpResponseHeaderMatchInputView<'msg> {

  pub fn to_owned(&self) -> HttpResponseHeaderMatchInput {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // header_name: optional string
  pub fn header_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `HttpResponseHeaderMatchInputView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpResponseHeaderMatchInputView<'_> {}

// SAFETY:
// - `HttpResponseHeaderMatchInputView` is `Send` because while its alive a `HttpResponseHeaderMatchInputMut` cannot.
// - `HttpResponseHeaderMatchInputView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpResponseHeaderMatchInputView<'_> {}

impl<'msg> ::protobuf::AsView for HttpResponseHeaderMatchInputView<'msg> {
  type Proxied = HttpResponseHeaderMatchInput;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpResponseHeaderMatchInput> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpResponseHeaderMatchInputView<'msg> {
  fn into_view<'shorter>(self) -> HttpResponseHeaderMatchInputView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpResponseHeaderMatchInput> for HttpResponseHeaderMatchInputView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpResponseHeaderMatchInput {
    let mut dst = HttpResponseHeaderMatchInput::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpResponseHeaderMatchInput> for HttpResponseHeaderMatchInputMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpResponseHeaderMatchInput {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpResponseHeaderMatchInput {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpResponseHeaderMatchInputView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpResponseHeaderMatchInputMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpResponseHeaderMatchInputMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpResponseHeaderMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpResponseHeaderMatchInputMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpResponseHeaderMatchInputMut<'msg> {
  type Message = HttpResponseHeaderMatchInput;
}

impl ::std::fmt::Debug for HttpResponseHeaderMatchInputMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpResponseHeaderMatchInput>> for HttpResponseHeaderMatchInputMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpResponseHeaderMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpResponseHeaderMatchInputMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpResponseHeaderMatchInput> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpResponseHeaderMatchInput {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // header_name: optional string
  pub fn header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `HttpResponseHeaderMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpResponseHeaderMatchInputMut<'_> {}

// SAFETY:
// - `HttpResponseHeaderMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpResponseHeaderMatchInputMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpResponseHeaderMatchInputMut<'msg> {
  type Proxied = HttpResponseHeaderMatchInput;
  fn as_view(&self) -> ::protobuf::View<'_, HttpResponseHeaderMatchInput> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpResponseHeaderMatchInputMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpResponseHeaderMatchInput>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpResponseHeaderMatchInputMut<'msg> {
  type MutProxied = HttpResponseHeaderMatchInput;
  fn as_mut(&mut self) -> HttpResponseHeaderMatchInputMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpResponseHeaderMatchInputMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpResponseHeaderMatchInputMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpResponseHeaderMatchInput {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpResponseHeaderMatchInput> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpResponseHeaderMatchInputView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpResponseHeaderMatchInputMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // header_name: optional string
  pub fn header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl HttpResponseHeaderMatchInput

impl ::std::ops::Drop for HttpResponseHeaderMatchInput {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpResponseHeaderMatchInput {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpResponseHeaderMatchInput {
  type Proxied = Self;
  fn as_view(&self) -> HttpResponseHeaderMatchInputView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpResponseHeaderMatchInput {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpResponseHeaderMatchInputMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpResponseHeaderMatchInput {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__HttpResponseHeaderMatchInput_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__HttpResponseHeaderMatchInput_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__HttpResponseHeaderMatchInput_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpResponseHeaderMatchInput {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpResponseHeaderMatchInput {
  type Msg = HttpResponseHeaderMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpResponseHeaderMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpResponseHeaderMatchInput {
  type Msg = HttpResponseHeaderMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpResponseHeaderMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpResponseHeaderMatchInputMut<'_> {
  type Msg = HttpResponseHeaderMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpResponseHeaderMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpResponseHeaderMatchInputMut<'_> {
  type Msg = HttpResponseHeaderMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpResponseHeaderMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpResponseHeaderMatchInputView<'_> {
  type Msg = HttpResponseHeaderMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpResponseHeaderMatchInput> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpResponseHeaderMatchInputMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__HttpResponseTrailerMatchInput_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpResponseTrailerMatchInput {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpResponseTrailerMatchInput>
}

impl ::protobuf::Message for HttpResponseTrailerMatchInput {
  type MessageView<'msg> = HttpResponseTrailerMatchInputView<'msg>;
  type MessageMut<'msg> = HttpResponseTrailerMatchInputMut<'msg>;
}

impl ::std::default::Default for HttpResponseTrailerMatchInput {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpResponseTrailerMatchInput {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpResponseTrailerMatchInput` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpResponseTrailerMatchInputMut`.
unsafe impl ::std::marker::Sync for HttpResponseTrailerMatchInput {}

// SAFETY:
// - `HttpResponseTrailerMatchInput` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpResponseTrailerMatchInput {}

impl ::protobuf::Proxied for HttpResponseTrailerMatchInput {
  type View<'msg> = HttpResponseTrailerMatchInputView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpResponseTrailerMatchInput {}

impl ::protobuf::MutProxied for HttpResponseTrailerMatchInput {
  type Mut<'msg> = HttpResponseTrailerMatchInputMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpResponseTrailerMatchInputView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpResponseTrailerMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpResponseTrailerMatchInputView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpResponseTrailerMatchInputView<'msg> {
  type Message = HttpResponseTrailerMatchInput;
}

impl ::std::fmt::Debug for HttpResponseTrailerMatchInputView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpResponseTrailerMatchInputView<'_> {
  fn default() -> HttpResponseTrailerMatchInputView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpResponseTrailerMatchInput>> for HttpResponseTrailerMatchInputView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpResponseTrailerMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpResponseTrailerMatchInputView<'msg> {

  pub fn to_owned(&self) -> HttpResponseTrailerMatchInput {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // header_name: optional string
  pub fn header_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `HttpResponseTrailerMatchInputView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpResponseTrailerMatchInputView<'_> {}

// SAFETY:
// - `HttpResponseTrailerMatchInputView` is `Send` because while its alive a `HttpResponseTrailerMatchInputMut` cannot.
// - `HttpResponseTrailerMatchInputView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpResponseTrailerMatchInputView<'_> {}

impl<'msg> ::protobuf::AsView for HttpResponseTrailerMatchInputView<'msg> {
  type Proxied = HttpResponseTrailerMatchInput;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpResponseTrailerMatchInput> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpResponseTrailerMatchInputView<'msg> {
  fn into_view<'shorter>(self) -> HttpResponseTrailerMatchInputView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpResponseTrailerMatchInput> for HttpResponseTrailerMatchInputView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpResponseTrailerMatchInput {
    let mut dst = HttpResponseTrailerMatchInput::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpResponseTrailerMatchInput> for HttpResponseTrailerMatchInputMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpResponseTrailerMatchInput {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpResponseTrailerMatchInput {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpResponseTrailerMatchInputView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpResponseTrailerMatchInputMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpResponseTrailerMatchInputMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpResponseTrailerMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpResponseTrailerMatchInputMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpResponseTrailerMatchInputMut<'msg> {
  type Message = HttpResponseTrailerMatchInput;
}

impl ::std::fmt::Debug for HttpResponseTrailerMatchInputMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpResponseTrailerMatchInput>> for HttpResponseTrailerMatchInputMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpResponseTrailerMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpResponseTrailerMatchInputMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpResponseTrailerMatchInput> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpResponseTrailerMatchInput {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // header_name: optional string
  pub fn header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `HttpResponseTrailerMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpResponseTrailerMatchInputMut<'_> {}

// SAFETY:
// - `HttpResponseTrailerMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpResponseTrailerMatchInputMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpResponseTrailerMatchInputMut<'msg> {
  type Proxied = HttpResponseTrailerMatchInput;
  fn as_view(&self) -> ::protobuf::View<'_, HttpResponseTrailerMatchInput> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpResponseTrailerMatchInputMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpResponseTrailerMatchInput>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpResponseTrailerMatchInputMut<'msg> {
  type MutProxied = HttpResponseTrailerMatchInput;
  fn as_mut(&mut self) -> HttpResponseTrailerMatchInputMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpResponseTrailerMatchInputMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpResponseTrailerMatchInputMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpResponseTrailerMatchInput {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpResponseTrailerMatchInput> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpResponseTrailerMatchInputView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpResponseTrailerMatchInputMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // header_name: optional string
  pub fn header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl HttpResponseTrailerMatchInput

impl ::std::ops::Drop for HttpResponseTrailerMatchInput {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpResponseTrailerMatchInput {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpResponseTrailerMatchInput {
  type Proxied = Self;
  fn as_view(&self) -> HttpResponseTrailerMatchInputView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpResponseTrailerMatchInput {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpResponseTrailerMatchInputMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpResponseTrailerMatchInput {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__HttpResponseTrailerMatchInput_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__HttpResponseTrailerMatchInput_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__HttpResponseTrailerMatchInput_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpResponseTrailerMatchInput {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpResponseTrailerMatchInput {
  type Msg = HttpResponseTrailerMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpResponseTrailerMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpResponseTrailerMatchInput {
  type Msg = HttpResponseTrailerMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpResponseTrailerMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpResponseTrailerMatchInputMut<'_> {
  type Msg = HttpResponseTrailerMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpResponseTrailerMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpResponseTrailerMatchInputMut<'_> {
  type Msg = HttpResponseTrailerMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpResponseTrailerMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpResponseTrailerMatchInputView<'_> {
  type Msg = HttpResponseTrailerMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpResponseTrailerMatchInput> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpResponseTrailerMatchInputMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__HttpRequestQueryParamMatchInput_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpRequestQueryParamMatchInput {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpRequestQueryParamMatchInput>
}

impl ::protobuf::Message for HttpRequestQueryParamMatchInput {
  type MessageView<'msg> = HttpRequestQueryParamMatchInputView<'msg>;
  type MessageMut<'msg> = HttpRequestQueryParamMatchInputMut<'msg>;
}

impl ::std::default::Default for HttpRequestQueryParamMatchInput {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpRequestQueryParamMatchInput {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpRequestQueryParamMatchInput` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpRequestQueryParamMatchInputMut`.
unsafe impl ::std::marker::Sync for HttpRequestQueryParamMatchInput {}

// SAFETY:
// - `HttpRequestQueryParamMatchInput` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpRequestQueryParamMatchInput {}

impl ::protobuf::Proxied for HttpRequestQueryParamMatchInput {
  type View<'msg> = HttpRequestQueryParamMatchInputView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpRequestQueryParamMatchInput {}

impl ::protobuf::MutProxied for HttpRequestQueryParamMatchInput {
  type Mut<'msg> = HttpRequestQueryParamMatchInputMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpRequestQueryParamMatchInputView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequestQueryParamMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRequestQueryParamMatchInputView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpRequestQueryParamMatchInputView<'msg> {
  type Message = HttpRequestQueryParamMatchInput;
}

impl ::std::fmt::Debug for HttpRequestQueryParamMatchInputView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpRequestQueryParamMatchInputView<'_> {
  fn default() -> HttpRequestQueryParamMatchInputView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequestQueryParamMatchInput>> for HttpRequestQueryParamMatchInputView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpRequestQueryParamMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRequestQueryParamMatchInputView<'msg> {

  pub fn to_owned(&self) -> HttpRequestQueryParamMatchInput {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // query_param: optional string
  pub fn query_param(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `HttpRequestQueryParamMatchInputView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpRequestQueryParamMatchInputView<'_> {}

// SAFETY:
// - `HttpRequestQueryParamMatchInputView` is `Send` because while its alive a `HttpRequestQueryParamMatchInputMut` cannot.
// - `HttpRequestQueryParamMatchInputView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpRequestQueryParamMatchInputView<'_> {}

impl<'msg> ::protobuf::AsView for HttpRequestQueryParamMatchInputView<'msg> {
  type Proxied = HttpRequestQueryParamMatchInput;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpRequestQueryParamMatchInput> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRequestQueryParamMatchInputView<'msg> {
  fn into_view<'shorter>(self) -> HttpRequestQueryParamMatchInputView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRequestQueryParamMatchInput> for HttpRequestQueryParamMatchInputView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRequestQueryParamMatchInput {
    let mut dst = HttpRequestQueryParamMatchInput::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpRequestQueryParamMatchInput> for HttpRequestQueryParamMatchInputMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpRequestQueryParamMatchInput {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpRequestQueryParamMatchInput {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpRequestQueryParamMatchInputView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpRequestQueryParamMatchInputMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpRequestQueryParamMatchInputMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestQueryParamMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpRequestQueryParamMatchInputMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpRequestQueryParamMatchInputMut<'msg> {
  type Message = HttpRequestQueryParamMatchInput;
}

impl ::std::fmt::Debug for HttpRequestQueryParamMatchInputMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestQueryParamMatchInput>> for HttpRequestQueryParamMatchInputMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestQueryParamMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpRequestQueryParamMatchInputMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpRequestQueryParamMatchInput> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpRequestQueryParamMatchInput {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // query_param: optional string
  pub fn query_param(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_query_param(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `HttpRequestQueryParamMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpRequestQueryParamMatchInputMut<'_> {}

// SAFETY:
// - `HttpRequestQueryParamMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpRequestQueryParamMatchInputMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpRequestQueryParamMatchInputMut<'msg> {
  type Proxied = HttpRequestQueryParamMatchInput;
  fn as_view(&self) -> ::protobuf::View<'_, HttpRequestQueryParamMatchInput> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpRequestQueryParamMatchInputMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpRequestQueryParamMatchInput>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpRequestQueryParamMatchInputMut<'msg> {
  type MutProxied = HttpRequestQueryParamMatchInput;
  fn as_mut(&mut self) -> HttpRequestQueryParamMatchInputMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpRequestQueryParamMatchInputMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpRequestQueryParamMatchInputMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpRequestQueryParamMatchInput {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpRequestQueryParamMatchInput> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpRequestQueryParamMatchInputView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpRequestQueryParamMatchInputMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // query_param: optional string
  pub fn query_param(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_query_param(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl HttpRequestQueryParamMatchInput

impl ::std::ops::Drop for HttpRequestQueryParamMatchInput {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpRequestQueryParamMatchInput {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpRequestQueryParamMatchInput {
  type Proxied = Self;
  fn as_view(&self) -> HttpRequestQueryParamMatchInputView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpRequestQueryParamMatchInput {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpRequestQueryParamMatchInputMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpRequestQueryParamMatchInput {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__HttpRequestQueryParamMatchInput_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__HttpRequestQueryParamMatchInput_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__HttpRequestQueryParamMatchInput_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRequestQueryParamMatchInput {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRequestQueryParamMatchInput {
  type Msg = HttpRequestQueryParamMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestQueryParamMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestQueryParamMatchInput {
  type Msg = HttpRequestQueryParamMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestQueryParamMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpRequestQueryParamMatchInputMut<'_> {
  type Msg = HttpRequestQueryParamMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestQueryParamMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestQueryParamMatchInputMut<'_> {
  type Msg = HttpRequestQueryParamMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestQueryParamMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpRequestQueryParamMatchInputView<'_> {
  type Msg = HttpRequestQueryParamMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpRequestQueryParamMatchInput> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpRequestQueryParamMatchInputMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



